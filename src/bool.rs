pub fn bool_to_yes_no(value: bool) -> String {
    (if value { "Yes" } else { "No" }).to_string()
}

pub fn string_to_bool(value: &str) -> Result<bool, String> {
    match value.to_lowercase().trim() {
        "y" | "yes" | "t" | "true" => Ok(true),
        "n" | "no" | "f" | "false" => Ok(false),
        _ => Err(format!("Unexpected boolean value = \"{}\".", value)),
    }
}

