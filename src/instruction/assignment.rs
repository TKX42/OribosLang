use crate::expression::{evaluate, Expression};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct AssignmentInstruction {
    var_id: i64,
    var_expression: Expression,
}

impl AssignmentInstruction {
    pub fn new(var_id: i64, var_expression: Expression) -> Box<dyn ExecutableInstruction> {
        Box::new(AssignmentInstruction {
            var_id: var_id,
            var_expression,
        })
    }
}

impl ExecutableInstruction for AssignmentInstruction {
    fn name(&self) -> String {
        String::from("assign")
    }

    // use new()!
    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter) -> Expression {
        let expr = evaluate(&self.var_expression, interpreter);
        interpreter.memory().assign(self.var_id, expr);
        self.var_expression.clone()     // TODO evaluate performance
    }
}