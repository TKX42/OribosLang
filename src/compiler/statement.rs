use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::compiler::compile::Compiler;
use crate::compiler::expression::Expression;
use crate::interpreter::instruction::Instruction;

pub mod print_stmt;
pub mod assign_stmt;
pub mod get_stmt;
pub mod if_stmt;
pub mod for_loop_stmt;
pub mod exit_stmt;
pub mod break_stmt;

dyn_clone::clone_trait_object!(CompilerStatement);
pub trait CompilerStatement: DynClone + Debug {
    fn name(&self) -> String;
    fn init(parameters: &[Expression]) -> Box<dyn CompilerStatement> where Self: Sized;
    fn compile(&self) -> Vec<Box<dyn Instruction>>;
}

pub struct Scope {
    pub(crate) _break: bool,
    pub(crate) _exit: bool,
    pub(crate) _exit_code: i32,
}

impl Scope {
    pub(crate) fn new() -> Scope {
        Scope {
            _break: false,
            _exit: false,
            _exit_code: 0,
        }
    }
}