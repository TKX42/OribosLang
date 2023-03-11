use crate::expression::{Data, DataExpression, evaluate, Expression, get_string};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct AssignmentInstruction {
    var_name: String,
    var_expression: Expression,
}

impl AssignmentInstruction {
    pub fn new(var_name: String, var_expression: Expression) -> Box<dyn ExecutableInstruction> {
        Box::new(AssignmentInstruction {
            var_name,
            var_expression,
        })
    }
}

impl ExecutableInstruction for AssignmentInstruction {
    fn name(&self) -> String {
        String::from("print")
    }

    // use new()!
    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter) -> Expression {
        println!("ASSIGNMENT");
        interpreter.add_var();
        Expression::DataExpression(DataExpression::empty())
    }
}