use dyn_clone::DynClone;

use crate::expression::Expression;

pub mod print;
pub mod time;

#[derive(Clone)]
pub struct Instruction {
    pub name: String,
}

impl Instruction {
    pub fn new(name: String) -> Instruction {
        Instruction {
            name
        }
    }
}

dyn_clone::clone_trait_object!(ExecutableInstruction);
pub trait ExecutableInstruction: DynClone {
    fn name(&self) -> &String;
    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> where Self: Sized;
    fn exec(&self) -> Expression;
    fn info(&self) -> String {
        return format!("[instruction {}]", self.name());
    }
}