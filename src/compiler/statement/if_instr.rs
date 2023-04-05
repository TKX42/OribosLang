use crate::compiler::compile::Compiler;
use crate::compiler::expression::{Data, evaluate, Expression};
use crate::compiler::statement::{CompilerStatement, Scope};

#[derive(Clone, Debug)]
pub struct IfStatement {
    comparison: Expression,
    true_statements: Vec<Box<dyn CompilerStatement>>,
    else_statements: Vec<Box<dyn CompilerStatement>>,
}

impl IfStatement {
    pub fn create(comparison: Expression, true_statements: Vec<Box<dyn CompilerStatement>>, else_statements: Vec<Box<dyn CompilerStatement>>) -> Box<dyn CompilerStatement> {
        Box::new(IfStatement {
            comparison,
            true_statements,
            else_statements,
        })
    }
}

impl CompilerStatement for IfStatement {
    fn name(&self) -> String {
        String::from("if")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self, interpreter: &mut Compiler, scope: &mut Scope) -> Data {
        let check = evaluate(&self.comparison, interpreter, scope);
        match check {
            Data::String(_) => { panic!("Error: Invalid type 'string' given as expression for if statement") }
            Data::Number(_) => { panic!("Error: Invalid type 'number' given as expression for if statement") }
            Data::Bool(b) => {
                if b {
                    interpreter.compile_statements(&self.true_statements, scope);
                } else {
                    interpreter.compile_statements(&self.else_statements, scope);
                }
            }
        }

        Data::Number(0.0)
    }
}