use crate::compiler::compile::Compiler;
use crate::compiler::expression::{compile, Expression, get_number};
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::interpreter::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct ForLoopStatement {
    counter_var_id: i64,
    start_i: Expression,
    end_i: Expression,
    statements: Vec<Box<dyn CompilerStatement>>,
}

impl ForLoopStatement {
    pub fn create(counter_var_id: i64, start_i: Expression, end_i: Expression, statements: Vec<Box<dyn CompilerStatement>>) -> Box<dyn CompilerStatement> {
        Box::new(ForLoopStatement {
            counter_var_id,
            start_i,
            end_i,
            statements,
        })
    }
}

impl CompilerStatement for ForLoopStatement {
    fn name(&self) -> String {
        String::from("for")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        unimplemented!()
    }
}