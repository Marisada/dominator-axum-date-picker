pub use time_datepicker_core::{
    config::{PickerConfigBuilder, date_constraints::DateConstraintsBuilder},
    dialog_view_type::DialogViewType,
};

use dominator::{Dom, DomBuilder, clone, html, window_offset, window_size};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use time_datepicker_core::config::{PickerConfig, date_constraints::DateConstraints};
use web_sys::{DomRect, HtmlElement, HtmlInputElement};

use super::datetime_pickers;

/// `DateTime` input with picker from `Mutable<String>`, update `Mutable<bool>` at the end of input/exit<br>
/// - `disable_signal`: ex. `futures_signal::signal::always(false)`
/// - `container_mixin`: ex. `|dom| dom.style("min-width","190px")`, `NOTE`: sm is `175px`)
/// - `label_mixin`: apply to label element, ex. `|dom| dom.class("form-control-sm")`
/// - `input_mixin`: apply to input element, ex. `|dom| dom.class("form-control-sm")`
///
/// `NOTE`: under `input-group` parent
/// - use `d-flex` and `flex-grow-1` classes to `container_mixin`
/// - use `rounded-0` or `rounded-start-0` or `rounded-end-0` class to `label_mixin` and `input_mixin`
pub fn datetime_picker<B, C, D, F, S, T>(
    datetime_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,
    disable_signal: S,
    container_mixin: B,
    label_mixin: C,
    input_mixin: D,
    update_fn: F,
    config_signal: T,
) -> Dom
where
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + Clone + 'static,
    F: Fn(String) -> String + Clone + 'static,
    S: Signal<Item = bool> + 'static,
    T: Signal<Item = Option<PickerConfig<DateConstraints>>> + 'static,
{
    datetime_pickers::datetime_input_with_picker(
        datetime_pickers::Picker::DateTime,
        datetime_mutable,
        changed_mutable,
        disable_signal,
        None,
        container_mixin,
        label_mixin,
        input_mixin,
        update_fn,
        config_signal,
    )
}

/// `Date` input with picker from `Mutable<String>`, update `Mutable<bool>` at the end of input/exit<br>
/// - `disable_signal`: ex. `futures_signal::signal::always(false)`
/// - `paired_mutable`: mutable of paired `Time` for calculate the same constrain
/// - `container_mixin`: ex. `|dom| dom.style("min-width","135px")`, `NOTE`: sm is `120px`)
/// - `label_mixin`: apply to label element, ex. `|dom| dom.class("form-control-sm")`
/// - `input_mixin`: apply to input element, ex. `|dom| dom.class("form-control-sm")`
///
/// `NOTE`: under `input-group` parent
/// - use `d-flex` and `flex-grow-1` classes to `container_mixin`
/// - use `rounded-0` or `rounded-start-0` or `rounded-end-0` class to `label_mixin` and `input_mixin`
pub fn date_picker<B, C, D, F, S, T>(
    date_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,
    disable_signal: S,
    paired_mutable: Option<Mutable<String>>,
    container_mixin: B,
    label_mixin: C,
    input_mixin: D,
    update_fn: F,
    config_signal: T,
) -> Dom
where
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + Clone + 'static,
    F: Fn(String) -> String + Clone + 'static,
    S: Signal<Item = bool> + 'static,
    T: Signal<Item = Option<PickerConfig<DateConstraints>>> + 'static,
{
    datetime_pickers::datetime_input_with_picker(
        datetime_pickers::Picker::Date,
        date_mutable,
        changed_mutable,
        disable_signal,
        paired_mutable,
        container_mixin,
        label_mixin,
        input_mixin,
        update_fn,
        config_signal,
    )
}

/// `Time` input with picker from `Mutable<String>`, update `Mutable<bool>` at the end of input/exit<br>
/// - `disable_signal`: ex. `futures_signal::signal::always(false)`
/// - `paired_mutable`: mutable of paired `Date` for calculate the same constrain
/// - `container_mixin`: ex. `|dom| dom.style("min-width","110px")`, `NOTE`: sm is `95px`)
/// - `label_mixin`: apply to label element, ex. `|dom| dom.class("form-control-sm")`
/// - `input_mixin`: apply to input element, ex. `|dom| dom.class("form-control-sm")`
///
/// `NOTE`: under `input-group` parent
/// - use `d-flex` and `flex-grow-1` classes to `container_mixin`
/// - use `rounded-0` or `rounded-start-0` or `rounded-end-0` class to `label_mixin` and `input_mixin`
pub fn time_picker<B, C, D, F, S, T>(
    time_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,
    disable_signal: S,
    paired_mutable: Option<Mutable<String>>,
    container_mixin: B,
    label_mixin: C,
    input_mixin: D,
    update_fn: F,
    config_signal: T,
) -> Dom
where
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + Clone + 'static,
    F: Fn(String) -> String + Clone + 'static,
    S: Signal<Item = bool> + 'static,
    T: Signal<Item = Option<PickerConfig<DateConstraints>>> + 'static,
{
    datetime_pickers::datetime_input_with_picker(
        datetime_pickers::Picker::Time,
        time_mutable,
        changed_mutable,
        disable_signal,
        paired_mutable,
        container_mixin,
        label_mixin,
        input_mixin,
        update_fn,
        config_signal,
    )
}

/// Box that will `fixed` appear under another `box with id`
pub fn under_box<F>(
    anchor_rect: DomRect,
    max_width: f64,
    max_height: f64,
    page_y_offset: f64,
    mixins: F,
) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class(["m-0","p-0"])
        .style("position","fixed")
        .style("z-index","3")
        .style_signal("width", window_size().map(move |ws| {
            let width = if ws.width < max_width {ws.width} else {max_width};
            [&width.to_string(),"px"].concat()
        }))
        .style_signal("left", map_ref!{
            let size = window_size(),
            let offset = window_offset() =>
            (*size, *offset)
        }.map(clone!(anchor_rect => move |(ws, wo)| {
            // we assume that starting window.scrollX = 0
            let est_left = anchor_rect.left();
            let width = if ws.width < max_width {ws.width} else {max_width};
            let left = if (est_left + width) < ws.width {
                est_left - wo.x
            } else {
                ws.width - width
            };
            [&left.to_string(),"px"].concat()
        })))
        .style_signal("top", map_ref!{
            let size = window_size(),
            let offset = window_offset() =>
            (*size, *offset)
        }.map(clone!(anchor_rect => move |(ws, wo)| {
            let y_diff = page_y_offset - wo.y;
            let anchor_top = anchor_rect.top();
            let anchor_bottom = anchor_rect.bottom();
            let top = if (anchor_bottom + max_height) < ws.height {
                anchor_bottom + y_diff
            } else if anchor_top > max_height {
                anchor_top - max_height + y_diff
            } else {
                0.0
            };
            [&top.to_string(),"px"].concat()
        })))
        .apply(mixins)
    })
}
