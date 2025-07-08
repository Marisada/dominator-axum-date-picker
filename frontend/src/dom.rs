use dominator::{clone, events, html, Dom, with_node, traits::MultiStr};
use futures_signals::signal::{Mutable, SignalExt};
use time_datepicker_core::config::{date_constraints::DateConstraints, PickerConfig, PickerConfigBuilder};
use web_sys::HtmlInputElement;

use picker_util::{
    date_8601, date_from_pat, date_pat, date_str_th, datetime_8601, datetime_from_pat, datetime_pat, datetime_str_th, js_now, time_8601, time_from_pat, time_pat, time_str_hm, JsTime
};

use crate::date_picker::DatePicker;

pub fn datetime_input_with_picker<B,C> (date_mutable: Mutable<String>, container_class: B, input_class: C) -> Dom
where 
    B: MultiStr,
    C: MultiStr + Clone,
{
    datetime_with_picker(true, date_mutable, container_class, input_class)
}

pub fn date_input_with_picker<B,C> (date_mutable: Mutable<String>, container_class: B, input_class: C) -> Dom
where 
    B: MultiStr,
    C: MultiStr + Clone,
{
    datetime_with_picker(false, date_mutable, container_class, input_class)
}

fn datetime_with_picker<B,C> (with_time: bool, date_mutable: Mutable<String>, container_class: B, input_class: C) -> Dom
where 
    B: MultiStr,
    C: MultiStr + Clone,
{
    let date_active = Mutable::new(false);
    let picker = Mutable::new(None);

    html!("div", {
        .class(container_class)
        .class("position-relative")
        .children([
            html!("div", {
                .class("form-control")
                .class(input_class.clone())
                .style("pointer-events", "none")
                .style("position", "absolute")
                .style("height", "100%")
                .style_signal("z-index", date_active.signal().map(|is_active| if is_active {"-1"} else {"1"}))
                .text_signal(date_mutable.signal_cloned().map(move |s| {
                    if with_time {
                        datetime_str_th(&s)
                    } else {
                        date_str_th(&s)
                    }
                }))
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class("form-control")
                .class(input_class)
                .attr("placeholder", if with_time {"เช่น 31/8/68 23:45"} else {"เช่น 31/8/68"})
                .attr("maxlength", if with_time {"16"} else {"10"})
                .prop_signal("value", date_mutable.signal_cloned().map(move |s| {
                    if with_time {
                        datetime_8601(&s).map(|dt| datetime_pat(&dt)).unwrap_or_default()
                    } else {
                        date_8601(&s).map(|d| date_pat(&d)).unwrap_or_default()
                    }
                }))
                .with_node!(element => {
                    .event(clone!(date_mutable => move |_:events::Change| {
                        let v = element.value();
                        let value = if with_time {
                            datetime_from_pat(&v).map(|dt| dt.js_string()).unwrap_or_default()
                        } else {
                            date_from_pat(&v).map(|d| d.to_string()).unwrap_or_default()
                        };
                        date_mutable.set(value);
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
                .attr("title", if with_time {"แสดงเครื่องมือเลือกวันที่และเวลา"} else {"แสดงเครื่องมือเลือกวันที่"})
                .with_node!(element => {
                    .event(clone!(date_mutable, picker => move |_:events::Click| {
                        if picker.get_cloned().is_none() {
                            let parent_position = element.parent_element().unwrap().get_bounding_client_rect();
                            let new_picker = if with_time {
                                DatePicker::new_datetime(date_mutable.clone(), picker.clone(), parent_position, PickerConfig::<DateConstraints>::default())
                            } else {
                                DatePicker::new_date(date_mutable.clone(), picker.clone(), parent_position, PickerConfig::<DateConstraints>::default())
                            };
                            picker.set(Some(new_picker));
                        } else {
                            picker.set(None);
                        }
                    }))
                })
            }),
        ])
        .child_signal(picker.signal_cloned().map(|opt| {
            opt.map(|picker| DatePicker::render(picker))
        }))
    })
}

pub fn time_input_with_picker<B,C> (time_mutable: Mutable<String>, container_class: B, input_class: C) -> Dom
where 
    B: MultiStr,
    C: MultiStr + Clone,
{
    let time_active = Mutable::new(false);
    let picker = Mutable::new(None);

    html!("div", {
        .class(container_class)
        .class("position-relative")
        .children([
            html!("div", {
                .class("form-control")
                .class(input_class.clone())
                .style("pointer-events", "none")
                .style("position", "absolute")
                .style("height", "100%")
                .style_signal("z-index", time_active.signal().map(|is_active| if is_active {"-1"} else {"1"}))
                .text_signal(time_mutable.signal_cloned().map(move |s| time_str_hm(&s)))
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class("form-control")
                .class(input_class)
                .attr("placeholder", "เช่น 23:45")
                .attr("maxlength","5")
                .prop_signal("value", time_mutable.signal_cloned().map(|s| {
                    time_8601(&s).map(|t| time_pat(&t)).unwrap_or_default()
                }))
                .with_node!(element => {
                    .event(clone!(time_mutable => move |_:events::Change| {
                        let v = time_from_pat(&element.value()).map(|t| t.js_string()).unwrap_or_default();
                        time_mutable.set(v);
                    }))
                })
                .event(clone!(time_active => move |_:events::Focus| {
                    time_active.set(true)
                }))
                .event(clone!(time_active => move |_:events::Blur| {
                    time_active.set(false)
                }))
            }),
            html!("i", {
                .class(["far","fa-clock"])
                .style("position", "absolute")
                .style("top", "calc(50% - 13px)")
                .style("right", "5px")
                .style("padding", "5px 10px")
                .style("opacity","75%")
                .style("color", "var(--bs-body-color)")
                .style("z-index","2")
                .attr("title", "แสดงเครื่องมือเลือกเวลา")
                .with_node!(element => {
                    .event(clone!(time_mutable, picker => move |_:events::Click| {
                        if picker.get_cloned().is_none() {
                            let parent_position = element.parent_element().unwrap().get_bounding_client_rect();
                            let config = PickerConfigBuilder::<DateConstraints>::default()
                                .initial_time(js_now().time())
                                .build()
                                .unwrap_or_default();
                            let new_picker = DatePicker::new_time(time_mutable.clone(), picker.clone(), parent_position, config);
                            picker.set(Some(new_picker));
                        } else {
                            picker.set(None);
                        }
                    }))
                })
            }),
        ])
        .child_signal(picker.signal_cloned().map(|opt| {
            opt.map(|picker| DatePicker::render(picker))
        }))
    })
}