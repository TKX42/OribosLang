use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::expression::Expression;

pub mod print;
pub mod time;
pub mod answer;

dyn_clone::clone_trait_object!(ExecutableInstruction);
pub trait ExecutableInstruction: DynClone + Debug {
    fn name(&self) -> String;
    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> where Self: Sized;
    fn exec(&self) -> Expression;
}