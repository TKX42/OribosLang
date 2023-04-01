#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{env, fs};
use crate::interpreter::Interpreter;
use crate::parser::parse;

mod expression;
mod instruction;
mod parser;
mod operators;
mod interpreter;
mod memory;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code_path = args.get(1).expect("No code path given");
    let code = fs::read_to_string(code_path).expect("Unable to read file");
    let ast = parse(code.as_str());
    let mut interpreter = Interpreter::new(ast);
    interpreter.run();
}
