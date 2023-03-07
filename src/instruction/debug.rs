use crate::expression::{Data, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

pub struct DebugInstruction {
    instruction: Instruction,
    data: Data,
}

impl ExecutableInstruction for DebugInstruction {
    fn name(&self) -> &String {
        return &self.instruction.name;
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(DebugInstruction {
            instruction: Instruction::new("debug".to_string()),
            data: parameters.get(0).expect("Invalid parameter for Debug").evaluate().clone(),
        })
    }

    fn exec(&self) {
        debug(&self.data)
    }
}

fn debug(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}