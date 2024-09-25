use std::fs::{metadata, File, Metadata, OpenOptions};
use std::io::{self, Read, Result, Seek, SeekFrom, Write};
use crate::base::page::Page;
use bincode::{serialize, deserialize};

const PAGE_SIZE: usize = 4096;


pub struct StorageEngine {
    file: File,
    next_page_id: u64,
}

impl StorageEngine {
    // open a file, creating it if it doesn't exist
    pub fn new(file_path: &str) -> Result<StorageEngine> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let meta_data: Metadata = file.metadata()?;
        let file_size: u64 = meta_data.len();
        let next_page_id: u64 = file_size / PAGE_SIZE as u64;

        Ok(StorageEngine {
            file,
            next_page_id
        })
    }

    pub fn allocate_page(&mut self) -> u64 {
        let page_id: u64 = self.next_page_id;
        self.next_page_id += 1;
        page_id
    }

    pub fn write_at(&mut self, offset: u64, data: &[u8]) -> Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(data)?;
        self.file.flush()?;
        
        Ok(())
    }

    pub fn read_at(&mut self, offset: u64, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; size];
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn write_page(&mut self, page_id: u64, page: &Page) -> Result<()> {
        // Step 1: Serialize the Page
        let data = serialize(page).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        // Step 2: Ensure the serialized page size doesn't exceed PAGE_SIZE
        if data.len() > PAGE_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Serialized page size exceeds PAGE_SIZE"));
        }

        // Step 3: Calculate the offset where the page will be written
        let offset = page_id * PAGE_SIZE as u64;

        // Step 4: Seek to the correct offset in the file
        self.file.seek(SeekFrom::Start(offset))?;

        // Step 5: Create a buffer of PAGE_SIZE and fill it with the serialized data
        let mut buffer = vec![0; PAGE_SIZE];
        buffer[..data.len()].copy_from_slice(&data);

        // Step 6: Write the buffer to the file
        self.file.write_all(&buffer)?;
        self.file.flush()?;  // Flush to ensure data is written to disk

        println!("Page written successfully to offset: {}", offset);  // Debugging line

        Ok(())
    }

    pub fn read_page(&mut self, page_id: u64) -> Result<Page>{
        let offset = page_id * PAGE_SIZE as u64;
        
        // Check the file size to see if the page exists
        let metadata = self.file.metadata()?;
        let file_size = metadata.len();
        
        self.file.seek(SeekFrom::Start(offset))?;
        
        // read PAGE_SIZE bytes into a buffer
        let mut buffer = vec![0;PAGE_SIZE];
        self.file.read_exact(&mut buffer)?;

        // deserialize the buffer into a Page struct
        let page:Page = deserialize(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(page)
    }
}