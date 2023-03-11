extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::interpreter::Interpreter;
use crate::parser::parse;

mod expression;
mod instruction;
mod parser;
mod operators;
mod interpreter;
mod memory;

fn demo_code() {
    println!("Executing demo code...");
    let demo_code = include_str!("code.obl");
    let ast = parse(demo_code);
    //println!("{:#?}", ast);
    let mut interpreter = Interpreter::new(ast);
    interpreter.run();
    //println!("{:#?}", interpreter);
}

fn main() {
    demo_code();
}
