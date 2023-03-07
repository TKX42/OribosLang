#[derive(Clone)]
pub enum Data {
    String(String),
    Number(f64),
}

pub trait Expression {
    fn evaluate(&self) -> Data;
}

pub struct DataExpression {
    data: Data,
}

impl DataExpression {
    pub(crate) fn new(data: Data) -> DataExpression {
        DataExpression {
            data
        }
    }

    pub(crate) fn empty() -> DataExpression {
        DataExpression::new(Data::Number(0.0))
    }
}

impl Expression for DataExpression {
    fn evaluate(&self) -> Data {
        self.data.clone()
    }
}