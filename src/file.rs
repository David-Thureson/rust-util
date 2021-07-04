use std::fs::DirEntry;
use chrono::{NaiveDate, DateTime, Local, Datelike};
use std::fs;
use std::path::{Path, PathBuf};
use crate::parse::after;
use crate::format::format_zeros;

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

pub fn path_create_if_necessary_result<P>(path: P) -> Result<(), String>
    where P: AsRef<Path>
{
    if fs::metadata(path.as_ref()).is_ok() {
        Ok(())
    } else {
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn back_up_folder_date_and_number_result<S, D>(path_source: S, path_dest_base: D, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    let path_dest= path_folder_next_number_result(path_dest_base, prefix, digits)?;
    match back_up_folder(path_source, &path_dest) {
        Ok(()) => Ok(path_dest),
        Err(msg) => Err(msg),
    }
}

pub fn back_up_folder<S, D>(path_source: S, path_dest: D) -> Result<(), String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    path_create_if_necessary_result(path_dest)




}

pub fn path_folder_highest_number_result<P>(path_base: P, prefix: &str) -> Result<Option<PathBuf>, String>
    where P: AsRef<Path>
{
    Ok(folder_highest_number_result(path_base.as_ref(), prefix)?
        .map(|(number, digits)| path_folder_with_number(path_base, prefix, number, digits)))
}

pub fn path_folder_next_number_result<P>(path_base: P, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where P: AsRef<Path>
{
    let found_highest_number = folder_highest_number_result(path_base.as_ref(), prefix)?.map(|(number, _)| number);
    let next_number = found_highest_number.unwrap_or(0) + 1;
    Ok(path_folder_with_number(path_base, prefix, next_number, digits))
}

// TO DO: Check whether each entry is really a folder.
fn folder_highest_number_result<P>(path_base: P, prefix: &str) -> Result<Option<(usize, usize)>, String>
    where P: AsRef<Path>
{
    let prefix = prefix.to_lowercase();
    let mut max_number = None;
    let mut digits = 1;
    match fs::read_dir(path_base) {
        Ok(iter) => {
            iter.for_each(|read_dir| {
                let dir_entry = read_dir.as_ref().unwrap();
                let folder_name = dir_entry_to_file_name(dir_entry).to_lowercase();
                if folder_name.starts_with(&prefix) {
                    let number = after(&folder_name, &prefix).trim();
                    let digits_this_entry = number.len();
                    let number = number.parse::<usize>().unwrap();
                    if number >= max_number.unwrap_or(0) {
                        max_number = Some(number);
                        digits = digits_this_entry;
                    }
                }
            });
        },
        Err(e) => {
            return Err(e.to_string())
        },
    }
    Ok(max_number.map(|max_number| (max_number, digits)))
}

fn path_folder_with_number<P>(path_base: P, prefix: &str, number: usize, digits: usize) -> PathBuf
    where P: AsRef<Path>
{
    let folder_name = format!("{} {}", prefix, format_zeros(number, digits));
    let mut path_buf = PathBuf::new();
    path_buf.push(path_base);
    path_buf.push(&folder_name);
    path_buf
}

/*
pub fn path_file_highest_number(path: &str, prefix: &str, extension: &str) -> Option<String> {
    file_highest_number(path, prefix, extension)
        .map(|(number, digits)| path_file_with_number(path, prefix, extension, number, digits))
}

pub fn path_file_next_number(path: &str, prefix: &str, extension: &str, digits: usize) -> String {
    let found_highest_number = file_highest_number(path, prefix, extension).map(|(number, _)| number);
    let next_number = found_highest_number.unwrap_or(0) + 1;
    path_file_with_number(path, prefix, extension, next_number, digits)
}

fn file_highest_number(path: &str, prefix: &str, extension: &str) -> Option<(usize, usize)> {
    let prefix = prefix.to_lowercase();
    let extension = format!(".{}", extension.to_lowercase());
    let mut max_number = None;
    let mut digits = 1;
    for path in fs::read_dir(path).unwrap() {
        let dir_entry = path.as_ref().unwrap();
        let file_name = file_io::dir_entry_to_file_name(dir_entry).to_lowercase();
        if file_name.starts_with(&prefix) && file_name.ends_with(&extension) {
            let number = between(&file_name, &prefix, &extension).trim();
            digits_this_entry = number.len();
            let number = number.parse::<usize>().unwrap();
            if number >= max_number.unwrap_or(0) {
                max_number = Some(number);
                digits = digits_this_entry;
            }
        }
    }
    max_number.map(|max_number| (max_number, digits))
}

fn path_file_with_number(path: &str, prefix: &str, extension: &str, number: usize, digits: usize) -> String {
    format!("{}/{} {}.{}", path, prefix, format_zeros(number, digits), extension)
}
*/




/*
pub fn path_folder_highest_number(path: &str, prefix: &str) -> Option<String> {
    folder_highest_number(path, prefix)
        .map(|(number, digits)| path_folder_with_number(path, prefix, number, digits))
}

pub fn path_folder_next_number(path: &str, prefix: &str, digits: usize) -> String {
    let found_highest_number = folder_highest_number(path, prefix).map(|(number, _)| number);
    let next_number = found_highest_number.unwrap_or(0) + 1;
    path_folder_with_number(path, prefix, next_number, digits)
}

// TO DO: Check whether each entry is really a folder.
fn folder_highest_number(path: &str, prefix: &str) -> Option<(usize, usize)> {
    let prefix = prefix.to_lowercase();
    let mut max_number = None;
    let mut digits = 1;
    for path in fs::read_dir(path).unwrap() {
        let dir_entry = path.as_ref().unwrap();
        let folder_name = file_io::dir_entry_to_file_name(dir_entry).to_lowercase();
        if folder_name.starts_with(&prefix) {
            let number = after(&file_name, &prefix).trim();
            digits_this_entry = number.len();
            let number = number.parse::<usize>().unwrap();
            if number >= max_number.unwrap_or(0) {
                max_number = Some(number);
                digits = digits_this_entry;
            }
        }
    }
    max_number.map(|max_number| (max_number, digits))
}

fn path_folder_with_number(path: &str, prefix: &str, number: usize, digits: usize) -> String {
    format!("{}/{} {}", path, prefix, format_zeros(number, digits))
}

pub fn path_file_highest_number(path: &str, prefix: &str, extension: &str) -> Option<String> {
    file_highest_number(path, prefix, extension)
        .map(|(number, digits)| path_file_with_number(path, prefix, extension, number, digits))
}

pub fn path_file_next_number(path: &str, prefix: &str, extension: &str, digits: usize) -> String {
    let found_highest_number = file_highest_number(path, prefix, extension).map(|(number, _)| number);
    let next_number = found_highest_number.unwrap_or(0) + 1;
    path_file_with_number(path, prefix, extension, next_number, digits)
}

fn file_highest_number(path: &str, prefix: &str, extension: &str) -> Option<(usize, usize)> {
    let prefix = prefix.to_lowercase();
    let extension = format!(".{}", extension.to_lowercase());
    let mut max_number = None;
    let mut digits = 1;
    for path in fs::read_dir(path).unwrap() {
        let dir_entry = path.as_ref().unwrap();
        let file_name = file_io::dir_entry_to_file_name(dir_entry).to_lowercase();
        if file_name.starts_with(&prefix) && file_name.ends_with(&extension) {
            let number = between(&file_name, &prefix, &extension).trim();
            digits_this_entry = number.len();
            let number = number.parse::<usize>().unwrap();
            if number >= max_number.unwrap_or(0) {
                max_number = Some(number);
                digits = digits_this_entry;
            }
        }
    }
    max_number.map(|max_number| (max_number, digits))
}

fn path_file_with_number(path: &str, prefix: &str, extension: &str, number: usize, digits: usize) -> String {
    format!("{}/{} {}.{}", path, prefix, format_zeros(number, digits), extension)
}
*/
















