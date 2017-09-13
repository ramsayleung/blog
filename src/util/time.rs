use chrono::NaiveDateTime;
use chrono::prelude::*;
pub fn get_now() -> NaiveDateTime {
    let dt = Local::now();
    let d = NaiveDate::from_ymd(dt.year(), dt.month(), dt.day());
    let t = NaiveTime::from_hms(dt.hour(), dt.minute(), dt.second());
    NaiveDateTime::new(d, t)
}
