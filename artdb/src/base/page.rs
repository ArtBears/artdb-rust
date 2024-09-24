use serde::{Serialize, Deserialize};

use super::record::Record;


#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub records: Vec<Record>,
}

impl Page {
    pub fn new() -> Page {
        Page { records: Vec::new() }
    }

    pub fn insert(&mut self, record: Record) {
        self.records.push(record);
    }

    pub fn find_record(&self, record_id: u32) -> Option<&Record> {
        self.records.iter().find(|record| record.id == record_id)
    }

    pub fn delete_record(&mut self, record_id: u32) -> bool {
        if let Some(pos) = self.records.iter().position(|rec| rec.id == record_id) {
            self.records.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn has_space(&self) -> bool {
        self.records.len() < 10
    }
}