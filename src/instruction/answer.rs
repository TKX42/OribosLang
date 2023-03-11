// JUST FOR DEBUGGING PURPOSE
use crate::expression::{Data, DataExpression, evaluate, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

#[derive(Clone)]
pub struct AnswerInstruction {
    instruction: Instruction,
}

impl ExecutableInstruction for AnswerInstruction {
    fn name(&self) -> &String {
        return &self.instruction.name;
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(AnswerInstruction {
            instruction: Instruction::new("answer".to_string()),
        })
    }

    fn exec(&self) -> Expression {
        Expression::DataExpression(DataExpression::new(Data::Number(42.0)))
    }
}