use crate::data::Data;
use crate::interpreter::instruction::{ Instruction};
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct CONST {
    data: Data
}

impl Instruction for CONST {
    fn new(parameter: Data) -> Box<dyn Instruction> {
        Box::new(CONST{ data: parameter})
    }

    fn exec(&self, stack: &mut Vec<Data>, memory: &mut Memory) {
        stack.push(self.data.clone());
    }
}