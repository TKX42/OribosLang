use crate::compiler::compile::Compiler;
use crate::compiler::expression::{compile, Expression};
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::data::Data;
use crate::interpreter::instruction::assign_instr::ASSIGN;
use crate::interpreter::instruction::const_instr::CONST;
use crate::interpreter::instruction::Instruction;

#[derive(Clone, Debug)]
pub struct AssignmentStatement {
    var_address: i64,
    var_expression: Expression,
}

impl AssignmentStatement {
    pub fn create(var_id: i64, var_expression: Expression) -> Box<dyn CompilerStatement> {
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
        result.push(ASSIGN::new(Data::Address(self.var_address)));        // TODO refactor
        result
    }
}