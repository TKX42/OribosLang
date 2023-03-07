use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

pub struct DebugInstruction {
    instruction: Instruction,
    data: Data,
}

impl ExecutableInstruction for DebugInstruction {
    fn name(&self) -> &String {
        return &self.instruction.name;
    }

    fn init(parameters: &Vec<Box<dyn Expression>>) -> Box<dyn ExecutableInstruction> {
        Box::new(DebugInstruction {
            instruction: Instruction::new("debug".to_string()),
            data: parameters.get(0).expect("Invalid parameter for Debug").evaluate().clone(),
        })
    }

    fn exec(&self) -> Box<dyn Expression> {
        debug(&self.data);
        Box::new(DataExpression::empty())
    }
}

fn debug(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}