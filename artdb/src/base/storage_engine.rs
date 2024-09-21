use std::fs::{File, OpenOptions};
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
        self.write_at(offset, &data)?;
        
        Ok(())
    }

    pub fn read_page(&mut self, offset: u64, size: usize) -> Result<Page>{
        let data = self.read_at(offset, size)?;
        let page: Page = deserialize(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(page)
    }
}