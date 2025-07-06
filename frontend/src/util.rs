use time::{PrimitiveDateTime, OffsetDateTime, Date, Month, Time, format_description::well_known::Iso8601, macros::offset};

pub fn js_now() -> PrimitiveDateTime {
    let local = OffsetDateTime::from_unix_timestamp(get_now_wasm() as i64).unwrap().to_offset(offset!(+7));
    PrimitiveDateTime::new(local.date(), local.time())
}

fn get_now_wasm() -> u64 {
    // UTC in milliseconds
    let now = js_sys::Date::now() as u64;
    // log::debug!("{now}");

    now.saturating_div(1_000)
}


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



/// parse Date to Thai string `24 ส.ค.2521`
pub fn date_th(date: &Date) -> String {
    let day = date.day();
    let month = date.month();
    let year = date.year();

    format!("{} {}{}", day, month_thai(&month), year + 543)
}
/// parse Time to Thai string `05:25 น.`
pub fn time_hm(time: &Time) -> String {
    let hour = time.hour();
    let minutes = time.minute();
    format!("{:0>2}:{:0>2} น.", hour, minutes)
}
// /// parse PrimitiveDateTime to Thai string `24 ส.ค.2521 05:25 น.`
// pub fn datetime_th(date_time: &PrimitiveDateTime) -> String {
//     let date = date_time.date();
//     let time = date_time.time();

//     [date_th(&date), time_hm(&time)].join(" ")
// }



/// parse Date to Buddhism `DD/MM/YYYY`
pub fn date_pat(date: &Date) -> String {
    let day = date.day();
    let month = date.month() as u8;
    let year = date.year();

    format!("{:0>2}/{:0>2}/{}", day, month, year + 543)
}
/// parse Time to `HH:MM`
pub fn time_pat(time: &Time) -> String {
    let hour = time.hour();
    let minutes = time.minute();
    format!("{:0>2}:{:0>2}", hour, minutes)
}
/// parse PrimitiveDateTime to Buddhism `DD/MM/YYYY HH:MM`
pub fn datetime_pat(date_time: &PrimitiveDateTime) -> String {
    let date = date_time.date();
    let time = date_time.time();

    [date_pat(&date), time_pat(&time)].join(" ")
}



/// parse Buddhism `DDMMYYYY HHMM` or `DD/MM/YYYY HH:MM` string to time::PrimitiveDateTime, error will be `None`<br>
/// any `NOT-NUMERIC` seperate supported
pub fn datetime_from_pat(text: &str) -> Option<PrimitiveDateTime> {
    let dt = text.split(' ').map(str::trim).collect::<Vec<&str>>();
    if dt.len() != 2 {
        return None;
    }
    let date = date_from_pat(dt[0]);
    let time = time_from_pat(dt[1]);

    if let (Some(d), Some(t)) = (date, time) { Some(PrimitiveDateTime::new(d, t)) } else { None }
}
/// parse Buddhism `DDMMYYYY` or `DD/MM/YYYY` string to time::Date, error will be `None`<br>
/// any `NOT-NUMERIC` seperate supported
pub fn date_from_pat(text: &str) -> Option<Date> {
    // try str.split() method fitst, not allocate, faster
    // failover with str.chars() method, allocate, slower
    if text.contains(['/','-','.',':']) {
        let dmy = text.split(['/','-','.',':']).map(str::trim).collect::<Vec<&str>>();
        if dmy.len() > 2 {
            if let (Some(d), Some(m), Some(y)) = (dmy[0].parse::<u8>().ok(), dmy[1].parse::<u8>().ok(), dmy[2].parse::<i32>().ok()) {
                let decate = ((js_now().year() + 543) / 100) * 100;
                date_from_pat_inner(d, m, if y > 543 {y} else {y + decate} - 543)  
            } else {
                None
            } 
        } else {
            None
        }
    } else {
        let c = text.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
        match c.len() {
            6..8 => {
                let dmy = c.chunks(2).map(|tuple| String::from_iter(tuple)).collect::<Vec<String>>();
                if let (Some(d), Some(m), Some(y)) = (dmy[0].parse::<u8>().ok(), dmy[1].parse::<u8>().ok(), dmy[2].parse::<i32>().ok()) {
                    let decate = ((js_now().year() + 543) / 100) * 100;
                    date_from_pat_inner(d, m, y + decate - 543)                   
                } else {
                    None
                }  
            }
            8.. => {
                if let (Some(d), Some(m), Some(y)) = (
                    String::from_iter([c[0], c[1]]).parse::<u8>().ok(),
                    String::from_iter([c[2], c[3]]).parse::<u8>().ok(),
                    String::from_iter([c[4], c[5], c[6], c[7]]).parse::<i32>().ok(),
                ) {
                    date_from_pat_inner(d, m, if y > 543 {y - 543} else {y})
                } else {
                    None
                } 
            }
            _ => None,
        }
    }
}
fn date_from_pat_inner(d: u8, m: u8, y: i32) -> Option<Date> {
    // convertable MMDDYYYY
    if m > 12 && d < 13 {
        if let Some(mm) = Month::try_from(d).ok() {
            Date::from_calendar_date(y, mm, m).ok()
        } else {
            None
        }
    } else if let Some(mm) = Month::try_from(m).ok() {
        Date::from_calendar_date(y, mm, d).ok()
    } else {
        None
    }
}
/// parse `HHMM` or `HH:MM` string to time::Time, error will be `None`<br>
/// any `NOT-NUMERIC` seperate supported
pub fn time_from_pat(text: &str) -> Option<Time> {
    // try str.split() method fitst, not allocate, faster
    // failover with str.chars() method, allocate, slower
    if text.contains(['/','-','.',':']) {
        let hm = text.split(['/','-','.',':']).map(str::trim).collect::<Vec<&str>>();
        if hm.len() > 1 {
            time_from_pat_inner(hm[0].parse::<u8>().ok(), hm[1].parse::<u8>().ok())
        } else {
            None
        }
    } else {
        let c = text.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
        match c.len() {
            4.. => {
                let hm = c.chunks(2).take(2).map(|tuple| String::from_iter(tuple)).collect::<Vec<String>>();
                time_from_pat_inner(hm[0].parse::<u8>().ok(), hm[1].parse::<u8>().ok())
            }
            3 => {
                let end_two = String::from_iter([c[1],c[2]]).parse::<u8>().ok();
                if end_two.map(|u| u > 60).unwrap_or_default() {
                    let first_two = String::from_iter([c[0],c[1]]).parse::<u8>().ok();
                    if first_two.map(|u| u < 24).unwrap_or_default() {
                        time_from_pat_inner(String::from_iter([c[0],c[1]]).parse::<u8>().ok(), String::from_iter([c[2]]).parse::<u8>().ok())
                    } else {
                        None
                    }
                } else {
                    time_from_pat_inner(String::from_iter([c[0]]).parse::<u8>().ok(), String::from_iter([c[1],c[2]]).parse::<u8>().ok())
                }
            }
            2 => {
                if (c[0] == '1') || (c[0] == '2' && ['0','1','2','3'].contains(&c[1])) {
                    time_from_pat_inner(String::from_iter([c[0],c[1]]).parse::<u8>().ok(), Some(0))
                } else {
                    time_from_pat_inner(String::from_iter([c[0]]).parse::<u8>().ok(), String::from_iter([c[1]]).parse::<u8>().ok())
                }
            }
            1 => {
                time_from_pat_inner(String::from_iter([c[0]]).parse::<u8>().ok(), Some(0))
            }
            0 => None,
        }
    }
}
fn time_from_pat_inner(h_opt: Option<u8>, m_opt: Option<u8>) -> Option<Time> {
    if let (Some(h), Some(m)) = (h_opt, m_opt) {
        Time::from_hms(h, m, 0).ok()
    } else {
        None
    }  
}


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

pub fn date_str_th(text: &str) -> String {
    date_8601(text).map(|d| date_th(&d)).unwrap_or_default()
}

pub fn time_str_hm(text: &str) -> String {
    time_8601(text).map(|t| time_hm(&t)).unwrap_or_default()
}

fn month_thai(mm: &Month) -> &'static str {
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

#[cfg(test)]
#[allow(dead_code)]
#[rustfmt::skip]
pub mod wasm_tests {
    // wasm-pack test --chrome frontend
    use super::*;
    use time::macros::{date, datetime, time};
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn test_date_pat() {
        assert_eq!(date_pat(&date!(2022-01-30)), String::from("30/01/2565"));
    }

    #[wasm_bindgen_test]
    pub fn test_time_pat() {
        assert_eq!(time_pat(&time!(14:55)), String::from("14:55"));
    }

    #[wasm_bindgen_test]
    pub fn test_datetime_pat() {
        assert_eq!(datetime_pat(&datetime!(2022-01-30 14:55)), String::from("30/01/2565 14:55"));
    }

    #[wasm_bindgen_test]
    pub fn test_date_from_pat() {
        let decate = ((js_now().year() + 543) / 100) * 100;
        // DD/MM/YYYY'
        assert_eq!(date_from_pat("30/01/2565"), Some(date!(2022-01-30)));
        assert_eq!(date_from_pat("30/01/65"), Date::from_calendar_date(decate + 65 - 543, Month::January, 30).ok());
        assert_eq!(date_from_pat("2/1/65"), Date::from_calendar_date(decate + 65 - 543, Month::January, 2).ok());
        assert_eq!(date_from_pat("30012565"), Some(date!(2022-01-30)));
        assert_eq!(date_from_pat("300165"), Date::from_calendar_date(decate + 65 - 543, Month::January, 30).ok());
        // MM/DD/YYYY
        assert_eq!(date_from_pat("11/13/2565"), Some(date!(2022-11-13)));
        assert_eq!(date_from_pat("11/13/65"), Date::from_calendar_date(decate + 65 - 543, Month::November, 13).ok());
        assert_eq!(date_from_pat("11132565"), Some(date!(2022-11-13)));
        assert_eq!(date_from_pat("111365"), Date::from_calendar_date(decate + 65 - 543, Month::November, 13).ok());
        // seperators
        assert_eq!(date_from_pat("30-01-2565"), Some(date!(2022-01-30)));
        assert_eq!(date_from_pat("30.01.2565"), Some(date!(2022-01-30)));
        assert_eq!(date_from_pat("30:01:2565"), Some(date!(2022-01-30)));
        assert_eq!(date_from_pat("30.01:2565"), Some(date!(2022-01-30)));
        assert_eq!(date_from_pat("30x01z2565"), Some(date!(2022-01-30)));
        // overflow
        assert_eq!(date_from_pat("300125659"), Some(date!(2022-01-30)));
        assert_eq!(date_from_pat("3001659"), Date::from_calendar_date(decate + 65 - 543, Month::January, 30).ok());
        // failed
        assert_eq!(date_from_pat("40/1/2565"), None);
        assert_eq!(date_from_pat("13/13/2565"), None);
        assert_eq!(date_from_pat("131313"), None);
        assert_eq!(date_from_pat("12121"), None);
    }

    #[wasm_bindgen_test]
    pub fn test_time_from_pat() {
        assert_eq!(time_from_pat("14:55"), Some(time!(14:55)));
        assert_eq!(time_from_pat("2:55"), Some(time!(02:55)));
        assert_eq!(time_from_pat("2:5"), Some(time!(02:05)));
        assert_eq!(time_from_pat("1455"), Some(time!(14:55)));
        assert_eq!(time_from_pat("025"), Some(time!(00:25))); // not 02:05
        assert_eq!(time_from_pat("125"), Some(time!(01:25))); // not 12:05
        assert_eq!(time_from_pat("235"), Some(time!(02:35))); // not 23:05
        assert_eq!(time_from_pat("245"), Some(time!(02:45)));
        assert_eq!(time_from_pat("345"), Some(time!(03:45)));
        assert_eq!(time_from_pat("066"), Some(time!(06:06)));
        assert_eq!(time_from_pat("166"), Some(time!(16:06)));
        assert_eq!(time_from_pat("02"), Some(time!(00:02)));
        assert_eq!(time_from_pat("10"), Some(time!(10:00)));
        assert_eq!(time_from_pat("23"), Some(time!(23:00)));
        assert_eq!(time_from_pat("24"), Some(time!(02:04)));
        assert_eq!(time_from_pat("99"), Some(time!(09:09)));
        assert_eq!(time_from_pat("0"), Some(time!(00:00)));
        // seperators
        assert_eq!(time_from_pat("14-55"), Some(time!(14:55)));
        assert_eq!(time_from_pat("14/55"), Some(time!(14:55)));
        assert_eq!(time_from_pat("14.55"), Some(time!(14:55)));
        assert_eq!(time_from_pat("14x55"), Some(time!(14:55)));
        // overflow
        assert_eq!(time_from_pat("14559"), Some(time!(14:55)));
        // failed
        assert_eq!(time_from_pat("25:55"), None);
        assert_eq!(time_from_pat("14:65"), None);
        assert_eq!(time_from_pat("9999"), None);
        assert_eq!(time_from_pat("999"), None);
        assert_eq!(time_from_pat(""), None);
    }

    #[wasm_bindgen_test]
    pub fn test_datetime_from_pat() {
        assert_eq!(datetime_from_pat("30/01/2565 14:55"), Some(datetime!(2022-01-30 14:55)));
        // failed
        assert_eq!(datetime_from_pat("30/01/2565T14:55"), None);
    }
}