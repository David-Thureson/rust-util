use std::fs::{DirEntry, FileType};
use chrono::{NaiveDate, DateTime, Local, Datelike};
use std::fs;
use std::path::{Path, PathBuf};
use crate::parse::after;
use crate::format::format_zeros;

use crate::*;

pub fn dir_entry_to_file_name(dir_entry: &DirEntry) -> String {
    dir_entry.file_name().to_str().unwrap().to_string()
}

pub fn path_buf(components: &[&str]) -> PathBuf {
    components.iter().collect::<PathBuf>()
}

pub fn path_name<P>(path: P) -> String
    where P: AsRef<Path>,
{
    path.as_ref().to_string_lossy().to_string()
}

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
pub fn rse<T>(result: Result<T, dyn ToString>) -> Result<T, String> {
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}
*/

pub fn file_type_r(entry: &DirEntry) -> Result<FileType, String> {
    rse!(entry.file_type())
}

pub fn path_is_directory_r<P>(path: P) -> Result<bool, String>
    where P: AsRef<Path>,
{
    path_exists_r(&path)?;
    let metadata = rse!(path.as_ref().metadata())?;
    Ok(metadata.is_dir())
}

pub fn path_entries_r<P>(path: P) -> Result<Vec<PathBuf>, String>
    where P: AsRef<Path>,
{
    path_exists_r(&path)?;
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

pub fn path_file_names_r<P>(path: P) -> Result<Vec<String>, String>
    where P: AsRef<Path>,
{
    path_exists_r(&path)?;
    let mut file_names = path_entries_r(&path)?.iter()
        .map(|path_buf| path_file_name_r(path_buf).unwrap())
        .collect::<Vec<_>>();
    file_names.sort();
    Ok(file_names)
}

pub fn dir_entry_to_naive_date(dir_entry: &DirEntry) -> NaiveDate {
    let date = dir_entry.metadata().unwrap().modified().unwrap();
    let date: DateTime<Local> = chrono::DateTime::from(date);
    NaiveDate::from_ymd(date.year(), date.month(), date.day())
}

pub fn write_file<P>(path: P, contents: &str) -> Result<(), String>
    where P: AsRef<Path>
{
    path_create_if_necessary_r(&path)?;
    rse!(fs::write(path, contents))
}

pub fn read_file_to_string<P>(path: P) -> Result<String, String>
    where P: AsRef<Path>
{
    path_exists_r(&path)?;
    rse!(fs::read_to_string(path))
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
    path_exists_r(&path_source)?;
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
    path_exists_r(&path_source)?;
    copy_folder_recursive(path_source, path_dest)
}

fn copy_folder_recursive<S, D>(path_source: S, path_dest: D) -> Result<(), String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    // From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust.
    path_exists_r(&path_source)?;
    path_create_if_necessary_r(&path_dest)?;
    for entry in path_entries_r(&path_source)? {
        let file_name = path_file_name_r(&entry)?;
        let path_dest_one = path_dest.as_ref().join(file_name);
        if path_is_directory_r(&entry)? {
            copy_folder_recursive(entry, path_dest_one)?;
        } else {
            rse!(fs::copy(entry, path_dest_one))?;
        }
    }
    Ok(())
}

pub fn path_folder_highest_number_r<P>(path_base: P, prefix: &str) -> Result<Option<PathBuf>, String>
    where P: AsRef<Path>
{
    path_create_if_necessary_r(&path_base)?;
    Ok(folder_highest_number_r(path_base.as_ref(), prefix)?
        .map(|(number, digits)| path_folder_with_number(path_base, prefix, number, digits)))
}

pub fn path_folder_next_number_r<P>(path_base: P, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where P: AsRef<Path>
{
    path_create_if_necessary_r(&path_base)?;
    let found_highest_number = folder_highest_number_r(path_base.as_ref(), prefix)?.map(|(number, _)| number);
    let next_number = found_highest_number.unwrap_or(0) + 1;
    Ok(path_folder_with_number(path_base, prefix, next_number, digits))
}

fn folder_highest_number_r<P>(path_base: P, prefix: &str) -> Result<Option<(usize, usize)>, String>
    where P: AsRef<Path>
{
    path_exists_r(&path_base)?;
    let prefix = prefix.to_lowercase();
    let mut max_number = None;
    let mut digits = 1;
    for entry in path_entries_r(&path_base)? {
        if path_is_directory_r(&entry)? {
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
    }
    Ok(max_number.map(|max_number| (max_number, digits)))
}

fn path_folder_with_number<P>(path_base: P, prefix: &str, number: usize, digits: usize) -> PathBuf
    where P: AsRef<Path>
{
    // It's not necessary for the path to exist yet. This function is simply creating a path from
    // pieces like the prefix and number.
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
    // Unit tests normally run in parallel, so if multiple tests are changing the same folder
    // they'll conflict. Therefore each test works on a subfolder of PATH_TEST named after the name
    // of the test function.
    //
    // To run single-threaded, call:
    //   cargo test -- --test-threads=1
    // or
    //   cargo test -- --nocapture --test-threads=1

    use super::*;

    const PATH_TEST: &str = r"C:\Test_Rust_File_Functions";
    const FOLDER_WITH_FILES: &str = "Subfolder With Files";

    fn setup(test_function_name: &str) -> PathBuf {
        let path = path_buf(&[PATH_TEST, test_function_name]);
        if path_exists(&path) {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[allow(dead_code)]
    fn assert_err<T>(result: Result<T, String>) {
        match result {
            Ok(_) => panic!("Result was not an error."),
            Err(_) => (),
        }
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

    fn assert_err_path_exists<T, P>(result: Result<T, String>, path: P)
        where P: AsRef<Path>
    {
        let exp_msg = format!("Path already exists: \"{}\".", path_name(&path));
        assert_err_msg(result, &exp_msg);
    }

    fn assert_err_path_not_found<T, P>(result: Result<T, String>, path: P)
        where P: AsRef<Path>
    {
        let exp_msg = format!("Path does not exist: \"{}\".", path_name(&path));
        assert_err_msg(result, &exp_msg);
    }

    fn create_test_folder_with_files(path_root: &PathBuf) -> (PathBuf, Vec<String>) {
        let path_folder = path_root.join(FOLDER_WITH_FILES);
        path_is_new_r(&path_folder).unwrap();
        let mut file_names = vec!["File G.txt", "File R.txt", "File B.txt"];
        fs::create_dir(&path_folder).unwrap();
        file_names.iter().for_each(|file_name| fs::write(path_folder.join(file_name), file_name).unwrap());
        file_names.sort();
        (path_folder, str_to_string_vector(&file_names))
    }

    fn add_files_to_test_folder(path_root: &PathBuf) -> (PathBuf, Vec<String>) {
        let path_folder = path_root.join(FOLDER_WITH_FILES);
        path_exists_r(&path_folder).unwrap();
        let file_names = vec!["File 12.txt", "File 08.txt"];
        file_names.iter().for_each(|file_name| fs::write(path_folder.join(file_name), file_name).unwrap());
        let file_names = path_file_names_r(&path_folder).unwrap();
        (path_folder, file_names)
    }

    #[test]
    fn test_path_file_name_r() {
        let path_test_root = setup("test_path_file_name_r");
        let file_name = "Abc.txt";
        let path_file = path_test_root.join(file_name);
        assert_eq!(file_name, path_file_name_r(&path_file).unwrap());
    }

    #[test]
    fn test_path_exists() {
        let path_test_root = setup("test_path_exists");
        let path = path_test_root.join("Subfolder A");
        assert_eq!(false, path_exists(&path));
        fs::create_dir_all(&path).unwrap();
        assert!(path_exists(path));
    }

    #[test]
    fn test_path_exists_r() {
        let path_test_root = setup("fn test_path_exists_r");
        let path = path_test_root.join("Subfolder B");
        assert_err_path_not_found(path_exists_r(&path), &path);
        fs::create_dir_all(&path).unwrap();
        assert!(path_exists_r(&path).is_ok());
    }

    #[test]
    fn test_path_is_new_r() {
        let path_test_root = setup("test_path_is_new_r");
        let path = path_test_root.join("Subfolder C");
        assert!(path_is_new_r(&path).is_ok());
        fs::create_dir_all(&path).unwrap();
        assert_err_path_exists(path_is_new_r(&path), &path);
    }

    #[test]
    fn test_path_is_directory() {
        let path_test_root = setup("test_path_is_directory");
        let path_folder = path_test_root.join("Subfolder D");
        let path_file = path_folder.join("File D.txt");
        assert_err_path_not_found(path_is_directory_r(&path_folder), &path_folder);
        assert_err_path_not_found(path_is_directory_r(&path_file), &path_file);

        fs::create_dir(&path_folder).unwrap();
        assert!(path_is_directory_r(&path_folder).unwrap());
        assert_err_path_not_found(path_is_directory_r(&path_file), &path_file);

        fs::write(&path_file, "File content.").unwrap();
        assert!(path_is_directory_r(&path_folder).unwrap());
        assert_eq!(false, path_is_directory_r(&path_file).unwrap());
    }

    #[test]
    fn test_path_entries_r() {
        let path_test_root = setup("test_path_entries_r");
        let (path_folder, exp_file_names) = create_test_folder_with_files(&path_test_root);
        let mut act_file_names = path_entries_r(&path_folder).unwrap().iter()
            .map(|path_buf| path_file_name_r(path_buf).unwrap())
            .collect::<Vec<_>>();
        act_file_names.sort();
        assert_eq!(exp_file_names, act_file_names);
    }

    #[test]
    fn test_back_up_folder_date_and_number_r() {
        let path_test_root = setup("test_back_up_folder_date_and_number_r");
        let path_missing_folder = path_test_root.join("Missing");
        assert_err_path_not_found(back_up_folder_date_and_number_r(&path_missing_folder, &path_test_root, "Back Red", 3), &path_missing_folder);

        let (path_source_folder, exp_file_names) = create_test_folder_with_files(&path_test_root);

        // The destination includes two levels of folders that don't exist yet.
        let path_dest_base = path_test_root.join("Backup").join("April");

        let path_dest_folder = back_up_folder_date_and_number_r(&path_source_folder, &path_dest_base, "Back Green", 3).unwrap();
        let act_file_names = path_file_names_r(&path_dest_folder).unwrap();
        assert_eq!(exp_file_names, act_file_names);
        let exp_dest_folder_name = format!("{}\\Backup\\April\\Back Green 001", path_name(&path_test_root));
        let act_dest_folder_name = path_name(&path_dest_folder);
        assert_eq!(exp_dest_folder_name, act_dest_folder_name);

        // Add some files to the source and create a second numbered backup.
        let (_, exp_file_names) = add_files_to_test_folder(&path_test_root);
        let path_dest_folder = back_up_folder_date_and_number_r(&path_source_folder, &path_dest_base, "Back Green", 3).unwrap();
        //bg!(path_name(&path_dest_folder));
        let act_file_names = path_file_names_r(&path_dest_folder).unwrap();
        assert_eq!(exp_file_names, act_file_names);
        let exp_dest_folder_name = format!("{}\\Backup\\April\\Back Green 002", path_name(&path_test_root));
        let act_dest_folder_name = path_name(&path_dest_folder);
        assert_eq!(exp_dest_folder_name, act_dest_folder_name);
    }
}
