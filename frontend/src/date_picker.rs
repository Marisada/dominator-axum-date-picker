pub use date_picker::config::{PickerConfig, PickerConfigBuilder, date_constraints::DateConstraints};

use date_picker::{
    config::date_constraints::HasDateConstraints,
    dialog_view_type::DialogViewType,
    style_names::*,
    utils::{create_dialog_title_text, should_display_next_button, should_display_previous_button},
    viewed_date::{year_group_range, ViewedDate},
};
use dominator::{clone, events, html, Dom, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use std::rc::Rc;
use time::{Date, Duration, Month, PrimitiveDateTime, Time, Weekday};

use picker_util::{date_8601, datetime_8601, time_8601, js_now, month_thai, weekday_thai, JsTime};

pub struct DatePicker {

    /// DateTime or Date or Time
    with_date: bool,
    with_time: bool,

    /// external state, DateTime or Date
    date_mutable: Mutable<String>,

    /// external self
    container: Mutable<Option<Rc<Self>>>,

    /// value of the date that is selected
    selected_date: Mutable<Option<Date>>,
    selected_hour: Mutable<Option<u8>>,
    selected_minute: Mutable<Option<u8>>,

    /// viewed date
    viewed_date: Mutable<Date>,

    /// dialog type
    dialog_view_type: Mutable<DialogViewType>,

    // /// dialog position style, describing the position of the dialog (left, top)
    // dialog_position_style: Mutable<Option<(f64, f64)>>,

    /// configuration of the picker, should be passed in during init and not modified later
    config: PickerConfig<DateConstraints>,
}

impl DatePicker {

    pub fn new_date(date_mutable: Mutable<String>, container: Mutable<Option<Rc<Self>>>, config: PickerConfig<DateConstraints>) -> Rc<Self> {
        Rc::new(Self {
            with_date: true,
            with_time: false,
            date_mutable,
            container,
            selected_date: Mutable::new(*config.initial_date()),
            selected_hour: Mutable::new(config.initial_time().map(|t| t.hour())),
            selected_minute: Mutable::new(config.initial_time().map(|t| t.minute())),
            // dialog_opened: Mutable::new(*config.initially_opened()),
            viewed_date: Mutable::new(config.guess_allowed_year_month()),
            dialog_view_type: Mutable::new(*config.initial_view_type()),
            // dialog_position_style: Mutable::new(None),
            config,
        })
    }

    pub fn new_time(time_mutable: Mutable<String>, container: Mutable<Option<Rc<Self>>>, config: PickerConfig<DateConstraints>) -> Rc<Self> {
        Rc::new(Self {
            with_date: false,
            with_time: true,
            date_mutable: time_mutable,
            container,
            selected_date: Mutable::new(*config.initial_date()),
            selected_hour: Mutable::new(config.initial_time().map(|t| t.hour())),
            selected_minute: Mutable::new(config.initial_time().map(|t| t.minute())),
            // dialog_opened: Mutable::new(*config.initially_opened()),
            viewed_date: Mutable::new(config.guess_allowed_year_month()),
            dialog_view_type: Mutable::new(*config.initial_view_type()),
            // dialog_position_style: Mutable::new(None),
            config,
        })
    }

    pub fn new_datetime(date_mutable: Mutable<String>, container: Mutable<Option<Rc<Self>>>, config: PickerConfig<DateConstraints>) -> Rc<Self> {
        Rc::new(Self {
            with_date: true,
            with_time: true,
            date_mutable,
            container,
            selected_date: Mutable::new(*config.initial_date()),
            selected_hour: Mutable::new(config.initial_time().map(|t| t.hour())),
            selected_minute: Mutable::new(config.initial_time().map(|t| t.minute())),
            // dialog_opened: Mutable::new(*config.initially_opened()),
            viewed_date: Mutable::new(config.guess_allowed_year_month()),
            dialog_view_type: Mutable::new(*config.initial_view_type()),
            // dialog_position_style: Mutable::new(None),
            config,
        })
    }

    fn should_display_previous_button(picker: Rc<Self>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let viewed_date = picker.viewed_date.signal(),
            let dialog_view_type = picker.dialog_view_type.signal_cloned() =>
            (viewed_date.clone(), *dialog_view_type)
        }.map(clone!(picker => move |(viewed_date, dialog_view_type)| {
            should_display_previous_button(&dialog_view_type, &viewed_date, &picker.config)
        }))
    }

    fn should_display_next_button(picker: Rc<Self>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let viewed_date = picker.viewed_date.signal(),
            let dialog_view_type = picker.dialog_view_type.signal_cloned() =>
            (viewed_date.clone(), *dialog_view_type)
        }.map(clone!(picker => move |(viewed_date, dialog_view_type)| {
            should_display_next_button(&dialog_view_type, &viewed_date, &picker.config)
        }))
    }

    fn create_dialog_title_text(&self) -> impl Signal<Item = String> + use<> {
        map_ref! {
            let viewed_date = self.viewed_date.signal(),
            let dialog_view_type = self.dialog_view_type.signal_cloned() =>
            create_dialog_title_text(dialog_view_type, viewed_date)
        }
    }

    pub fn render(picker: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class(DATEPICKER_BACKDROP)
                    .event(clone!(picker => move |_:events::Click| {
                        picker.exit();
                    }))
                }),
                html!("div", {
                    .future(picker.date_mutable.signal_cloned().for_each(clone!(picker => move |date_mutable| {
                        if picker.with_date && picker.with_time {
                            let datetime_opt = datetime_8601(&date_mutable);
                            picker.selected_date.set(datetime_opt.map(|dt| dt.date()));
                            picker.selected_hour.set(datetime_opt.map(|dt| dt.time().hour()));
                            picker.selected_minute.set(datetime_opt.map(|dt| dt.time().minute()));
                        } else if picker.with_date {
                            let date_opt = date_8601(&date_mutable);
                            picker.selected_date.set(date_opt);
                        } else if picker.with_time && !date_mutable.is_empty() {
                            let time_opt = time_8601(&date_mutable);
                            picker.selected_hour.set(time_opt.map(|t| t.hour()));
                            picker.selected_minute.set(time_opt.map(|t| t.minute()));
                        }
                        async {}
                    })))
                    .class(DATEPICKER_ROOT)
                    .apply_if(picker.with_date, |dom| { dom
                        .child(html!("div", {
                            .class(DATE_CONTAINER)
                            .child(Self::render_header(picker.clone()))
                            .child_signal(picker.dialog_view_type.signal_cloned().map(clone!(picker => move |dialog_view_type| {
                                Some(match dialog_view_type {
                                    DialogViewType::Days => Self::render_dialog_days(picker.clone()),
                                    DialogViewType::Months => Self::render_dialog_months(picker.clone()),
                                    DialogViewType::Years => Self::render_dialog_years(picker.clone()),
                                })
                            })))
                            .child(Self::render_footer(picker.clone()))
                        }))
                    })
                    .apply_if(picker.with_time, |dom| { dom
                        .child(html!("div", {
                            .class(TIME_CONTAINER)
                            .children([
                                html!("div", {
                                    .class(BODY)
                                    .children([
                                        Self::render_dialog_hours(picker.clone()),
                                        Self::render_dialog_minutes(picker.clone()),
                                    ])
                                }),
                                html!("div", {
                                    .class(FOOTER)
                                    .child(Self::render_exit(picker.clone()))
                                }),
                            ])
                        }))
                    })
                })
            ])
        })
    }

    fn render_header(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(HEADER)
            .children([
                html!("button", {
                    .attr("type", "button")
                    .class([BUTTON, PREVIOUS])
                    .style_signal("visibility", Self::should_display_previous_button(picker.clone()).map(|display| {
                        if display { 
                            "visible"
                        } else {
                            "hidden"
                        }
                    }))
                    .child(html!("i", {.class(["fas","fa-arrow-left"])}))
                    .event(clone!(picker => move |_:events::Click| {
                        let current = picker.viewed_date.get();
                        let viewed_date = match picker.dialog_view_type.get_cloned() {
                            DialogViewType::Days => current.previous_month(),
                            DialogViewType::Months => current.previous_year(),
                            DialogViewType::Years => current.previous_year_group(),
                        };
                        picker.viewed_date.set(viewed_date);
                    }))
                }),
                html!("span", {
                    .class(TITLE)
                    .attr("role", "heading")
                    .text_signal(picker.create_dialog_title_text())
                    .event(clone!(picker => move |_:events::Click| {
                        if let Some(new_dialog_type) = picker.dialog_view_type.get_cloned().larger_type() {
                            picker.dialog_view_type.set(new_dialog_type);
                        }
                    }))
                }),
                html!("button", {
                    .attr("type", "button")
                    .class([BUTTON, NEXT])
                    .style_signal("visibility", Self::should_display_next_button(picker.clone()).map(|display| {
                        if display { 
                            "visible"
                        } else {
                            "hidden"
                        }
                    }))
                    .child(html!("i", {.class(["fas","fa-arrow-right"])}))
                    .event(clone!(picker => move |_:events::Click| {
                        let current = picker.viewed_date.get();
                        let viewed_date = match picker.dialog_view_type.get_cloned() {
                            DialogViewType::Days => current.next_month(),
                            DialogViewType::Months => current.next_year(),
                            DialogViewType::Years => current.next_year_group(),
                        };
                        picker.viewed_date.set(viewed_date);
                    }))
                }),
            ])
            .apply_if(!picker.with_time, |dom| { dom
                .child(Self::render_exit(picker.clone()))
            })
        })
    }

    fn render_footer(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(FOOTER)
            .children([
                html!("button", {
                    .attr("type", "button")
                    .class([BUTTON, EMPTY])
                    .style_signal("visibility", picker.selected_date.signal_cloned().map(|opt| {
                        if opt.is_some() { 
                            "visible"
                        } else {
                            "hidden"
                        }
                    }))
                    .text("ล้างข้อมูล")
                    .event(clone!(picker => move |_:events::Click| {
                        picker.date_mutable.set_neq(String::new());
                        picker.container.set(None);
                    }))
                }),
                html!("button", {
                    .attr("type", "button")
                    .class([BUTTON, TODAY])
                    .text("วันนี้")
                    .event(clone!(picker => move |_:events::Click| {
                        let now = js_now();
                        if picker.with_time {
                            picker.date_mutable.set_neq(now.js_string());
                        } else {
                            picker.date_mutable.set_neq(now.date().to_string());
                        }
                        picker.container.set(None);
                    }))
                }),
            ])
        })
    }

    fn render_dialog_years(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(BODY)
            .style("grid-template-columns", "1fr ".repeat(4))
            .children_signal_vec(picker.viewed_date.signal().map(clone!(picker => move |d| {
                year_group_range(d.year()).map(|y| Self::render_year_cell(y, picker.clone())).collect::<Vec<Dom>>()
            })).to_signal_vec())
        })
    }

    fn render_year_cell(display_year: i32, picker: Rc<Self>) -> Dom {
        let is_year_forbidden = picker.config.is_year_forbidden(display_year);
        html!("span", {
            .text(&(display_year + 543).to_string())
            .class(if is_year_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            })
            .class_signal(SELECTED, picker.selected_date.signal_cloned().map(move |opt| {
                opt.map_or(false, |optval| optval.year() == display_year)
            }))
            .attr("role", "gridcell")
            .prop_signal("aria-selected", picker.selected_date.signal_cloned().map(move |opt| {
                if opt.map_or(false, |optval| optval.year() == display_year) {"true"} else {"false"}
            }))
            .apply_if(!is_year_forbidden, |dom| { dom
                .event(clone!(picker => move |_:events::Click| {
                    let new_year = Date::from_calendar_date(display_year, Month::January, 1).unwrap();
                    picker.viewed_date.set(new_year);
                    if picker.config.selection_type() == &DialogViewType::Years {
                        picker.selected_date.set(Some(new_year));
                        if !picker.with_time {
                            picker.date_mutable.set_neq(new_year.to_string());
                            picker.container.set(None);
                        }
                    } else {
                        picker.dialog_view_type.set(DialogViewType::Months);
                    }
                }))
            })
        })
    }

    fn render_dialog_months(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(BODY)
            .style("grid-template-columns", "1fr ".repeat(3))
            .children_signal_vec(picker.viewed_date.signal().map(clone!(picker => move |d| {
                (1..=12u8).map(|m| {
                    // this unwrap() never fail
                    let new_month = Date::from_calendar_date(d.year(), Month::try_from(m).unwrap(), 1).unwrap();
                    Self::render_month_cell(new_month, picker.clone())
                }).collect::<Vec<Dom>>()
            })).to_signal_vec())
        })
    }

    fn render_month_cell(display_month: Date, picker: Rc<Self>) -> Dom {
        let is_month_forbidden = picker.config.is_month_forbidden(&display_month);
        html!("span", {
            .text(&month_thai(&display_month.month()))
            .class(if is_month_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            })
            .class_signal(SELECTED, map_ref! {
                let selected_date = picker.selected_date.signal_cloned(),
                let dialog_view_type = picker.dialog_view_type.signal_cloned() =>
                (selected_date.clone(), dialog_view_type.clone())
            }.map(move |(selected_date, dialog_view_type)| {
                selected_date.map_or(false, |optval| display_month.contains(&dialog_view_type, &optval))
            }))
            .attr("role", "gridcell")
            .prop_signal("aria-selected", map_ref! {
                let selected_date = picker.selected_date.signal_cloned(),
                let dialog_view_type = picker.dialog_view_type.signal_cloned() =>
                (selected_date.clone(), dialog_view_type.clone())
            }.map(move |(selected_date, dialog_view_type)| {
                if selected_date.map_or(false, |optval| display_month.contains(&dialog_view_type, &optval)) {"true"} else {"false"}
            }))
            .apply_if(!is_month_forbidden, |dom| { dom
                .event(clone!(picker => move |_:events::Click| {
                    let new_month = Date::from_calendar_date(display_month.year(), display_month.month(), 1).unwrap();
                    picker.viewed_date.set(new_month);
                    if picker.config.selection_type() == &DialogViewType::Months {
                        picker.selected_date.set(Some(new_month));
                        if !picker.with_time {
                            picker.date_mutable.set_neq(new_month.to_string());
                            picker.container.set(None);
                        }
                    } else {
                        picker.dialog_view_type.set(DialogViewType::Days);
                    }
                }))
            })
        })
    }

    fn render_dialog_days(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(BODY)
            .style("grid-template-columns", "1fr ".repeat(7))
            .children([
                render_weekday_name(Weekday::Sunday),
                render_weekday_name(Weekday::Monday),
                render_weekday_name(Weekday::Tuesday),
                render_weekday_name(Weekday::Wednesday),
                render_weekday_name(Weekday::Thursday),
                render_weekday_name(Weekday::Friday),
                render_weekday_name(Weekday::Saturday),
            ])
            .children_signal_vec(picker.viewed_date.signal().map(clone!(picker => move |d| {
                let first_day_of_month = d.first_day_of_month();
                let offset = first_day_of_month.weekday().number_days_from_sunday();
                let first_day_of_calendar = first_day_of_month - Duration::new(offset as i64 * 24 * 60 * 60, 0);
                first_day_of_calendar.dates_fill_calendar(offset).iter().map(|d| {
                    Self::render_day_cell(*d, picker.clone())
                }).collect::<Vec<Dom>>()
            })).to_signal_vec())
        })
    }

    fn render_day_cell(display_day: Date, picker: Rc<Self>) -> Dom {
        let is_day_forbidden = picker.config.is_day_forbidden(&display_day);
        html!("span", {
            .text(&display_day.day().to_string())
            .class(if is_day_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            })
            .class_signal(OTHER_MONTH, picker.viewed_date.signal().map(move |viewed_date| viewed_date.month() != display_day.month()))
            .class_signal(SELECTED, picker.selected_date.signal_cloned().map(move |opt| opt.map_or(false, |optval| optval == display_day)))
            .attr("role", "gridcell")
            .prop_signal("aria-selected", picker.selected_date.signal_cloned().map(move |opt| {
                if opt.map_or(false, |optval| optval == display_day) {"true"} else {"false"}
            }))
            .apply_if(!is_day_forbidden, |dom| { dom
                .event(clone!(picker => move |_:events::Click| {
                    picker.selected_date.set(Some(display_day));
                    picker.viewed_date.set(display_day);
                    if !picker.with_time {
                        picker.date_mutable.set_neq(display_day.to_string());
                        picker.container.set(None);
                    }
                }))
            })
        })
    }

    fn render_dialog_hours(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(HOUR_CONTAINER)
            .apply_if(picker.with_date, |dom| { dom
                .style("border-left", "1px solid var(--bs-border-color)")
            })
            .child(html!("div", {
                .class(HOUR)
                .style("grid-template-columns", "1fr")
                .children((0..=23u8).map(|h| {
                    // never fail
                    Self::render_hour_cell(h, picker.clone())
                }))
            }))
            .with_node!(element => {
                .future(picker.selected_hour.signal().for_each(clone!(element => move |opt| {
                    if let Some(selected_hour) = opt {
                        // gap 3px, padding-top 3px, padding-bottom 3px
                        element.set_scroll_top(selected_hour as i32 * 30);
                    }
                    async {}
                })))
            })
        })
    }

    fn render_hour_cell(display_hour: u8, picker: Rc<Self>) -> Dom {
        html!("span", {
            .text(&display_hour.to_string())
            .class(SELECTABLE)
            .class_signal(SELECTED, picker.selected_hour.signal_cloned().map(move |opt| opt.map_or(false, |selected_hour| selected_hour == display_hour)))
            .attr("role", "gridcell")
            .prop_signal("aria-selected", picker.selected_hour.signal_cloned().map(move |opt| {
                if opt.map_or(false, |selected_hour| selected_hour == display_hour) {"true"} else {"false"}
            }))
            .event(clone!(picker => move |_:events::Click| {
                picker.selected_hour.set(Some(display_hour));
            }))
        })
    }

    fn render_dialog_minutes(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(MINUTE_CONTAINER)
            .style("border-left", "1px solid var(--bs-border-color)")
            .child(html!("div", {
                .class(MINUTE)
                .style("grid-template-columns", "1fr")
                .children((0..=59u8).map(|m| {
                    Self::render_minute_cell(m, picker.clone())
                }))
            }))
            .with_node!(element => {
                .future(picker.selected_minute.signal().for_each(clone!(element => move |opt| {
                    if let Some(selected_minute) = opt {
                        // gap 3px, padding-top 3px, padding-bottom 3px
                        element.set_scroll_top(selected_minute as i32 * 30);
                    }
                    async {}
                })))
            })
        })
    }

    fn render_minute_cell(display_minute: u8, picker: Rc<Self>) -> Dom {
        html!("span", {
            .text(&display_minute.to_string())
            .class(SELECTABLE)
            .class_signal(SELECTED, picker.selected_minute.signal_cloned().map(move |opt| opt.map_or(false, |selected_minute| selected_minute == display_minute)))
            .attr("role", "gridcell")
            .prop_signal("aria-selected", picker.selected_hour.signal_cloned().map(move |opt| {
                if opt.map_or(false, |selected_hour| selected_hour == display_minute) {"true"} else {"false"}
            }))
            .event(clone!(picker => move |_:events::Click| {
                picker.selected_minute.set(Some(display_minute));
            }))
        })
    }

    fn render_exit(picker: Rc<Self>) -> Dom {
        html!("button", {
            .attr("type", "button")
            .class([BUTTON, CLOSE])
            .apply(|dom| {
                if picker.with_time {
                    dom.text("บันทึก")
                } else {
                    dom.child(html!("i", {.class(["fa","fa-xmark"])}))
                }
            })
            .event(clone!(picker => move |_:events::Click| {
                picker.exit();
            }))
        })
    }

    fn exit(&self) {
        if self.with_date {
            if let Some(selected_date) = self.selected_date.get_cloned() {
                if self.with_time {
                    let select_hour = self.selected_hour.get().unwrap_or_default();
                    let select_minute = self.selected_minute.get().unwrap_or_default();
                    // selected_hour and selected_minute comes from defined valid value, cannot panic
                    let selected_time = Time::from_hms(select_hour, select_minute, 0).unwrap();
                    // forced to save
                    self.date_mutable.set(PrimitiveDateTime::new(selected_date, selected_time).js_string());
                } else {
                    self.date_mutable.set_neq(selected_date.to_string());
                }
            }
        } else if self.with_time {
            let select_hour = self.selected_hour.get().unwrap_or_default();
            let select_minute = self.selected_minute.get().unwrap_or_default();
            // selected_hour and selected_minute comes from defined valid value, cannot panic
            let selected_time = Time::from_hms(select_hour, select_minute, 0).unwrap();
            // forced to save
            self.date_mutable.set(selected_time.js_string());
        }
        self.container.set(None);
    }
}

fn render_weekday_name(day: Weekday) -> Dom {
    html!("span", {
        .text(weekday_thai(&day))
        .class(GRID_HEADER)
        .attr("role", "columnheader")
    })
}
