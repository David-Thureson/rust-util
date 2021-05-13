use itertools::Itertools;

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

#[derive(Debug)]
pub struct ExtractedSubstring {
    pub value: String,
    pub is_delimited: bool,
}

#[derive(Debug)]
pub struct ExtractedSubstringList {
    pub list: Vec<ExtractedSubstring>,
}

impl ExtractedSubstring {
    pub fn new(value: &str, is_delimited: bool) -> Self {
        let substring = Self {
            value: value.to_string(),
            is_delimited,
        };
        substring.invariant();
        substring
    }

    fn invariant(&self) {
        assert!(!self.value.is_empty());
    }
}

impl ExtractedSubstringList {
    pub fn new() -> Self {
        Self {
            list: vec![],
        }
    }

    pub fn from_str(val: &str, left_delimiter: &str, right_delimiter: &str) -> Result<Self, String> {
        let mut list = Self::new();
        let mut val = &val[..];
        loop {
            if val.is_empty() {
                list.invariant();
                return Ok(list);
            }
            let left_pos = val.find(left_delimiter);
            match left_pos {
                Some(left_pos) => {
                    let after_left_pos = left_pos + left_delimiter.len();
                    if after_left_pos >= val.len() {
                        return Err(format!("Left delimiter \"{}\" found at end of string in \"{}\".", left_delimiter, val));
                    }
                    let right_pos = val[after_left_pos..].find(right_delimiter);
                    match right_pos {
                        Some (right_pos) => {
                            let right_pos = right_pos + after_left_pos;
                            if right_pos < left_pos {
                                return Err(format!("Right delimiter \"{}\" found before left delimiter \"{}\" in string \"{}\".",
                                                   right_delimiter, left_delimiter, val));
                            }
                            if left_pos > 0 {
                                // There's some plain text before the left delimiter.
                                list.list.push(ExtractedSubstring::new(&val[..left_pos], false));
                            }
                            //bg!(val, left_pos, after_left_pos, right_pos);
                            let delimited_entry = ExtractedSubstring::new(&val[after_left_pos..right_pos], true);
                            //bg!(&delimited_entry);
                            list.list.push(delimited_entry);
                            val = &val[right_pos + right_delimiter.len()..];
                        },
                        None => {
                            return Err(format!("Right delimiter \"{}\" not found to match left delimiter \"{}\" (at index {}) in string \"{}\".",
                                               right_delimiter, left_delimiter, left_pos, val));
                        }
                    }
                },
                None => {
                    // There's some plain text remaining beyond the last right delimiter.
                    list.list.push(ExtractedSubstring::new(&val[..], false));
                    list.invariant();
                    return Ok(list);
                }
            }
        }
    }

    pub fn from_str_for_test(values: &str, start_delimited: bool) -> Self {
        let mut list = Self::new();
        let mut is_delimited = start_delimited;
        for val in values.split(" ") {
            list.list.push(ExtractedSubstring::new(val, is_delimited));
            is_delimited = !is_delimited;
        }
        list
    }

    pub fn join(&self, left_delimiter: &str, right_delimiter: &str) -> String {
        self.list.iter()
            .map(|entry| if entry.is_delimited {
                format!("{}{}{}", left_delimiter, entry.value, right_delimiter)
            } else {
                entry.value.clone()
            })
            .join("")
    }

    pub fn assert_equals(&self, other: &Self) {
        if self.list.len() != other.list.len() {
            dbg!(self, other);
            panic!("Lengths don't match.");
        }
        for (index, (entry_self, entry_other)) in self.list.iter().zip(other.list.iter()).enumerate() {
            if entry_self.value != entry_other.value {
                dbg!(self, other);
                panic!("Strings don't match at entry {}.", index);
            }
            if entry_self.is_delimited != entry_self.is_delimited {
                dbg!(self, other);
                panic!("is_delimited doesn't match at entry {}.", index);
            }
        }
    }

    fn invariant(&self) {
        if !self.list.is_empty() {
            let mut prev_is_delimited: Option<bool> = None;
            for (index, entry) in self.list.iter().enumerate() {
                entry.invariant();
                if index > 0 && !prev_is_delimited.unwrap() {
                    // It's OK to have multiple delimited entries in a row, but not to have two
                    // non-delimited entries in a row.
                    assert!(entry.is_delimited);
                }
                prev_is_delimited = Some(entry.is_delimited);
            }
        }
    }

    fn try_from_str(val: &str, expect_success: bool) {
        match Self::from_str(val, "{", "}") {
            Ok(list) => {
                let expected_join_brackets = val.replace("{", "[").replace("}", "]");
                let join_brackets = list.join("[", "]");
                //bg!(val, &list, &expected_join_brackets, &join_brackets);
                if expect_success {
                    if join_brackets != expected_join_brackets {
                        panic!("For val = \"{}\", expected \"{}\", got \"{}\".", val, expected_join_brackets, join_brackets);
                    } else {
                        println!("\nFor val = \"{}\", join = \"{}\"", val, join_brackets);
                    }
                    Self::try_round_trip(val, "{", "}", "[", "]");
                    Self::try_round_trip(val, "{", "}", "[[", "]]]");
                    Self::try_round_trip(val, "{", "}", "|", "||");
                    Self::try_round_trip(val, "{", "}", "||", "|");
                    Self::try_round_trip(val, "{", "}", "&*#%", "~");
                } else {
                    panic!("For val = \"{}\", expected a failure but got \"{}\".", val, join_brackets);
                }
            },
            Err(msg) => {
                if expect_success {
                    panic!("For val = \"{}\", expected a success but got a failure.", val);
                } else {
                    println!("\nFor val = \"{}\", expected a failure and the message is \"{}\".", val, msg);
                }
            },
        }
    }

    fn try_round_trip(val: &str, left_a: &str, right_a: &str, left_b: &str, right_b: &str) {
        let list = Self::from_str(val, left_a, right_a).unwrap();
        let join = list.join(left_b, right_b);
        let round_trip_join = Self::from_str(&join, left_b, right_b).unwrap()
            .join(left_a, right_a);
        if round_trip_join == val {
            println!("\tFor val = \"{}\", join = \"{}\", round_trip_join = \"{}\".", val, join, round_trip_join);
        } else {
            panic!("\tFor val = \"{}\", join = \"{}\", round_trip_join = \"{}\".", val, join, round_trip_join);
        }
    }
}

pub fn try_extract_multiple() {
    ExtractedSubstringList::try_from_str("",true);
    ExtractedSubstringList::try_from_str("a",true);
    ExtractedSubstringList::try_from_str("abc",true);
    ExtractedSubstringList::try_from_str("ab{c}",true);
    ExtractedSubstringList::try_from_str("ab{c}de{fgh}i",true);
    ExtractedSubstringList::try_from_str("ab{c}de{fgh}i",true);
    ExtractedSubstringList::try_from_str("{ab}cde{fg}h{i}",true);
    ExtractedSubstringList::try_from_str("{ab}cde{fgh}{i}",true);
}
