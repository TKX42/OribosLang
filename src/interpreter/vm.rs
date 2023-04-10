use crate::data::Data;
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Debug)]
pub struct VM {
    instructions: Vec<Box<dyn Instruction>>,
    memory: Memory,
    stack: Vec<Data>,
    instr_pointer: usize,
    exit_code: i64,
}

impl VM {
    pub fn new(ast: Vec<Box<dyn Instruction>>) -> VM {
        VM {
            instructions: ast,
            memory: Memory::new(),
            stack: vec![],
            instr_pointer: 0,
            exit_code: 0,
        }
    }

    pub fn run(&mut self) {
        println!("{self:#?}");
        self.run_instructions(&self.instructions.clone());
        println!("Process ended with exit code {}", self.exit_code)
    }

    pub fn run_instructions(&mut self, instructions: &[Box<dyn Instruction>]) {
        while self.instr_pointer < instructions.len() {
            let instr = &instructions[self.instr_pointer];
            self.instr_pointer = self.run_instruction(instr);
        }
    }

    pub fn run_instruction(&mut self, instruction: &Box<dyn Instruction>) -> usize {
        instruction.exec(&mut self.stack, &mut self.memory, self.instr_pointer)
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }
}