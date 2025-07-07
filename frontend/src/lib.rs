use dominator::{clone, events, html, Dom, with_node};
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::*;

mod dom;
mod date_picker;

use picker_util::{JsTime, js_now};

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

    let date_mutable = Mutable::new(String::new());
    let time_mutable = Mutable::new(String::new());
    let datetime_mutable = Mutable::new(String::new());

    html!("div", {
        .class(["container-fluid","my-3"])
        .style("max-width","500px")
        .attr_signal("data-bs-theme", is_dark_mutable.signal().map(|is_dark| if is_dark {"dark"} else {"light"}))
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
            .class("d-flex")
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
                    .class(["btn","btn-primary","me-1"])
                    .text("Light Theme")
                    .event(clone!(is_dark_mutable => move |_:events::Click| {
                        is_dark_mutable.set(false);
                    }))
                }),
                html!("button", {
                    .attr("type", "button")
                    .class(["btn","btn-primary","me-1"])
                    .text("Dark Theme")
                    .event(clone!(is_dark_mutable => move |_:events::Click| {
                        is_dark_mutable.set(true);
                    }))
                }),
            ])
        }))
        .children([
            html!("div", {
                .class(["row","mx-1","my-3","p-2","border"])
                .children([
                    html!("div", {.text("Default picker")}),
                    html!("input" => HtmlInputElement, {
                        .attr("type", "date")
                        .class(["form-control","my-1"])
                        .prop_signal("value", date_mutable.signal_cloned())
                        .with_node!(element => {
                            .event(clone!(date_mutable => move |_:events::Change| {
                                date_mutable.set(element.value());
                            }))
                        })
                    }),
                    html!("br"),
                    html!("input" => HtmlInputElement, {
                        .attr("type", "time")
                        .class(["form-control","my-1"])
                        .prop_signal("value", time_mutable.signal_cloned())
                        .with_node!(element => {
                            .event(clone!(time_mutable => move |_:events::Change| {
                                time_mutable.set(element.value());
                            }))
                        })
                    }),
                    html!("br"),
                    html!("input" => HtmlInputElement, {
                        .attr("type", "datetime-local")
                        .class(["form-control","my-1"])
                        .prop_signal("value", datetime_mutable.signal_cloned())
                        .with_node!(element => {
                            .event(clone!(datetime_mutable => move |_:events::Change| {
                                datetime_mutable.set(element.value());
                            }))
                        })
                    }),
                ])
            }),
            html!("div", {
                .class(["row","mx-1","my-3","p-2","border"])
                .children([
                    html!("div", {.text("Raw data")}),
                    html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .class(["form-control","my-1"])
                        .prop_signal("value", date_mutable.signal_cloned())
                        .with_node!(element => {
                            .event(clone!(date_mutable => move |_:events::Change| {
                                date_mutable.set(element.value());
                            }))
                        })
                    }),
                    html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .class(["form-control","my-1"])
                        .prop_signal("value", time_mutable.signal_cloned())
                        .with_node!(element => {
                            .event(clone!(time_mutable => move |_:events::Change| {
                                time_mutable.set(element.value());
                            }))
                        })
                    }),
                    html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .class(["form-control","my-1"])
                        .prop_signal("value", datetime_mutable.signal_cloned())
                        .with_node!(element => {
                            .event(clone!(datetime_mutable => move |_:events::Change| {
                                datetime_mutable.set(element.value());
                            }))
                        })
                    }),
                ])
            }),
            html!("div", {
                .class(["row","mx-1","my-3","p-2","border"])
                .children([
                    html!("div", {.text("Customized")}),
                    dom::datetime_input_with_picker(false, date_mutable.clone(), ["p-0","my-1"]),
                    dom::time_input_with_picker(time_mutable.clone(), ["p-0","my-1"]),
                    dom::datetime_input_with_picker(true, datetime_mutable.clone(), ["p-0","my-1"])
                ])
            }),
        ])
    })
}
