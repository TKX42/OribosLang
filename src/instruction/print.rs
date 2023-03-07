use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

pub struct PrintInstruction {
    instruction: Instruction,
    data: Data,
}

impl ExecutableInstruction for PrintInstruction {
    fn name(&self) -> &String {
        return &self.instruction.name;
    }

    fn init(parameters: &Vec<Box<dyn Expression>>) -> Box<dyn ExecutableInstruction> {
        Box::new(PrintInstruction {
            instruction: Instruction::new("print".to_string()),
            data: parameters.get(0).expect("Invalid parameter for Print").evaluate().clone(),
        })
    }

    fn exec(&self) -> Box<dyn Expression> {
        print(&self.data);
        Box::new(DataExpression::empty())
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}