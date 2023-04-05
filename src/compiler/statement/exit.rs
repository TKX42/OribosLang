use crate::compiler::statement::{CompilerStatement, Scope};
use crate::compiler::expression::{Data, evaluate, Expression, get_number};
use crate::compiler::compile::Compiler;

#[derive(Clone, Debug)]
pub struct ExitStatement {
    data: Expression,
}

impl CompilerStatement for ExitStatement {
    fn name(&self) -> String {
        String::from("exit")
    }

    fn init(parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        Box::new(ExitStatement {
            data: parameters.get(0).expect("Invalid parameter for Exit").clone(),
        })
    }

    fn compile(&self, interpreter: &mut Compiler, scope: &mut Scope) -> Data {
        let exit_code = &evaluate(&self.data, interpreter, scope);
        scope._exit = true;
        scope._exit_code = get_number(exit_code) as i32;
        Data::Number(0.0)
    }
}