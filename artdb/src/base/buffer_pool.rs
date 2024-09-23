use std::collections::{HashMap, VecDeque};
use std::io::{self, Result, Seek, SeekFrom, Write};
use std::rc::Rc;
use std::cell::RefCell;

use bincode::{deserialize, serialize};

use super::page::{self, Page};
use super::storage_engine::StorageEngine;

pub struct BufferPool {
    pool: HashMap<u64, Rc<RefCell<BufferPage>>>,
    usage_queue: VecDeque<u64>,
    capacity: usize,
}

#[derive(Debug)]
pub struct BufferPage {
    pub data: Vec<u8>,
    pub is_dirty: bool,
    pub is_pinned: bool,
}

impl BufferPage {
    pub fn new(data: Vec<u8>) -> Self {
        BufferPage {
            data,
            is_dirty: false,
            is_pinned: false,
        }
    }
}

impl BufferPool {
    pub fn new(capacity: usize) -> Self {
        BufferPool {
            pool: HashMap::new(),
            usage_queue: VecDeque::new(),
            capacity,
        }
    }

    pub fn get_page(&mut self, page_id: u64, engine: &mut StorageEngine) -> Result<Rc<RefCell<BufferPage>>> {
        // If the page is already in the buffer pool, return it
        if let Some(page) = self.pool.get(&page_id) {
            let page_clone = Rc::clone(&page);
            self.mark_page_as_used(page_id);
            return Ok(page_clone);
        }

        // Load the page from disk if it's not in the buffer pool
        let page = engine.read_page(page_id)?;
        let buffer_page = Rc::new(RefCell::new(BufferPage::new(serialize(&page).unwrap())));

        // Insert the page into the buffer pool
        self.add_page_to_pool(page_id, Rc::clone(&buffer_page), engine);

        Ok(buffer_page)

    }

    pub fn mark_page_as_used(&mut self, page_id: u64) {
        if let Some(pos) = self.usage_queue.iter().position(|&id| id == page_id) {
            self.usage_queue.remove(pos);
        }

        self.usage_queue.push_back(page_id);
    }

    pub fn add_page_to_pool(&mut self, page_id: u64, page: Rc<RefCell<BufferPage>>, engine: &mut StorageEngine) {
        if self.usage_queue.len() >= self.capacity {
            let _ = self.evict(engine);
        }

        self.pool.insert(page_id, page);
        self.usage_queue.push_back(page_id);
        
    }

    pub fn evict(&mut self, engine: &mut StorageEngine) -> Result<()>{
        while let Some(lru_page_id) = self.usage_queue.pop_front() {
            if let Some(page) = self.pool.remove(&lru_page_id) {
                // if page is dirty, save it to disk
                let buffer_page = Rc::clone(&page);
                let mut_page = buffer_page.borrow_mut();
                
                if mut_page.is_pinned == true {
                    self.pool.insert(lru_page_id, Rc::clone(&buffer_page));
                    continue;
                    
                }
                
                if mut_page.is_dirty == true {
                    let i_page: Page = deserialize(&buffer_page.data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    engine.write_page(lru_page_id, &i_page)?;
                }
            }
            return Ok(());
        }
        Err(io::Error::new(io::ErrorKind::Other, "No page could be evicted."))
    }

    pub fn pin_page(&mut self, page_id: u64) {
        if let Some(page) = self.pool.get_mut(&page_id) {
            page.borrow_mut().is_pinned = true;
        }
    }

    pub fn unpin_page(&mut self, page_id: u64) {
        if let Some(page) = self.pool.get_mut(&page_id) {
            page.borrow_mut().is_pinned = false;
        }
    }

}