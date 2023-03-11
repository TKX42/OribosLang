use std::time::{SystemTime, UNIX_EPOCH};

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
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        Expression::DataExpression(DataExpression::new(Data::String(since_the_epoch.as_millis().to_string())))
    }
}