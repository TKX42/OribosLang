use std::process;
use crate::data::Data;
use crate::interpreter::instruction::{Instruction};
use crate::memory::Memory;

#[derive(Debug)]
pub struct Interpreter {
    instructions: Vec<Box<dyn Instruction>>,
    memory: Memory,
    stack: Vec<Data>
}

impl Interpreter {
    pub fn new(ast: Vec<Box<dyn Instruction>>) -> Interpreter {
        Interpreter {
            instructions: ast,
            memory: Memory::new(),
            stack: vec![]
        }
    }

    pub fn run(&mut self) {
        self.run_instructions(&self.instructions.clone());
    }

    pub fn run_instructions(&mut self, statements: &[Box<dyn Instruction>]) {
        for instr in statements {
            self.run_instruction(instr);
        }
    }

    pub fn run_instruction(&mut self, statement: &Box<dyn Instruction>) {
        statement.exec(&mut self.stack, &mut self.memory);
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }
}