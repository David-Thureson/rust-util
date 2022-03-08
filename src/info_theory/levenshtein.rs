pub fn main() {
    test();
}

pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    lev(&a.chars().collect::<Vec<_>>(), &b.chars().collect::<Vec<_>>())
}

fn lev(a: &[char], b: &[char]) -> usize {
    if b.is_empty() {
        return a.len();
    }
    if a.is_empty() {
        return b.len();
    }
    if a[0] == b[0] {
        return lev(tail(a), tail(b))
    }
    1 + [
        lev(tail(a), b),
        lev(a, tail(b)),
        lev(tail(a), tail(b))
    ].iter().min().unwrap()
}

fn tail(x: &[char]) -> &[char] {
    &x[1..]
}

fn test() {
    test_one("", "", 0);

    // One substitution.
    test_one("kitten", "sitten", 1);

    // One substitution.
    test_one("sitten", "sittin", 1);

    // One insertion.
    test_one("sittin", "sitting", 1);

    // Two substitutions and an insertion.
    test_one("kitten", "sitting", 3);

    // Two insertions and a substitution
    test_one("Sunday", "Saturday", 3);
}

fn test_one(a: &str, b: &str, exp_result: usize) {
    let act_result = levenshtein_distance(a, b);
    let error = if act_result != exp_result { "*** ERROR *** " } else { "" };
    println!("{}a = \"{}\"; b = \"{}\", exp = {}, act = {}", error, a, b, exp_result, act_result);
}
