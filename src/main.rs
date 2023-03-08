extern crate chrono;

use std::io::Write;
use std::ops::Deref;

use crate::expression::{DataExpression, evaluate, Expression};
use crate::expression::Data;
use crate::instruction::{ExecutableInstruction, Instruction};
use crate::instruction::print::PrintInstruction;
use crate::instruction::time::TimeInstruction;

mod expression;
mod instruction;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();
    return result.trim().to_string();
}

fn execute_instruction<'list, T>(name: &String, instruction_list: &'list Vec<Box<dyn ExecutableInstruction>>) -> Option<&'list Box<dyn ExecutableInstruction>> {
    for instruction in instruction_list {
        if instruction.name() == name {
            return Some(instruction);
        }
    }

    None
}

fn exec(instruction: Box<dyn ExecutableInstruction>) {
    println!("executing {}...", instruction.name());
    match evaluate(&instruction.exec()) {
        Data::String(s) => { println!("returned {}", s) }
        Data::Number(n) => { println!("returned {}", n) }
    }
}

fn main() {
    let instruction_list = vec![
        // print("Hello World")
        PrintInstruction::init(&vec![
            Expression::DataExpression(DataExpression::new(Data::String("Hello World!".to_string())))
        ]),

        // print(time())
        PrintInstruction::init(&vec![
            Expression::ExecutableInstruction(TimeInstruction::init(&vec![]))
        ]),
    ];


    for instruction in instruction_list {
        exec(instruction);
    }


    /* interactive
    loop {
        let instruction_name = get_input("");
        if let Some(instruction) = execute_instruction(&instruction_name, &instruction_list) {
            instruction.exec();
        } else {
            println!("Error: Unknown instruction type");
        }
    }
    */
}
