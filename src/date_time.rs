use chrono::{NaiveDate, DateTime, Local, Datelike};

pub fn naive_date_now() -> NaiveDate {
    let date_time: DateTime<Local> = Local::now();
    NaiveDate::from_ymd(date_time.year(), date_time.month(), date_time.day())
}

