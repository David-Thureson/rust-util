use std::{fs, path};
use std::io::{self, BufRead, BufReader};
use std::collections::btree_map::BTreeMap;
use glob::{glob_with, MatchOptions};
use std::fs::File;
use crate::err_context;
use itertools::Itertools;

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

pub fn read_file_into_sections_by_line(file_name: &str, header_prefix: &str, header_suffix: Option<&str>) -> BTreeMap<String, Vec<String>> {
    break_into_sections_by_line(&read_file_as_lines(file_name), header_prefix, header_suffix)
}

pub fn break_into_sections_by_line(lines: &[String], header_prefix: &str, header_suffix: Option<&str>) -> BTreeMap<String, Vec<String>> {
    let mut map = BTreeMap::new();
    let mut header = "".to_string();
    for line in lines {
        let line = line.trim();
        if line.starts_with(header_prefix) {
            header = between(line, header_prefix, header_suffix.unwrap_or("")).to_string();
        } else {
            if !line.is_empty() {
                map.entry(header.clone()).or_insert(vec![]).push(line.to_string());
            }
        }
    }
    map
}

pub fn parse_name_value_pairs<'a>(lines: &'a [String], delimiter: &str, comment_prefix: Option<&str>) -> (BTreeMap<String, String>, Vec<&'a str>) {
    // Read a set of name-value pairs such as those found in a TOML file:
    //   name = "language"
    //   version = "0.1.0"
    //   authors = ["David Thureson <David.G.Thureson@gmail.com>"]
    //   edition = "2018"
    // The values may be quoted or unquoted. Lines without the delimiter such as an equal sign are
    // returned in an array. Blank lines are ignored. A given key such as "version" above may appear
    // only once.
    let comment_prefix = comment_prefix.unwrap_or("{no comment prefix}");
    let mut map = BTreeMap::new();
    let mut remaining_lines = vec![];
    for line in lines {
        let line = line.trim();
        if !line.is_empty() && !line.starts_with(comment_prefix) {
            let (a, b) = split_1_or_2(line.trim(), delimiter);
            match b {
                Some(b) => {
                    let name = a.trim();
                    assert!(!map.contains_key(name), "Duplicated key in name-value pair: \"{}\"", name);
                    let value = unquote(b);
                    map.insert(name.to_string(), value.to_string());
                },
                None => {
                    remaining_lines.push(line);
                }
            }
        }
    }
    (map, remaining_lines)
}

pub fn remove_delimiters<'a>(value: &'a str, left_delimiter: &str, right_delimiter: &str) -> &'a str {
    let mut value = value.trim();
    if value.starts_with(left_delimiter) {
        value = &value[left_delimiter.len()..];
    }
    if value.ends_with(right_delimiter) {
        let new_length = value.len() - right_delimiter.len();
        value = &value[..new_length];
    }
    value.trim()
}

pub fn unquote<'a>(value: &'a str) -> &'a str {
    remove_delimiters(value, "\"", "\"")
    // value.trim().trim_start_matches("\"").trim_end_matches("\"")
}

pub fn before<'a>(value: &'a str, pat: &str) -> &'a str {
    if value.len() == 0 || pat.len() == 0 {
        value
    } else {
        value.splitn(2, pat).into_iter().next().unwrap()
    }
}

pub fn before_ci<'a>(value: &'a str, pat: &str) -> &'a str {
    if value.len() == 0 || pat.len() == 0 {
        value
    } else {
        let pos = value.to_lowercase().find(&pat.to_lowercase());
        //rintln!("\"{}\"\t\"{}\"\t{:?}", value.to_lowercase(), pat.to_lowercase(), pos);
        match pos {
            Some(pos) => &value[..pos],
            None => value,
        }
    }
}

pub fn after<'a>(value: &'a str, pat: &str) -> &'a str {
    if value.len() == 0 || pat.len() == 0 {
        value
    } else {
        value.splitn(2, pat).into_iter().last().unwrap()
    }
}

pub fn between<'a>(value: &'a str, pat_before: &str, pat_after: &str) -> &'a str {
    rbefore(after(value, pat_before), pat_after)
}

pub fn between_trim<'a>(value: &'a str, pat_before: &str, pat_after: &str) -> &'a str {
    between(value, pat_before, pat_after).trim()
}

pub fn between_optional<'a>(value: &'a str, pat_before: &str, pat_after: &str) -> Option<&'a str> {
    if let Some(before_index) = value.find(pat_before) {
        let just_after_before = before_index + pat_before.len();
        if value[just_after_before..].find(pat_after).is_some() {
            return Some(between(value, pat_before, pat_after));
        }
    }
    None
}

pub fn between_optional_trim<'a>(value: &'a str, pat_before: &str, pat_after: &str) -> Option<&'a str> {
    between_optional(value, pat_before, pat_after).map(|x| x.trim())
}

pub fn rbefore<'a>(value: &'a str, pat: &str) -> &'a str {
    if value.len() == 0 || pat.len() == 0 {
        value
    } else {
        value.rsplitn(2, pat).into_iter().last().unwrap()
    }
}

pub fn rafter<'a>(value: &'a str, pat: &str) -> &'a str {
    if value.len() == 0 || pat.len() == 0 {
        value
    } else {
        value.rsplitn(2, pat).into_iter().next().unwrap()
    }
}

pub fn split_1_or_2<'a>(value: &'a str, pat: &str) -> (&'a str, Option<&'a str>) {
    let mut split = value.splitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for value = \"{}\"", value)),
        split.next()
    )
}

pub fn split_1_or_2_trim<'a>(value: &'a str, pat: &str) -> (&'a str, Option<&'a str>) {
    let split = split_1_or_2(value, pat);
    (split.0.trim(), split.1.map(|x| x.trim()))
}

pub fn split_2<'a>(value: &'a str, pat: &str) -> (&'a str, &'a str) {
    assert!(pat.len() > 0);
    let mut split = value.splitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for value = \"{}\"", value)),
        split.next().expect(&format!("No second split item found for value = \"{}\"", value))
    )
}

pub fn split_2_r<'a>(value: &'a str, pat: &str) -> Result<(&'a str, &'a str), String> {
    assert!(pat.len() > 0);
    let mut split = value.splitn(2, pat);
    match (split.next(), split.next()) {
        (Some(split_0), Some(split_1)) => Ok((split_0, split_1)),
        _ => Err(format!("Unable to split \"{}\" into two parts using \"{}\"", value, pat)),
    }
}

pub fn split_2_rc<'a>(value: &'a str, pat: &str, context: &str) -> Result<(&'a str, &'a str), String> {
    err_context(split_2_r(value, pat), context)
}

pub fn split_2_trim<'a>(value: &'a str, pat: &str) -> (&'a str, &'a str) {
    let (a, b) = split_2(value, pat);
    (a.trim(), b.trim())
}

pub fn rsplit_2<'a>(value: &'a str, pat: &str) -> (&'a str, &'a str) {
    assert!(pat.len() > 0);
    let mut split = value.rsplitn(2, pat);
    (
        split.next().expect(&format!("No first (rightmost) split item found for value = \"{}\"", value)),
        split.next().expect(&format!("No second (leftmost) split item found for value = \"{}\"", value))
    )
}

pub fn split_3_two_delimiters<'a>(value: &'a str, pat_1: &str, pat_2: &str) -> (&'a str, &'a str, &'a str) {
    //bg!(value, pat_1, pat_2);
    let (first, rest) = split_2(value, pat_1);
    //bg!(first, rest);
    let (second, third) = split_2(&rest, pat_2);
    //bg!(second, third);
    (first, second, third)
}

pub fn split_3_two_delimiters_r<'a>(value: &'a str, pat_1: &str, pat_2: &str) -> Result<(&'a str, &'a str, &'a str), String> {
    //bg!(value, pat_1, pat_2);
    let (first, rest) = split_2_r(value, pat_1)?;
    //bg!(first, rest);
    let (second, third) = split_2_r(&rest, pat_2)?;
    //bg!(second, third);
    Ok((first, second, third))
}

pub fn split_3_two_delimiters_rc<'a>(value: &'a str, pat_1: &str, pat_2: &str, context: &str) -> Result<(&'a str, &'a str, &'a str), String> {
    err_context(split_3_two_delimiters_r(value, pat_1, pat_2), context)
}

pub fn split_trim(value: &str, pat: &str) -> Vec<String> {
    value.split(pat).map(|x| x.trim().to_string()).collect()
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

pub fn split_delimited_and_normal_rc(text: &str, left_delimiter: &str, right_delimiter: &str, context: &str) -> Result<Vec<(bool, String)>, String> {
    // if text.eq("on_data_structures_and_algorithms_with_rust|Hands-On Data Structures and Algorithms with Rust]]") { panic!() }
    // let debug = text.contains("on_data_structures_and_algorithms_with_rust|Hands");
    let debug = false;
    if debug { dbg!(text, left_delimiter, right_delimiter, context); }
    let err_func = |pos: usize, msg: &str| Err(
        format!("{} split_delimited_and_normal_rc: pos = {}: {} left = \"{}\", right = \"{}\", text = \"{}\".",
        context, pos, msg, left_delimiter, right_delimiter, text));
    let mut v = vec![];
    let mut pos = 0;
    while pos < text.len() {
        let next_left = text[pos..].find(left_delimiter).map(|x| x + pos);
        let next_right = if left_delimiter == right_delimiter {
            // The left and right delimiters are the same, so we have to assume that the first
            // delimiter found is the left one, and the right delimiter can only be sought after
            // that left delimiter.
            if text.len() - 1 > pos {
                text[pos + 1..].find(right_delimiter).map(|x| x + pos + 1)
            } else {
                // There are no characters after pos.
                None
            }
        } else {
            // The left and right delimiters are different, so we don't have to be as precise about
            // where to look.
            text[pos..].find(right_delimiter).map(|x| x + pos)
        };
        if debug { println!("\nTop of loop:"); dbg!(pos, next_left, next_right); }
        match next_left {
            Some(next_left) => {
                // We expect that we're seeing a non-delimited substring (which may be zero length)
                // followed by a delimited substring, then the remainder of the text.
                // Don't bother adding a non-delimited substring if the text starts with a
                // delimited substring, that is a left delimiter in position 0, or if there's a new
                // delimited substring immediately following the last one.
                if next_left > pos {
                    v.push((false, text[pos..next_left].to_string()));
                }
                // Presumably there's a right delimiter to match the left one.
                match next_right {
                    Some(next_right) => {
                        if next_right < next_left {
                            if debug { panic!("Extra right delimiter."); }
                            return err_func(pos, "Extra right delimiter.");
                        }
                        let pos_substring_start = next_left + left_delimiter.len();
                        if debug { dbg!(pos_substring_start); }
                        v.push((true, text[pos_substring_start..next_right].to_string()));
                        pos = next_right + right_delimiter.len();
                    },
                    None => {
                        if debug { panic!("Missing right delimiter."); }
                        return err_func(pos, "Missing right delimiter.");
                    },
                };
            },
            None => {
                // The remainder of the text is assumed to be a non-delimited substring.
                if next_right.is_some() {
                    // There's an unexpected delimiter.
                    return err_func(pos, "Extra right delimiter.");
                }
                v.push((false, text[pos..].to_string()));
                break;
            }
        }
    }
    Ok(v)
}

pub fn replace_within_delimiters_rc(text: &str, left_delimiter: &str, right_delimiter: &str, from: &str, to: &str, context: &str) -> Result<String, String> {
    // Replace one substring with another, but only within the parts of the text that are between
    // certain delimiters. This is used when we want to do something with those "from" substrings
    // only where they don't appear between the delimiters.
    // For example, here the pipe characters with spaces around them are considered cell
    // separators, but the pipes within the bracketed links separate the link destination from the
    // label, a completely separate meaning:
    //   | [[tools:nav:attributes#Narrator|Narrator]] | [[tools:nav:attribute_values#Mark Steinberg|Mark Steinberg]] |
    // So we call this function and replace the pipes inside the brackets with a placeholder:
    //   | [[tools:nav:attributes#Narrator{{{pipe}}}Narrator]] | [[tools:nav:attribute_values#Mark Steinberg{{{pipe}}}Mark Steinberg]] |
    // and now we can easily count or split on the pipes that are cell separators.
    let delimited_splits = split_delimited_and_normal_rc(text, left_delimiter, right_delimiter, context)?;
    let new_text = delimited_splits.iter()
        .map(|(split_is_delimited, split_text)| if *split_is_delimited {
            // Put the left and right delimiters back, and do the replacement.
            format!("{}{}{}", left_delimiter, split_text.replace(from, to), right_delimiter)
        } else {
            split_text.clone()
        })
        .join("");
    Ok(new_text)
}

pub fn split_outside_of_delimiters_rc(text: &str, split_delimiter: &str, left_delimiter: &str, right_delimiter: &str, context: &str) -> Result<Vec<String>, String> {
    // Split based on a delimiter, but ignore cases where that split delimiter appears inside some
    // other delimiters.
    // For example, here the pipe characters with spaces around them are considered cell
    // separators, but the pipes within the bracketed links separate the link destination from the
    // label, a completely separate meaning:
    //   | [[tools:nav:attributes#Narrator|Narrator]] | [[tools:nav:attribute_values#Mark Steinberg|Mark Steinberg]] |
    // We want to split the cells on the pipe characters, ignoring those pipe characters that
    // appear inside the bracketed parts.
    let placeholder = "{{{split_outside_of_delimiters}}}";
    // Replace the split delimiters we don't want to split on with a placeholder.
    let temp_with_replacements = replace_within_delimiters_rc(text, left_delimiter, right_delimiter, split_delimiter, placeholder, context)?;
    // Split on the remaining split delimiters, then put back the ones we took out earlier.
    let splits = temp_with_replacements.split(split_delimiter).into_iter()
        .map(|split| split.replace(placeholder, split_delimiter))
        .collect::<Vec<_>>();
    Ok(splits)
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

pub fn digits_only(value: &str) -> String {
    value.chars().filter(|char| char.is_digit(10)).collect()
}

pub fn count_characters(strings: Vec<String>) {
    let mut grouper = crate::group::Grouper::new("Characters");
    for string in strings.iter() {
        for char in string.chars() {
            grouper.record_entry(&char);
        }
    }
    grouper.print_by_key(0, None);
}

pub fn count_leading_character(value: &str, char: char) -> usize {
    let mut count = 0;
    for c in value.chars() {
        if c == char {
            count += 1;
        } else {
            return count;
        }
    }
    count
}

pub fn count_trailing_character(value: &str, char: char) -> usize {
    let mut count = 0;
    for c in value.chars().rev() {
        if c == char {
            count += 1;
        } else {
            return count;
        }
    }
    count
}

pub fn count_leading_spaces(value: &str) -> usize {
    count_leading_character(value, ' ')
}

pub fn trim_linefeeds(value: &str) -> String {
    let leading_count = count_leading_character(value, '\n');
    let trailing_count = count_trailing_character(value, '\n');
    if leading_count == 0 && trailing_count == 0 {
        value.to_string()
    } else {
        let trailing_index = value.len() - trailing_count;
        value[leading_count..trailing_index].to_string()
    }
}

pub fn print_chars(value: &str) {
    println!("\nutil::parse::print_chars() - length = {}; value = \"{}\"", value.len(), value);
    for (index, c) in value.chars().enumerate() {
        let ascii = if c.is_ascii() {
            (c as u8).to_string()
        } else {
            "not ASCII".to_string()
        };
        println!("[{}] '{}' ASCII {}", index, c, ascii);
    }
    println!();
}

pub fn remove_zero_width_no_break_space(text: &str) -> String {
    text.replace("\u{feff}", "")
}

pub fn try_split_delimited_and_normal_rc() {
    // Normal case:
    let context = "Test";

    let text = "ab [[c]]e [[fg ]] hij";
    let exp = vec![(false, "ab "), (true, "c"), (false, "e "), (true, "fg "), (false, " hij")];
    let exp = test_split_delimited_exp_to_strings(exp);
    let act = split_delimited_and_normal_rc(text, "[[", "]]", context).unwrap();
    //bg!(&exp, &act);
    assert!(exp.eq(&act));

    // Start and end with delimited substrings, and have delimited substrings that touch without
    // a non-delimited substring between them.
    let text = "[[ab]][[ c ]]  d [[efg]]";
    let exp = vec![(true, "ab"), (true, " c "), (false, "  d "), (true, "efg")];
    let exp = test_split_delimited_exp_to_strings(exp);
    let act = split_delimited_and_normal_rc(text, "[[", "]]", context).unwrap();
    //bg!(&exp, &act);
    assert!(exp.eq(&act));

    // Error: Start with a right delimiter.
    let text = "]]a[[b]]";
    println!("{}", split_delimited_and_normal_rc(text, "[[", "]]", context).err().unwrap());

    // Error: Start with a right delimiter, no left delimiters.
    let text = "]]a";
    println!("{}", split_delimited_and_normal_rc(text, "[[", "]]", context).err().unwrap());

    // Error: First right delimiter before first left delimiter.
    let text = "abc]]a[[b]]";
    println!("{}", split_delimited_and_normal_rc(text, "[[", "]]", context).err().unwrap());

    // Error: Right delimiter but no left delimiter.
    let text = "abc]]a";
    println!("{}", split_delimited_and_normal_rc(text, "[[", "]]", context).err().unwrap());

    // Error: Delimited substring doesn't end (no right delimiter).
    let text = "[[abc]]a[[b]]c[[d";
    println!("{}", split_delimited_and_normal_rc(text, "[[", "]]", context).err().unwrap());
}

fn test_split_delimited_exp_to_strings(v: Vec<(bool, &str)>) -> Vec<(bool, String)> {
    v.iter().map(|(b, s)| (*b, s.to_string())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
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
        // Different case.
        assert_eq!("abc", before("abc", "B"));
        assert_eq!("aBc", before("aBc", "b"));
        // Match right away.
        assert_eq!("", before("abc", "a"));
        // Three possible matches but we want the first one.
        assert_eq!("ab", before("abc def c abc ghi", "c"));
    }

    #[test]
    fn test_before_ci() {
        // Blank value.
        assert_eq!("", before_ci("", ": "));
        // Blank pattern,
        assert_eq!("abc", before_ci("abc", ""));
        // No match.
        assert_eq!("abc", before_ci("abc", "xyz"));
        // Normal.
        assert_eq!("a", before_ci("abc", "b"));
        assert_eq!("a", before_ci("abc", "B"));
        assert_eq!("a", before_ci("aBc", "b"));
        // Match right away.
        assert_eq!("", before_ci("abc", "a"));
        assert_eq!("", before_ci("abc", "A"));
        assert_eq!("", before_ci("Abc", "a"));
        // Three possible matches but we want the first one.
        assert_eq!("ab", before_ci("abc def c abc ghi", "c"));
        assert_eq!("ab", before_ci("abc def c abC ghi", "C"));
        assert_eq!("ab", before_ci("abC def c abc ghi", "c"));
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

    #[test]
    fn test_unquote() {
        assert_eq!("abc", unquote("\"abc\""));
        assert_eq!("abc", unquote("   \"abc\"  "));
        assert_eq!("abc", unquote("   abc\"  "));
        assert_eq!("abc", unquote("  \"  abc "));
        assert_eq!("abc", unquote("  abc "));
        assert_eq!("\" abc   \"\"", unquote(" \"\" abc   \"\"\""));
    }
    */

    #[test]
    fn test_replace_within_delimiters_rc() {
        let context = "test_replace_within_delimiters_rc";
        let text = "| [[tools:nav:attributes#Narrator|Narrator]] | [[tools:nav:attribute_values#Mark Steinberg|Mark Steinberg]] |";
        let exp = "| [[tools:nav:attributes#Narrator{{{pipe}}}Narrator]] | [[tools:nav:attribute_values#Mark Steinberg{{{pipe}}}Mark Steinberg]] |";
        let act = replace_within_delimiters_rc(text, "[[", "]]", "|", "{{{pipe}}}", context).unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn test_split_outside_of_delimiters_rc() {
        let context = "test_split_outside_of_delimiters_rc";
        let text = "| [[tools:nav:attributes#Narrator|Narrator]] | [[tools:nav:attribute_values#Mark Steinberg|Mark Steinberg]] |";
        let exp = "~ [[tools:nav:attributes#Narrator|Narrator]] ~ [[tools:nav:attribute_values#Mark Steinberg|Mark Steinberg]] ~";
        let splits = split_outside_of_delimiters_rc(text, "|", "[[", "]]", context).unwrap();
        let act = splits.iter().join("~");
        assert_eq!(exp, act);
    }

    #[test]
    fn test_trim_linefeeds() {
        assert_eq!("abc", trim_linefeeds("abc"));
        assert_eq!("ab\nc", trim_linefeeds("ab\nc"));
        assert_eq!("ab\nc", trim_linefeeds("\nab\nc"));
        assert_eq!("ab\nc", trim_linefeeds("ab\nc\n"));
        assert_eq!("ab\nc", trim_linefeeds("ab\nc\n\n"));
        assert_eq!("ab\nc", trim_linefeeds("\n\nab\nc\n\n\n"));
    }
}
