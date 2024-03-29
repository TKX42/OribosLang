use crate::expression::{Data, evaluate, Expression};
use crate::instruction::{ExecutableInstruction, Scope};
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct AssignmentInstruction {
    var_id: i64,
    var_expression: Expression,
}

impl AssignmentInstruction {
    pub fn create(var_id: i64, var_expression: Expression) -> Box<dyn ExecutableInstruction> {
        Box::new(AssignmentInstruction {
            var_id,
            var_expression,
        })
    }
}

impl ExecutableInstruction for AssignmentInstruction {
    fn name(&self) -> String {
        String::from("assign")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let expr = evaluate(&self.var_expression, interpreter, scope);
        interpreter.memory().assign(self.var_id, expr);
        //self.var_expression.clone()     // TODO evaluate performance
        Data::Number(0.0)
    }
}