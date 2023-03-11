use chrono::Local;

use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

#[derive(Clone)]
pub struct TimeInstruction {
    instruction: Instruction,
}

impl ExecutableInstruction for TimeInstruction {
    fn name(&self) -> &String {
        &self.instruction.name
    }

    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(TimeInstruction {
            instruction: Instruction::new("time".to_string())
        })
    }

    fn exec(&self) -> Expression {
        let date = Local::now();
        Expression::DataExpression(DataExpression::new(Data::String(format!("{}", date.format("%Y-%m-%d/%H:%M:%S:%f")))))
    }
}