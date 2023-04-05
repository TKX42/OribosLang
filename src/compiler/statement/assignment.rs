use crate::compiler::statement::{CompilerStatement, Scope};
use crate::compiler::expression::{Data, evaluate, Expression};
use crate::compiler::compile::Compiler;

#[derive(Clone, Debug)]
pub struct AssignmentStatement {
    var_id: i64,
    var_expression: Expression,
}

impl AssignmentStatement {
    pub fn create(var_id: i64, var_expression: Expression) -> Box<dyn CompilerStatement> {
        Box::new(AssignmentStatement {
            var_id,
            var_expression,
        })
    }
}

impl CompilerStatement for AssignmentStatement {
    fn name(&self) -> String {
        String::from("assign")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self, interpreter: &mut Compiler, scope: &mut Scope) -> Data {
        let expr = evaluate(&self.var_expression, interpreter, scope);
        interpreter.memory().assign(self.var_id, expr);
        //self.var_expression.clone()     // TODO evaluate performance
        Data::Number(0.0)
    }
}