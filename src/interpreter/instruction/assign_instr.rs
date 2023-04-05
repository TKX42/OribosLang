use crate::compiler::expression::get_memory_address;
use crate::data::Data;
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct ASSIGN {
    address: i64,
}

impl Instruction for ASSIGN {
    fn new(parameter: Data) -> Box<dyn Instruction> {
        Box::new(ASSIGN { address: get_memory_address(&parameter) as i64 })
    }

    fn exec(&self, stack: &mut Vec<Data>, memory: &mut Memory, instr_pointer: usize) -> usize {
        memory.assign(self.address, stack.pop().unwrap());
        instr_pointer + 1
    }
}