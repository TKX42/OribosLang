use std::process;

use crate::compiler::statement::{CompilerStatement, Scope};
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Debug)]
pub struct Compiler {
    ast: Vec<Box<dyn CompilerStatement>>,
}

impl Compiler {
    pub fn new(ast: Vec<Box<dyn CompilerStatement>>) -> Compiler {
        Compiler {
            ast,
        }
    }

    pub fn compile(&mut self) -> Vec<Box<dyn Instruction>> {
        compile_statements(&self.ast.clone())
    }
}


pub fn compile_statements(statements: &[Box<dyn CompilerStatement>]) -> Vec<Box<dyn Instruction>> {
    let mut result = vec![];
    for instr in statements {
        result.append(&mut compile_statement(instr));
    }
    result
}

pub fn compile_statement(statement: &Box<dyn CompilerStatement>) -> Vec<Box<dyn Instruction>> {
    statement.compile()
}