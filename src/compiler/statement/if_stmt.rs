use crate::compiler::compile::Compiler;
use crate::compiler::expression::{compile, Expression};
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::interpreter::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct IfStatement {
    comparison: Expression,
    true_statements: Vec<Box<dyn CompilerStatement>>,
    else_statements: Vec<Box<dyn CompilerStatement>>,
}

impl IfStatement {
    pub fn create(comparison: Expression, true_statements: Vec<Box<dyn CompilerStatement>>, else_statements: Vec<Box<dyn CompilerStatement>>) -> Box<dyn CompilerStatement> {
        Box::new(IfStatement {
            comparison,
            true_statements,
            else_statements,
        })
    }
}

impl CompilerStatement for IfStatement {
    fn name(&self) -> String {
        String::from("if")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        unimplemented!()
    }
}