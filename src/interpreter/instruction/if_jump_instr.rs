use crate::compiler::expression::{get_bool, get_jump_address};
use crate::data::Data;
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct IFJUMP {
    instr_pointer: isize,        // represents the addition to current instr_pointer -> negative means go up in the instructions
}

impl Instruction for IFJUMP {
    fn new(parameter: Data) -> Box<dyn Instruction> {
        Box::new(IFJUMP { instr_pointer: get_jump_address(&parameter) })
    }

    fn exec(&self, stack: &mut Vec<Data>, _memory: &mut Memory, instr_pointer: usize) -> usize {
        let cond = get_bool(&stack.pop().unwrap());
        if cond {
            ((instr_pointer as isize) + self.instr_pointer) as usize
        } else {
            instr_pointer + 1
        }
    }
}