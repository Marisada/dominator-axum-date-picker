use dominator::{clone, events, html, Dom, with_node, traits::MultiStr};
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlInputElement;

use crate::{
    date_picker::{DatePicker, DateConstraints, PickerConfig},
    util::{JsTime, date_str_th, date_8601, date_pat, date_from_pat ,datetime_str_th, datetime_8601, datetime_pat, datetime_from_pat},
};

pub fn date_input_with_picker<B> (date_mutable: Mutable<String>, container_class: B) -> Dom
where 
    B: MultiStr 
{
    let date_active = Mutable::new(false);
    let picker = Mutable::new(None);

    html!("div", {
        .class(container_class)
        .class("position-relative")
        .children([
            html!("div", {
                .class("form-control")
                .style("pointer-events", "none")
                .style("position", "absolute")
                .style("height", "100%")
                .style_signal("z-index", date_active.signal().map(|is_active| if is_active {"-1"} else {"1"}))
                .text_signal(date_mutable.signal_cloned().map(|s| {
                    date_str_th(&s)
                }))
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class("form-control")
                .attr("placeholder", "DD/MM/YYYY")
                .attr("maxlength", "10")
                .prop_signal("value", date_mutable.signal_cloned().map(|s| {
                    if let Some(d) = date_8601(&s) {
                        date_pat(&d)
                    } else {
                        String::new()
                    }
                }))
                .with_node!(element => {
                    .event(clone!(date_mutable => move |_:events::Change| {
                        let v = if let Some(d) = date_from_pat(&element.value()) {
                            d.to_string()
                        } else {
                            String::new()
                        };
                        date_mutable.set(v);
                    }))
                })
                .event(clone!(date_active => move |_:events::Focus| {
                    date_active.set(true)
                }))
                .event(clone!(date_active => move |_:events::Blur| {
                    date_active.set(false)
                }))
            }),
            html!("i", {
                .class(["far","fa-calendar"])
                .style("position", "absolute")
                .style("top", "calc(50% - 13px)")
                .style("right", "5px")
                .style("padding", "5px 10px")
                .style("opacity","75%")
                .style("color", "var(--bs-body-color)")
                .style("z-index","2")
                .attr("title", "แสดงเครื่องมือเลือกวันที่")
                .event(clone!(date_mutable, picker => move |_:events::Click| {
                    if picker.get_cloned().is_none() {
                        let new_picker = DatePicker::new(date_mutable.clone(), picker.clone(), PickerConfig::<DateConstraints>::default());
                        picker.set(Some(new_picker));
                    } else {
                        picker.set(None);
                    }
                }))
            }),
        ])
        .child_signal(picker.signal_cloned().map(|opt| {
            opt.map(|picker| DatePicker::render(picker))
        }))
    })
}


pub fn datetime_input_with_picker<B> (datetime_mutable: Mutable<String>, container_class: B) -> Dom
where 
    B: MultiStr 
{
    let datetime_active = Mutable::new(false);
    html!("div", {
        .class(container_class)
        .class("position-relative")
        .children([
            html!("div", {
                .class(["form-control", "form-control-sm"])
                .style("pointer-events", "none")
                .style("position", "absolute")
                .style("height", "100%")
                .style_signal("z-index", datetime_active.signal().map(|is_active| if is_active {"-1"} else {"1"}))
                .text_signal(datetime_mutable.signal_cloned().map(|s| {
                    datetime_str_th(&s)
                }))
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class(["form-control", "form-control-sm"])
                .attr("placeholder", "DD/MM/YYYY HH:MM")
                .attr("maxlength", "16")
                .prop_signal("value", datetime_mutable.signal_cloned().map(|s| {
                    if let Some(dt) = datetime_8601(&s) {
                        datetime_pat(&dt)
                    } else {
                        String::new()
                    }
                }))
                .with_node!(element => {
                    .event(clone!(datetime_mutable => move |_:events::Change| {
                        let v = if let Some(dt) = datetime_from_pat(&element.value()) {
                            dt.js_string()
                        } else {
                            String::new()
                        };
                        datetime_mutable.set(v);
                    }))
                })
                .event(clone!(datetime_active => move |_:events::Focus| {
                    datetime_active.set(true)
                }))
                .event(clone!(datetime_active => move |_:events::Blur| {
                    datetime_active.set(false)
                }))
            }),
            html!("i", {
                .class(["far","fa-calendar"])
                .style("position", "absolute")
                .style("top", "calc(50% - 13px)")
                .style("right", "5px")
                .style("padding", "5px 10px")
                .style("opacity","75%")
                .style("color", "var(--bs-body-color)")
                .style("z-index","2")
                .attr("title", "แสดงเครื่องมือเลือกวันที่และเวลา")
                .event(clone!(datetime_active => move |_:events::Click| {
                    // TODO
                }))
            }),
        ])
    })
}