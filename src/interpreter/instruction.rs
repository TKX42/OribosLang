use std::fmt;
use std::fmt::Debug;
use dyn_clone::DynClone;
use crate::data::Data;
use crate::memory::Memory;

pub mod print_instr;
pub mod const_instr;
pub mod assign_instr;
pub mod get_instr;

dyn_clone::clone_trait_object!(Instruction);
pub trait Instruction: DynClone + Debug {
    fn new(parameter: Data) -> Box<dyn Instruction> where Self: Sized;
    fn exec(&self, stack: &mut Vec<Data>, memory: &mut Memory);
}