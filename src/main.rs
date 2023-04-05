extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{env, fs};

use crate::compiler::compile::Compiler;
use crate::interpreter::interpret::Interpreter;
use crate::parser::parse;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod compiler;
mod parser;
mod memory;
mod interpreter;
mod data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code_path = args.get(1).expect("No code path given");
    let code = fs::read_to_string(code_path).expect("Unable to read file");
    let ast = parse(code.as_str());
    let mut interpreter = Compiler::new(ast);
    let instructions = interpreter.compile();

    let mut interpreter = Interpreter::new(instructions);
    interpreter.run();
}
