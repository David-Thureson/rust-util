use chrono::{NaiveDate, DateTime, Local, Datelike};

pub fn naive_date_now() -> NaiveDate {
    let date_time: DateTime<Local> = Local::now();
    NaiveDate::from_ymd(date_time.year(), date_time.month(), date_time.day())
}

pub fn date_for_file_name(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn date_for_file_name_now() -> String {
    date_for_file_name(naive_date_now())
}

