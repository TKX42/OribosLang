use crate::compiler::expression::{compile, Expression};
use crate::compiler::statement::CompilerStatement;
use crate::data::Data;
use crate::interpreter::instruction::assign_instr::ASSIGN;
use crate::interpreter::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct AssignmentStatement {
    var_address: usize,
    var_expression: Expression,
}

impl AssignmentStatement {
    pub fn create(var_id: usize, var_expression: Expression) -> Box<dyn CompilerStatement> {
        Box::new(AssignmentStatement {
            var_address: var_id,
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

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        let mut result = vec![];
        result.append(&mut compile(&self.var_expression));
        result.push(ASSIGN::new(Data::MemoryAddress(self.var_address)));        // TODO refactor
        result
    }
}