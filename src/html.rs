use std::fs;
use std::path;

use crate::parse;
use crate::format;

#[derive(Debug)]
pub struct BookmarkSet {
    pub name: String,
    pub links: Vec<BookmarkLink>,
    pub sets: Vec<Box<BookmarkSet>>,
}

#[derive(Debug)]
pub struct BookmarkLink {
    pub url: String,
    pub label: String,
}

impl BookmarkSet {
    fn new(name: &str) -> Self {
        assert!(name.len() > 0);
        Self {
            name: name.to_string(),
            links: vec![],
            sets: vec![],
        }
    }

    pub fn display_deep(&self, depth: usize) {
        // rintln!("{}", &self.name);
        format::println_indent_tab(depth, &self.name);
        for link in &self.links {
            format::println_indent_tab(depth + 1, &format!("\"{}\":\t\"{}\"", link.label, link.url));
        }
        for set in &self.sets {
            set.display_deep(depth + 1);
        }
    }

}

impl BookmarkLink {
    fn new(url: &str, label: &str) -> Self {
        assert!(url.len() > 0);
        assert!(label.len() > 0);
        Self {
            label: label.to_string(),
            url: url.to_string(),
        }
    }
}

pub fn parse_chrome_bookmarks(path_file: &path::Path) -> BookmarkSet {
    assert!(path_file.is_file());
    let lines: Vec<String> = fs::read_to_string(path_file)
        .unwrap()
        .split("\r\n")
        .map(|line| line.to_string())
        .collect();
    parse_chrome_bookmarks_internal(&lines[..])
}

pub fn parse_chrome_bookmarks_internal(lines: &[String]) -> BookmarkSet {
    let dt_h3 = "<DT><H3 ";
    let dl_p = "</DL><p>";
    let first_line = &lines[0];
    let name = parse::before(first_line, "</H3>");
    let name = parse::rafter(name, ">");
    let indent = first_line.find(dt_h3).unwrap();
    //bg!(&first_line, &name, &indent);

    let mut b = BookmarkSet::new(name);

    let last_line = lines.last().unwrap();
    let expected_prefix = format!("{}{}", " ".repeat(indent), dl_p);
    //bg!(&last_line, &expected_prefix);
    assert!(last_line.starts_with(&expected_prefix));

    let mut line_index = 2;
    let spaces = " ".repeat(indent + 4);
    let prefix_link = format!("{}<DT><A HREF=", spaces);
    let prefix_nested_set_first = format!("{}{}", spaces, dt_h3);
    let prefix_nested_set_last = format!("{}{}", spaces, dl_p);
    while line_index < lines.len() - 1 {
        let line = &lines[line_index];
        //bg!(&line_index, &line);
        if line.starts_with(&prefix_link) {
            let url = parse::after(&line, "A HREF=\"");
            let url = parse::before(url, "\"");
            let label = parse::rbefore(&line, "</A>");
            let label = parse::rafter(&label, ">");
            //dbg!(&url, &label);
            b.links.push(BookmarkLink::new(url, label));
        } else if line.starts_with(&prefix_nested_set_first) {
            let line_index_nested_start = line_index;
            line_index += 2;
            while !lines[line_index].starts_with(&prefix_nested_set_last) {
                line_index += 1;
            }
            b.sets.push(Box::new(parse_chrome_bookmarks_internal(&lines[line_index_nested_start..line_index + 1])));
        } else {
            panic!("Unexpected line: {}", line);
        }
        line_index += 1;
    }

    b
}
