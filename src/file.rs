use std::fs::{DirEntry, FileType};
use chrono::{NaiveDate, DateTime, Local, Datelike};
use std::fs;
use std::path::{Path, PathBuf};
use crate::parse::after;
use crate::format::format_zeros;

use crate::*;

// pub fn dir_entry_to_file_name(dir_entry: &DirEntry) -> String {
//     dir_entry.file_name().to_str().unwrap().to_string()
// }

pub fn path_file_name_r<P>(path: P) -> Result<String, String>
    where P: AsRef<Path>,
{
    match path.as_ref().file_name() {
        Some(file_name) => Ok(file_name.to_string_lossy().parse().unwrap()),
        None => Err(format!("No file name found for path \"{}\".", path.as_ref().to_string_lossy())),
    }
}

pub fn path_exists<P>(path: P) -> bool
    where P: AsRef<Path>,
{
    fs::metadata(path).is_ok()
}

pub fn path_exists_r<P>(path: P) -> Result<(), String>
    where P: AsRef<Path>,
{
    if path_exists(&path) {
        Ok(())
    } else {
        Err(format!("Path does not exist: \"{}\".", path.as_ref().to_string_lossy()))
    }
}

pub fn path_is_new_r<P>(path: P) -> Result<(), String>
    where P: AsRef<Path>,
{
    if path_exists(&path) {
        Err(format!("Path already exists: \"{}\".", path.as_ref().to_string_lossy()))
    } else {
        Ok(())
    }
}

/*
pub fn result_to_string_error<T>(result: Result<T, dyn ToString>) -> Result<T, String> {
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}
*/

pub fn file_type_r(entry: &DirEntry) -> Result<FileType, String> {
    result_to_string_error!(entry.file_type())
}

pub fn is_directory_r<P>(path: P) -> Result<bool, String>
    where P: AsRef<Path>,
{
    let metadata = result_to_string_error!(path.as_ref().metadata())?;
    Ok(metadata.is_dir())
}



pub fn path_entries<P>(path: P) -> Result<Vec<PathBuf>, String>
    where P: AsRef<Path>,
{
    match fs::read_dir(path) {
        Ok(read_dir) => {
            let mut entries: Vec<PathBuf> = vec![];
            for dir_entry_r in read_dir {
                match dir_entry_r.as_ref() {
                    Ok(dir_entry) => {
                        entries.push(dir_entry.path());
                    },
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };
            }
            Ok(entries)
        },
        Err(e) => {
            return Err(e.to_string())
        },
    }
}

pub fn dir_entry_to_naive_date(dir_entry: &DirEntry) -> NaiveDate {
    let date = dir_entry.metadata().unwrap().modified().unwrap();
    let date: DateTime<Local> = chrono::DateTime::from(date);
    NaiveDate::from_ymd(date.year(), date.month(), date.day())
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

pub fn path_create_if_necessary_r<P>(path: P) -> Result<(), String>
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

pub fn back_up_folder_date_and_number_r<S, D>(path_source: S, path_dest_base: D, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    let path_dest= path_folder_next_number_r(path_dest_base, prefix, digits)?;
    match copy_folder_to_new_folder(path_source, &path_dest) {
        Ok(()) => Ok(path_dest),
        Err(msg) => Err(msg),
    }
}

pub fn copy_folder_to_new_folder<S, D>(path_source: S, path_dest: D) -> Result<(), String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    // The path must not already exist.
    path_is_new_r(&path_dest)?;
    copy_folder(path_source, path_dest)
}

pub fn copy_folder<S, D>(path_source: S, path_dest: D) -> Result<(), String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    // path_create_if_necessary_r(&path_dest)?;
    copy_folder_recursive(path_source, path_dest)
}

fn copy_folder_recursive<S, D>(path_source: S, path_dest: D) -> Result<(), String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    // From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust.
    path_create_if_necessary_r(&path_dest)?;
    for entry in path_entries(&path_source)? {
        let file_name = path_file_name_r(&entry)?;
        let path_dest_one = path_dest.as_ref().join(file_name);
        if is_directory_r(&entry)? {
            copy_folder_recursive(entry, path_dest_one)?;
        } else {
            result_to_string_error!(fs::copy(entry, path_dest_one))?;
        }
    }
    Ok(())
}

pub fn path_folder_highest_number_r<P>(path_base: P, prefix: &str) -> Result<Option<PathBuf>, String>
    where P: AsRef<Path>
{
    Ok(folder_highest_number_r(path_base.as_ref(), prefix)?
        .map(|(number, digits)| path_folder_with_number(path_base, prefix, number, digits)))
}

pub fn path_folder_next_number_r<P>(path_base: P, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where P: AsRef<Path>
{
    let found_highest_number = folder_highest_number_r(path_base.as_ref(), prefix)?.map(|(number, _)| number);
    let next_number = found_highest_number.unwrap_or(0) + 1;
    Ok(path_folder_with_number(path_base, prefix, next_number, digits))
}

// TO DO: Check whether each entry is really a folder.
fn folder_highest_number_r<P>(path_base: P, prefix: &str) -> Result<Option<(usize, usize)>, String>
    where P: AsRef<Path>
{
    let prefix = prefix.to_lowercase();
    let mut max_number = None;
    let mut digits = 1;
    for entry in path_entries(&path_base)? {
        let folder_name = path_file_name_r(entry)?.to_lowercase();
        if folder_name.starts_with(&prefix) {
            let number = after(&folder_name, &prefix).trim();
            let digits_this_entry = number.len();
            let number = number.parse::<usize>().unwrap();
            if number >= max_number.unwrap_or(0) {
                max_number = Some(number);
                digits = digits_this_entry;
            }
        }
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
pub fn main() {
    try_
}
*/

#[cfg(test)]
mod tests {
    // This must be run single-threaded, so run:
    //   cargo test -- --test-threads=1
    // or
    //   cargo test -- --nocapture --test-threads=1

    use super::*;

    const PATH_TEST: &str = r"C:\Test_Rust_File_Functions";

    fn setup() {
        if path_exists(PATH_TEST) {
            fs::remove_dir_all(PATH_TEST).unwrap();
        }
        fs::create_dir_all(PATH_TEST).unwrap();
    }

    //#[inline]
    fn assert_err_msg<T>(result: Result<T, String>, exp_msg: &str) {
        match result {
            Ok(_) => panic!("Result was not an error. Expected the message \"{}\".", exp_msg),
            Err(msg) => {
                if msg != exp_msg {
                    panic!("Result was an error as expected, but the message was \"{}\". Expected the message \"{}\".", msg, exp_msg)
                }
            }
        }
    }

    #[test]
    fn test_path_file_name_r() {
        setup();
        let file_name = "Abc.txt";
        let path = [PATH_TEST, file_name].iter().collect::<PathBuf>();
        assert_eq!(file_name, path_file_name_r(path).unwrap());
    }

    #[test]
    fn test_path_exists() {
        setup();
        let path = [PATH_TEST, "Subfolder A"].iter().collect::<PathBuf>();
        assert_eq!(false, path_exists(&path));
        fs::create_dir_all(&path).unwrap();
        assert!(path_exists(path));
    }

    #[test]
    fn test_path_exists_r() {
        setup();
        let folder_name = "Subfolder B";
        let path = [PATH_TEST, folder_name].iter().collect::<PathBuf>();
        let exp_msg = format!("Path does not exist: \"{}\\{}\".", PATH_TEST, folder_name);
        assert_err_msg(path_exists_r(&path), &exp_msg);
        fs::create_dir_all(&path).unwrap();
        assert!(path_exists_r(&path).is_ok());
    }

}


