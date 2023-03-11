use crate::expression::Data;

#[derive(Debug)]
pub struct Memory {
    mem: Vec<Data>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![]
        }
    }

    pub fn assign(&mut self, id: i64, val: Data) {
        if self.mem.len() < id as usize {
            self.mem.push(val);
        } else {
            self.mem[(id - 1) as usize] = val;
        }
    }

    pub fn get(&self, id: i64) -> &Data {
        self.mem.get((id - 1) as usize).unwrap()
    }
}