use crate::compiler::statement::{CompilerStatement, Scope};
use crate::compiler::expression::{Data, evaluate, Expression};
use crate::compiler::compile::Compiler;

#[derive(Clone, Debug)]
pub struct PrintStatement {
    data: Expression,
}

impl CompilerStatement for PrintStatement {
    fn name(&self) -> String {
        String::from("print")
    }

    fn init(parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        Box::new(PrintStatement {
            data: parameters.get(0).expect("Invalid parameter for Print").clone(),
        })
    }

    fn compile(&self, interpreter: &mut Compiler, scope: &mut Scope) -> Data {
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