use num_format::{Locale, ToFormattedStr, ToFormattedString};
use std::fmt::Display;
use std::time::{Instant, SystemTime};
use itertools::Itertools;
use textwrap;

use super::parse;
use chrono::{DateTime, Local, NaiveDate};

//const ACRONYMS: [&str; 1] = ["TV"];

pub fn indent_space(depth: usize) -> String {
    "    ".repeat(depth)
}

pub fn format_indent_space(depth: usize, line: &str) -> String {
    format!("{}{}", "    ".repeat(depth), line)
}

pub fn format_indent_line_space(depth: usize, line: &str) -> String {
    format!("\n{}", format_indent_space(depth, line))
}

pub fn println_indent_space(depth: usize, line: &str) {
    println!("{}", format_indent_space(depth, line));
}

pub fn format_indent_tab(depth: usize, line: &str) -> String {
    format!("{}{}", "\t".repeat(depth), line)
}

pub fn format_indent_line_tab(depth: usize, line: &str) -> String {
    format!("\n{}", format_indent_tab(depth, line))
}

pub fn println_indent_tab(depth: usize, line: &str) {
    println!("{}", format_indent_tab(depth, line));
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

/*
pub fn format_count(val: usize) -> String {
    val.to_formatted_string(&Locale::en)
}
*/
pub fn format_count<T: ToFormattedStr>(val: T) -> String {
    val.to_formatted_string(&Locale::en)
}

pub fn format_count_opt<T: ToFormattedStr>(val: Option<T>) -> String {
    val.map_or("None".to_string(),|val| format_count(val))
}

pub fn format_int_locale<T>(val: T, locale: &Locale) -> String
    where T: ToFormattedStr
{
    val.to_formatted_string(locale)
}

pub fn format_float_locale<T>(val: T, locale: &Locale, precision: usize) -> String
    where T: Into<f64>
{
    let val = val.into();
    if val.is_finite() {
        // let locale = SystemLocale::default().unwrap();
        let left = format_int_locale(val.trunc().abs() as i128, locale);
        let right = &format!("{:.*}", precision, val.fract().abs())[2..];
        let minus_sign = if val.is_sign_negative() { locale.minus_sign() } else { "" };
        format!("{}{}{}{}", minus_sign, left, locale.decimal(), right)
    } else {
        format!("{:?}", val)
    }
}

pub fn format_float<T>(val: T, precision: usize) -> String
    where T: Into<f64>
{
    format!("{:.*}", precision, val.into())
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

pub fn format_naive_date_sortable(value: &NaiveDate) -> String {
    value.format("%Y-%m-%d").to_string()
}

// Return the value passed. This is used to show whether a function call is eager- or
// lazy-evaluated.
pub fn produce_value<T: Display>(val: T) -> T {
    println!("produce_value({})", val);
    val
}

// Return the value passed. This is used to show whether a function call is eager- or
// lazy-evaluated.
pub fn produce_value_label<T, L: Display>(val: T, label: L) -> T {
    println!("produce_value({})", label);
    val
}

pub fn vec_copy_for_display<T: Display>(vec: &Vec<T>) -> Vec<String> {
    vec.iter().map(|x| format!("{}", x)).collect()
}

pub fn force_substring_case(s: &str, substring: &str) -> String {
    if s.len() == 0 || substring.len() == 0 {
        return s.to_string();
    }
    let s_lower = s.to_lowercase();
    let mut s = s.to_string();
    for one_match in s_lower
            .match_indices(&substring.to_lowercase()) {
        let index = one_match.0;
        s = format!("{}{}{}", &s[..index], substring, &s[index + substring.len()..]);
    }
    s
}
/*
fn force_substring_case(s: &str, substring: &str) -> String {
    match s.to_lowercase().find(&substring.to_lowercase()) {
        Some(index) => format!("{}{}{}", &s[..index], substring, &s[index + substring.len()..]),
        None => s.to_string(),
    }
}
*/

pub fn force_substring_cases(s: &str, substrings: Option<&[&str]>) -> String {
    match substrings {
        Some(substrings) => {
            let mut s = s.to_string();
            for substring in substrings {
                s = force_substring_case(&s, substring);
            }
            s
        },
        None => s.to_string(),
    }
}

pub fn title_case(s: &str, force_case_strings: Option<&[&str]>) -> String {

    // The titlecase() function will trim whitespace.
    let s_trimmed = s.trim();
    let index = s.find(s_trimmed).unwrap();
    let whitespace_left = &s[..index];
    let whitespace_right = &s[index + s_trimmed.len()..];
    let s = format!("{}{}{}", whitespace_left, titlecase::titlecase(s_trimmed), whitespace_right);

    force_substring_cases(&s, force_case_strings)
}

pub fn title_case_file_name(s: &str, force_case_strings: Option<&[&str]>) -> String {
    // With a string like "abc def.txt" normally the title case function will see "def.txt" as a
    // single word and leave it unchanged. So remove the period and file extension, title case the
    // part before that, and put the period and file extension back.
    let (extension, name) = parse::rsplit_2(s, ".");
    format!{"{}.{}", title_case(name, force_case_strings), extension}
}

pub fn substitute_characters(s: &str, chars_to_remove: &str, substitute: &str) -> String {
    let mut s = s.to_string();
    for c in chars_to_remove.chars() {
        s = s.replace(c, substitute);
    }
    s
}

pub fn windows_file_name(s: &str, substitute: &str) -> String {
    // This doesn't check for characters with an ASCII value between 1 and 31, which are also not
    // allowed.
    substitute_characters(s, "<>:\"/\\|?*", substitute)
}

pub fn wrap_hanging_indent(s: &str, label: &str, depth: usize, width: usize) -> String {
    let initial_indent = format!("{}{}", indent_space(depth), label);
    let subsequent_indent = indent_space(depth + 1);
    let options = textwrap::Options::new(width)
        .initial_indent(&initial_indent)
        .subsequent_indent(&subsequent_indent);
    textwrap::fill(s, options)
}

pub fn print_wrap_hanging_indent(s: &str, label: &str, depth: usize, width: usize) {
    println!("{}", wrap_hanging_indent(s, label, depth, width));
}

pub fn header(level: usize, label: &str, width: usize) -> String {
    match level {
        0 => format!("\n{}\n{}\n{}\n\n// {}", "/".repeat(width), "/".repeat(width), "/".repeat(width), label),
        1 => format!("\n{}\n\n// {}", "/".repeat(width), label),
        _ => format!("\n// {}", label),
    }
}

pub fn print_header(level: usize, label: &str, width: usize) {
    println!("{}", header(level, label, width));
}

pub fn list_flags(labels: &[&str], flags: &[bool]) -> String {
    debug_assert_eq!(labels.len(), flags.len());
    labels
        .iter()
        .zip(flags.iter())
        .map(|(label, flag)| if *flag { format!(" {}", label) } else { "".to_string() })
        .join("")
}

pub fn list_flags_with_not(labels: &[&str], flags: &[bool]) -> String {
    debug_assert_eq!(labels.len(), flags.len());
    labels
        .iter()
        .zip(flags.iter())
        .map(|(label, flag)| format!("{}{}", if *flag { "" } else { "not " }, label))
        .join(", ")
}

pub fn remove_punctuation(value: &str, punctuation_chars: &str, replacement: char) -> String {
    let value: String = value.chars()
        .map(|c| if punctuation_chars.contains(c) {
            replacement
        } else {
            c
        })
        .join("");
    value
}

/*
pub fn extract_quoted_strings(value: &str) -> Vec<String> {

}
*/

#[allow(unreachable_code)]
pub fn remove_repeated(value: &str, substring: &str) -> String {
    let mut value = value.to_string();
    let substring_doubled = substring.repeat(2);
    loop {
        let len = value.len();
        value = value.replace(&substring_doubled, substring);
        if value.len() == len {
            return value;
        }
    }
    unreachable!()
}

pub fn remove_surrounding_delimiters(value: &str, left: &str, right: &str) -> String {
    if value.starts_with(left) && value.ends_with(right) {
        value[left.len()..value.len()-right.len()].to_string()
    } else {
        value.to_string()
    }
}

pub fn add_indefinite_article(value: &str) -> String {
    assert!(!value.is_empty());
    let vowels = ["a", "e", "i", "o", "u"];
    if vowels.iter().any(|c| value.to_lowercase().starts_with(c)) {
        format!("an {}", value)
    } else {
        format!("a {}", value)
    }
}

pub fn first_cap_phrase(value: &str) -> String {
    if value.is_empty() {
        "".to_string()
    } else {
        value.split(" ").into_iter()
            .map(|word| first_cap_word(word))
            .join(" ")
    }
}

pub fn first_cap_word(value: &str) -> String {
    if value.is_empty() {
        "".to_string()
    } else if value.len() == 1 {
        value.to_uppercase()
    } else {
        let (first, rest) = value.split_at(1);
        // format!("{}{}", value[0..1], value[1..])
        format!("{}{}", first.to_uppercase(), rest.to_lowercase())
    }
}

pub fn limit_length(value: &str, max_length: Option<usize>) -> String {
    if let Some(max_length) = max_length {
        assert!(max_length >= 4);
        if value.len() <= max_length {
            value.to_string()
        } else {
            format!("{}...", &value[..max_length - 3])
        }
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_indent_space() {
        assert_eq!("", format_indent_space(0, ""));
        assert_eq!("    ", format_indent_space(1, ""));
        assert_eq!("        ", format_indent_space(2, ""));
        assert_eq!("abc", format_indent_space(0, "abc"));
        assert_eq!("    abc", format_indent_space(1, "abc"));
        assert_eq!("        abc", format_indent_space(2, "abc"));
    }

    #[test]
    fn test_format_indent_line_space() {
        assert_eq!("\n", format_indent_line_space(0, ""));
        assert_eq!("\n    ", format_indent_line_space(1, ""));
        assert_eq!("\n        ", format_indent_line_space(2, ""));
        assert_eq!("\nabc", format_indent_line_space(0, "abc"));
        assert_eq!("\n    abc", format_indent_line_space(1, "abc"));
        assert_eq!("\n        abc", format_indent_line_space(2, "abc"));
    }

    #[test]
    fn test_format_indent_tab() {
        assert_eq!("", format_indent_tab(0, ""));
        assert_eq!("\t", format_indent_tab(1, ""));
        assert_eq!("\t\t", format_indent_tab(2, ""));
        assert_eq!("abc", format_indent_tab(0, "abc"));
        assert_eq!("\tabc ", format_indent_tab(1, "abc "));
        assert_eq!("\t\tabc  ", format_indent_tab(2, "abc  "));
    }

    #[test]
    fn test_format_line_indent_tab() {
        assert_eq!("\n", format_indent_line_tab(0, ""));
        assert_eq!("\n\t", format_indent_line_tab(1, ""));
        assert_eq!("\n\t\t", format_indent_line_tab(2, ""));
        assert_eq!("\nabc", format_indent_line_tab(0, "abc"));
        assert_eq!("\n\tabc ", format_indent_line_tab(1, "abc "));
        assert_eq!("\n\t\tabc  ", format_indent_line_tab(2, "abc  "));
    }

    #[test]
    fn test_format_count() {
        assert_eq!("0", format_count(0u8));
        assert_eq!("0", format_count(0usize));
        assert_eq!("12,345,678", format_count(12_345_678u32));
    }

    #[test]
    fn test_force_substring_case() {
        // Empty string.
        assert_eq!("", force_substring_case("", "abc"));

        // Empty substring.
        assert_eq!("  Abc", force_substring_case("  Abc", ""));

        // No match.
        assert_eq!("Abc DEF  Ghi   ", force_substring_case("Abc DEF  Ghi   ", "xyz"));

        // One match that doesn't change anything.
        assert_eq!(" Abc DEF Ghi", force_substring_case(" Abc DEF Ghi", "DEF"));

        // One match that changes the string.
        assert_eq!("Abc def Ghi", force_substring_case("Abc DEF Ghi", "def"));
        assert_eq!("Abc Def Ghi", force_substring_case("Abc DEF Ghi", "Def"));
        assert_eq!("Abc DEF Ghi ", force_substring_case("Abc DEF Ghi ", "DEF"));
        assert_eq!("AbC dEF Ghi", force_substring_case("Abc DEF Ghi", "C d"));
        assert_eq!("abc DEF Ghi", force_substring_case("Abc DEF Ghi", "a"));

        // Three matches.
        assert_eq!("  aBC Def aBCaBC    ", force_substring_case("  Abc Def AbcAbc    ", "aBC"));
    }

    #[test]
    fn test_force_substring_cases() {
        // Empty string.
        assert_eq!("", force_substring_cases("", Some(&["abc"])));

        // No substrings.
        assert_eq!("Abc", force_substring_cases("Abc", None));
        assert_eq!("Abc", force_substring_cases("Abc", Some(&[])));

        // No matches.
        assert_eq!("Abc DEF Ghi", force_substring_cases("Abc DEF Ghi", Some(&["xyz", "123"])));

        // Two matches.
        assert_eq!("Abc dEf gHI", force_substring_cases("Abc DEF Ghi", Some(&["gHI", "dEf"])));

        // Two matches where the second overrides part of the first.
        assert_eq!("Abc deF ghi", force_substring_cases("Abc DEF Ghi", Some(&["def", "F g"])));

        //assert_eq!("", force_substring_cases("", Some(&["abc"])));

    }

    #[test]
    fn test_title_case() {

        // No forced case substrings. Make sure we keep the whitespace before and after.
        assert_eq!("History of Rome ", title_case("history of rome ", None));
        assert_eq!(" History of Rome", title_case(" history of rome", None));
        assert_eq!("\t\tHistory   of Rome   \t", title_case("\t\thistory   of rome   \t", None));
    }

    #[test]
    fn test_title_case_file_name() {

        // No forced case substrings.
        assert_eq!("Abc Def.txt", title_case_file_name("abc def.txt", None));
        assert_eq!("Abc Def.txt", title_case_file_name("abc def.txt", Some(&[])));

        // No matches.
        assert_eq!("Abc Def.txt", title_case_file_name("abc def.txt", Some(&["xyz", "123"])));

        // Prove that the extension is unaffected.
        assert_eq!("Abc Def.TXT", title_case_file_name("abc def.TXT", Some(&["xyz", "123"])));

        // Two forced case substrings.
        assert_eq!("Abc of CON entire Def.TXT", title_case_file_name("abc of con entire def.TXT", Some(&["entire", "CON", "xyz"])));
    }

    #[test]
    fn test_remove_repeated() {
        assert_eq!("a&b   & c&", remove_repeated("a&&&&b   & c&&&", "&"));
        assert_eq!("a&&&&b & c&&&", remove_repeated("a&&&&b   & c&&&", " "));
        assert_eq!("a&&&&b   & c&&&", remove_repeated("a&&&&b   & c&&&", "c"));
        assert_eq!("a&&&&b   & c&&&", remove_repeated("a&&&&b   & c&&&", "d"));
        assert_eq!("a&&b   & c&&&", remove_repeated("a&&&&b   & c&&&", "&&"));
    }

    #[test]
    fn test_remove_surrounding_delimiters() {
        assert_eq!("abcd", remove_surrounding_delimiters("\"abcd\"", "\"", "\""));
        assert_eq!("\"abcd\" ", remove_surrounding_delimiters("\"abcd\" ", "\"", "\""));
        assert_eq!("\"abcd", remove_surrounding_delimiters("\"abcd", "\"", "\""));
        assert_eq!("abcd\"", remove_surrounding_delimiters("abcd\"", "\"", "\""));
        assert_eq!("abcd", remove_surrounding_delimiters("{abcd}", "{", "}"));
        assert_eq!("}abcd{", remove_surrounding_delimiters("}abcd{", "{", "}"));
        assert_eq!("", remove_surrounding_delimiters("{}", "{", "}"));
    }
}

