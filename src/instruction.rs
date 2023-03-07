use crate::expression::{Data, Expression};

pub mod print;
pub mod debug;
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
    fn init(parameters: &Vec<Box<dyn Expression>>) -> Box<dyn ExecutableInstruction> where Self: Sized;
    fn exec(&self) -> Box<dyn Expression>;
    fn info(&self) -> String {
        return format!("[instruction {}]", self.name());
    }
}

impl Expression for &dyn ExecutableInstruction {
    fn evaluate(&self) -> Data {
        self.exec().evaluate().clone()
    }
}