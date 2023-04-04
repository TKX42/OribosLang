use crate::expression::{Data, evaluate, Expression, get_number};
use crate::instruction::ExecutableInstruction;
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

    fn exec(&self, interpreter: &mut Interpreter) -> Data {
        let exit_code = &evaluate(&self.data, interpreter);
        interpreter.exit_with_code(get_number(exit_code) as i64);
        Data::Number(0.0)
    }
}