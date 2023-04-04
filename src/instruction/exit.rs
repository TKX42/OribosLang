use crate::expression::{Data, evaluate, Expression, get_number};
use crate::instruction::{ExecutableInstruction, Scope};
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct ExitInstruction {
    data: Expression,
}

impl ExecutableInstruction for ExitInstruction {
    fn name(&self) -> String {
        String::from("print")
    }

    fn init(parameters: &[Expression]) -> Box<dyn ExecutableInstruction> {
        Box::new(ExitInstruction {
            data: parameters.get(0).expect("Invalid parameter for Exit").clone(),
        })
    }

    fn exec(&self, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let exit_code = &evaluate(&self.data, interpreter, scope);
        scope._exit = true;
        scope._exit_code = get_number(exit_code) as i32;
        Data::Number(0.0)
    }
}