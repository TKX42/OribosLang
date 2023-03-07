use chrono::Local;

use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

pub struct TimeInstruction {
    instruction: Instruction,
}

impl ExecutableInstruction for TimeInstruction {
    fn name(&self) -> &String {
        &self.instruction.name
    }

    fn init(parameters: &Vec<Box<dyn Expression>>) -> Box<dyn ExecutableInstruction> {
        Box::new(TimeInstruction {
            instruction: Instruction::new("time".to_string())
        })
    }

    fn exec(&self) -> Box<dyn Expression> {
        let date = Local::now();
        Box::new(DataExpression::new(Data::String(format!("{}", date.format("%Y-%m-%d/%H:%M:%S:%f")))))
    }
}