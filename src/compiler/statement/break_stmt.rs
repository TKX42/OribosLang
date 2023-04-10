use crate::compiler::expression::Expression;
use crate::compiler::statement::CompilerStatement;
use crate::interpreter::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct BreakStatement {}

impl CompilerStatement for BreakStatement {
    fn name(&self) -> String {
        String::from("break")
    }

    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        Box::new(BreakStatement {})
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        unimplemented!()
    }
}