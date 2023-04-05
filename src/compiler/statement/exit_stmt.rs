use crate::compiler::compile::Compiler;
use crate::compiler::expression::{compile, Expression, get_number};
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::interpreter::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct ExitStatement {
    data: Expression,
}

impl CompilerStatement for ExitStatement {
    fn name(&self) -> String {
        String::from("exit")
    }

    fn init(parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        Box::new(ExitStatement {
            data: parameters.get(0).expect("Invalid parameter for Exit").clone(),
        })
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        unimplemented!()
    }
}