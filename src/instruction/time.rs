use std::time::{SystemTime, UNIX_EPOCH};

use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct TimeInstruction {}

impl ExecutableInstruction for TimeInstruction {
    fn name(&self) -> String {
        String::from("time")
    }

    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(TimeInstruction {})
    }

    fn exec(&self, interpreter: &mut Interpreter) -> Expression {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        Expression::DataExpression(DataExpression::new(Data::String(since_the_epoch.as_millis().to_string())))
    }
}