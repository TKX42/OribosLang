use crate::compiler::compile::Compiler;
use crate::compiler::expression::{Expression};
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::data::Data;
use crate::interpreter::instruction::get_instr::GET;
use crate::interpreter::instruction::Instruction;

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

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        vec![
            GET::new(Data::Address(self.var_id))
        ]
    }
}