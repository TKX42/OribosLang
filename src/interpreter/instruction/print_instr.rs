use crate::data::Data;
use crate::interpreter::instruction::{Instruction};
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct PRINT {

}

impl Instruction for PRINT {
    fn new(parameter: Data) -> Box<dyn Instruction> {
        Box::new(PRINT{})
    }

    fn exec(&self, stack: &mut Vec<Data>, memory: &mut Memory) {
        let data = stack.pop().unwrap();
        print(&data);
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{s}") }
        Data::Number(n) => { println!("{n}") }
        Data::Bool(b) => { println!("{b}") }
        _ => {println!()}
    }
}