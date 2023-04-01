use crate::instruction::ExecutableInstruction;
use crate::memory::Memory;

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
        self.run_statements(&self.ast.clone());
    }

    pub fn run_statements(&mut self, statements: &[Box<dyn ExecutableInstruction>]) {
        statements.iter().for_each(|instr|{instr.exec(self);});
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }
}