use serde::{Serialize, Deserialize};
use std::io::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub id: u32,
    pub fields: Vec<(String, String)>,
}

impl Record {
    pub fn new(id: u32, fields: Vec<(String, String)>) -> Record {
        Record {
            id,
            fields,
        }
    }

    pub fn get_field(&self, field_name: &str) -> Option<&String> {
        for(name, field) in &self.fields {
            if name == field_name {
                return Some(field);
            }
        }
        None
    }

    pub fn put_field(&mut self, field_name: &str, field_value: &str) -> Result<()> {
        for(name, field) in &mut self.fields {
            if name == field_name {
                *field = field_value.to_string();
                return Ok(())
            }
        }
        self.fields.push((field_name.to_string(), field_value.to_string()));
        Ok(())

    }
}