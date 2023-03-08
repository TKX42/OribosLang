extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::instruction::ExecutableInstruction;
use crate::parser::parse;

mod expression;
mod instruction;
mod parser;

fn exec(ast: Vec<Box<dyn ExecutableInstruction>>) {
    for instruction in ast {
        instruction.exec();
    }
}

fn demo_code() {
    let demo_code = include_str!("code.obl");
    let ast = parse(demo_code);
    exec(ast);
}

fn main() {
    demo_code();
}