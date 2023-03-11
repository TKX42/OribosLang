use crate::expression::{Data, DataExpression, evaluate, Expression};
use crate::instruction::ExecutableInstruction;

#[derive(Clone, Debug)]
pub struct PrintInstruction {
    data: Expression,
}

impl ExecutableInstruction for PrintInstruction {
    fn name(&self) -> String {
        String::from("print")
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(PrintInstruction {
            data: parameters.get(0).expect("Invalid parameter for Print").clone(),
        })
    }

    fn exec(&self) -> Expression {
        print(&evaluate(&self.data));
        Expression::DataExpression(DataExpression::empty())
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}