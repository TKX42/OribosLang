use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use nohash_hasher::NoHashHasher;

use crate::expression::Data;

#[derive(Debug)]
pub struct Memory {
    mem: HashMap<i64, Data, BuildHasherDefault<NoHashHasher<i64>>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: HashMap::with_hasher(BuildHasherDefault::default())
        }
    }

    pub fn assign(&mut self, id: i64, val: Data) {
        self.mem.insert(id, val);
    }

    pub fn get(&self, id: i64) -> &Data {
        self.mem.get(&id).unwrap()
    }
}