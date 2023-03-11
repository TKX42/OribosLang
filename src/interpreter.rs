use crate::instruction::ExecutableInstruction;
use crate::Memory::Memory;

#[derive(Debug)]
pub struct Interpreter {
    ast: Vec<Box<dyn ExecutableInstruction>>,
    memory: Memory,
}

impl Interpreter {
    pub fn new(ast: Vec<Box<dyn ExecutableInstruction>>) -> Interpreter {
        Interpreter {
            ast,
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        for instruction in &self.ast.clone() {
            instruction.exec(self);
        }
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }
}