use crate::compiler::expression::{get_jump_address, get_number};
use crate::data::Data;
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct JUMP {
    instr_pointer: isize,        // represents the addition to current instr_pointer -> negative means go up in the instructions
}

impl Instruction for JUMP {
    fn new(parameter: Data) -> Box<dyn Instruction> {
        Box::new(JUMP { instr_pointer: get_jump_address(&parameter) as isize })
    }

    fn exec(&self, _stack: &mut Vec<Data>, _memory: &mut Memory, instr_pointer: usize) -> usize {
        ((instr_pointer as isize) + self.instr_pointer) as usize
    }
}