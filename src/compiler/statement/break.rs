use crate::compiler::statement::{CompilerStatement, Scope};
use crate::compiler::expression::{Data, Expression};
use crate::compiler::compile::Compiler;

#[derive(Clone, Debug)]
pub struct BreakStatement {
}

impl CompilerStatement for BreakStatement {
    fn name(&self) -> String {
        String::from("break")
    }

    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        Box::new(BreakStatement {})
    }

    fn compile(&self, _interpreter: &mut Compiler, scope: &mut Scope) -> Data {
        scope._break = true;
        Data::Number(0.0)
    }
}