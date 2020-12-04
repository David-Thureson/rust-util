use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LOG: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub fn clear() {
    LOG.lock().unwrap().clear();
}

pub fn log(line: &str) {
    LOG.lock().unwrap().push(line.to_string());
}

pub fn get() -> Vec<String> {
    LOG.lock().unwrap().clone()
}

pub fn get_sorted() -> Vec<String> {
    let mut v = get();
    v.sort();
    v
}