use std::{fs, path};
use std::io::{self, BufRead, BufReader};
use std::collections::btree_map::BTreeMap;
use glob::{glob_with, MatchOptions};
use std::fs::File;

pub fn read_file_as_lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>()
}

pub fn read_file_into_sections(file_name: &str, header_prefix: &str) -> BTreeMap<String, String> {
    let content = fs::read_to_string(file_name).unwrap();
    break_into_sections(content, header_prefix)
}

pub fn break_into_sections(content: String, header_prefix: &str) -> BTreeMap<String, String> {
    let content = content.replace("\u{feff}", "");
    //bg!(&content[..300], &header_prefix);
    let mut map = BTreeMap::new();
    for split in content.split(header_prefix) {
        //bg!(&split);
        if !split.trim().is_empty() {
            let (header, section_content) = split_2(split, "\n");
            map.insert(header.trim().to_string(), section_content.trim().to_string());
        }
    }
    map
}

pub fn before<'a>(line: &'a str, pat: &str) -> &'a str {
    if line.len() == 0 || pat.len() == 0 {
        line
    } else {
        line.splitn(2, pat).into_iter().next().unwrap()
    }
}

pub fn after<'a>(line: &'a str, pat: &str) -> &'a str {
    if line.len() == 0 || pat.len() == 0 {
        line
    } else {
        line.splitn(2, pat).into_iter().last().unwrap()
    }
}

// Both delimiters must appear in the line.
pub fn between<'a>(line: &'a str, pat_before: &str, pat_after: &str) -> &'a str {
    rbefore(after(line, pat_before), pat_after)
}

pub fn rbefore<'a>(line: &'a str, pat: &str) -> &'a str {
    if line.len() == 0 || pat.len() == 0 {
        line
    } else {
        line.rsplitn(2, pat).into_iter().last().unwrap()
    }
}

pub fn rafter<'a>(line: &'a str, pat: &str) -> &'a str {
    if line.len() == 0 || pat.len() == 0 {
        line
    } else {
        line.rsplitn(2, pat).into_iter().next().unwrap()
    }
}

pub fn split_1_or_2<'a>(line: &'a str, pat: &str) -> (&'a str, Option<&'a str>) {
    let mut split = line.splitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for line = \"{}\"", line)),
        split.next()
    )
}

pub fn split_2<'a>(line: &'a str, pat: &str) -> (&'a str, &'a str) {
    assert!(pat.len() > 0);
    let mut split = line.splitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for line = \"{}\"", line)),
        split.next().expect(&format!("No second split item found for line = \"{}\"", line))
    )
}

pub fn rsplit_2<'a>(line: &'a str, pat: &str) -> (&'a str, &'a str) {
    assert!(pat.len() > 0);
    let mut split = line.rsplitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for line = \"{}\"", line)),
        split.next().expect(&format!("No second split item found for line = \"{}\"", line))
    )
}

pub fn split_3_two_delimiters<'a>(line: &'a str, pat_1: &str, pat_2: &str) -> (&'a str, &'a str, &'a str) {
    //bg!(line, pat_1, pat_2);
    let (first, rest) = split_2(line, pat_1);
    //bg!(first, rest);
    let (second, third) = split_2(&rest, pat_2);
    //bg!(second, third);
    (first, second, third)
}

pub fn delimited_entries(text: &str, left_delimiter: &str, right_delimiter: &str) -> Vec<String> {
    let mut v = vec![];
    for s in text.split(left_delimiter).skip(1) {
        //bg!(s);
        let one_value = s.split(right_delimiter).nth(0).unwrap().to_string();
        //bg!(&one_value);
        v.push(one_value);
    }
    v
}

pub fn delimited_entries_trim(text: &str, left_delimiter: &str, right_delimiter: &str) -> Vec<String> {
    trim_string_vector(&delimited_entries(text, left_delimiter, right_delimiter))
}

pub fn split_once_with_option(value: &str, delimiter: &str) -> (String, Option<String>) {
    if value.contains(delimiter) {
        let (a, b) = value.split_once(delimiter).unwrap();
        (a.to_string(), Some(b.to_string()))
    } else {
        (value.to_string(), None)
    }
}

pub fn trim_string_vector(v: &Vec<String>) -> Vec<String> {
    v.iter().map(|x| x.trim().to_string()).collect()
}

pub fn find_in_string<'a>(s: &'a str, pat_1: &str, pat_2: &str) -> Vec<&'a str> {
    let mut v = vec![];
    for (index, _) in s.match_indices(pat_1) {
        let (part, _) = split_1_or_2(&s[index + pat_1.len()..], pat_2);
        v.push(part);
    }
    v
}

pub fn find_in_file(path: &path::Path, pat_1: &str, pat_2: &str) -> io::Result<Vec<String>> {
    assert!(path.is_file());
    let content = fs::read_to_string(path)?;
    // The &str values returned from find_in_string() are bound to the lifetime of content, so we
    // need to turn them into strings.
    Ok(find_in_string(&content, pat_1, pat_2).iter().map(|x| x.to_string()).collect())
}

pub fn find_in_files_ci(path: &path::Path, wildcard: &str, pat_1: &str, pat_2: &str) -> io::Result<Vec<String>> {
    assert!(path.is_dir());
    assert!(wildcard.len() > 0);

    let mut v = vec![];
    for path in get_files_ci(path, wildcard).unwrap() {
        for item in find_in_file(&path, pat_1, pat_2)? {
            v.push(item);
        }
    }

    Ok(v)
}

pub fn get_files_ci(path: &path::Path, wildcard: &str) -> Result<Vec<path::PathBuf>, glob::PatternError> {
    assert!(path.is_dir());
    assert!(wildcard.len() > 0);

    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };
    let pat = path.join(wildcard).to_str().unwrap().to_string();
    Ok(glob_with(&pat, options)?
        .map(|result| result.unwrap())
        .collect())
}


pub fn extract_optional(val: &str, left_delimiter: &str, right_delimiter: &str) -> (String, Option<String>) {
    let left_pos = val.find(left_delimiter);
    let right_pos = val.find(right_delimiter);
    match (left_pos, right_pos) {
        (Some(left_pos), Some(right_pos)) => {
            if left_pos < right_pos {
                let remainder = format!("{} {}", val[..left_pos].trim(), val[right_pos + 1..].trim());
                let extracted = val[left_pos + 1..right_pos].trim().to_string();
                (remainder, Some(extracted))
            } else {
                (val.trim().to_string(), None)
            }
        },
        _ => {
            (val.trim().to_string(), None)
        }
    }
}

pub fn digits_only(value: &str) -> String {
    value.chars().filter(|char| char.is_digit(10)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_1_or_2() {
        assert_eq!(("abc", Some("def")), split_1_or_2("abc..def", ".."));
        assert_eq!(("abc.def", None), split_1_or_2("abc.def", "x"));
        assert_eq!(("", None), split_1_or_2("", "x"));
    }

    #[test]
    fn test_split_2() {
        assert_eq!(("abc", "def"), split_2("abc..def", ".."));
        assert_eq!(("", ""), split_2("..", ".."));
        assert_eq!(("abc", ""), split_2("abc..", ".."));
        assert_eq!(("", "def"), split_2("..def", ".."));
    }

    #[test]
    #[should_panic]
    fn test_split_2_fail() {
        split_2("abc..def", "x");
    }

    #[test]
    fn test_rsplit_2() {
        assert_eq!(("def", "abc"), rsplit_2("abc..def", ".."));
        assert_eq!(("", ""), rsplit_2("..", ".."));
        assert_eq!(("", "abc"), rsplit_2("abc..", ".."));
        assert_eq!(("def", ""), rsplit_2("..def", ".."));
    }

    #[test]
    fn test_split_3_two_delimiters() {
        assert_eq!(("ab ", "cde", " fghi"), split_3_two_delimiters("ab (cde) fghi", "(", ")"));
    }

    #[test]
    fn test_before() {
        // Blank value.
        assert_eq!("", before("", ": "));
        // Blank pattern,
        assert_eq!("abc", before("abc", ""));
        // No match.
        assert_eq!("abc", before("abc", "xyz"));
        // Normal.
        assert_eq!("a", before("abc", "b"));
        // Match right away.
        assert_eq!("", before("abc", "a"));
        // Three possible matches but we want the first one.
        assert_eq!("ab", before("abc def c abc ghi", "c"));
    }

    #[test]
    fn test_after() {
        // Blank value.
        assert_eq!("", after("", ": "));
        // Blank pattern,
        assert_eq!("abc", after("abc", ""));
        // No match.
        assert_eq!("abc", after("abc", "xyz"));
        // Normal.
        assert_eq!("c", after("abc", "b"));
        // Match at end.
        assert_eq!("", after("abc", "bc"));
        // Three possible matches but we want the first one.
        assert_eq!(" def c abc ghi", after("abc def c abc ghi", "c"));
    }

    #[test]
    fn test_rbefore() {
        // Blank value.
        assert_eq!("", rbefore("", ": "));
        // Blank pattern,
        assert_eq!("abc", rbefore("abc", ""));
        // No match.
        assert_eq!("abc", rbefore("abc", "xyz"));
        // Normal.
        assert_eq!("a", rbefore("abc", "b"));
        // Match right away.
        assert_eq!("", rbefore("abc", "a"));
        // Three possible matches but we want the rightmost one.
        assert_eq!("abc def c ab", rbefore("abc def c abc ghi", "c"));
    }

    #[test]
    fn test_rafter() {
        // Blank value.
        assert_eq!("", after("", ": "));
        // Blank pattern,
        assert_eq!("abc", after("abc", ""));
        // No match.
        assert_eq!("abc", after("abc", "xyz"));
        // Normal.
        assert_eq!("c", after("abc", "b"));
        // Match at end.
        assert_eq!("", after("abc", "bc"));
        // Three possible matches but we want the rightmost one.
        assert_eq!(" ghi", rafter("abc def c abc ghi", "c"));
    }

}