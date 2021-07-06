use std::fs::{DirEntry, FileType};
use chrono::{NaiveDate, DateTime, Local, Datelike};
use std::fs;
use std::path::{Path, PathBuf};
use crate::parse::after;
use crate::format::format_zeros;

use crate::*;
use crate::date_time::date_for_file_name_now;

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

pub fn path_create_if_necessary_r<P>(path: P) -> Result<bool, String>
    where P: AsRef<Path>
{
    if fs::metadata(path.as_ref()).is_ok() {
        // The path already exists, so return false meaning it wasn't created in this call.
        Ok(false)
    } else {
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(true), // true means the folder was created in this call.
            Err(e) => Err(e.to_string()),
        }
    }
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

pub fn copy_folder_recursive_r<S, D>(path_source: S, path_dest: D) -> Result<(), String>
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
            copy_folder_recursive_r(entry, path_dest_one)?;
        } else {
            rse!(fs::copy(entry, path_dest_one))?;
        }
    }
    Ok(())
}

pub fn copy_folder_to_new_folder_r<S, D>(path_source: S, path_dest: D) -> Result<(), String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    // The path must not already exist.
    path_is_new_r(&path_dest)?;
    copy_folder_recursive_r(path_source, path_dest)
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

pub fn path_folder_dated_next_number_r<P>(path_base: P, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where P: AsRef<Path>
{
    let date_string = date_for_file_name_now();
    let prefix = format!("{} {}", prefix, date_string);
    path_folder_next_number_r(path_base, &prefix, digits)
}

pub fn back_up_folder_next_number_r<S, D>(path_source: S, path_dest_base: D, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    path_exists_r(&path_source)?;
    let path_dest= path_folder_next_number_r(path_dest_base, &prefix, digits)?;
    match copy_folder_to_new_folder_r(path_source, &path_dest) {
        Ok(()) => Ok(path_dest),
        Err(msg) => Err(msg),
    }
}

pub fn back_up_folder_dated_next_number_r<S, D>(path_source: S, path_dest_base: D, prefix: &str, digits: usize) -> Result<PathBuf, String>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
{
    let date_string = date_for_file_name_now();
    let prefix = format!("{} {}", prefix, date_string);
    back_up_folder_next_number_r(path_source, path_dest_base, &prefix, digits)
}

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

    fn create_test_folders<P>(path: P, folder_names: &[&str])
        where P: AsRef<Path>
    {
        path_create_if_necessary_r(&path).unwrap();
        let path = PathBuf::from(path.as_ref());
        folder_names.iter().for_each(|folder_name| fs::create_dir(path.join(folder_name)).unwrap());
    }

    fn create_test_folder<P>(path: P, folder_name: &str)
        where P: AsRef<Path>
    {
        create_test_folders(path, &[folder_name])
    }

    fn create_test_files<P>(path: P, file_names: &[&str])
        where P: AsRef<Path>
    {
        path_create_if_necessary_r(&path).unwrap();
        let path = PathBuf::from(path.as_ref());
        file_names.iter().for_each(|file_name| fs::write(path.join(file_name), file_name).unwrap());
    }

    fn create_test_folder_with_files<P>(path_root: P) -> (PathBuf, Vec<String>)
        where P: AsRef<Path>
    {
        let path_folder = PathBuf::from(path_root.as_ref()).join(FOLDER_WITH_FILES);
        let mut file_names = vec!["File G.txt", "File R.txt", "File B.txt"];
        create_test_files(&path_folder, &file_names);
        file_names.sort();
        (path_folder, str_to_string_vector(&file_names))
    }

    fn add_files_to_test_folder<P>(path_root: P) -> (PathBuf, Vec<String>)
        where P: AsRef<Path>
    {
        let path_folder = PathBuf::from(path_root.as_ref()).join(FOLDER_WITH_FILES);
        create_test_files(&path_folder, &["File 12.txt", "File 08.txt"]);
        let file_names = path_file_names_r(&path_folder).unwrap();
        (path_folder, file_names)
    }

    fn create_test_folder_with_subfolders<P>(path_root: P) -> PathBuf
        where P: AsRef<Path>
    {
        let path_folder = PathBuf::from(path_root.as_ref()).join(FOLDER_WITH_FILES);
        create_test_files(&path_folder,&["File Root_1.txt", "File Root 2.txt"]);

        let path_1 = path_folder.join("One");
        create_test_files(&path_1, &["File One_1.txt", "File One_2.txt", "File One_3.txt"]);

        let path_1a = path_1.join("A");
        create_test_files(&path_1a,&["File One_A_1.txt", "File One_A_2.txt"]);

        let path_2 = path_folder.join("Two");
        create_test_files(&path_2, &["File Two_1.txt", "File Two_2.txt"]);

        path_folder
    }

    fn create_numbered_test_folders<P>(path: P, prefix: &str) -> (usize, usize)
        where P: AsRef<Path>
    {
        // Create several folders that match the prefix and have numbers. The numbers normally
        // would all be the same length with leading zeros, but in this case the lengths vary so we
        // can prove that we get the number of digits (3) from the largest number (65).
        ["0011", "4", "065", "0019", "2", "033"].iter().for_each(|number| {
            let folder_name = format!("{} {}", prefix, number);
            create_test_folder(&path, &folder_name);
        });
        (65, 3)
    }

    fn create_dated_numbered_test_folders<P>(path: P, prefix: &str) -> (usize, usize)
        where P: AsRef<Path>
    {
        let date_string = date_for_file_name_now();
        ["6", "8", "2", "1"].iter().for_each(|number| {
            let folder_name = format!("{} {} {}", prefix, date_string, number);
            create_test_folder(&path, &folder_name);
        });
        (8, 1)
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
    fn test_path_file_names_r() {
        let path_test_root = setup("test_path_file_names_r");
        let path_test_missing = path_test_root.join("Missing");
        assert_err_path_not_found(path_file_names_r(&path_test_missing), &path_test_missing);

        let (path_folder, exp_file_names) = create_test_folder_with_files(&path_test_root);
        let act_file_names = path_file_names_r(&path_folder).unwrap();
        assert_eq!(exp_file_names, act_file_names);
    }

    #[test]
    fn test_path_create_if_necessary_r() {
        let path_test_root = setup("test_path_create_if_necessary_r");

        // Place the subfolder a few levels down to confirm that we'll create all levels.
        let path_subfolder = path_test_root.join("Sub1").join("Sub2").join("Sub3");
        assert!(path_exists(&path_test_root));
        assert_eq!(false, path_exists(&path_subfolder));

        let was_created = path_create_if_necessary_r(&path_subfolder).unwrap();
        assert!(was_created);
        assert!(path_exists(&path_subfolder));

        // Add a next-level subfolder so we'll know if we subsequently drop and recreate the first
        // subfolder.
        let path_next_subfolder = path_subfolder.join("Sub4");
        let was_created = path_create_if_necessary_r(&path_next_subfolder).unwrap();
        assert!(was_created);
        assert!(path_exists(&path_next_subfolder));

        // Call to create the original subfolder if necessary. Nothing should happen.
        let was_created = path_create_if_necessary_r(&path_subfolder).unwrap();
        assert_eq!(false, was_created);
        assert!(path_exists(&path_subfolder));
        // The last subfolder should still be there since its parent folder was not dropped and
        // recreated.
        assert!(path_exists(&path_next_subfolder));
    }

    #[test]
    fn test_copy_folder_recursive_r() {
        let path_test_root = setup("test_copy_folder_recursive_r");
        let path_dest = path_test_root.join("Dest");
        let path_missing_folder = path_test_root.join("Missing");
        assert_err_path_not_found(copy_folder_recursive_r(&path_missing_folder, &path_dest),&path_missing_folder);

        let path_source = create_test_folder_with_subfolders(&path_test_root);
        copy_folder_recursive_r(&path_source, &path_dest).unwrap();

        // This time the destination folder will already exist and have a subfolder with some files
        // in it. The subfolder and files should not be touched.
        let path_dest = path_test_root.join("Existing Dest");
        fs::create_dir(&path_dest).unwrap();
        create_test_folder_with_files(&path_dest);
        copy_folder_recursive_r(&path_source, &path_dest).unwrap();
    }

    #[test]
    fn test_copy_folder_to_new_folder_r() {
        let path_test_root = setup("test_copy_folder_to_new_folder_r");
        let path_dest = path_test_root.join("Dest");
        let path_missing_folder = path_test_root.join("Missing");
        assert_err_path_not_found(copy_folder_to_new_folder_r(&path_missing_folder, &path_dest),&path_missing_folder);

        let path_source = create_test_folder_with_subfolders(&path_test_root);
        // The destination folder does not exist yet so this should be fine.
        copy_folder_to_new_folder_r(&path_source, &path_dest).unwrap();

        // Now that the destination folder exists, the function should fail.
        assert_err_path_exists(copy_folder_to_new_folder_r(&path_source, &path_dest), &path_dest);
    }

    #[test]
    fn test_path_folder_with_number() {
        let exp_path_name = format!("C:\\One\\Abc 2021-03-15 0021");
        let act_path_name = path_name(&path_folder_with_number(Path::new("C:\\One"), "Abc 2021-03-15", 21, 4));
        assert_eq!(exp_path_name, act_path_name);

        // Make sure it works when the number is too high for the number of digits. It should use
        // the whole number anyway.
        let exp_path_name = format!("C:\\One\\Abc 2021-03-15 12345");
        let act_path_name = path_name(&path_folder_with_number(Path::new("C:\\One"), "Abc 2021-03-15", 12345, 3));
        assert_eq!(exp_path_name, act_path_name);
    }

    #[test]
    fn test_folder_highest_number_r() {
        let path_test_root = setup("test_folder_highest_number_r");
        let prefix = "Abc";

        let path_missing_folder = path_test_root.join("Missing");
        assert_err_path_not_found(folder_highest_number_r(&path_missing_folder, prefix),&path_missing_folder);

        // Create several folders which should be ignored even though they have numbers, since they
        // don't match the prefix.
        create_test_folders(&path_test_root, &["One 0564", "Two 12", "Three 4500"]);

        // At first there are no folders matching the prefix.
        assert_eq!(None, folder_highest_number_r(&path_test_root, prefix).unwrap());

        // Special case where there's only one file and its number is zero.
        create_test_folder(&path_test_root, &format!("{} 0", prefix));
        assert_eq!(Some((0, 1)), folder_highest_number_r(&path_test_root, prefix).unwrap());

        let (exp_number, exp_digits) = create_numbered_test_folders(&path_test_root, prefix);
        assert_eq!(Some((exp_number, exp_digits)), folder_highest_number_r(&path_test_root, prefix).unwrap());
    }

    #[test]
    fn test_path_folder_highest_number_r() {
        let path_test_root = setup("test_path_folder_highest_number_r");
        // The path will be created.
        let path = path_test_root.join("Sub1").join("Sub2");
        let prefix = "Xyz";

        // At first there are no folders matching the prefix.
        assert_eq!(None, path_folder_highest_number_r(&path, prefix).unwrap());

        let (highest_number, digits) = create_numbered_test_folders(&path, prefix);
        let exp_path_name = format!("{}\\{} {}", path_name(&path), prefix, format_zeros(highest_number, digits));
        let act_path_name= path_name(&path_folder_highest_number_r(&path, prefix).unwrap().unwrap());
        assert_eq!(exp_path_name, act_path_name);
    }

    #[test]
    fn test_path_folder_next_number_r() {
        let path_test_root = setup("test_path_folder_next_number_r");
        // The path will be created.
        let path = path_test_root.join("Sub1");
        let prefix = "Def";
        let digits = 4;

        // At first there are no folders matching the prefix, so the folder will end with a 1.
        let exp_path_name = format!("{}\\{} {}", path_name(&path), prefix, format_zeros(1, digits));
        let act_path_name= path_name(&path_folder_next_number_r(&path, prefix, digits).unwrap());
        assert_eq!(exp_path_name, act_path_name);

        let (highest_number, _) = create_numbered_test_folders(&path, prefix);
        let exp_path_name = format!("{}\\{} {}", path_name(&path), prefix, format_zeros(highest_number + 1, digits));
        let act_path_name= path_name(&path_folder_next_number_r(&path, prefix, digits).unwrap());
        assert_eq!(exp_path_name, act_path_name);
    }

    // pub fn path_folder_dated_next_number_r<P>(path_base: P, prefix: &str, digits: usize) -> Result<PathBuf, String>
    #[test]
    fn test_path_folder_dated_next_number_r() {
        let path_test_root = setup("test_path_folder_dated_next_number_r");
        // The path will be created.
        let path = path_test_root.join("Sub1").join("Sub2");
        let prefix = "Jkl";
        let digits = 3;
        let date_string = date_for_file_name_now();

        // Create several folders that have the prefix but no date, and thus should not be counted.
        create_numbered_test_folders(&path, prefix);

        // At first there are no folders matching the prefix, so the folder will end with a 1.
        let exp_path_name = format!("{}\\{} {} {}", path_name(&path), prefix, date_string, format_zeros(1, digits));
        let act_path_name= path_name(&path_folder_dated_next_number_r(&path, prefix, digits).unwrap());
        assert_eq!(exp_path_name, act_path_name);

        let (highest_number, _) = create_dated_numbered_test_folders(&path, prefix);
        let exp_path_name = format!("{}\\{} {} {}", path_name(&path), prefix, date_string, format_zeros(highest_number + 1, digits));
        let act_path_name= path_name(&path_folder_dated_next_number_r(&path, prefix, digits).unwrap());
        assert_eq!(exp_path_name, act_path_name);
    }

    #[test]
    fn test_back_up_folder_next_number_r() {
        let path_test_root = setup("test_back_up_folder_next_number_r");
        let path_missing_folder = path_test_root.join("Missing");
        assert_err_path_not_found(back_up_folder_next_number_r(&path_missing_folder, &path_test_root, "Back Red", 3), &path_missing_folder);

        let (path_source_folder, exp_file_names) = create_test_folder_with_files(&path_test_root);

        // The destination includes two levels of folders that don't exist yet.
        let path_dest_base = path_test_root.join("Backup").join("April");

        let path_dest_folder = back_up_folder_next_number_r(&path_source_folder, &path_dest_base, "Back Green", 3).unwrap();
        let act_file_names = path_file_names_r(&path_dest_folder).unwrap();
        assert_eq!(exp_file_names, act_file_names);
        let exp_dest_folder_name = format!("{}\\Backup\\April\\Back Green 001", path_name(&path_test_root));
        let act_dest_folder_name = path_name(&path_dest_folder);
        assert_eq!(exp_dest_folder_name, act_dest_folder_name);

        // Add some files to the source and create a second numbered backup.
        let (_, exp_file_names) = add_files_to_test_folder(&path_test_root);
        let path_dest_folder = back_up_folder_next_number_r(&path_source_folder, &path_dest_base, "Back Green", 3).unwrap();
        //bg!(path_name(&path_dest_folder));
        let act_file_names = path_file_names_r(&path_dest_folder).unwrap();
        assert_eq!(exp_file_names, act_file_names);
        let exp_dest_folder_name = format!("{}\\Backup\\April\\Back Green 002", path_name(&path_test_root));
        let act_dest_folder_name = path_name(&path_dest_folder);
        assert_eq!(exp_dest_folder_name, act_dest_folder_name);
    }
    #[test]

    fn test_back_up_folder_dated_next_number_r() {
        let path_test_root = setup("test_back_up_folder_dated_next_number_r");
        let path_missing_folder = path_test_root.join("Missing");
        assert_err_path_not_found(back_up_folder_dated_next_number_r(&path_missing_folder, &path_test_root, "Back Red", 3), &path_missing_folder);

        let date_string = date_for_file_name_now();

        let (path_source_folder, exp_file_names) = create_test_folder_with_files(&path_test_root);

        // The destination includes two levels of folders that don't exist yet.
        let path_dest_base = path_test_root.join("Backup").join("April");

        let path_dest_folder = back_up_folder_dated_next_number_r(&path_source_folder, &path_dest_base, "Back Green", 3).unwrap();
        let act_file_names = path_file_names_r(&path_dest_folder).unwrap();
        assert_eq!(exp_file_names, act_file_names);
        let exp_dest_folder_name = format!("{}\\Backup\\April\\Back Green {} 001", path_name(&path_test_root), date_string);
        let act_dest_folder_name = path_name(&path_dest_folder);
        assert_eq!(exp_dest_folder_name, act_dest_folder_name);

        // Add some files to the source and create a second numbered backup.
        let (_, exp_file_names) = add_files_to_test_folder(&path_test_root);
        let path_dest_folder = back_up_folder_dated_next_number_r(&path_source_folder, &path_dest_base, "Back Green", 3).unwrap();
        //bg!(path_name(&path_dest_folder));
        let act_file_names = path_file_names_r(&path_dest_folder).unwrap();
        assert_eq!(exp_file_names, act_file_names);
        let exp_dest_folder_name = format!("{}\\Backup\\April\\Back Green {} 002", path_name(&path_test_root), date_string);
        let act_dest_folder_name = path_name(&path_dest_folder);
        assert_eq!(exp_dest_folder_name, act_dest_folder_name);
    }

}
