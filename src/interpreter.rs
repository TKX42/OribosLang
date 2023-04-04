use std::process;
use crate::instruction::{ExecutableInstruction, Scope};
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
        self.run_statements(&self.ast.clone(), &mut Scope::new());
    }

    pub fn run_statements(&mut self, statements: &[Box<dyn ExecutableInstruction>], scope: &mut Scope) {
        for instr in statements {
            self.run_statement(instr, scope);
        }
    }

    pub fn run_statement(&mut self, statement: &Box<dyn ExecutableInstruction>, scope: &mut Scope) {
        if scope._exit { self.exit(scope._exit_code); }
        statement.exec(self, scope);
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    fn exit(&self, exit_code: i32) {
        println!("Process ended with exit code {0}", exit_code);
        process::exit(0);
    }
}