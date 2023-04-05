use crate::compiler::compile::Compiler;
use crate::compiler::expression::{compile, Expression};
use crate::compiler::statement::{CompilerStatement, Scope};
use crate::data::Data;
use crate::interpreter::instruction::{Instruction};
use crate::interpreter::instruction::print_instr::PRINT;

#[derive(Clone, Debug)]
pub struct PrintStatement {
    data: Expression,
}

impl CompilerStatement for PrintStatement {
    fn name(&self) -> String {
        String::from("print")
    }

    fn init(parameters: &[Expression]) -> Box<dyn CompilerStatement> {
        Box::new(PrintStatement {
            data: parameters.get(0).expect("Invalid parameter for Print").clone(),
        })
    }

    fn compile(&self) -> Vec<Box<dyn Instruction>> {
        let mut result = vec![];
        result.append(&mut compile(&self.data));
        result.push(PRINT::new(Data::None));
        result
    }
}