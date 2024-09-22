use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub key: u32,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub entries: Vec<Entry>,
}

impl Page {
    pub fn new() -> Page {
        Page { entries: Vec::new() }
    }

    pub fn insert(&mut self, key: u32, value: String) {
        let entry = Entry {key, value};
        match self.entries.binary_search_by_key(&key, |e| e.key) {
            Ok(pos) => self.entries[pos] = entry,
            Err(pos) => self.entries.insert(pos, entry),
        }
    }

    pub fn find(&self, key: u32) -> Option<&Entry> {
        self.entries.binary_search_by_key(&key, |e| e.key)
            .ok()
            .and_then(|pos| self.entries.get(pos))
    }
}