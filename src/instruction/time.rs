use chrono::Local;

use crate::expression::{Data, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

pub struct TimeInstruction {
    instruction: Instruction,
}

impl ExecutableInstruction for TimeInstruction {
    fn name(&self) -> &String {
        &self.instruction.name
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(TimeInstruction {
            instruction: Instruction::new("time".to_string())
        })
    }

    fn exec(&self) {
        let date = Local::now();
        println!("{}", date.format("%Y-%m-%d/%H:%M:%S:%f"));
    }
}