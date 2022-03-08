// https://en.wikipedia.org/wiki/Levenshtein_distance

use std::fmt::Display;

pub fn main() {
    // test_char();
    // test_char_generic();
    // test_words();
    test_change_record();
}

pub fn levenshtein_distance_char_generic(a: &str, b: &str) -> usize {
    lev(&a.chars().collect::<Vec<_>>(), &b.chars().collect::<Vec<_>>())
}

pub fn levenshtein_distance_words(a: &str, b: &str) -> usize {
    lev(&a.split(" ").collect::<Vec<_>>(), &b.split(" ").collect::<Vec<_>>())
}

pub fn levenshtein_distance_words_record(a: &str, b: &str) -> ChangeRecord<String> {
    let a = a.split(" ").map(|x| x.to_string()).collect::<Vec<_>>();
    let b = b.split(" ").map(|x| x.to_string()).collect::<Vec<_>>();
    lev_record(&a, &b)
}

fn lev<T>(a: &[T], b: &[T]) -> usize
    where T: PartialEq
{
    // The tail(x) function is expressed as &x[1..].
    if b.is_empty() {
        return a.len();
    }
    if a.is_empty() {
        return b.len();
    }
    if a[0] == b[0] {
        return lev(&a[1..], &b[1..])
    }
    1 + [
        lev(&a[1..], b),
        lev(a, &b[1..]),
        lev(&a[1..], &b[1..])
    ].iter().min().unwrap()
}

pub struct ChangeRecord<T>
    where T: Display
{
    changes: Vec<Change<T>>,
}

pub enum Change<T>
    where T: Display
{
    Deletion {
        value: T,
    },
    Insertion {
        value: T,
    },
    Replacement {
        from: T,
        to: T,
    },
}

impl <T> ChangeRecord<T>
    where T: Display
{
    pub fn new() -> Self {
        Self {
            changes: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.changes.len()
    }

    pub fn print(&self) {
        self.changes.iter().for_each(|change| change.print());
    }
}

impl <T> Change<T>
    where T: Display
{
    pub fn new_deletion(value: T) -> Self {
        Self::Deletion {
            value,
        }
    }

    pub fn new_insertion(value: T) -> Self {
        Self::Insertion {
            value,
        }
    }

    pub fn new_replacement(from: T, to: T) -> Self {
        Self::Replacement {
            from,
            to,
        }
    }

    pub fn print(&self) {
        let description = match self {
            Self::Deletion { value } => format!("delete \"{}\"", value),
            Self::Insertion { value } => format!("insert \"{}\"", value),
            Self::Replacement { from, to } => format!("replace \"{}\" with \"{}\"", from, to),
        };
        println!("\t{}", description);
    }
}

fn lev_record<T>(a: &[T], b: &[T]) -> ChangeRecord<T>
    where T: Clone + Display + PartialEq
{
    // The tail(x) function is expressed as &x[1..].
    if b.is_empty() {
        let mut change_record = ChangeRecord::new();
        for value in a.iter() {
            change_record.changes.push(Change::new_deletion(value.clone()))
        }
        return change_record;
    }
    if a.is_empty() {
        let mut change_record = ChangeRecord::new();
        for value in b.iter() {
            change_record.changes.push(Change::new_insertion(value.clone()))
        }
        return change_record;
    }
    if a[0] == b[0] {
        return lev_record(&a[1..], &b[1..]);
    }
    let mut deletion = lev_record(&a[1..], b);
    let mut insertion = lev_record(a, &b[1..]);
    let mut replacement = lev_record(&a[1..], &b[1..]);
    let mut change_record = ChangeRecord::new();
    if replacement.len() < deletion.len() && replacement.len() < insertion.len() {
        change_record.changes.push(Change::new_replacement(a[0].clone(), b[0].clone()));
        change_record.changes.append(&mut replacement.changes);
    } else if deletion.len() < insertion.len() {
        change_record.changes.push(Change::new_deletion(a[0].clone()));
        change_record.changes.append(&mut deletion.changes);
    } else {
        change_record.changes.push(Change::new_insertion(b[0].clone()));
        change_record.changes.append(&mut insertion.changes);
    }
    change_record
}

#[allow(dead_code)]
fn test_char_generic() {
    test_one_char_generic("", "", 0);

    // One substitution.
    test_one_char_generic("kitten", "sitten", 1);

    // One substitution.
    test_one_char_generic("sitten", "sittin", 1);

    // One insertion.
    test_one_char_generic("sittin", "sitting", 1);

    // Two substitutions and an insertion.
    test_one_char_generic("kitten", "sitting", 3);

    // Two insertions and a substitution
    test_one_char_generic("Sunday", "Saturday", 3);
}

#[allow(dead_code)]
fn test_one_char_generic(a: &str, b: &str, exp_result: usize) {
    let act_result = levenshtein_distance_char_generic(a, b);
    let error = if act_result != exp_result { "*** ERROR *** " } else { "" };
    println!("{}a = \"{}\"; b = \"{}\", exp = {}, act = {}", error, a, b, exp_result, act_result);
}

#[allow(dead_code)]
fn test_words() {
    test_one_words("", "", 0);

    // Identical.
    test_one_words("due to a number of reasons", "due to a number of reasons", 0);

    // One substitution.
    test_one_words("due to a number of reasons", "due to a set of reasons", 1);

    // Two substitutions and two deletions.
    test_one_words("due to a number of reasons", "thanks to several reasons", 4);

    test_one_words("due to a number of reasons", "thanks to several factors in play", 5);
}

#[allow(dead_code)]
fn test_one_words(a: &str, b: &str, exp_result: usize) {
    let act_result = levenshtein_distance_words(a, b);
    let error = if act_result != exp_result { "*** ERROR *** " } else { "" };
    println!("{}a = \"{}\"; b = \"{}\", exp = {}, act = {}", error, a, b, exp_result, act_result);
}

#[allow(dead_code)]
fn test_change_record() {
    test_one_change_record("", "", 0);

    // Identical.
    test_one_change_record("due to a number of reasons", "due to a number of reasons", 0);

    // One replacement.
    test_one_change_record_both_ways("due to a number of reasons", "due to a set of reasons", 1);

    // Two replacements and two deletions.
    test_one_change_record_both_ways("due to a number of reasons", "thanks to several reasons", 4);

    // Five substitutions.
    test_one_change_record_both_ways("due to a number of reasons", "thanks to several factors in play", 5);

    // Five substitutions and two insertions.
    test_one_change_record_both_ways("due to a number of reasons", "thanks to several more factors in play now", 7);
}

#[allow(dead_code)]
fn test_one_change_record_both_ways(a: &str, b: &str, exp_result: usize) {
    test_one_change_record(a, b, exp_result);
    test_one_change_record(b, a, exp_result);
}

#[allow(dead_code)]
fn test_one_change_record(a: &str, b: &str, exp_result: usize) {
    let change_record = levenshtein_distance_words_record(a, b);
    let act_result = change_record.len();
    let error = if act_result != exp_result { "*** ERROR *** " } else { "" };
    println!("{}a = \"{}\"; b = \"{}\", exp = {}, act = {}", error, a, b, exp_result, act_result);
    change_record.print();
    println!();
}

pub fn levenshtein_distance_char(a: &str, b: &str) -> usize {
    lev_char(&a.chars().collect::<Vec<_>>(), &b.chars().collect::<Vec<_>>())
}

fn lev_char(a: &[char], b: &[char]) -> usize {
    if b.is_empty() {
        return a.len();
    }
    if a.is_empty() {
        return b.len();
    }
    if a[0] == b[0] {
        return lev_char(tail_char(a), tail_char(b))
    }
    1 + [
        lev_char(tail_char(a), b),
        lev_char(a, tail_char(b)),
        lev_char(tail_char(a), tail_char(b))
    ].iter().min().unwrap()
}

#[inline]
fn tail_char(x: &[char]) -> &[char] {
    &x[1..]
}

#[allow(dead_code)]
fn test_char() {
    test_one_char("", "", 0);

    // One substitution.
    test_one_char("kitten", "sitten", 1);

    // One substitution.
    test_one_char("sitten", "sittin", 1);

    // One insertion.
    test_one_char("sittin", "sitting", 1);

    // Two substitutions and an insertion.
    test_one_char("kitten", "sitting", 3);

    // Two insertions and a substitution
    test_one_char("Sunday", "Saturday", 3);
}

#[allow(dead_code)]
fn test_one_char(a: &str, b: &str, exp_result: usize) {
    let act_result = levenshtein_distance_char(a, b);
    let error = if act_result != exp_result { "*** ERROR *** " } else { "" };
    println!("{}a = \"{}\"; b = \"{}\", exp = {}, act = {}", error, a, b, exp_result, act_result);
}
