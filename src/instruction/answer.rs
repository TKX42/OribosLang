// JUST FOR DEBUGGING PURPOSE
use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct AnswerInstruction {}

impl ExecutableInstruction for AnswerInstruction {
    fn name(&self) -> String {
        String::from("answer")
    }

    fn init(_parameters: &[Expression]) -> Box<dyn ExecutableInstruction> {
        Box::new(AnswerInstruction {})
    }

    fn exec(&self, _interpreter: &mut Interpreter) -> Expression {
        Expression::Data(DataExpression::new(Data::Number(42.0)))
    }
}