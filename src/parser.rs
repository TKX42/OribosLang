use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

use pest::iterators::Pair;
use pest::Parser;

use crate::expression::{Data, DataExpression, Expression, OperationExpression};
use crate::instruction::assignment::AssignmentInstruction;
use crate::instruction::ExecutableInstruction;
use crate::instruction::exit::ExitInstruction;
use crate::instruction::for_loop::ForLoopInstruction;
use crate::instruction::get::GetInstruction;
use crate::instruction::if_instr::IfInstruction;
use crate::instruction::print::PrintInstruction;
use crate::instruction::r#break::BreakInstruction;
use crate::instruction::time::TimeInstruction;
use crate::operators::{Add, Div, Equals, Greater, Lesser, Modulo, Mul, NotEquals, Operator, Sub};

#[derive(Parser)]
#[grammar = "oriboslang.pest"]
pub struct OribosParser;

struct IdentifierTable {
    i: i64,
    identifier: HashMap<String, i64>,
}

fn get_string(s: &str) -> Data {
    Data::String(String::from(&s[1..s.len() - 1]))
}

fn get_number(s: &str) -> Data {
    Data::Number(s.parse().unwrap())
}

fn get_bool(s: &str) -> Data {
    Data::Bool(s.parse().unwrap())
}

fn parse_primitive(primitive: Pair<Rule>) -> Expression {
    for primitive_type in primitive.into_inner() {
        match primitive_type.as_rule() {
            Rule::string => { return Expression::Data(DataExpression::new(get_string(primitive_type.as_str()))); }
            Rule::number => { return Expression::Data(DataExpression::new(get_number(primitive_type.as_str()))); }
            Rule::bool => { return Expression::Data(DataExpression::new(get_bool(primitive_type.as_str()))); }
            _ => unreachable!()
        };
    }

    panic!("Error: Could not parse primitive!");
}

fn parse_operation(operation: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Expression {
    let mut operations: Vec<Expression> = vec![];
    let mut operators: Vec<Box<dyn Operator>> = vec![];

    for operation_type in operation.into_inner() {
        match operation_type.as_rule() {
            Rule::value => { operations.push(parse_value(operation_type, identifier_table)) }
            Rule::add => { operators.push(Add::create()) }
            Rule::sub => { operators.push(Sub::create()) }
            Rule::mul => { operators.push(Mul::create()) }
            Rule::div => { operators.push(Div::create()) }
            Rule::modulo => { operators.push(Modulo::create()) }
            Rule::equals => { operators.push(Equals::create()) }
            Rule::not_equals => { operators.push(NotEquals::create()) }
            Rule::greater => { operators.push(Greater::create()) }
            Rule::lesser => { operators.push(Lesser::create()) }
            _ => { operations.push(parse_operation(operation_type, identifier_table)) }       // operation needs to be further resolved
        }
    }

    parse_operations(operations, operators)
}

fn parse_operations(operations: Vec<Expression>, operators: Vec<Box<dyn Operator>>) -> Expression {
    if operations.len() == 1 {
        return operations.deref().get(0).cloned().unwrap();
    } else {
        let left_operation = parse_operations(operations.get(0..operations.len() - 1).unwrap().to_vec(), operators.get(0..operators.len() - 1).unwrap().to_vec());
        let right_operation = operations.last().cloned().unwrap();
        let operator = operators.last().cloned().unwrap();
        Expression::Operation(Box::new(OperationExpression::new(left_operation, right_operation, operator)))
    }
}

fn parse_value(value: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Expression {
    parse_expression(value, identifier_table)     // workaround because of parser not allowing left-recursion
}

fn parse_expression(expression: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Expression {
    for expression_type in expression.into_inner() {
        match expression_type.as_rule() {
            Rule::primitive => { return parse_primitive(expression_type); }
            Rule::instr => { return Expression::ExecutableInstruction(parse_instr(expression_type, identifier_table)); }
            Rule::operation => { return parse_operation(expression_type, identifier_table); }
            Rule::variable => { return Expression::ExecutableInstruction(parse_variable_get(expression_type, identifier_table)); }       // print(x) -> "x" will be converted into print(get(x))
            _ => unreachable!()
        }
    }

    Expression::Data(DataExpression::empty())
}

fn parse_variable_get(variable: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Box<dyn ExecutableInstruction> {
    let var_id = get_var_id(variable.as_str().to_string(), identifier_table);
    GetInstruction::create(var_id)
}

fn parse_instr_parameters(parameters: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Vec<Expression> {
    let mut result: Vec<Expression> = vec![];

    for parameter in parameters.into_inner() {
        for expression in parameter.into_inner() {
            match expression.as_rule() {
                Rule::expression => { result.push(parse_expression(expression, identifier_table)) }
                _ => unreachable!()
            }
        }
    }

    result
}

fn parse_instr(instr: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Box<dyn ExecutableInstruction> {
    let mut instr_name = String::new();
    let mut instr_parameters = vec![];
    for field in instr.into_inner() {
        match field.as_rule() {
            Rule::instr_name => { instr_name = field.as_str().parse().unwrap() }
            Rule::parameter_list => { instr_parameters = parse_instr_parameters(field, identifier_table) }
            _ => unreachable!()
        }
    }

    create_instruction(instr_name, instr_parameters)
}

fn create_instruction(instr_name: String, instr_parameters: Vec<Expression>) -> Box<dyn ExecutableInstruction> {
    match instr_name.as_str() {
        "print" => PrintInstruction::init(&instr_parameters),
        "time" => TimeInstruction::init(&instr_parameters),
        "exit" => ExitInstruction::init(&instr_parameters),
        "break" => BreakInstruction::init(&instr_parameters),
        _ => { panic!("Error: Unknown instruction '{instr_name}'") }
    }
}

fn parse_assignment(assignment: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Box<dyn ExecutableInstruction> {
    let mut var_name = String::new();
    let mut var_expression = Expression::Data(DataExpression::empty());

    for assignment_field in assignment.into_inner() {
        match assignment_field.as_rule() {
            Rule::variable => { var_name = assignment_field.as_str().to_string() }
            Rule::expression => { var_expression = parse_expression(assignment_field, identifier_table) }
            _ => unreachable!()
        }
    }

    AssignmentInstruction::create(get_var_id(var_name, identifier_table), var_expression)
}

fn get_var_id(var_name: String, identifier_table: &mut IdentifierTable) -> i64 {
    return match identifier_table.identifier.get(&*var_name) {
        None => {
            identifier_table.i += 1;
            identifier_table.identifier.insert(var_name, identifier_table.i);
            identifier_table.i
        }
        Some(x) => { *x }
    };
}

fn parse_if_expr(if_expr: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Box<dyn ExecutableInstruction> {
    let mut comparison = Expression::Data(DataExpression::empty());
    let mut true_statements = vec![];
    let mut else_statements = vec![];

    for field in if_expr.into_inner() {
        match field.as_rule() {
            Rule::comparison => { comparison = parse_operation(field, identifier_table) }
            Rule::statements => { true_statements = parse_statements(identifier_table, field) }
            Rule::else_expr => { else_statements = parse_statements(identifier_table, field.into_inner().next().expect("Error: No else statements given")) }
            _ => unreachable!()
        }
    }

    IfInstruction::create(comparison, true_statements, else_statements)
}

fn parse_for_loop(for_loop: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Box<dyn ExecutableInstruction> {
    let mut counter_var_id = -1;
    let mut start_i = Expression::Data(DataExpression::empty());
    let mut end_i = Expression::Data(DataExpression::empty());
    let mut statements = vec![];

    for field in for_loop.into_inner() {
        match field.as_rule() {
            Rule::variable => { counter_var_id = get_var_id(field.as_str().to_string(), identifier_table) }
            Rule::start_i => { start_i = parse_expression(field.into_inner().next().expect("Error: No start i in for loop"), identifier_table) }
            Rule::end_i => { end_i = parse_expression(field.into_inner().next().expect("Error: No end i in for loop"), identifier_table) }
            Rule::statements => { statements = parse_statements(identifier_table, field) }
            _ => unreachable!()
        }
    }

    ForLoopInstruction::create(counter_var_id, start_i, end_i, statements)
}

fn parse_statement(statement: Pair<Rule>, identifier_table: &mut IdentifierTable) -> Box<dyn ExecutableInstruction> {
    for statement_type in statement.into_inner() {
        match statement_type.as_rule() {
            Rule::instr => { return parse_instr(statement_type, identifier_table); }
            Rule::assignment => { return parse_assignment(statement_type, identifier_table); }
            Rule::if_expr => { return parse_if_expr(statement_type, identifier_table); }
            Rule::for_expr => { return parse_for_loop(statement_type, identifier_table); }
            _ => unreachable!()
        };
    }

    panic!("Error: Could not parse statement")
}

pub fn parse(code_str: &str) -> Vec<Box<dyn ExecutableInstruction>> {
    let mut identifier_table = IdentifierTable { i: 0, identifier: Default::default() };

    let code = OribosParser::parse(Rule::code, code_str).unwrap_or_else(|e| panic!("{}", e)).next().expect("Error parsing");
    parse_statements(&mut identifier_table, code)
}

fn parse_statements(identifier_table: &mut IdentifierTable, statements: Pair<Rule>) -> Vec<Box<dyn ExecutableInstruction>> {
    let mut result = vec![];
    for statement in statements.into_inner() {
        match statement.as_rule() {
            Rule::statement => {
                result.push(parse_statement(statement, identifier_table));
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    result
}