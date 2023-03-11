use crate::instruction::ExecutableInstruction;
use crate::operators::Operator;

#[derive(Clone, Debug)]
pub enum Data {
    String(String),
    Number(f64),
}

#[derive(Clone, Debug)]
pub enum Expression {
    DataExpression(DataExpression),
    ExecutableInstruction(Box<dyn ExecutableInstruction>),
    OperationExpression(Box<OperationExpression>),
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

    pub fn evaluate(&self) -> Data {
        self.data.clone()
    }
}

#[derive(Clone, Debug)]
pub struct OperationExpression {
    operator: Box<dyn Operator>,
    left: Expression,
    right: Expression,
}

impl OperationExpression {
    pub(crate) fn new(left: Expression, right: Expression, operator: Box<dyn Operator>) -> OperationExpression {
        OperationExpression {
            operator,
            left,
            right,
        }
    }

    pub fn evaluate(&self) -> Data {
        self.operator.evaluate(&self.left, &self.right)
    }
}

pub fn evaluate(expression: &Expression) -> Data {
    match expression {
        Expression::DataExpression(dexpr) => { dexpr.evaluate() }
        Expression::ExecutableInstruction(instr) => { evaluate(&instr.exec()) }
        Expression::OperationExpression(opexpr) => { opexpr.evaluate() }
    }
}