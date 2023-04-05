use std::fmt;

use crate::compiler::compile::Compiler;
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::data::Data;
use crate::interpreter::instruction::const_instr::CONST;
use crate::interpreter::instruction::Instruction;
use crate::interpreter::instruction::operator_instr::{ADD, EQ};

#[derive(Clone, Debug)]
pub enum Expression {
    Data(DataExpression),
    ExecutableInstruction(Box<dyn CompilerStatement>),
    Operation(Box<OperationExpression>),
}

#[derive(Clone, Debug)]
pub struct DataExpression {
    data: Data,
}

impl DataExpression {
    pub fn new(data: Data) -> DataExpression {
        DataExpression {
            data
        }
    }

    pub fn empty() -> DataExpression {
        DataExpression::new(Data::Number(0.0))
    }

    pub fn evaluate(&self) -> &Data {
        &self.data
    }
}

#[derive(Clone, Debug)]
pub enum Operator {
    add,
    sub,
    mul,
    div,
    eq,
    neq,
    neg,
}

#[derive(Clone, Debug)]
pub struct OperationExpression {
    operator: Operator,
    left: Expression,
    right: Expression,
}

impl OperationExpression {
    pub(crate) fn new(left: Expression, right: Expression, operator: Operator) -> OperationExpression {
        OperationExpression {
            operator,
            left,
            right,
        }
    }

    pub fn compile(&self) -> Vec<Box<dyn Instruction>> {
        let mut result = vec![];
        result.append(&mut compile(&self.left));
        result.append(&mut compile(&self.right));
        result.push(compile_operator(&self.operator));
        result
    }
}

fn compile_operator(operator: &Operator) -> Box<dyn Instruction> {
    match operator {
        Operator::add => { ADD::new(Data::None) }
        Operator::eq => { EQ::new(Data::None) }
        _ => { unimplemented!() }     // TODO: implement
    }
}

pub fn compile(expression: &Expression) -> Vec<Box<dyn Instruction>> {
    match expression {
        Expression::Data(dexpr) => { vec![CONST::new(dexpr.data.clone())] }
        Expression::ExecutableInstruction(instr) => { instr.compile() }
        Expression::Operation(opexpr) => { opexpr.compile() }
    }
}

pub fn get_number(data: &Data) -> f64 {
    match data {
        Data::Number(n) => { *n }
        _ => { panic!("Error: Could not parse number of data {data:?}") }
    }
}

pub fn get_memory_address(data: &Data) -> usize {
    match data {
        Data::MemoryAddress(a) => { *a }
        _ => { panic!("Error: Invalid memory address data type") }
    }
}

pub fn get_jump_address(data: &Data) -> isize {
    match data {
        Data::JumpAddress(a) => { *a }
        _ => { panic!("Error: Invalid jump address data type") }
    }
}

pub fn get_bool(data: &Data) -> bool {
    match data {
        Data::Bool(b) => { *b }
        x => { panic!("Error: Expected boolean, got {:?}", x) }
    }
}