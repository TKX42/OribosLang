use pest::iterators::Pair;
use pest::Parser;

use crate::expression::{Data, DataExpression, Expression};
use crate::instruction::ExecutableInstruction;
use crate::instruction::print::PrintInstruction;
use crate::instruction::time::TimeInstruction;

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

fn parse_expression(expression: Pair<Rule>) -> Expression {
    for expression_type in expression.into_inner() {
        match expression_type.as_rule() {
            Rule::primitive => { return parse_primitive(expression_type); }
            Rule::instr => { return Expression::ExecutableInstruction(parse_instr(expression_type)); }
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
        _ => { panic!("Error: Unknown instruction '{}'", instr_name) }
    }
}

pub fn parse(code_str: &str) -> Vec<Box<dyn ExecutableInstruction>> {
    let mut ast: Vec<Box<dyn ExecutableInstruction>> = vec![];

    let code = OribosParser::parse(Rule::code, code_str)
        .expect("Error: Unsuccessful parse")
        .next().unwrap();

    for instr in code.into_inner() {
        match instr.as_rule() {
            Rule::instr => {
                ast.push(parse_instr(instr));
            }
            Rule::CRLF => {}        // ignore
            _ => unreachable!(),
        }
    }

    ast
}