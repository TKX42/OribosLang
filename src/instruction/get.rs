use crate::expression::{DataExpression, Expression};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct GetInstruction {
    var_id: i64,
}

impl GetInstruction {
    pub fn new(var_id: i64) -> Box<dyn ExecutableInstruction> {
        Box::new(GetInstruction {
            var_id: var_id,
        })
    }
}

impl ExecutableInstruction for GetInstruction {
    fn name(&self) -> String {
        String::from("get")
    }

    // use new()!
    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter) -> Expression {
        Expression::DataExpression(DataExpression::new(interpreter.memory().get(self.var_id).clone()))      // TODO: evaluate clone for runtime performance
    }
}