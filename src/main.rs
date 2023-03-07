extern crate chrono;

use std::io::Write;

use crate::expression::Expression;
use crate::expression::Data;
use crate::instruction::ExecutableInstruction;
use crate::instruction::debug::DebugInstruction;
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

fn main() {
    let instruction_list = vec![
        TimeInstruction::init(&vec![]),
        PrintInstruction::init(&vec![Expression::new(Data::String(String::from("Hello Oribos Lang!")))]),
        DebugInstruction::init(&vec![Expression::new(Data::String(String::from("A debugging...\n\t\tmessage")))]),
        DebugInstruction::init(&vec![Expression::new(Data::Number(42.0))]),
        TimeInstruction::init(&vec![]),
    ];

    for instruction in instruction_list {
        instruction.exec();
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
