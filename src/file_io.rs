use std::fs::DirEntry;
use chrono::{NaiveDate, DateTime, Local, Datelike};
use std::fs;
use std::path::Path;

pub fn dir_entry_to_naive_date(dir_entry: &DirEntry) -> NaiveDate {
    let date = dir_entry.metadata().unwrap().modified().unwrap();
    let date: DateTime<Local> = chrono::DateTime::from(date);
    NaiveDate::from_ymd(date.year(), date.month(), date.day())
}

pub fn dir_entry_to_file_name(dir_entry: &DirEntry) -> String {
    dir_entry.file_name().to_str().unwrap().to_string()
}

pub fn write_file<P>(path: P, contents: &str)
    where P: AsRef<Path>
{
    let path_string = path.as_ref().to_str().unwrap().to_string();
    fs::write(path, contents).expect(&format!("Unable to write file \"{}\".", path_string));
}

pub fn read_file_to_string<P>(path: P) -> String
    where P: AsRef<Path>
{
    let path_string = path.as_ref().to_str().unwrap().to_string();
    fs::read_to_string(path).expect(&format!("Unable to read file \"{}\".", path_string))
}
