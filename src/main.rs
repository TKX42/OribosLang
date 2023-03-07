extern crate chrono;

use std::fmt::{Debug, Display};
use std::io::Write;

use chrono::Local;

use crate::expression::{Data, Expression};

mod expression;

struct Instruction {
    name: String,
}

impl Instruction {
    fn new(name: String) -> Instruction {
        Instruction {
            name
        }
    }
}

trait ExecutableInstruction {
    fn name(&self) -> &String;
    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> where Self: Sized;
    fn exec(&self);
    fn info(&self) -> String {
        return format!("[Instruction {}]", self.name());
    }
}

// region "PRINT"

struct PrintInstruction {
    instruction: Instruction,
    data: Data,
}

impl ExecutableInstruction for PrintInstruction {
    fn name(&self) -> &String {
        return &self.instruction.name;
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(PrintInstruction {
            instruction: Instruction::new("print".to_string()),
            data: parameters.get(0).expect("Invalid parameter for Print").evaluate().clone(),
        })
    }

    fn exec(&self) {
        print(&self.data)
    }
}

fn print(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}

// endregion

// region "DEBUG"

struct DebugInstruction {
    instruction: Instruction,
    data: Data,
}

impl DebugInstruction {}

impl ExecutableInstruction for DebugInstruction {
    fn name(&self) -> &String {
        return &self.instruction.name;
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(DebugInstruction {
            instruction: Instruction::new("debug".to_string()),
            data: parameters.get(0).expect("Invalid parameter for Debug").evaluate().clone(),
        })
    }

    fn exec(&self) {
        debug(&self.data)
    }
}

fn debug(data: &Data) {
    match data {
        Data::String(s) => { println!("{}", s) }
        Data::Number(n) => { println!("{}", n) }
    }
}

// endregion

// region "TIME"

struct TimeInstruction {
    instruction: Instruction,
}

impl ExecutableInstruction for TimeInstruction {
    fn name(&self) -> &String {
        &self.instruction.name
    }

    fn init(parameters: &Vec<Expression>) -> Box<dyn ExecutableInstruction> {
        Box::new(TimeInstruction {
            instruction: Instruction::new("time".to_string())
        })
    }

    fn exec(&self) {
        let date = Local::now();
        println!("{}", date.format("%Y-%m-%d/%H:%M:%S:%f"));
    }
}

// endregion

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
