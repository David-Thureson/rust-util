pub fn main() {
    // test_char();
    // test_char_generic();
    test_words();
}

pub fn levenshtein_distance_char_generic(a: &str, b: &str) -> usize {
    lev(&a.chars().collect::<Vec<_>>(), &b.chars().collect::<Vec<_>>())
}

pub fn levenshtein_distance_words(a: &str, b: &str) -> usize {
    lev(&a.split(" ").collect::<Vec<_>>(), &b.split(" ").collect::<Vec<_>>())
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

/*
fn lev_track<T>(a: &[T], b: &[T]) -> (usize, (usize, usize, usize))
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
*/

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
