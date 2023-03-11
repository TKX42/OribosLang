use crate::expression::{Data, DataExpression, evaluate, Expression};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct IfInstruction {
    comparison: Expression,
    true_statements: Vec<Box<dyn ExecutableInstruction>>,
    else_statements: Vec<Box<dyn ExecutableInstruction>>,
}

impl IfInstruction {
    pub fn new(comparison: Expression, true_statements: Vec<Box<dyn ExecutableInstruction>>, else_statements: Vec<Box<dyn ExecutableInstruction>>) -> Box<dyn ExecutableInstruction> {
        Box::new(IfInstruction {
            comparison,
            true_statements,
            else_statements,
        })
    }
}

impl ExecutableInstruction for IfInstruction {
    fn name(&self) -> String {
        String::from("assign")
    }

    // use new()!
    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter) -> Expression {
        let check = evaluate(&self.comparison, interpreter);
        match check {
            Data::String(_) => { panic!("Error: Invalid type 'string' given as expression for if instruction") }
            Data::Number(_) => { panic!("Error: Invalid type 'number' given as expression for if instruction") }
            Data::Bool(b) => {
                if b {
                    for instr in &self.true_statements {
                        instr.exec(interpreter);
                    }
                } else {
                    for instr in &self.else_statements {
                        instr.exec(interpreter);
                    }
                }
            }
        }

        Expression::DataExpression(DataExpression::empty())
    }
}