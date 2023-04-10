use crate::compiler::expression::Expression;
use crate::compiler::statement::CompilerStatement;
use crate::interpreter::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct ExitStatement {
    exit_code: Expression,
}

impl CompilerStatement for ExitStatement {
    fn name(&self) -> String {
        String::from("exit")
    }

    fn init(parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        Box::new(ExitStatement {
            exit_code: parameters.get(0).expect("Invalid parameter for Exit").clone(),
        })
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        unimplemented!()
    }
}