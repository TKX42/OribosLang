use crate::compiler::expression::get_memory_address;
use crate::data::Data;
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct LOAD {
    address: i64,
}

impl Instruction for LOAD {
    fn new(parameter: Data) -> Box<dyn Instruction> {
        Box::new(LOAD { address: get_memory_address(&parameter) as i64 })
    }

    fn exec(&self, stack: &mut Vec<Data>, memory: &mut Memory, instr_pointer: usize) -> usize {
        stack.push(memory.get(self.address).clone());
        instr_pointer + 1
    }
}