use crate::expression::{Data, evaluate, Expression};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct IfInstruction {
    comparison: Expression,
    true_statements: Vec<Box<dyn ExecutableInstruction>>,
    else_statements: Vec<Box<dyn ExecutableInstruction>>,
}

impl IfInstruction {
    pub fn create(comparison: Expression, true_statements: Vec<Box<dyn ExecutableInstruction>>, else_statements: Vec<Box<dyn ExecutableInstruction>>) -> Box<dyn ExecutableInstruction> {
        Box::new(IfInstruction {
            comparison,
            true_statements,
            else_statements,
        })
    }
}

impl ExecutableInstruction for IfInstruction {
    fn name(&self) -> String {
        String::from("if")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter) -> Data {
        let check = evaluate(&self.comparison, interpreter);
        match check {
            Data::String(_) => { panic!("Error: Invalid type 'string' given as expression for if instruction") }
            Data::Number(_) => { panic!("Error: Invalid type 'number' given as expression for if instruction") }
            Data::Bool(b) => {
                if b {
                    interpreter.run_statements(&self.true_statements);
                } else {
                    interpreter.run_statements(&self.else_statements);
                }
            }
        }

        Data::Number(0.0)
    }
}