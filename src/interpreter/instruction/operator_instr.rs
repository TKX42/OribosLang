use crate::data::Data;
use crate::interpreter::instruction::Instruction;
use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct ADD {
    name: String,
}

impl Instruction for ADD {
    fn new(_parameter: Data) -> Box<dyn Instruction> {
        Box::new(ADD { name: "Add".to_string() })
    }

    fn exec(&self, stack: &mut Vec<Data>, _memory: &mut Memory, instr_pointer: usize) -> usize {
        let right_val = stack.pop().unwrap();
        let left_val = stack.pop().unwrap();
        match left_val {
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { stack.push(Data::Number(left_x + right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }

        instr_pointer + 1
    }
}

#[derive(Clone, Debug)]
pub struct EQ {
    name: String,
}

impl Instruction for EQ {
    fn new(_parameter: Data) -> Box<dyn Instruction> {
        Box::new(EQ { name: "equals".to_string() })
    }

    fn exec(&self, stack: &mut Vec<Data>, _memory: &mut Memory, instr_pointer: usize) -> usize {
        let right_val = stack.pop().unwrap();
        let left_val = stack.pop().unwrap();
        match left_val {
            Data::String(ref left_x) => {
                match right_val {
                    Data::String(ref right_x) => { stack.push(Data::Bool(left_x == right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { stack.push(Data::Bool(left_x == right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Bool(ref left_x) => {
                match right_val {
                    Data::Bool(ref right_x) => { stack.push(Data::Bool(left_x == right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }

        instr_pointer + 1
    }
}

#[derive(Clone, Debug)]
pub struct LESS {
    name: String,
}

impl Instruction for LESS {
    fn new(_parameter: Data) -> Box<dyn Instruction> {
        Box::new(LESS { name: "Less".to_string() })
    }

    fn exec(&self, stack: &mut Vec<Data>, _memory: &mut Memory, instr_pointer: usize) -> usize {
        let right_val = stack.pop().unwrap();
        let left_val = stack.pop().unwrap();
        match left_val {
            Data::String(ref left_x) => {
                match right_val {
                    Data::String(ref right_x) => { stack.push(Data::Bool(left_x < right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { stack.push(Data::Bool(left_x < right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Bool(ref left_x) => {
                match right_val {
                    Data::Bool(ref right_x) => { stack.push(Data::Bool(left_x < right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }

        instr_pointer + 1
    }
}

#[derive(Clone, Debug)]
pub struct MODULO {
    name: String,
}

impl Instruction for MODULO {
    fn new(_parameter: Data) -> Box<dyn Instruction> {
        Box::new(MODULO { name: "Modulo".to_string() })
    }

    fn exec(&self, stack: &mut Vec<Data>, _memory: &mut Memory, instr_pointer: usize) -> usize {
        let right_val = stack.pop().unwrap();
        let left_val = stack.pop().unwrap();
        match left_val {
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(right_x) => { stack.push(Data::Number(left_x % right_x)) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }

        instr_pointer + 1
    }
}

fn operator_error(operation: &String, type_left: String, type_right: String) {
    panic!("Error: Cannot perform '{operation}' on '{type_left}' and '{type_right}'");
}

fn operator_error_single(operation: &String, type_left: String) {
    panic!("Error: Cannot perform '{operation}' on '{type_left}'");
}