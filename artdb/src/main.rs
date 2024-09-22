use base::{page::Page, storage_engine::StorageEngine};
use std::fs::metadata;

mod base;

fn main() -> std::io::Result<()> {
    let file_path = "data.db";
    let mut engine: StorageEngine = StorageEngine::new(&file_path)?;
    let mut page: Page = Page::new();

    page.insert(1, "One".to_string());
    page.insert(2, "Two".to_string());

    engine.write_page(0, &page)?;

    
    let read_page = engine.read_page(0)?;
    for entry in &read_page.entries {
        println!("Key: {}, Value: {}", entry.key, entry.value);
    }

    Ok(())
}
