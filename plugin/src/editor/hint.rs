//! Tooltips for whole parameter rows (the label and its slider), with a choice of where the tooltip
//! appears.
//!
//! VIZIA's own `.tooltip()` modifier always places the box below the control, which runs off the
//! bottom of the window for the lowest rows, and it can only be attached to a single control. A row
//! here shows one tooltip whether you hover its label or its slider, and the bottom rows show it
//! above instead of below.

use nih_plug::prelude::*;
use nih_plug_vizia::vizia::modifiers::{TooltipEvent, TooltipModel};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;

/// Where a tooltip appears relative to its row.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TipPlacement {
    Below,
    Above,
}

// The row's tooltip state is VIZIA's own `TooltipModel`, not a private one: the `Tooltip` view
// installs a listener that reads `TooltipModel` on every event, so with any other model above it,
// the first event the window receives panics ("Failed to get data from context").

/// Makes hovering over `handle` show its row's tooltip. Call it on every control in a row that
/// should trigger it.
pub fn hover_tip<V: View>(handle: Handle<'_, V>) -> Handle<'_, V> {
    handle
        .on_hover(|cx| cx.emit(TooltipEvent::ShowTooltip))
        .on_hover_out(|cx| cx.emit(TooltipEvent::HideTooltip))
}

/// A row with a tooltip. `content` builds the row's controls; pass each of them through
/// [`hover_tip()`] so hovering them shows the tooltip.
pub fn hinted_row<'a>(
    cx: &'a mut Context,
    text: Option<&'static str>,
    placement: TipPlacement,
    content: impl FnOnce(&mut Context),
) -> Handle<'a, HStack> {
    HStack::new(cx, move |cx| {
        TooltipModel {
            tooltip_visible: false,
        }
        .build(cx);
        content(cx);

        if let Some(text) = text {
            let tooltip = Tooltip::new(cx, move |cx| {
                Label::new(cx, text);
            })
            .toggle_class("vis", TooltipModel::tooltip_visible);

            if placement == TipPlacement::Above {
                // Sit just above the row. The box's height is not known when it is built, so it
                // is estimated from the text: the label is 232 px wide and wraps at roughly 36
                // characters a line, with a line about 15 px high plus the box's padding.
                let lines = text.len().div_ceil(36) as f32;
                let height = lines * 15.0 + 14.0;
                tooltip
                    .top(Pixels(-(height + 6.0)))
                    .translate((Pixels(0.0), Pixels(0.0)));
            }
        }
    })
}

/// The same slider `GenericUi` would draw for `param_ptr`, but returning its handle.
pub fn draw_param<L, PsRef, Ps>(
    cx: &mut Context,
    params: L,
    param_ptr: ParamPtr,
) -> Handle<'_, ParamSlider>
where
    L: Lens<Target = PsRef>,
    PsRef: AsRef<Ps> + 'static,
    Ps: Params + 'static,
{
    unsafe {
        match param_ptr {
            ParamPtr::FloatParam(p) => ParamSlider::new(cx, params, move |_| &*p),
            ParamPtr::IntParam(p) => ParamSlider::new(cx, params, move |_| &*p),
            ParamPtr::BoolParam(p) => ParamSlider::new(cx, params, move |_| &*p),
            ParamPtr::EnumParam(p) => ParamSlider::new(cx, params, move |_| &*p),
        }
    }
    .set_style(match unsafe { param_ptr.step_count() } {
        Some(step_count) if step_count <= 1 => ParamSliderStyle::CurrentStepLabeled { even: true },
        Some(step_count) if step_count <= 2 => ParamSliderStyle::CurrentStep { even: true },
        Some(_) => ParamSliderStyle::FromLeft,
        None => ParamSliderStyle::Centered,
    })
    .class("widget")
}

/// A label and its slider as one row, with a tooltip.
pub fn param_row<L, PsRef, Ps>(
    cx: &mut Context,
    label: &str,
    params: L,
    param_ptr: ParamPtr,
    text: Option<&'static str>,
    placement: TipPlacement,
) where
    L: Lens<Target = PsRef>,
    PsRef: AsRef<Ps> + 'static,
    Ps: Params + 'static,
{
    let label = label.to_owned();
    hinted_row(cx, text, placement, move |cx| {
        hover_tip(Label::new(cx, label.as_str()).class("label"));
        hover_tip(draw_param(cx, params, param_ptr));
    })
    .class("row");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_large_stack(test: impl FnOnce() + Send + 'static) {
        std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .spawn(test)
            .expect("failed to spawn test thread")
            .join()
            .expect("test thread panicked");
    }

    #[test]
    fn a_row_with_a_tooltip_builds_below_and_above() {
        with_large_stack(|| {
            let cx = &mut Context::default();
            for placement in [TipPlacement::Below, TipPlacement::Above] {
                hinted_row(cx, Some("A tooltip."), placement, |cx| {
                    hover_tip(Label::new(cx, "Label"));
                })
                .class("row");
            }
            hinted_row(cx, None, TipPlacement::Below, |_| {});
        });
    }
}
