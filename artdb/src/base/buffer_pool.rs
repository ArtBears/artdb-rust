use std::collections::{HashMap, VecDeque};
use std::io::{self, Result, Seek, SeekFrom, Write};
use std::rc::Rc;
use std::cell::RefCell;

use bincode::{deserialize, serialize};

use super::page::{self, Page, BufferPage};
use super::storage_engine::StorageEngine;

pub struct BufferPool {
    pool: HashMap<u64, Rc<RefCell<BufferPage>>>,
    usage_queue: VecDeque<u64>,
    capacity: usize,
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
        let page = match engine.read_page(page_id) {
            Ok(page) => page,
            Err(_) => {
                let empty_page = Page::new();
                let _ = engine.write_page(page_id, &empty_page);
                empty_page
            }
        };

        let buffer_page = Rc::new(RefCell::new(BufferPage::new(page)));

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

    pub fn mark_page_as_dirty(&mut self, page_id: u64) {
        if let Some(page) = self.pool.get(&page_id) {
            page.borrow_mut().is_dirty = true;
        }
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
                    let i_page: &Page = &mut_page.page;
                    engine.write_page(lru_page_id, i_page)?;
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

    pub fn write_page_to_disk(&mut self, page_id: u64, engine: &mut StorageEngine) -> bool {
        if let Some(buffer_page) = self.pool.get(&page_id) {
            let page = &buffer_page.borrow_mut().page;
            let _ = engine.write_page(page_id, page);

            return true;
        }
        false
    }

}