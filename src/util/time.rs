use chrono::prelude::*;
use chrono::NaiveDateTime;
pub fn get_now() -> NaiveDateTime {
    let dt = Local::now();
    let d = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).expect("Failed to parse NavieDate from ymd");
    let t = NaiveTime::from_hms_opt(dt.hour(), dt.minute(), dt.second()).expect("Failed to parse NavieTime from hms");
    NaiveDateTime::new(d, t)
}
