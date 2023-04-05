use std::process;

use crate::compiler::statement::{CompilerStatement, Scope};
use crate::memory::Memory;

#[derive(Debug)]
pub struct Compiler {
    ast: Vec<Box<dyn CompilerStatement>>,
    memory: Memory,
}

impl Compiler {
    pub fn new(ast: Vec<Box<dyn CompilerStatement>>) -> Compiler {
        Compiler {
            ast,
            memory: Memory::new(),
        }
    }

    pub fn compile(&mut self) {
        self.compile_statements(&self.ast.clone(), &mut Scope::new());
    }

    pub fn compile_statements(&mut self, statements: &[Box<dyn CompilerStatement>], scope: &mut Scope) {
        for instr in statements {
            if scope._break { break; }
            self.compile_statement(instr, scope);
        }
    }

    pub fn compile_statement(&mut self, statement: &Box<dyn CompilerStatement>, scope: &mut Scope) {
        if scope._exit { self.exit(scope._exit_code); }
        statement.compile(self, scope);
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    fn exit(&self, exit_code: i32) {
        println!("Process ended with exit code {0}", exit_code);
        process::exit(0);
    }
}