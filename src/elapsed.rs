use std::time::Instant;
use std::collections::BTreeMap;

pub struct ElapsedFlat {
    pub context: String,
    pub entries: BTreeMap<String, ElapsedFlatEntry>,
}

pub struct ElapsedFlatEntry {
    pub times: Vec<(Instant, Option<Instant>)>,
}

impl ElapsedFlat {
    pub fn new(context: &str) -> Self {
        Self {
            context: context.to_string(),
            entries: Default::default(),
        }
    }

    pub fn start(&mut self, name: &str) {
        let entry = self.entries.entry(name.to_string()).or_insert(ElapsedFlatEntry::new());
        entry.start();
    }

    pub fn end(&mut self, name: &str) {
        self.entries.get_mut(name).unwrap().end();
    }
}

impl ElapsedFlatEntry {
    fn new() -> Self {
        Self {
            times: vec![],
        }
    }

    fn start(&mut self) {
        self.times.push((Instant::now(), None));
    }

    fn end(&mut self) {
        self.times.last_mut().unwrap().1 = Some(Instant::now());
    }
}