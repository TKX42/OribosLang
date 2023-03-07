use crate::expression::{Data, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

pub struct PrintInstruction {
    instruction: Instruction,
    data: Data,
}

impl ExecutableInstruction for PrintInstruction {
    fn name(&self) -> &String {
        return &self.instruction.name;
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(PrintInstruction {
            instruction: Instruction::new("print".to_string()),
            data: parameters.get(0).expect("Invalid parameter for Print").evaluate().clone(),
        })
    }

    fn exec(&self) {
        print(&self.data)
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}