// Date/time formatting escape sequences: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html

use chrono::{NaiveDate, DateTime, Local, Datelike};
use std::time::{SystemTime, Instant};
use std::collections::BTreeMap;

const FORMAT_DATE_SORTABLE: &str = "%Y-%m-%d";  // Like "2022-01-03".
const FORMAT_DATE_COMPACT: &str = "%Y%m%d";  // Like "20220103".
const FORMAT_DATE_DOC: &str = "%Y-%b-%d"; // Like "2022-Jan-03".
const FORMAT_DATE_MON: &str = "%b %-d, %Y"; // Like "Jan 3, 2022".

pub fn naive_date_now() -> NaiveDate {
    let date_time: DateTime<Local> = Local::now();
    NaiveDate::from_ymd(date_time.year(), date_time.month(), date_time.day())
}

pub fn date_for_file_name(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn date_for_file_name_now() -> String {
    date_for_file_name(&naive_date_now())
}

// Like 2022-01-03.
pub fn naive_date_to_sortable_format(value: &NaiveDate) -> String {
    value.format(FORMAT_DATE_SORTABLE).to_string()
}

// Like 2022-01-03.
pub fn naive_date_from_sortable_format(value: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(value.trim(), FORMAT_DATE_SORTABLE) {
        Ok(date) => Ok(date),
        Err(err) => Err(format!("Error trying to parse \"{}\" as a sortable-format date which should look like \"2022-01-03\": \"{}\".", value, err)),
    }
}

// Like 20220103.
pub fn naive_date_to_compact_format(value: &NaiveDate) -> String {
    value.format(FORMAT_DATE_COMPACT).to_string()
}

// Like 2022-01-03.
pub fn naive_date_from_compact_format(value: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(value.trim(), FORMAT_DATE_COMPACT) {
        Ok(date) => Ok(date),
        Err(err) => Err(format!("Error trying to parse \"{}\" as a compact-format date which should look like \"20220103\": \"{}\".", value, err)),
    }
}

// Like "2022-Jan-03".
pub fn naive_date_to_doc_format(date: &NaiveDate) -> String {
    date.format(FORMAT_DATE_DOC).to_string()
}

// Like "Jan 3, 2022".
pub fn naive_date_to_mon_format(date: &NaiveDate) -> String {
    date.format(FORMAT_DATE_MON).to_string()
}

// Like "2022-Jan".
pub fn year_month_to_doc_format(year: i32, month: u32) -> String {
    let date = NaiveDate::from_ymd(year, month, 1);
    naive_date_to_doc_format(&date)[..8].to_string()
}

// Like "Jan, 2022".
pub fn year_month_to_mon_format(year: i32, month: u32) -> String {
    let date = NaiveDate::from_ymd(year, month, 1);
    let mon = &naive_date_to_mon_format(&date)[..3];
    format!("{}, {}", mon, year)
}

// Like "2022-Jan-03".
pub fn naive_date_from_doc_format(value: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(value.trim(), FORMAT_DATE_DOC) {
        Ok(date) => Ok(date),
        Err(err) => Err(format!("Error trying to parse \"{}\" as a doc-format date which should look like \"2022-Jan-03\": \"{}\".", value, err)),
    }
}

pub fn datetime_as_date(value: &DateTime<Local>) -> String {
    value.format("%Y-%m-%d").to_string()
}

pub fn datetime(value: &DateTime<Local>) -> String {
    value.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn systemtime_as_date(value: &SystemTime) -> String {
    datetime_as_date(&DateTime::<Local>::from(*value))
}

pub fn systemtime(value: &SystemTime) -> String {
    datetime(&DateTime::<Local>::from(*value))
}

pub fn print_elapsed<F>(display: bool, case_label: &str, step_label: &str, mut f: F)
    where
        F: FnMut() -> (),
{
    let start = Instant::now();
    f();
    print_elapsed_from_start(display, case_label, step_label, start);
}

pub fn print_elapsed_from_start(display: bool, case_label: &str, step_label: &str, start: Instant) {
    if display {
        println!("\n{}: {} = {:?}", case_label, step_label, start.elapsed());
    }
}

pub fn year_map(mut dates: Vec<NaiveDate>) -> BTreeMap<i32, Vec<NaiveDate>> {
    dates.sort();
    dates.dedup();
    let mut map = BTreeMap::new();
    for date in dates.drain(..) {
        let entry = map.entry(date.year()).or_insert(vec![]);
        entry.push(date);
    }
    map
}

pub fn year_month_map(mut dates: Vec<NaiveDate>) -> BTreeMap<i32, BTreeMap<u32, Vec<NaiveDate>>> {
    dates.sort();
    dates.dedup();
    let mut map = BTreeMap::new();
    for date in dates.drain(..) {
        let year_entry = map.entry(date.year()).or_insert(BTreeMap::new());
        let month_entry = year_entry.entry(date.month()).or_insert(vec![]);
        month_entry.push(date);
    }
    map
}

