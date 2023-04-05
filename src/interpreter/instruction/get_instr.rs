use crate::compiler::expression::{get_address};
use crate::data::Data;
use crate::interpreter::instruction::{ Instruction};
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct GET {
    address: i64
}

impl Instruction for GET {
    fn new(parameter: Data) -> Box<dyn Instruction> {
        Box::new(GET{ address: get_address(&parameter) as i64 })
    }

    fn exec(&self, stack: &mut Vec<Data>, memory: &mut Memory) {
        stack.push(memory.get(self.address).clone());
    }
}