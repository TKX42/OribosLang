use crate::expression::{Data, Expression};
use crate::instruction::{ExecutableInstruction, Scope};
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct BreakInstruction {
}

impl ExecutableInstruction for BreakInstruction {
    fn name(&self) -> String {
        String::from("break")
    }

    fn init(_parameters: &[Expression]) -> Box<dyn ExecutableInstruction> {
        Box::new(BreakInstruction {})
    }

    fn exec(&self, _interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        scope._break = true;
        Data::Number(0.0)
    }
}