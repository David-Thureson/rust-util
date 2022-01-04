pub fn usize_from_string(value: &str) -> Result<usize, String> {
    match value.trim().parse::<usize>() {
        Ok(number) => Ok(number),
        Err(err) => Err(format!("Error trying to parse \"{}\" as a usize: \"{}\".", value, err)),
    }
}
