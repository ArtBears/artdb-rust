use std::fs::{File, OpenOptions, metadata};
use std::io::{self, Read, Result, Seek, SeekFrom, Write};
use crate::base::page::Page;
use bincode::{serialize, deserialize};

pub struct StorageEngine {
    file: File,
}

impl StorageEngine {
    // open a file, creating it if it doesn't exist
    pub fn new(file_path: &str) -> Result<StorageEngine> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        Ok(StorageEngine {file})
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

    pub fn write_page(&mut self, offset: u64, page: &Page) -> Result<()> {
        let data = serialize(page).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let page_size = data.len() as u64;
        
        // Write the size of the page first (Header)
        self.write_at(offset, &page_size.to_le_bytes())?;

        self.write_at(offset + 8, &data)?;
        
        Ok(())
    }

    pub fn read_page(&mut self, offset: u64) -> Result<Page>{
        // Read the 8-byte header first
        let size_buffer = self.read_at(offset, 8)?;
        let size_array = size_buffer.try_into().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to convert to array"))?;
        let page_size = u64::from_le_bytes(size_array);
        
        let data = self.read_at(offset + 8, page_size as usize)?;
        let page: Page = deserialize(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(page)
    }
}