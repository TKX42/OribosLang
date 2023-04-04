use std::process;
use crate::instruction::ExecutableInstruction;
use crate::memory::Memory;

#[derive(Debug)]
pub struct Interpreter {
    ast: Vec<Box<dyn ExecutableInstruction>>,
    memory: Memory,
    exit: bool,
    exit_code: i64
}

impl Interpreter {
    pub fn new(ast: Vec<Box<dyn ExecutableInstruction>>) -> Interpreter {
        Interpreter {
            ast,
            memory: Memory::new(),
            exit: false,
            exit_code: 0
        }
    }

    pub fn run(&mut self) {
        self.run_statements(&self.ast.clone());
    }

    pub fn run_statements(&mut self, statements: &[Box<dyn ExecutableInstruction>]) {
        for instr in statements {
            if self.exit { self.exit(); }
            instr.exec(self);
        }
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn exit_with_code(&mut self, exit_code: i64) {
        self.exit = true;
        self.exit_code = exit_code;
    }

    fn exit(&self) {
        println!("Process ended with exit code {0}", self.exit_code);
        process::exit(0);
    }
}