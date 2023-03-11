use crate::expression::{Data, DataExpression, evaluate, Expression};
use crate::instruction::{ExecutableInstruction, Instruction};

#[derive(Clone)]
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
            data: evaluate(parameters.get(0).expect("Invalid parameter for Print")),
        })
    }

    fn exec(&self) -> Expression {
        print(&self.data);
        Expression::DataExpression(DataExpression::empty())
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}