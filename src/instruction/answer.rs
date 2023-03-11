// JUST FOR DEBUGGING PURPOSE
use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::ExecutableInstruction;

#[derive(Clone, Debug)]
pub struct AnswerInstruction {}

impl ExecutableInstruction for AnswerInstruction {
    fn name(&self) -> String {
        return String::from("answer");
    }

    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(AnswerInstruction {})
    }

    fn exec(&self) -> Expression {
        Expression::DataExpression(DataExpression::new(Data::Number(42.0)))
    }
}