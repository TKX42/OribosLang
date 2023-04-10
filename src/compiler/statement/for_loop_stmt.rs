use crate::compiler::compile::compile_statements;
use crate::compiler::expression::{Expression, OperationExpression, Operator};
use crate::compiler::statement::CompilerStatement;
use crate::compiler::statement::assign_stmt::AssignmentStatement;
use crate::compiler::statement::get_stmt::GetStatement;
use crate::data::Data;
use crate::data::Data::JumpAddress;
use crate::interpreter::instruction::assign_instr::ASSIGN;
use crate::interpreter::instruction::const_instr::CONST;
use crate::interpreter::instruction::if_jump_instr::IFJUMP;
use crate::interpreter::instruction::Instruction;
use crate::interpreter::instruction::operator_instr::ADD;

#[derive(Clone, Debug)]
pub struct ForLoopStatement {
    counter_var_address: usize,
    start_i: Expression,
    end_i: Expression,
    statements: Vec<Box<dyn CompilerStatement>>,
}

impl ForLoopStatement {
    pub fn create(counter_var_address: usize, start_i: Expression, end_i: Expression, statements: Vec<Box<dyn CompilerStatement>>) -> Box<dyn CompilerStatement> {
        Box::new(ForLoopStatement {
            counter_var_address,
            start_i,
            end_i,
            statements,
        })
    }
}

impl CompilerStatement for ForLoopStatement {
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
        result.append(&mut AssignmentStatement::create(self.counter_var_address, self.start_i.clone()).compile());
        result.append(&mut loop_statements);

        // increase counter variable
        result.append(&mut GetStatement::create(self.counter_var_address).compile());
        result.push(CONST::new(Data::Number(1.0)));
        result.push(ADD::new(Data::None));
        result.push(ASSIGN::new(Data::MemoryAddress(self.counter_var_address)));

        // counter < end_i
        result.append(&mut OperationExpression::new(
            Expression::Statement(GetStatement::create(self.counter_var_address)),
            self.end_i.clone(),
            Operator::Less,
        ).compile());

        // if counter is Less than end -> jump back to the beginning of the loop statements
        result.push(IFJUMP::new(JumpAddress(-((result.len() - 2)
            as isize))));

        result
    }
}