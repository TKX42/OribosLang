use crate::expression::{Data, DataExpression, evaluate, Expression, get_number};
use crate::instruction::ExecutableInstruction;
use crate::interpreter::Interpreter;

#[derive(Clone, Debug)]
pub struct ForLoopInstruction {
    counter_var_id: i64,
    start_i: Expression,
    end_i: Expression,
    statements: Vec<Box<dyn ExecutableInstruction>>,
}

impl ForLoopInstruction {
    pub fn new(counter_var_id: i64, start_i: Expression, end_i: Expression, statements: Vec<Box<dyn ExecutableInstruction>>) -> Box<dyn ExecutableInstruction> {
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

    // use new()!
    fn init(_parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        unreachable!()
    }

    fn exec(&self, interpreter: &mut Interpreter) -> Expression {
        let start = get_number(evaluate(&self.start_i, interpreter)) as i64;
        let end = get_number(evaluate(&self.end_i, interpreter)) as i64;

        for i in start..end {
            interpreter.memory().assign(self.counter_var_id, Data::Number(i as f64));
            interpreter.run_statements(&self.statements);
        }

        Expression::DataExpression(DataExpression::empty())
    }
}