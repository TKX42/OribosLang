use crate::compiler::compile::Compiler;
use crate::compiler::expression::{Data, evaluate, Expression, get_number};
use crate::compiler::statement::{CompilerStatement, Scope};

#[derive(Clone, Debug)]
pub struct ForLoopStatement {
    counter_var_id: i64,
    start_i: Expression,
    end_i: Expression,
    statements: Vec<Box<dyn CompilerStatement>>,
}

impl ForLoopStatement {
    pub fn create(counter_var_id: i64, start_i: Expression, end_i: Expression, statements: Vec<Box<dyn CompilerStatement>>) -> Box<dyn CompilerStatement> {
        Box::new(ForLoopStatement {
            counter_var_id,
            start_i,
            end_i,
            statements,
        })
    }
}

impl CompilerStatement for ForLoopStatement {
    fn name(&self) -> String {
        String::from("for")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self, interpreter: &mut Compiler, scope: &mut Scope) -> Data {
        let start = get_number(&evaluate(&self.start_i, interpreter, scope)) as i64;
        let end = get_number(&evaluate(&self.end_i, interpreter, scope)) as i64;

        for i in start..end {
            interpreter.memory().assign(self.counter_var_id, Data::Number(i as f64));
            interpreter.compile_statements(&self.statements, scope);
        }

        scope._break = false;   // clear for possible outer loop
        Data::Number(0.0)
    }
}