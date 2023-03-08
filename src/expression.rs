use crate::instruction::ExecutableInstruction;

#[derive(Clone)]
pub enum Data {
    String(String),
    Number(f64),
}

pub enum Expression {
    DataExpression(DataExpression),
    ExecutableInstruction(Box<dyn ExecutableInstruction>),
}

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

    pub fn evaluate(&self) -> Data {
        self.data.clone()
    }
}

pub fn evaluate(expression: &Expression) -> Data {
    match expression {
        Expression::DataExpression(dexpr) => { dexpr.evaluate() }
        Expression::ExecutableInstruction(instr) => { evaluate(&instr.exec()) }
    }
}