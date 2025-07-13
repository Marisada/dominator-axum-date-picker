use dominator::{Dom, clone, events, html, with_node};
use futures_signals::signal::{Mutable, SignalExt, always, not};
use time::{Date, Duration, PrimitiveDateTime, Time, Weekday};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

mod datetime_pickers;
mod doms;
mod picker;

use picker_util::{JsTime, class, date_8601, datetime_8601, datetime_th, js_now, time_8601};

#[wasm_bindgen(start)]
pub fn main_js() {
    wasm_logger::init(wasm_logger::Config::default());

    // std::panic::set_hook(Box::new(on_panic));
    console_error_panic_hook::set_once();
    log::info!("wasm logging enabled");

    dominator::append_dom(&dominator::get_id("app"), render());
}

fn render() -> Dom {
    let is_dark_mutable = Mutable::new(false);
    let disable_mutable = Mutable::new(false);
    let changed_mutable = Mutable::new(false);
    let config_label_mutable = Mutable::new(String::from("ไม่มีเงื่อนไข"));
    let config_mutable = Mutable::new(None);

    let date_mutable = Mutable::new(String::new());
    let time_mutable = Mutable::new(String::new());
    let datetime_mutable = Mutable::new(String::new());

    let now = js_now();
    let now_datetime = PrimitiveDateTime::new(
        now.date(),
        Time::from_hms(now.hour(), now.minute(), 0).unwrap(),
    );
    let min_datetime_label =
        ["เงื่อนไข ไม่ก่อน ", &datetime_th(&now_datetime), " และไม่ใช่วันพุธ"].concat();
    let max_datetime_label = [
        "เงื่อนไข ไม่หลัง ",
        &datetime_th(&now_datetime),
        " และไม่ใช่วันที่ 1,9,17,25",
    ]
    .concat();

    html!("div", {
        .attr_signal("data-bs-theme", is_dark_mutable.signal().map(|is_dark| if is_dark {"dark"} else {"light"}))
        .class("p-3")
        .style("background-color", "var(--bs-body-bg)")
        .style("color", "var(--bs-body-color)")
        .child(html!("div", {
            .class(["container-fluid","p-0"])
            .style("max-width","500px")
            .text("Hello World !!!")
            .child(html!("div", {
                .class(["row","mb-3"])
                .child(html!("a", {
                    .attr("href","api/greet")
                    .attr("target","_blank")
                    .text("Greeting page")
                }))
            }))
            .child(html!("div", {
                .class(["d-flex","sticky-top","p-2","border","rounded","justify-content-center"])
                .style("background-color", "var(--bs-body-bg)")
                .children([
                    html!("button", {
                        .attr("type", "button")
                        .class(["btn","btn-primary","me-1"])
                        .text("Now")
                        .event(clone!(date_mutable, time_mutable, datetime_mutable => move |_:events::Click| {
                            let now = js_now();
                            date_mutable.set(now.date().to_string());
                            time_mutable.set(now.time().js_string());
                            datetime_mutable.set(now.js_string());
                        }))
                    }),
                    html!("button", {
                        .attr("type", "button")
                        .class(["btn","btn-primary","me-1"])
                        .text("Clear")
                        .event(clone!(date_mutable, time_mutable, datetime_mutable => move |_:events::Click| {
                            date_mutable.set(String::new());
                            time_mutable.set(String::new());
                            datetime_mutable.set(String::new());
                        }))
                    }),
                    html!("button", {
                        .attr("type", "button")
                        .class(["btn","me-1"])
                        .class_signal("btn-warning", disable_mutable.signal())
                        .class_signal("btn-primary", not(disable_mutable.signal()))
                        .text_signal(disable_mutable.signal().map(|disabled| if disabled {"Set Enabled"} else {"Set Disabled"}))
                        .event(clone!(disable_mutable => move |_:events::Click| {
                            disable_mutable.set(!disable_mutable.get())
                        }))
                    }),
                    html!("button", {
                        .attr("type", "button")
                        .class(["btn","me-2"])
                        .class_signal("btn-warning", is_dark_mutable.signal())
                        .class_signal("btn-primary", not(is_dark_mutable.signal()))
                        .text_signal(is_dark_mutable.signal().map(|is_dark| if is_dark {"Set Light Theme"} else {"Set Dark Theme"}))
                        .event(clone!(is_dark_mutable => move |_:events::Click| {
                            is_dark_mutable.set(!is_dark_mutable.get());
                        }))
                    }),
                    html!("button", {
                        .attr("type", "button")
                        .class(["btn","p-0"])
                        .style_signal("color", changed_mutable.signal().map(|is_changed| if is_changed {"gold"} else {"gray"}))
                        .child(html!("i", {.class(["fas","fa-lightbulb"]).style("font-size", "35px")}))
                        .event(clone!(changed_mutable => move |_:events::Click| {
                            changed_mutable.set_neq(false);
                        }))
                    }),
                ])
            }))
            .children([
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .children([
                        html!("div", {.class("fw-bold").text("Browser's default date picker")}),
                        html!("input" => HtmlInputElement, {
                            .attr("type", "date")
                            .class(["form-control","my-1"])
                            .prop_signal("value", date_mutable.signal_cloned())
                            .with_node!(element => {
                                .event(clone!(date_mutable, element => move |_:events::Change| {
                                    date_mutable.set(element.value());
                                }))
                                .future(disable_mutable.signal().for_each(move |v| {
                                    element.set_disabled(v);
                                    async {}
                                }))
                            })
                        }),
                        html!("input" => HtmlInputElement, {
                            .attr("type", "time")
                            .class(["form-control","my-1"])
                            .prop_signal("value", time_mutable.signal_cloned())
                            .with_node!(element => {
                                .event(clone!(time_mutable, element => move |_:events::Change| {
                                    time_mutable.set(element.value());
                                }))
                                .future(disable_mutable.signal().for_each(move |v| {
                                    element.set_disabled(v);
                                    async {}
                                }))
                            })
                        }),
                        html!("input" => HtmlInputElement, {
                            .attr("type", "datetime-local")
                            .class(["form-control","my-1"])
                            .prop_signal("value", datetime_mutable.signal_cloned())
                            .with_node!(element => {
                                .event(clone!(datetime_mutable, element => move |_:events::Change| {
                                    datetime_mutable.set(element.value());
                                }))
                                .future(disable_mutable.signal().for_each(move |v| {
                                    element.set_disabled(v);
                                    async {}
                                }))
                            })
                        }),
                        html!("div", {
                            .class(["input-group","input-group-sm","my-1","p-0"])
                            .children([
                                html!("span", {
                                    .class("input-group-text")
                                    .text("วัน-เวลา")
                                }),
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "datetime-local")
                                    .class("form-control")
                                    .prop_signal("value", datetime_mutable.signal_cloned())
                                    .with_node!(element => {
                                        .event(clone!(datetime_mutable, element => move |_:events::Change| {
                                            datetime_mutable.set(element.value());
                                        }))
                                        .future(disable_mutable.signal().for_each(move |v| {
                                            element.set_disabled(v);
                                            async {}
                                        }))
                                    })
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(["btn","btn-sm","btn-danger"])
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .event(clone!(datetime_mutable => move |_:events::Click| {
                                        datetime_mutable.set_neq(String::new());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .children([
                        html!("div", {.class("fw-bold").text("Raw data")}),
                        html!("input" => HtmlInputElement, {
                            .attr("type", "text")
                            .class(["form-control","my-1"])
                            .prop_signal("value", date_mutable.signal_cloned())
                            .with_node!(element => {
                                .event(clone!(date_mutable, element => move |_:events::Change| {
                                    date_mutable.set(element.value());
                                }))
                                .future(disable_mutable.signal().for_each(move |v| {
                                    element.set_disabled(v);
                                    async {}
                                }))
                            })
                        }),
                        html!("input" => HtmlInputElement, {
                            .attr("type", "text")
                            .class(["form-control","my-1"])
                            .prop_signal("value", time_mutable.signal_cloned())
                            .with_node!(element => {
                                .event(clone!(time_mutable, element => move |_:events::Change| {
                                    time_mutable.set(element.value());
                                }))
                                .future(disable_mutable.signal().for_each(move |v| {
                                    element.set_disabled(v);
                                    async {}
                                }))
                            })
                        }),
                        html!("input" => HtmlInputElement, {
                            .attr("type", "text")
                            .class(["form-control","form-control-sm","my-1"])
                            .prop_signal("value", datetime_mutable.signal_cloned())
                            .with_node!(element => {
                                .event(clone!(datetime_mutable, element => move |_:events::Change| {
                                    datetime_mutable.set(element.value());
                                }))
                                .future(disable_mutable.signal().for_each(move |v| {
                                    element.set_disabled(v);
                                    async {}
                                }))
                            })
                        }),
                    ])
                }),
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .children([
                        html!("div", {.class("fw-bold").text("Thai date picker")}),
                        doms::date_picker(
                            date_mutable.clone(),
                            changed_mutable.clone(),
                            disable_mutable.signal(),
                            None,
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class("rounded-1"),
                            |d| d.class("rounded-1"),
                            |s| s,
                            always(None),
                        ),
                        doms::time_picker(
                            time_mutable.clone(),
                            changed_mutable.clone(),
                            disable_mutable.signal(),
                            None,
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class("rounded-2"),
                            |d| d.class("rounded-2"),
                            |s| s,
                            always(None),
                        ),
                        doms::datetime_picker(
                            datetime_mutable.clone(),
                            changed_mutable.clone(),
                            disable_mutable.signal(),
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |s| s,
                            always(None),
                        ),
                    ])
                }),
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .children([
                        html!("div", {.class("fw-bold").text("Reversing the disabled flag")}),
                        doms::date_picker(
                            date_mutable.clone(),
                            changed_mutable.clone(),
                            not(disable_mutable.signal()),
                            None,
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class("rounded-1"),
                            |d| d.class("rounded-1"),
                            |s| s,
                            always(None),
                        ),
                        doms::time_picker(
                            time_mutable.clone(),
                            changed_mutable.clone(),
                            not(disable_mutable.signal()),
                            None,
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class("rounded-2"),
                            |d| d.class("rounded-2"),
                            |s| s,
                            always(None),
                        ),
                        doms::datetime_picker(
                            datetime_mutable.clone(),
                            changed_mutable.clone(),
                            not(disable_mutable.signal()),
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |s| s,
                            always(None),
                        ),
                    ])
                }),
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .children([
                        html!("div", {.class("fw-bold").text("With function (+ 1 day and 1 minute)")}),
                        doms::date_picker(
                            date_mutable.clone(),
                            changed_mutable.clone(),
                            disable_mutable.signal(),
                            None,
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class("rounded-1"),
                            |d| d.class("rounded-1"),
                            |s| {
                                if let Some(d) = date_8601(&s) {
                                    (d + Duration::days(1)).to_string()
                                } else {
                                    String::new()
                                }
                            },
                            always(None),
                        ),
                        doms::time_picker(
                            time_mutable.clone(),
                            changed_mutable.clone(),
                            disable_mutable.signal(),
                            None,
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class("rounded-2"),
                            |d| d.class("rounded-2"),
                            |s| {
                                if let Some(t) = time_8601(&s) {
                                    (t + Duration::minutes(1)).js_string()
                                } else {
                                    String::new()
                                }
                            },
                            always(None),
                        ),
                        doms::datetime_picker(
                            datetime_mutable.clone(),
                            changed_mutable.clone(),
                            disable_mutable.signal(),
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |s| {
                                if let Some(dt) = datetime_8601(&s) {
                                    (dt + Duration::days(1) + Duration::minutes(1)).js_string()
                                } else {
                                    String::new()
                                }
                            },
                            always(None),
                        ),
                    ])
                }),
                html!("div", {
                    .class(["d-flex","flex-wrap","p-2","border","rounded","justify-content-center"])
                    .children([
                        html!("button", {
                            .attr("type", "button")
                            .class(["btn","btn-info","w-100","mb-1"])
                            .text(&min_datetime_label)
                            .event(clone!(config_label_mutable, config_mutable => move |_:events::Click| {
                                config_label_mutable.set(min_datetime_label.clone());
                                config_mutable.set(Some(doms::PickerConfigBuilder::default()
                                    .date_constraints(doms::DateConstraintsBuilder::default()
                                        .disabled_weekdays([Weekday::Wednesday].into())
                                        .min_datetime(now_datetime)
                                        .build().unwrap()
                                    ).build().unwrap()
                                ))
                            }))
                        }),
                        html!("button", {
                            .attr("type", "button")
                            .class(["btn","btn-info","w-100","mb-1"])
                            .text(&max_datetime_label)
                            .event(clone!(config_label_mutable, config_mutable => move |_:events::Click| {
                                config_label_mutable.set(max_datetime_label.clone());
                                config_mutable.set(Some(doms::PickerConfigBuilder::default()
                                    .date_constraints(doms::DateConstraintsBuilder::default()
                                        .disabled_monthly_dates([1,9,17,25].into())
                                        .max_datetime(now_datetime)
                                        .build().unwrap()
                                    ).build().unwrap()
                                ))
                            }))
                        }),
                        html!("button", {
                            .attr("type", "button")
                            .class(["btn","btn-info","w-100","mb-1"])
                            .text("เงื่อนไข เฉพาะวันแรกของเดือน ไม่ก่อนเดือนปัจจุบัน")
                            .event(clone!(config_label_mutable, config_mutable => move |_:events::Click| {
                                config_label_mutable.set(String::from("เงื่อนไข เฉพาะวันแรกของเดือน ไม่ก่อนเดือนปัจจุบัน"));
                                config_mutable.set(Some(doms::PickerConfigBuilder::default()
                                    .date_constraints(doms::DateConstraintsBuilder::default()
                                        .min_datetime(PrimitiveDateTime::new(Date::from_calendar_date(now.year(), now.month(), 1).unwrap(), Time::MIDNIGHT))
                                        .build().unwrap()
                                    )
                                    .selection_type(doms::DialogViewType::Months)
                                    .initial_view_type(doms::DialogViewType::Months)
                                    .build().unwrap()
                                ))
                            }))
                        }),
                        html!("button", {
                            .attr("type", "button")
                            .class(["btn","btn-info","w-100"])
                            .text("เงื่อนไข เฉพาะวันแรกของปี ไม่ก่อนปีปัจจุบัน")
                            .event(clone!(config_label_mutable, config_mutable => move |_:events::Click| {
                                config_label_mutable.set(String::from("เงื่อนไข เฉพาะวันแรกของปี ไม่ก่อนปีปัจจุบัน"));
                                config_mutable.set(Some(doms::PickerConfigBuilder::default()
                                    .date_constraints(doms::DateConstraintsBuilder::default()
                                        .min_datetime(PrimitiveDateTime::new(Date::from_calendar_date(now.year(), time::Month::January, 1).unwrap(), Time::MIDNIGHT))
                                        .build().unwrap()
                                    )
                                    .selection_type(doms::DialogViewType::Years)
                                    .initial_view_type(doms::DialogViewType::Years)
                                    .build().unwrap()
                                ))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .children([
                        html!("div", {.class("fw-bold").text_signal(config_label_mutable.signal_cloned())}),
                        html!("div", {
                            .class(["d-flex","p-0"])
                            .children([
                                doms::date_picker(
                                    date_mutable.clone(),
                                    changed_mutable.clone(),
                                    disable_mutable.signal(),
                                    Some(time_mutable.clone()),
                                    |d| d.class(["flex-fill","p-0","my-1","me-1"]),
                                    |d| d.class("rounded-1"),
                                    |d| d.class("rounded-1"),
                                    |s| s,
                                    config_mutable.signal_cloned(),
                                ),
                                doms::time_picker(
                                    time_mutable.clone(),
                                    changed_mutable.clone(),
                                    disable_mutable.signal(),
                                    Some(date_mutable.clone()),
                                    |d| d.class(["flex-fill","p-0","my-1","ms-1"]),
                                    |d| d.class("rounded-2"),
                                    |d| d.class("rounded-2"),
                                    |s| s,
                                    config_mutable.signal_cloned(),
                                ),
                            ])
                        }),
                        doms::datetime_picker(
                            datetime_mutable.clone(),
                            changed_mutable.clone(),
                            disable_mutable.signal(),
                            |d| d.class(["p-0","my-1"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |d| d.class(["form-control-sm","rounded-3"]),
                            |s| s,
                            config_mutable.signal_cloned(),
                        ),
                    ])
                }),
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .children([
                        html!("div", {.class("fw-bold").text("Child of 'overflow-hidden' parent")}),
                        html!("div", {
                            .class(["mt-1","pt-2","border","border-primary","overflow-hidden"])
                            .style("height","65px")
                            .child(doms::date_picker(
                                date_mutable.clone(),
                                changed_mutable.clone(),
                                disable_mutable.signal(),
                                None,
                                |d| d.class(["p-0","my-1"]),
                                |d| d.class("rounded-2"),
                                |d| d.class("rounded-2"),
                                |s| s,
                                always(None),
                            ))
                        }),
                        html!("div", {
                            .class(["mt-1","pt-2","border","border-primary","overflow-hidden"])
                            .style("height","65px")
                            .child(doms::time_picker(
                                time_mutable.clone(),
                                changed_mutable.clone(),
                                disable_mutable.signal(),
                                None,
                                |d| d.class(["p-0","my-1"]),
                                |d| d.class("rounded-2"),
                                |d| d.class("rounded-2"),
                                |s| s,
                                always(None),
                            ))
                        }),
                        html!("div", {
                            .class(["mt-1","pt-2","border","border-primary","overflow-hidden"])
                            .style("height","65px")
                            .child(doms::datetime_picker(
                                datetime_mutable.clone(),
                                changed_mutable.clone(),
                                disable_mutable.signal(),
                                |d| d.class(["p-0","my-1"]),
                                |d| d.class(["form-control-sm","rounded-2"]),
                                |d| d.class(["form-control-sm","rounded-2"]),
                                |s| s,
                                always(None),
                            ))
                        }),
                    ])
                }),
                html!("div", {
                    .class(["row","mx-1","my-3","p-2","border"])
                    .child(html!("div", {
                        .class(["col-6","offset-6","p-0"])
                        .children([
                            html!("div", {.class("fw-bold").text("Bootstrap Input Group")}),
                            html!("div", {
                                .class(["input-group","flex-nowrap","p-0","my-1"])
                                .children([
                                    html!("span", {
                                        .class("input-group-text")
                                        .text("วันที่")
                                    }),
                                    doms::date_picker(
                                        date_mutable.clone(),
                                        changed_mutable.clone(),
                                        disable_mutable.signal(),
                                        None,
                                        |d| d.class(["d-flex","flex-grow-1"]),
                                        |d| d.class("rounded-start-0"),
                                        |d| d.class("rounded-start-0"),
                                        |s| s,
                                        always(None),
                                    ),
                                ])
                            }),
                            html!("div", {
                                .class(["input-group","flex-nowrap","p-0","my-1"])
                                .children([
                                    doms::time_picker(
                                        time_mutable.clone(),
                                        changed_mutable.clone(),
                                        disable_mutable.signal(),
                                        None,
                                        |d| d.class(["d-flex","flex-grow-1"]),
                                        |d| d.class("rounded-end-0"),
                                        |d| d.class("rounded-end-0"),
                                        |s| s,
                                        always(None),
                                    ),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(["btn","btn-danger"])
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(time_mutable => move |_:events::Click| {
                                            time_mutable.set_neq(String::new());
                                        }))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(["input-group","input-group-sm","flex-nowrap","p-0","my-1"])
                                .children([
                                    html!("span", {
                                        .class("input-group-text")
                                        .text("Small")
                                    }),
                                    doms::datetime_picker(
                                        datetime_mutable.clone(),
                                        changed_mutable.clone(),
                                        disable_mutable.signal(),
                                        |d| d.class(["d-flex","flex-grow-1"]),
                                        |d| d.class(["form-control-sm","rounded-0"]),
                                        |d| d.class(["form-control-sm","rounded-0"]),
                                        |s| s,
                                        always(None),
                                    ),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(["btn","btn-danger"])
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(datetime_mutable => move |_:events::Click| {
                                            datetime_mutable.set_neq(String::new());
                                        }))
                                    }),
                                ])
                            }),
                        ])
                    }))
                }),
            ])
        }))
    })
}
