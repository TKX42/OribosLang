use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::expression::Expression;
use crate::interpreter::Interpreter;

pub mod print;
pub mod time;
pub mod answer;
pub mod assignment;

dyn_clone::clone_trait_object!(ExecutableInstruction);
pub trait ExecutableInstruction: DynClone + Debug {
    fn name(&self) -> String;
    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> where Self: Sized;
    fn exec(&self, interpreter: &mut Interpreter) -> Expression;
}