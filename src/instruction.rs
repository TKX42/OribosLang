use crate::expression::{Data, Expression};

pub mod print;
pub mod time;

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

pub trait ExecutableInstruction {
    fn name(&self) -> &String;
    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> where Self: Sized;
    fn exec(&self) -> Expression;
    fn info(&self) -> String {
        return format!("[instruction {}]", self.name());
    }
}