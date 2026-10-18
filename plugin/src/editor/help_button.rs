//! The HELP button: opens the manual that ships inside the plug-in.
//!
//! The manual is written in `docs/MANUAL.md`, turned into one self-contained web page by
//! `docs/gen_manual.py`, and embedded below, so it works offline and cannot get lost or go out of
//! step with the version you are running. Clicking HELP writes the page to a temporary folder and
//! asks the operating system to open it.
//!
//! Two things in here are deliberate, and each was a real bug in other products of this family:
//!
//! 1. **The operating system is handed a plain file path, never a `file:` URL.** On macOS a
//!    hand-built `file:` URL gets read as a relative path and opens nothing, and on every system
//!    the platform escapes odd characters (spaces, accents) in a path by itself.
//! 2. **Nothing in the click handler is allowed to take the host down.** The extraction and the
//!    open run on a worker thread, behind `catch_unwind`, and every failure turns into a message
//!    in the editor's status label instead of a panic.

use std::io;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use nih_plug_vizia::vizia::prelude::*;

/// The generated manual page. Rebuild it with `python3 docs/gen_manual.py` after editing
/// `docs/MANUAL.md`; CI refuses a commit where the two disagree.
pub const MANUAL_HTML: &[u8] = include_bytes!("../../../docs/MANUAL.html");

/// Sent to the editor when a click on HELP has finished, so it can report a failure.
#[derive(Debug)]
pub enum HelpEvent {
    Finished(Result<PathBuf, String>),
}

/// The status label text for a finished HELP click. A success needs no message: the manual is
/// already open in front of the user.
pub fn help_status_text(result: &Result<PathBuf, String>) -> String {
    match result {
        Ok(_) => String::new(),
        Err(err) => format!("Could not open the manual: {err}"),
    }
}

/// A button that opens the manual.
pub struct HelpButton;

impl HelpButton {
    pub fn new<T>(cx: &mut Context, label: impl Res<T> + Clone) -> Handle<'_, Self>
    where
        T: ToString,
    {
        Self.build(cx, |cx| {
            Label::new(cx, label).hoverable(false);
        })
        .class("editor-mode")
    }
}

impl View for HelpButton {
    fn element(&self) -> Option<&'static str> {
        Some("param-button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseDoubleClick(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                meta.consume();
                if cx.is_disabled() {
                    return;
                }

                // Off the UI thread: on Windows `open::that` ends up in `ShellExecuteW`, which can
                // hang or crash when called from a plug-in host's UI thread.
                cx.spawn(|proxy| {
                    let result = panic::catch_unwind(AssertUnwindSafe(open_manual))
                        .unwrap_or_else(|_| Err(String::from("something went wrong")));
                    let _ = proxy.emit(HelpEvent::Finished(result));
                });
            }
            _ => {}
        });
    }
}

/// Writes the manual to the temporary folder and opens it in the default browser.
fn open_manual() -> Result<PathBuf, String> {
    let dir = std::env::temp_dir().join("Slackwater");
    let path = write_manual(&dir).map_err(|err| format!("{}: {err}", dir.display()))?;

    // A plain path, see the module docs.
    open::that(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    Ok(path)
}

/// Writes the embedded manual into `dir` (creating it) and returns the file's path. It is written
/// afresh on every call, which is cheap and means a half-deleted temporary folder cannot leave a
/// broken file behind.
pub fn write_manual(dir: &Path) -> io::Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    let path = dir.join(format!(
        "Slackwater-Manual-{}.html",
        env!("CARGO_PKG_VERSION")
    ));
    std::fs::write(&path, MANUAL_HTML)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("slackwater-help-{label}-{nanos}"))
    }

    #[test]
    fn the_manual_is_written_byte_for_byte_to_a_plain_path() {
        let dir = temp_dir("plain");
        let path = write_manual(&dir).unwrap();

        assert_eq!(std::fs::read(&path).unwrap(), MANUAL_HTML);
        assert!(path.is_absolute());
        assert!(path.extension().is_some_and(|ext| ext == "html"));
        // A path the OS can open as it is, not a URL.
        assert!(!path.to_string_lossy().starts_with("file:"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn folders_with_spaces_and_accents_work() {
        let dir = temp_dir("sp ace é ü 空").join("Slackwater Manual");
        let path = write_manual(&dir).unwrap();

        assert_eq!(std::fs::read(&path).unwrap(), MANUAL_HTML);
        let _ = std::fs::remove_dir_all(dir.parent().unwrap());
    }

    #[test]
    fn writing_twice_replaces_the_file_and_a_missing_folder_is_recreated() {
        let dir = temp_dir("twice");
        let first = write_manual(&dir).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        let second = write_manual(&dir).unwrap();

        assert_eq!(first, second);
        assert_eq!(std::fs::read(&second).unwrap(), MANUAL_HTML);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_folder_that_cannot_be_created_is_an_error_not_a_panic() {
        // A file where the folder should be.
        let blocker = temp_dir("blocker");
        std::fs::write(&blocker, b"not a folder").unwrap();

        assert!(write_manual(&blocker.join("inside")).is_err());
        let _ = std::fs::remove_file(blocker);
    }

    #[test]
    fn status_text_is_silent_on_success_and_explains_a_failure() {
        assert_eq!(help_status_text(&Ok(PathBuf::from("/tmp/x.html"))), "");
        let text = help_status_text(&Err(String::from("no browser")));
        assert_eq!(text, "Could not open the manual: no browser");
    }

    #[test]
    fn the_manual_documents_every_parameter_a_host_can_see() {
        use crate::SpectralCompressor;
        use nih_plug::prelude::*;

        std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .spawn(|| {
                let html = std::str::from_utf8(MANUAL_HTML).unwrap();
                let plugin = SpectralCompressor::default();
                let missing: Vec<String> = plugin
                    .params()
                    .param_map()
                    .into_iter()
                    .filter(|(_, ptr, _)| !unsafe { ptr.flags() }.contains(ParamFlags::HIDDEN))
                    .map(|(_, ptr, _)| unsafe { ptr.name() }.to_owned())
                    .filter(|name| !html.contains(name.as_str()))
                    .collect();

                assert!(
                    missing.is_empty(),
                    "parameters the manual never mentions: {missing:?}"
                );
            })
            .unwrap()
            .join()
            .expect("test thread panicked");
    }

    #[test]
    fn the_embedded_manual_is_a_web_page_for_this_version() {
        let html = std::str::from_utf8(MANUAL_HTML).expect("the manual must be UTF-8");

        assert!(html
            .trim_start()
            .to_lowercase()
            .starts_with("<!doctype html>"));
        assert!(
            html.contains(&format!("Version {}", env!("CARGO_PKG_VERSION"))),
            "docs/MANUAL.html is for a different version than Cargo.toml; \
            run `python3 docs/gen_manual.py`"
        );
    }
}
