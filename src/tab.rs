use crate::format;
use crate::parse;
use chrono::NaiveDate;

pub fn cell_as_usize(val: &str) -> usize {
    let val = clean_cell(val);
    let val_trim = val.replace("\"", "").replace(",", "").trim().to_string();
    if val_trim.is_empty() || val_trim == "-".to_string() || val_trim == "(0)".to_string() {
        0
    } else {
        //bg!(&val_trim);
        val_trim.parse::<usize>().expect(&format!("Unable to parse \"{}\" as usize, val = \"{}\"", &val_trim, &val))
    }
}

pub fn cell_as_usize_optional(val: &str) -> Option<usize> {
    if clean_number(val).trim().is_empty() {
        None
    } else {
        Some(cell_as_usize(val))
    }
}

pub fn cell_as_usize_opt_no_zero(val: &str) -> Option<usize> {
    cell_as_usize_optional(val).filter(|val_usize| *val_usize > 0)
}

pub fn cell_as_usize_result(val: &str) -> Result<usize, String> {
    let val = clean_cell(val);
    let val_trim = val.replace("\"", "").replace(",", "").trim().to_string();
    if val_trim.is_empty() || val_trim == "-".to_string() || val_trim == "(0)".to_string() {
        Ok(0)
    } else {
        //bg!(&val_trim);
        match val_trim.parse::<usize>() {
            Ok(val_usize) => Ok(val_usize),
            Err(err) => Err(format!("Can't parse \"{}\" as usize: {}", val, err.to_string())),
        }
    }
}

pub fn cell_as_usize_optional_result(val: &str) -> Result<Option<usize>, String> {
    if clean_number(val).trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(cell_as_usize_result(val)?))
    }
}

pub fn cell_as_usize_opt_no_zero_result(val: &str) -> Result<Option<usize>, String> {
    Ok(cell_as_usize_optional_result(val)?.filter(|val_usize| *val_usize > 0))
}

pub fn cell_as_date(val: &str) -> NaiveDate {
    let val = clean_cell(val);
    //bg!(val);
    NaiveDate::parse_from_str(val.trim(), "%m/%d/%y").unwrap()
}

pub fn cell_as_date_optional(val: &str) -> Option<NaiveDate> {
    let val = clean_cell(val);
    if val.is_empty() {
        None
    } else {
        Some(cell_as_date(&val))
    }
}

pub fn cell_as_date_result(val: &str) -> Result<NaiveDate, String> {
    let val = clean_cell(val);
    match NaiveDate::parse_from_str(val.trim(), "%m/%d/%y") {
        Ok(date) => Ok(date),
        Err(err) => Err(format!("Can't parse \"{}\" as date: {}", val, err.to_string())),
    }
}

pub fn cell_as_date_optional_result(val: &str) -> Result<Option<NaiveDate>, String> {
    let val = clean_cell(val);
    if val.is_empty() {
        Ok(None)
    } else {
        Ok(Some(cell_as_date_result(&val)?))
    }
}

pub fn cell_as_string(val: &str) -> String {
    let val = clean_cell(val);
    val.to_string().replace("\"\"","\"")
}

pub fn cell_as_string_or(val: &str, default: &str) -> String {
    let val = clean_cell(val);
    if val.is_empty() {
        default.to_string()
    } else {
        cell_as_string(&val)
    }
}

pub fn cell_as_string_optional(val: &str) -> Option<String> {
    let val = clean_cell(val);
    if val.is_empty() {
        None
    } else {
        Some(cell_as_string(&val))
    }
}

pub fn cell_as_f32(val: &str) -> f32 {
    clean_cell(val).trim().parse::<f32>().unwrap()
}

pub fn cell_as_f32_optional(val: &str) -> Option<f32> {
    let val = clean_cell(val);
    if val.is_empty() || val.eq("-") {
        None
    } else {
        Some(cell_as_f32(&val))
    }
}

pub fn cell_as_f32_opt_no_zero(val: &str) -> Option<f32> {
    cell_as_f32_optional(val).filter(|val_f32| *val_f32 != 0.0)
}

pub fn cell_as_f32_result(val: &str) -> Result<f32, String> {
    match clean_cell(val).trim().parse::<f32>() {
        Ok(val_f32) => Ok(val_f32),
        Err(err) => Err(format!("Can't parse \"{}\" as f32: {}", val, err.to_string())),
    }
}

pub fn cell_as_f32_optional_result(val: &str) -> Result<Option<f32>, String> {
    let val = clean_cell(val);
    if val.is_empty() || val.eq("-") {
        Ok(None)
    } else {
        Ok(Some(cell_as_f32_result(&val)?))
    }
}

pub fn cell_as_f32_opt_no_zero_result(val: &str) -> Result<Option<f32>, String> {
    Ok(cell_as_f32_optional_result(val)?.filter(|val_f32| *val_f32 != 0.0))
}

pub fn cell_as_price(val: &str) -> f32 {
    //bg!(val);
    let val = clean_price(val);
    //bg!(&val);
    cell_as_f32(&val)
}

pub fn cell_as_price_optional(val: &str) -> Option<f32> {
    cell_as_f32_optional(&clean_price(val))
}

pub fn cell_as_price_opt_no_zero(val: &str) -> Option<f32> {
    cell_as_price_optional(val).filter(|price| *price != 0.0)
}

pub fn cell_as_price_result(val: &str) -> Result<f32, String> {
    let val = clean_price(val);
    cell_as_f32_result(&val)
}

pub fn cell_as_price_optional_result(val: &str) -> Result<Option<f32>, String> {
    cell_as_f32_optional_result(&clean_price(val))
}

pub fn cell_as_price_opt_no_zero_result(val: &str) -> Result<Option<f32>, String> {
    cell_as_f32_opt_no_zero_result(&clean_price(val))
}

pub fn cell_as_bool(val: &str) -> bool {
    val.trim() == "1"
}

pub fn cell_as_bool_result(val: &str) -> Result<bool, String> {
    let val = clean_cell(val);
    match val.trim().to_lowercase().as_str() {
        "1" | "t" | "true" | "y" | "yes" => Ok(true),
        "" | "0" | "f" | "false" | "n" | "no" => Ok(false),
        _ => Err(format!("Unable to parse \"{}\" as bool.", val)),
    }
}

fn clean_cell(val: &str) -> String {
    let val = val.replace("\u{0}", "");
    let val = format::remove_surrounding_delimiters(val.trim(), "\"", "\"");
    val.trim().to_string()
}

fn clean_number(val: &str) -> String {
    let val = clean_cell(val);
    val.replace(",", "").replace("\"", "").trim().to_string()
}

fn clean_price(val: &str) -> String {
    let val = clean_cell(val);
    clean_number(parse::after(val.trim(), "$")).trim().to_string()
}
