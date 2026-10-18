use std::path::PathBuf;
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use nih_plug_vizia::vizia::prelude::*;

use crate::frozen_ir::{write_frozen_ir_wav, FrozenIrData};

/// Progress of an IR export, sent to the editor so it can show it in the status label. The export
/// runs on a worker task, so without this a failed write (disk full, no permission, a missing
/// file-dialog portal) would look like nothing happened.
#[derive(Debug)]
pub enum ExportIrEvent {
    /// The save dialog is about to open.
    Started,
    /// The export ended: the path that was written, `None` if the user cancelled, or the error.
    Finished(Result<Option<PathBuf>, String>),
}

/// The status label text for a finished export.
pub fn export_status_text(result: &Result<Option<PathBuf>, String>) -> String {
    match result {
        Ok(Some(path)) => {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            format!("Saved IR: {name}")
        }
        Ok(None) => String::new(),
        Err(err) => format!("IR export failed: {err}"),
    }
}

/// The suggested file name. The exported IR is centred, so its peak is `window_size` samples in;
/// putting that in the name saves the user from having to find it when aligning the IR.
fn default_file_name(snapshot: &FrozenIrData) -> String {
    format!(
        "polarity-sc-dark-freeze-ir-peak-{}smp.wav",
        snapshot.window_size
    )
}

/// Exports the current frozen compressor snapshot as a WAV IR file.
pub struct ExportIrButton {
    frozen_ir_data: Arc<AtomicRefCell<triple_buffer::Output<FrozenIrData>>>,
}

impl ExportIrButton {
    pub fn new<LData, T>(
        cx: &mut Context,
        frozen_ir_lens: LData,
        label: impl Res<T> + Clone,
    ) -> Handle<'_, Self>
    where
        LData: Lens<Target = Arc<AtomicRefCell<triple_buffer::Output<FrozenIrData>>>>,
        T: ToString,
    {
        Self {
            frozen_ir_data: frozen_ir_lens.get(cx),
        }
        .build(cx, |cx| {
            Label::new(cx, label).hoverable(false);
        })
        .class("editor-mode")
    }
}

impl View for ExportIrButton {
    fn element(&self) -> Option<&'static str> {
        Some("param-button")
    }

    /// Starts the export when the button is clicked. The file dialog and the actual
    /// WAV write happen on a worker thread so the editor stays responsive.
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseDoubleClick(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                meta.consume();
                if cx.is_disabled() {
                    return;
                }

                start_export(cx, self.frozen_ir_data.clone());
            }
            _ => {}
        });
    }
}

/// Grabs the latest valid frozen snapshot and kicks off the async export. The outcome comes back
/// as an [`ExportIrEvent::Finished`].
fn start_export(
    cx: &mut EventContext,
    frozen_ir_data: Arc<AtomicRefCell<triple_buffer::Output<FrozenIrData>>>,
) {
    let snapshot = if let Ok(mut frozen_ir_data) = frozen_ir_data.try_borrow_mut() {
        frozen_ir_data.read().clone()
    } else {
        return;
    };
    if !snapshot.valid {
        return;
    }

    cx.emit(ExportIrEvent::Started);
    cx.spawn(move |proxy| {
        let result = export_current_snapshot(snapshot);
        let _ = proxy.emit(ExportIrEvent::Finished(result));
    });
}

/// Opens a save dialog and writes the passed frozen snapshot to a WAV file. Returns
/// the chosen path on success, `None` when the user cancels, or an error message on
/// failure.
fn export_current_snapshot(snapshot: FrozenIrData) -> Result<Option<PathBuf>, String> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Wave", &["wav"])
        .set_file_name(default_file_name(&snapshot))
        .save_file()
    else {
        return Ok(None);
    };

    write_frozen_ir_wav(&snapshot, &path)?;
    Ok(Some(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn snapshot(window_size: usize) -> FrozenIrData {
        FrozenIrData {
            valid: true,
            sample_rate: 48_000.0,
            window_size,
            overlap_times: 4,
            num_channels: 1,
            ..FrozenIrData::default()
        }
    }

    #[test]
    fn a_failed_write_reaches_the_status_label() {
        let path = Path::new("/definitely/not/a/real/dir/ir.wav");
        let result = write_frozen_ir_wav(&snapshot(64), path).map(|()| Some(path.to_owned()));

        assert!(result.is_err(), "writing into a missing folder must fail");
        let text = export_status_text(&result);
        assert!(text.starts_with("IR export failed: "), "{text}");
        assert!(text.contains("ir.wav"), "{text}");
    }

    #[test]
    fn success_names_the_file_and_cancel_clears_the_label() {
        let saved = Ok(Some(PathBuf::from("/some/dir/freeze.wav")));
        assert_eq!(export_status_text(&saved), "Saved IR: freeze.wav");
        assert_eq!(export_status_text(&Ok(None)), "");
    }

    #[test]
    fn default_file_name_carries_the_peak_offset() {
        assert_eq!(
            default_file_name(&snapshot(2048)),
            "polarity-sc-dark-freeze-ir-peak-2048smp.wav"
        );
    }
}
