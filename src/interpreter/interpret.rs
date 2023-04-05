use std::process;

use crate::data::Data;
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Debug)]
pub struct Interpreter {
    instructions: Vec<Box<dyn Instruction>>,
    memory: Memory,
    stack: Vec<Data>,
    instr_pointer: usize,
    exit_code: i64,
}

impl Interpreter {
    pub fn new(ast: Vec<Box<dyn Instruction>>) -> Interpreter {
        Interpreter {
            instructions: ast,
            memory: Memory::new(),
            stack: vec![],
            instr_pointer: 0,
            exit_code: 0,
        }
    }

    pub fn run(&mut self) {
        println!("{:#?}", self);
        self.run_instructions(&self.instructions.clone());
        println!("Process ended with exit code {}", self.exit_code)
    }

    pub fn run_instructions(&mut self, statements: &[Box<dyn Instruction>]) {
        while self.instr_pointer < statements.len() {
            let instr = &statements[self.instr_pointer];
            self.instr_pointer = self.run_instruction(&instr);
        }
    }

    pub fn run_instruction(&mut self, statement: &Box<dyn Instruction>) -> usize {
        statement.exec(&mut self.stack, &mut self.memory, self.instr_pointer)
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }
}