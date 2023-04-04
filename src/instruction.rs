use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::expression::{Data, Expression};
use crate::interpreter::Interpreter;

pub mod print;
pub mod time;
pub mod assignment;
pub mod get;
pub mod if_instr;
pub mod for_loop;
pub mod exit;
pub mod r#break;

dyn_clone::clone_trait_object!(ExecutableInstruction);
pub trait ExecutableInstruction: DynClone + Debug {
    fn name(&self) -> String;
    fn init(parameters: &[Expression]) -> Box<dyn ExecutableInstruction> where Self: Sized;
    fn exec(&self, interpreter: &mut Interpreter, scope: &mut Scope) -> Data;
}

pub struct Scope {
    _break: bool
}

impl Scope {
    pub(crate) fn new() -> Scope {
        Scope {
            _break: false,
        }
    }
}