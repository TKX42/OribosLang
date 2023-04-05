use crate::compiler::compile::Compiler;
use crate::compiler::expression::{Data, Expression};
use crate::compiler::statement::{CompilerStatement, Scope};

#[derive(Clone, Debug)]
pub struct GetStatement {
    var_id: i64,
}

impl GetStatement {
    pub fn create(var_id: i64) -> Box<dyn CompilerStatement> {
        Box::new(GetStatement {
            var_id,
        })
    }
}

impl CompilerStatement for GetStatement {
    fn name(&self) -> String {
        String::from("get")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self, interpreter: &mut Compiler, _scope: &mut Scope) -> Data {
        interpreter.memory().get(self.var_id).clone()
    }
}