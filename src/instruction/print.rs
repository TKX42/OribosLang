use crate::expression::{Data, evaluate, Expression};
use crate::instruction::{ExecutableInstruction, Scope};
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct PrintInstruction {
    data: Expression,
}

impl ExecutableInstruction for PrintInstruction {
    fn name(&self) -> String {
        String::from("print")
    }

    fn init(parameters: &[Expression]) -> Box<dyn ExecutableInstruction> {
        Box::new(PrintInstruction {
            data: parameters.get(0).expect("Invalid parameter for Print").clone(),
        })
    }

    fn exec(&self, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        print(&evaluate(&self.data, interpreter, scope));
        Data::Number(0.0)
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{s}") }
        Data::Number(n) => { println!("{n}") }
        Data::Bool(b) => { println!("{b}") }
    }
}