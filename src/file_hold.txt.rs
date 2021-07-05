





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

/*
        fn copy_folder_recursive<S, D>(path_source: S, path_dest: D) -> Result<(), String>
        where
        S: AsRef<Path>,
        D: AsRef<Path>,
    {
        // From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust.
        path_create_if_necessary_result(&path_dest)?;
        match fs::read_dir(path_source) {
        Ok(iter) => {
        iter.for_each(|read_dir| {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
        copy_folder_recursive(entry.path(), path_dest.as_ref().join(entry.file_name()))?;
        } else {
        fs::copy(entry.path(), path_dest.as_ref().join(entry.file_name()))?;
        }
        }
        Ok(())
        }
*/

