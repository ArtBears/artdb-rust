use base::{page::Page, storage_engine::StorageEngine, record::Record};
use std::fs::metadata;

mod base;

fn main() -> std::io::Result<()> {
    let mut engine = StorageEngine::new("data.db")?;

    // Create a new Page with some records
    let mut page = Page::new();
    page.records.push(Record {
        id: 1,
        fields: vec![("name".to_string(), "Alice".to_string())]
    });
    page.records.push(Record {
        id: 2,
        fields: vec![("name".to_string(), "Bob".to_string())]
    });

    // Write the page to disk
    let page_id = engine.allocate_page();
    engine.write_page(page_id, &page)?;

    // Read the page back from disk
    let read_page = engine.read_page(page_id)?;
    println!("Read page: {:?}", read_page);

    Ok(())
}
