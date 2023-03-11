use std::collections::HashMap;
use std::iter::Map;

use crate::expression::{Data, Expression};

#[derive(Debug)]
pub struct Memory {
    mem: HashMap<i64, Data>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: HashMap::new()
        }
    }

    pub fn add(&mut self, id: i64, val: Data) {
        self.mem.insert(id, val);
    }
}