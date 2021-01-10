use std::collections::BTreeMap;
use std::fmt::Display;

use crate::format::{format_count, format_count_opt, println_indent_tab};
use std::cmp::Ordering;

pub fn count_distinct<T> (values: &[T]) -> usize
    where T: Ord + Clone
{
    let mut v = values.to_vec();
    v.sort();
    v.dedup();
    v.len()
}

pub fn list_duplicates<T> (values: &[T]) -> Vec<T>
    where T: Ord + Clone
{
    let mut v = values.to_vec();
    v.sort();
    let (_, duplicates) = v.partition_dedup();
    duplicates.to_vec()
}

#[derive(Debug)]
pub struct Grouper<T>
    where T: Ord + Display + Clone
{
    name: String,
    entries: BTreeMap<T, GrouperEntry<T>>,
}

#[derive(Debug)]
pub struct GrouperEntry<T>
    where T: Ord + Display + Clone
{
    key: T,
    count: usize,
}

impl <T> Grouper<T>
    where T: Ord + Display + Clone
{
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: BTreeMap::new(),
        }
    }

    pub fn record_entry(&mut self, key: &T) {
        self.record_entry_with_count(key, 1);
    }

    pub fn record_entry_with_count(&mut self, key: &T, count: usize) {
        let mut entry = self.entries.entry(key.clone()).or_insert_with(|| { GrouperEntry::new(key.clone()) } );
        entry.count += count;
    }

    pub fn list_by_key(&self) {
        let count_width = self.count_width();
        for entry in self.entries.values() {
            println!("{:>width$} - {}", format_count(entry.count), entry.key, width=count_width)
        }
    }

    pub fn print_by_count(&self, depth: usize, max_entries: Option<usize>) {
        self.print(depth, max_entries, true);
    }

    /*
    pub fn print_by_count(&self, depth: usize, max_entries: Option<usize>) {
        let count_width = self.count_width();
        let mut v = self.entries.values().collect::<Vec<_>>();
        v.sort_by(|a, b| { a.count.cmp(&b.count).reverse().then(a.key.cmp(&b.key)) } );
        let limit = max_entries.unwrap_or(v.len());
        println_indent_tab(depth, &self.label_line());
        for entry in v.iter().take(limit) {
            println_indent_tab(depth + 1, &format!("{:>width$} - {}", format_count(entry.count), entry.key, width=count_width));
        }
    }
    */

    pub fn print_by_key(&self, depth: usize, max_entries: Option<usize>) {
        self.print(depth, max_entries, false);
    }

    fn print(&self, depth: usize, max_entries: Option<usize>, sort_by_count: bool) {
        let count_width = self.count_width();
        let mut v = self.entries.values().collect::<Vec<_>>();
        if sort_by_count {
            v.sort_by(|a, b| { a.count.cmp(&b.count).reverse().then(a.key.cmp(&b.key)) } );
        }
        let limit = max_entries.unwrap_or(v.len());
        println_indent_tab(depth, &self.label_line());
        for entry in v.iter().take(limit) {
            println_indent_tab(depth + 1, &format!("{:>width$} - {}", format_count(entry.count), entry.key, width=count_width));
        }
    }

    pub fn label_line(&self) -> String {
        format!("\nname: {}: entries: {}, items: {}, counts: {}..={}",
                self.name,
                format_count(self.entry_count()),
                format_count(self.item_count()),
                format_count_opt(self.min_count()),
                format_count_opt(self.max_count()))
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn item_count(&self) -> usize {
        self.entries.values().map(|x| x.count).sum()
    }

    pub fn min_count(&self) -> Option<usize> {
        self.entries.values().map(|x| x.count).min()
    }

    pub fn max_count(&self) -> Option<usize> {
        self.entries.values().map(|x| x.count).max()
    }

    pub fn min_key(&self) -> Option<T> {
        self.entries.keys().min().map(|x| x.clone())
    }

    pub fn max_key(&self) -> Option<T> {
        self.entries.keys().max().map(|x| x.clone())
    }

    fn count_width(&self) -> usize {
        format_count(self.max_count().unwrap_or(0)).len()
    }

}

impl <T> GrouperEntry<T>
    where T: Ord + Display + Clone
{
    fn new(key: T) -> Self {
        Self {
            key,
            count: 0,
        }
    }
}

