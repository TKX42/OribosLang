use std::collections::HashMap;

use crate::expression::Data;

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

    pub fn assign(&mut self, id: i64, val: Data) {
        self.mem.insert(id, val);
    }

    pub fn get(&self, id: i64) -> &Data {
        self.mem.get(&id).expect(format!("Memory Error: Could not find variable {}", id).as_str())
    }
}