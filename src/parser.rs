use std::ops::Deref;

use pest::iterators::Pair;
use pest::Parser;

use crate::expression::{Data, DataExpression, Expression, OperationExpression};
use crate::instruction::answer::AnswerInstruction;
use crate::instruction::ExecutableInstruction;
use crate::instruction::print::PrintInstruction;
use crate::instruction::time::TimeInstruction;
use crate::operators::{Add, Div, Mul, Operator, Sub};

#[derive(Parser)]
#[grammar = "oriboslang.pest"]
pub struct OribosParser;

fn get_string(s: &str) -> Data {
    Data::String(String::from(&s[1..s.len() - 1]))
}

fn get_number(s: &str) -> Data {
    Data::Number(s.parse().unwrap())
}

fn parse_primitive(primitive: Pair<Rule>) -> Expression {
    for primitive_type in primitive.into_inner() {
        match primitive_type.as_rule() {
            Rule::string => { return Expression::DataExpression(DataExpression::new(get_string(primitive_type.as_str()))); }
            Rule::number => { return Expression::DataExpression(DataExpression::new(get_number(primitive_type.as_str()))); }
            _ => unreachable!()
        };
    }

    panic!("Error: Could not parse primitive!");
}

fn parse_operation(operation: Pair<Rule>) -> Expression {
    let mut operations: Vec<Expression> = vec![];
    let mut operators: Vec<Box<dyn Operator>> = vec![];

    for operation_type in operation.into_inner() {
        match operation_type.as_rule() {
            Rule::value => { operations.push(parse_value(operation_type)) }
            Rule::add => { operators.push(Add::new()) }
            Rule::sub => { operators.push(Sub::new()) }
            Rule::mul => { operators.push(Mul::new()) }
            Rule::div => { operators.push(Div::new()) }
            _ => { operations.push(parse_operation(operation_type)) }       // operation needs to be further resolved
        }
    }

    parse_operations(operations, operators)
}

fn parse_operations(operations: Vec<Expression>, operators: Vec<Box<dyn Operator>>) -> Expression {
    if operations.len() == 1 {
        return operations.deref().get(0).cloned().unwrap();
    } else {
        let left_operation = parse_operations(operations.get(0..operations.len() - 1).unwrap().to_vec(), operators.get(0..operators.len() - 1).unwrap().to_vec());
        let right_operation = operations.get(operations.len() - 1).cloned().unwrap();
        let operator = operators.get(operators.len() - 1).cloned().unwrap();
        return Expression::OperationExpression(Box::new(OperationExpression::new(left_operation, right_operation, operator)));
    }
}

fn parse_value(value: Pair<Rule>) -> Expression {
    return parse_expression(value);     // workaround because of parser not allowing left-recursion
}

fn parse_expression(expression: Pair<Rule>) -> Expression {
    for expression_type in expression.into_inner() {
        match expression_type.as_rule() {
            Rule::primitive => { return parse_primitive(expression_type); }
            Rule::instr => { return Expression::ExecutableInstruction(parse_instr(expression_type)); }
            Rule::operation => { return parse_operation(expression_type); }
            _ => unreachable!()
        }
    }

    Expression::DataExpression(DataExpression::empty())
}

fn parse_instr_parameters(parameters: Pair<Rule>) -> Vec<Expression> {
    let mut result: Vec<Expression> = vec![];

    for parameter in parameters.into_inner() {
        for expression in parameter.into_inner() {
            match expression.as_rule() {
                Rule::expression => { result.push(parse_expression(expression)) }
                _ => unreachable!()
            }
        }
    }

    result
}

fn parse_instr(instr: Pair<Rule>) -> Box<dyn ExecutableInstruction> {
    let mut instr_name = String::new();
    let mut instr_parameters = vec![];
    for field in instr.into_inner() {
        match field.as_rule() {
            Rule::instr_name => { instr_name = field.as_str().parse().unwrap() }
            Rule::parameter_list => { instr_parameters = parse_instr_parameters(field) }
            _ => unreachable!()
        }
    }

    create_instruction(instr_name, instr_parameters)
}

fn create_instruction(instr_name: String, instr_parameters: Vec<Expression>) -> Box<dyn ExecutableInstruction> {
    match instr_name.as_str() {
        "print" => PrintInstruction::init(&instr_parameters),
        "time" => TimeInstruction::init(&instr_parameters),
        "answer" => AnswerInstruction::init(&instr_parameters),
        _ => { panic!("Error: Unknown instruction '{}'", instr_name) }
    }
}

pub fn parse(code_str: &str) -> Vec<Box<dyn ExecutableInstruction>> {
    let mut ast: Vec<Box<dyn ExecutableInstruction>> = vec![];

    let code = OribosParser::parse(Rule::code, code_str).unwrap_or_else(|e| panic!("{}", e)).next().expect("Error parsing");
    for instr in code.into_inner() {
        match instr.as_rule() {
            Rule::instr => {
                ast.push(parse_instr(instr));
            }
            _ => unreachable!(),
        }
    }

    ast
}