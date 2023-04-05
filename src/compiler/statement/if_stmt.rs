use crate::compiler::compile::{compile_statements, Compiler};
use crate::compiler::expression::{compile, Expression};
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::data::Data;
use crate::interpreter::instruction::if_jump_instr::IFJUMP;
use crate::interpreter::instruction::Instruction;
use crate::interpreter::instruction::jump_instr::JUMP;

#[derive(Clone, Debug)]
pub struct IfStatement {
    comparison: Expression,
    true_statements: Vec<Box<dyn CompilerStatement>>,
    else_statements: Vec<Box<dyn CompilerStatement>>,
}

impl IfStatement {
    pub fn create(comparison: Expression, true_statements: Vec<Box<dyn CompilerStatement>>, else_statements: Vec<Box<dyn CompilerStatement>>) -> Box<dyn CompilerStatement> {
        Box::new(IfStatement {
            comparison,
            true_statements,
            else_statements,
        })
    }
}

impl CompilerStatement for IfStatement {
    fn name(&self) -> String {
        String::from("if")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        let mut result = vec![];
        result.append(&mut compile(&self.comparison));

        let mut true_instrs = compile_statements(&self.true_statements);
        let mut else_instrs = compile_statements(&self.else_statements);

        result.push(IFJUMP::new(Data::JumpAddress(2)));     // pointer to the beginning of the true statements
        result.push(JUMP::new(Data::JumpAddress((true_instrs.len() + 2) as isize)));    // pointer to the beginning of the else statements
        result.append(&mut true_instrs);
        result.push(JUMP::new(Data::JumpAddress((else_instrs.len() + 1) as isize)));        // after if block, skip else block
        result.append(&mut else_instrs);
        result
    }
}