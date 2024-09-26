use serde::{Serialize, Deserialize}

use super::{buffer_pool::{self, BufferPool}, storage_engine::StorageEngine};

pub enum BPlusTreeNode {
    Internal(InternalNode),
    Leaf(LeafNode),
}

pub struct InternalNode {
    pub keys: Vec<u32>, // Keys for navigation
    pub children: Vec<u64>, // Child pointers (page_id)
}

pub struct LeafNode {
    pub keys: Vec<u32>,     // Keys stored in the leaf node
    pub values: Vec<u64>,   // Values or page_ids pointing to actual data
}

pub struct BPlusTree {
    root_page_id: u64,      // Page ID of the root node
    order: usize,           // Max number of keys per node
    pool: BufferPool,       // Reference to the Buffer Pool for page management
    engine: StorageEngine,  // Storage engine to handle disk I/O
}

impl BPlusTree {
    pub fn new(order: usize, engine: StorageEngine) -> BPlusTree {
        let buffer_pool = BufferPool::new(100);
        BPlusTree {
            root_page_id: 0,
            order,
            pool: buffer_pool,
            engine
        }
    }

    // Method to search for key
    pub fn search(&mut self, key: u32) -> Option<u64> {
        let mut page_id = self.root_page_id;

    }
}