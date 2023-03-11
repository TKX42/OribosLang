use crate::expression::{Data, DataExpression, evaluate, Expression};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

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

    fn exec(&self, interpreter: &mut Interpreter) -> Expression {
        print(&evaluate(&self.data, interpreter));
        Expression::DataExpression(DataExpression::empty())
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
        Data::Bool(b) => { println!("{}", b) }
    }
}