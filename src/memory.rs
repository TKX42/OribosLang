use crate::data::Data;

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

    pub fn assign(&mut self, address: i64, val: Data) {
        if self.mem.len() < address as usize {
            self.mem.push(val);
        } else {
            self.mem[(address - 1) as usize] = val;
        }
    }

    pub fn get(&self, address: i64) -> &Data {
        match self.mem.get((address - 1) as usize) {
            None => {
                memory_error(address);
                unreachable!()
            }
            Some(x) => { x }
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

fn memory_error(id: i64) {
    panic!("Error: Unknown variable {id}");
}