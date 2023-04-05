use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::compiler::compile::Compiler;
use crate::compiler::expression::{Data, Expression};

pub mod print;
pub mod assignment;
pub mod get;
pub mod if_instr;
pub mod for_loop;
pub mod exit;
pub mod r#break;

dyn_clone::clone_trait_object!(CompilerStatement);
pub trait CompilerStatement: DynClone + Debug {
    fn name(&self) -> String;
    fn init(parameters: &[Expression]) -> Box<dyn CompilerStatement> where Self: Sized;
    fn compile(&self, interpreter: &mut Compiler, scope: &mut Scope) -> Data;
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