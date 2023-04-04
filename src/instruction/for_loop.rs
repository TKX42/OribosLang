use crate::expression::{Data, evaluate, Expression, get_number};
use crate::instruction::{ExecutableInstruction, Scope};
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct ForLoopInstruction {
    counter_var_id: i64,
    start_i: Expression,
    end_i: Expression,
    statements: Vec<Box<dyn ExecutableInstruction>>,
}

impl ForLoopInstruction {
    pub fn create(counter_var_id: i64, start_i: Expression, end_i: Expression, statements: Vec<Box<dyn ExecutableInstruction>>) -> Box<dyn ExecutableInstruction> {
        Box::new(ForLoopInstruction {
            counter_var_id,
            start_i,
            end_i,
            statements,
        })
    }
}

impl ExecutableInstruction for ForLoopInstruction {
    fn name(&self) -> String {
        String::from("for")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let start = get_number(&evaluate(&self.start_i, interpreter, scope)) as i64;
        let end = get_number(&evaluate(&self.end_i, interpreter, scope)) as i64;

        for i in start..end {
            interpreter.memory().assign(self.counter_var_id, Data::Number(i as f64));

            for statement in &self.statements {
                if scope._break { break; }
                interpreter.run_statement(statement, scope);
            }
        }

        scope._break = false;   // clear for possible outer loop
        Data::Number(0.0)
    }
}