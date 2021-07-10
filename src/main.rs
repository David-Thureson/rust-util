#![allow(dead_code)]

const PATH_CHROME_BOOKMARKS: &str = r"E:\Temp\bookmarks_1_29_20.html";

use std::path;
use num_format::Locale;
use util::*;
use terminal_size::{Width, Height, terminal_size};

// use util-rust::html;
// use util-rust::format;

fn main() {
    println!("\nUtil start.\n");

    // gen_bookmarks_for_connectedtext();
    // try_format_locale();
    // try_terminal_size();
    // try_hanging_indent();
    // extract::try_extract_multiple();
    parse::try_split_delimited_and_normal_rc();

    println!("\nUtil done.\n");
}

fn gen_bookmarks_for_connectedtext() {
    let bookmark_set = html::parse_chrome_bookmarks(&path::Path::new(PATH_CHROME_BOOKMARKS));
    bookmark_set.display_deep(0);
}

fn try_format_locale() {
    dbg!(format::format_int_locale(1_000_000, &Locale::en));
    dbg!(format::format_int_locale(1_000_000, &Locale::vi));
    dbg!(format::format_float_locale(1_234_567.987654321, &Locale::en, 6));
}

fn try_terminal_size() {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
    } else {
        println!("Unable to get terminal size");
    }
}

fn try_hanging_indent() {
    let s = "uet s tha stsuhaosu, uashuas uau aosuthu! aua ouasu ustisiuaoesu ao.";
    println!("{}", format::wrap_hanging_indent(s, "Label: ", 2, 30));
}
