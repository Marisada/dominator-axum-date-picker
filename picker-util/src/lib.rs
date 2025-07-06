use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, Weekday, format_description::well_known::Iso8601, macros::offset};

// ===== ===== ===== //
//    Time Related   //
// ===== ===== ===== //

pub trait JsTime {
    fn js_string(&self) -> String;
}

impl JsTime for Time {
    fn js_string(&self) -> String {
        // HH:mm
        format!("{:02}:{:02}", self.hour(), self.minute())
        // // HH:mm:ss
        // format!("{:02}:{:02}:{:02}",
        //     self.hour(), self.minute(), self.second()
        // )
        // // HH:mm:ss:SSS
        // format!("{:02}:{:02}:{:02}.{:03}",
        // self.hour(), self.minute(), self.second(), self.nanosecond() / 1_000_000,
        // )
    }
}

impl JsTime for PrimitiveDateTime {
    fn js_string(&self) -> String {
        // HH:mm
        format!("{}T{:02}:{:02}", self.date(), self.hour(), self.minute())
        // // HH:mm:ss
        // format!("{}T{:02}:{:02}:{:02}", self.date(), self.hour(), self.minute(), self.second())
        // // HH:mm:ss:SSS
        // format!("{} {:02}:{:02}:{:02}.{:03}",
        // self.date(), self.hour(), self.minute(), self.second(), self.nanosecond() / 1_000_000,
        // )
    }
}

/// GMT +7 datetime
pub fn js_now() -> PrimitiveDateTime {
    let local = OffsetDateTime::from_unix_timestamp(get_now_wasm() as i64).unwrap().to_offset(offset!(+7));
    PrimitiveDateTime::new(local.date(), local.time())
}

/// GMT +7 datetime
pub fn now() -> PrimitiveDateTime {
    let local = OffsetDateTime::now_utc().to_offset(offset!(+7));
    PrimitiveDateTime::new(local.date(), local.time())
}

/// UTC timestamp in seconds
pub fn get_now_wasm() -> u64 {
    // UTC in milliseconds
    let now = js_sys::Date::now() as u64;
    // log::debug!("{now}");

    now.saturating_div(1_000)
}

/// parse ISO-8601 string to time::PrimitiveDateTime, error will be `None`, discard offset
pub fn datetime_8601(text: &str) -> Option<PrimitiveDateTime> {
    let sanitize = text.replace('T', " ");
    let dt = sanitize.split(' ').map(str::trim).collect::<Vec<&str>>();
    if dt.len() != 2 {
        return None;
    }
    let date = date_8601(dt[0]);
    let time = time_8601(dt[1]);

    if let (Some(d), Some(t)) = (date, time) { Some(PrimitiveDateTime::new(d, t)) } else { None }
}
/// parse ISO-8601 string to time::Date, error will be `None`
pub fn date_8601(text: &str) -> Option<Date> {
    Date::parse(text, &Iso8601::DEFAULT).ok()
}
/// parse ISO-8601 string to time::Time, error will be `None`, discard offset
pub fn time_8601(text: &str) -> Option<Time> {
    Time::parse(text, &Iso8601::DEFAULT).ok()
}

/// get UNIX timestamp milliseconds from GMT+7 datetime
pub fn datetime_ts(date_time: &PrimitiveDateTime) -> i64 {
    (date_time.assume_offset(offset!(+7)).unix_timestamp_nanos() / 1_000_000) as i64
}

pub fn minutes_after_midnight(t: &Time) -> u16 {
    let h = t.hour() as u16;
    let m = t.minute() as u16;
    h.saturating_mul(24).saturating_add(m)
}

// /// parse Ulid to Thai GMT+7 string `24 ส.ค.2521 05:25 น.`
// pub fn datetime_th_from_ulid(uid: &Ulid) -> String {
//     OffsetDateTime::from_unix_timestamp((uid.timestamp_ms() / 1000) as i64)
//         .map(|utc| utc.to_offset(offset!(+7)))
//         .map(|local| [date_th(&local.date()), time_hm(&local.time())].join(" "))
//         .unwrap_or_default()
// }

/// parse PrimitiveDateTime to Thai string `24 ส.ค.2521 05:25 น.`
pub fn datetime_th(date_time: &PrimitiveDateTime) -> String {
    let date = date_time.date();
    let time = date_time.time();

    [date_th(&date), time_hm(&time)].join(" ")
}
/// parse PrimitiveDateTime to Thai string `24 ส.ค.2521 05:25 น.`
pub fn datetime_th_opt(date_time_opt: &Option<PrimitiveDateTime>) -> String {
    date_time_opt.as_ref().map(datetime_th).unwrap_or_default()
}
/// parse PrimitiveDateTime to Thai string `24 ส.ค.2521 05:25 น.`
pub fn datetime_th_relative(date_time: &PrimitiveDateTime) -> String {
    let date_now = js_now().date();
    let date = date_time.date();
    let time = date_time.time();
    let date_show = if date == date_now {
        String::from("วันนี้")
    } else if Some(date) == date_now.previous_day() {
        String::from("เมื่อวาน")
    } else {
        date_th(&date)
    };
    [date_show, time_hm(&time)].join(" ")
}
/// parse PrimitiveDateTime to Thai string `24 ส.ค.2521 05:25 น.`
pub fn datetime_th_opt_relative(date_time_opt: &Option<PrimitiveDateTime>) -> String {
    date_time_opt.as_ref().map(datetime_th_relative).unwrap_or_default()
}
/// parse Date to Thai string `24 ส.ค.2521`
pub fn date_th(date: &Date) -> String {
    let day = date.day();
    let month = date.month();
    let year = date.year();

    format!("{} {}{}", day, month_thai(&month), year + 543)
}
/// parse Date to Thai string `24 ส.ค.2521`
pub fn date_th_relative(date: &Date) -> String {
    let date_now = js_now().date();
    if *date == date_now {
        String::from("วันนี้")
    } else if Some(date) == date_now.previous_day().as_ref() {
        String::from("เมื่อวาน")
    } else {
        date_th(date)
    }
}
/// parse Option<Date> to Thai string `24 ส.ค.2521`
pub fn date_th_opt_relative(date_opt: &Option<Date>) -> String {
    date_opt.as_ref().map(date_th_relative).unwrap_or_default()
}
/// parse Option<Date> to Thai string `24 ส.ค.2521`
pub fn date_th_opt(date_opt: &Option<Date>) -> String {
    date_opt.as_ref().map(date_th).unwrap_or_default()
}
/// parse Time to Thai string `05:25 น.`
pub fn time_hm(time: &Time) -> String {
    let hour = time.hour();
    let minutes = time.minute();
    format!("{:0>2}:{:0>2} น.", hour, minutes)
}
/// parse Option<Time> to Thai string `05:25 น.`
pub fn time_hm_opt(time_opt: &Option<Time>) -> String {
    time_opt.as_ref().map(time_hm).unwrap_or_default()
}
/// parse ISO-8601 string to Thai string `1978-08-24 05:25:30` -> `24 ส.ค.2521 05:25 น.`
pub fn datetime_str_th(text: &str) -> String {
    let sanitize = text.replace('T', " ");
    let dt = sanitize.split(' ').map(str::trim).collect::<Vec<&str>>();
    if dt.len() != 2 {
        return String::new();
    }
    let date = date_str_th(dt[0]);
    let time = time_str_hm(dt[1]);
    if date.is_empty() || time.is_empty() {
        return String::new();
    }
    [date, time].join(" ")
}
/// parse ISO-8601 string to Thai string `1978-08-24 05:25:30` -> `24 ส.ค.2521 05:25 น.`
pub fn datetime_str_th_relative(text: &str) -> String {
    let dt = datetime_8601(text);
    datetime_th_opt_relative(&dt)
}
/// parse ISO-8601 string to Thai string `1978-08-24` -> `24 ส.ค.2521`
pub fn date_str_th(text: &str) -> String {
    date_8601(text).map(|d| date_th(&d)).unwrap_or_default()
}
/// parse ISO-8601 string to HH:MM "16:44:55.123" -> "16:44 น."
pub fn time_str_hm(text: &str) -> String {
    time_8601(text).map(|t| time_hm(&t)).unwrap_or_default()
}

/// floor value to `xx ชั่วโมง yy นาที`
pub fn duration_hm(duration: time::Duration) -> String {
    let secs = duration.whole_seconds();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    [
        (hours != 0).then(|| [&hours.to_string(), " ชั่วโมง"].concat()).unwrap_or_default(),
        (hours != 0 && minutes != 0).then(|| String::from(" ")).unwrap_or_default(),
        (minutes != 0).then(|| [&minutes.to_string(), " นาที"].concat()).unwrap_or_default(),
    ]
    .concat()
}

pub fn month_thai(mm: &Month) -> &'static str {
    match mm {
        Month::January => "ม.ค.",
        Month::February => "ก.พ.",
        Month::March => "มี.ค.",
        Month::April => "เม.ย.",
        Month::May => "พ.ค.",
        Month::June => "มิ.ย.",
        Month::July => "ก.ค.",
        Month::August => "ส.ค.",
        Month::September => "ก.ย.",
        Month::October => "ต.ค.",
        Month::November => "พ.ย.",
        Month::December => "ธ.ค.",
    }
}

pub fn weekday_thai(ww: &Weekday) -> &'static str {
    match ww {
        Weekday::Sunday => "อา",
        Weekday::Monday => "จ",
        Weekday::Tuesday => "อ",
        Weekday::Wednesday => "พ",
        Weekday::Thursday => "พฤ",
        Weekday::Friday => "ศ",
        Weekday::Saturday => "ส",
    }
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use super::*;
    use time::{
        macros::{date, datetime, time},
        Date, Duration, Month, Time,
    };

    #[test]
    fn test_datetime_8601() {
        assert_eq!(datetime_8601("2024-01-30T12:34"), Some(datetime!(2024-01-30 12:34)));
        assert_eq!(datetime_8601("2024-01-30T12:34:56"), Some(datetime!(2024-01-30 12:34:56)));
        assert_eq!(datetime_8601("2024-01-30T12:34:56.123456789"), Some(datetime!(2024-01-30 12:34:56.123456789)));
        assert_eq!(datetime_8601("2024-01-30T12:34:56.123456789Z"), Some(datetime!(2024-01-30 12:34:56.123456789)));
        assert_eq!(datetime_8601("2024-01-30T12:34:56.123456789+07:00"), Some(datetime!(2024-01-30 12:34:56.123456789)));
        assert_eq!(datetime_8601("2024-01-30T12:34:56.123456789-08:00"), Some(datetime!(2024-01-30 12:34:56.123456789)));

        assert_eq!(datetime_8601("2024-01-30 12:34"), Some(datetime!(2024-01-30 12:34)));
        assert_eq!(datetime_8601("2024-01-30 12:34:56"), Some(datetime!(2024-01-30 12:34:56)));
        assert_eq!(datetime_8601("2024-01-30 12:34:56.123456789"), Some(datetime!(2024-01-30 12:34:56.123456789)));
        assert_eq!(datetime_8601("2024-01-30 12:34:56.123456789Z"), Some(datetime!(2024-01-30 12:34:56.123456789)));
        assert_eq!(datetime_8601("2024-01-30 12:34:56.123456789+07:00"), Some(datetime!(2024-01-30 12:34:56.123456789)));
        assert_eq!(datetime_8601("2024-01-30 12:34:56.123456789-08:00"), Some(datetime!(2024-01-30 12:34:56.123456789)));

        assert_eq!(datetime_8601("2024-01-30 23:45:67"), None);
        assert_eq!(datetime_8601("2024-01-30"), None);
        assert_eq!(datetime_8601("12:34:56"), None);
    }

    #[test]
    fn test_date_8601() {
        assert_eq!(date_8601("2024-01-30"), Date::from_calendar_date(2024, Month::January, 30).ok());
        assert_eq!(date_8601("2024-01-33"), None);
    }

    #[test]
    fn test_time_8601() {
        assert_eq!(time_8601("12:34"), Time::from_hms(12, 34, 0).ok());
        assert_eq!(time_8601("12:34:56"), Time::from_hms(12, 34, 56).ok());
        assert_eq!(time_8601("12:34:56.123456789"), Time::from_hms_nano(12, 34, 56, 123456789).ok());
        assert_eq!(time_8601("12:34:56.123456789Z"), Time::from_hms_nano(12, 34, 56, 123456789).ok());
        assert_eq!(time_8601("12:34:56.123456789+07:00"), Time::from_hms_nano(12, 34, 56, 123456789).ok());
        assert_eq!(time_8601("12:34:56.123456789-08:00"), Time::from_hms_nano(12, 34, 56, 123456789).ok());

        assert_eq!(time_8601("23:45:67"), None);
    }

    #[test]
    fn test_datetime_ts() {
        assert_eq!(datetime_ts(&datetime!(2024-01-30 01:23:45)), 1706552625000);
    }

    // #[test]
    // fn test_datetime_th_from_ulid() {
    //     assert_eq!(datetime_th_from_ulid(&Ulid::from_parts(1706552625000, 1)), String::from("30 ม.ค.2567 01:23 น."));
    // }

    #[test]
    fn test_datetime_th() {
        assert_eq!(datetime_th(&datetime!(2024-01-30 01:23)), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_th(&datetime!(2024-01-30 01:23:45)), String::from("30 ม.ค.2567 01:23 น."));
    }

    #[test]
    fn test_date_th() {
        assert_eq!(date_th(&date!(2024-01-30)), String::from("30 ม.ค.2567"));
    }

    #[test]
    fn test_time_hm() {
        assert_eq!(time_hm(&time!(1:23)), String::from("01:23 น."));
        assert_eq!(time_hm(&time!(1:23:45)), String::from("01:23 น."));
        assert_eq!(time_hm(&time!(1:23:45.123456789)), String::from("01:23 น."));
    }

    #[test]
    fn test_datetime_str_th() {
        assert_eq!(datetime_str_th("2024-01-30T01:23"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30T01:23:45"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30T01:23:45.123456789"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30T01:23:45.123456789Z"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30T01:23:45.123456789+07:00"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30T01:23:45.123456789-08:00"), String::from("30 ม.ค.2567 01:23 น."));

        assert_eq!(datetime_str_th("2024-01-30 01:23"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30 01:23:45"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30 01:23:45.123456789"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30 01:23:45.123456789Z"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30 01:23:45.123456789+07:00"), String::from("30 ม.ค.2567 01:23 น."));
        assert_eq!(datetime_str_th("2024-01-30 01:23:45.123456789-08:00"), String::from("30 ม.ค.2567 01:23 น."));

        assert_eq!(datetime_str_th("2024-01-33 01:23"), String::new());
        assert_eq!(datetime_str_th("2024-01-30"), String::new());
        assert_eq!(datetime_str_th("01:23"), String::new());
    }

    #[test]
    fn test_date_str_th() {
        assert_eq!(date_str_th("2024-01-30"), String::from("30 ม.ค.2567"));
        assert_eq!(date_str_th("2024-01-33"), String::new());
    }

    #[test]
    fn test_time_str_hm() {
        assert_eq!(time_str_hm("01:23"), String::from("01:23 น."));
        assert_eq!(time_str_hm("01:23:45"), String::from("01:23 น."));
        assert_eq!(time_str_hm("01:23:45.123456789"), String::from("01:23 น."));
        assert_eq!(time_str_hm("01:23:45.123456789Z"), String::from("01:23 น."));
        assert_eq!(time_str_hm("01:23:45.123456789+07:00"), String::from("01:23 น."));
        assert_eq!(time_str_hm("01:23:45.123456789-08:00"), String::from("01:23 น."));

        assert_eq!(time_str_hm("01:66"), String::new());
    }

    #[test]
    fn test_duration_hm() {
        assert_eq!(duration_hm(Duration::new(119, 0)), String::from("1 นาที"));
        assert_eq!(duration_hm(Duration::new(32400, 0)), String::from("9 ชั่วโมง"));
        assert_eq!(duration_hm(Duration::new(32460, 0)), String::from("9 ชั่วโมง 1 นาที"));
    }
}
