
#[allow(dead_code)]
const PATH_CHROME_BOOKMARKS: &str = r"E:\Temp\bookmarks_1_29_20.html";

use std::path;
use num_format::Locale;

use util::html;
use util::format;

fn main() {
    println!("\nUtil start.\n");

    //gen_bookmarks_for_connectedtext();
    try_format_locale();

    println!("\nUtil done.\n");
}

#[allow(dead_code)]
fn gen_bookmarks_for_connectedtext() {
    let bookmark_set = html::parse_chrome_bookmarks(&path::Path::new(PATH_CHROME_BOOKMARKS));
    bookmark_set.display_deep(0);
}

fn try_format_locale() {
    dbg!(format::format_int_locale(1_000_000, &Locale::en));
    dbg!(format::format_int_locale(1_000_000, &Locale::vi));
    dbg!(format::format_float_locale(1_234_567.987654321, &Locale::en, 6));
}

