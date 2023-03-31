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
        match self.mem.get((id - 1) as usize) {
            None => {
                memory_error(id);
                unreachable!()
            }
            Some(x) => {x}
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

fn memory_error(id: i64) {
    panic!("Error: Unknown variable {}", id);
}