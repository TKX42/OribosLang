use crate::compiler::compile::compile_statements;
use crate::compiler::expression::{DataExpression, Expression, OperationExpression, Operator};
use crate::compiler::statement::assign_stmt::AssignmentStatement;
use crate::compiler::statement::CompilerStatement;
use crate::compiler::statement::get_stmt::GetStatement;
use crate::data::Data;
use crate::data::Data::JumpAddress;
use crate::interpreter::instruction::assign_instr::ASSIGN;
use crate::interpreter::instruction::const_instr::CONST;
use crate::interpreter::instruction::if_jump_instr::IFJUMP;
use crate::interpreter::instruction::Instruction;
use crate::interpreter::instruction::operator_instr::ADD;

#[derive(Clone, Debug)]
pub struct WhileLoopStatement {
    expression: Expression,
    statements: Vec<Box<dyn CompilerStatement>>,
}

impl WhileLoopStatement {
    pub fn create(expression: Expression, statements: Vec<Box<dyn CompilerStatement>>) -> Box<dyn CompilerStatement> {
        Box::new(WhileLoopStatement {
            expression,
            statements,
        })
    }
}

impl CompilerStatement for WhileLoopStatement {
    fn name(&self) -> String {
        String::from("for")
    }

    // use create()!
    fn init(_parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        unreachable!()
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        let mut result = vec![];

        let mut loop_statements = compile_statements(&self.statements);
        result.append(&mut loop_statements);

        // evaluate if expression is true
        result.append(&mut OperationExpression::new(
            self.expression.clone(),
            Expression::Data(DataExpression::new(Data::Bool(true))),
            Operator::Eq,
        ).compile());

        // if expression is true -> jump back to the beginning of the loop statements
        result.push(IFJUMP::new(JumpAddress(-((result.len()) as isize))));

        result
    }
}