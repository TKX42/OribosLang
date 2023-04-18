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

    pub fn assign(&mut self, address: usize, val: Data) {
        if self.mem.len() < address as usize {
            self.mem.push(val);
        } else {
            self.mem[(address - 1) as usize] = val;
        }
    }

    pub fn get(&self, address: usize) -> &Data {
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

fn memory_error(id: usize) {
    panic!("Error: Unknown variable {id}");
}