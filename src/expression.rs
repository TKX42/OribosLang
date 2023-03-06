#[derive(Clone)]
pub enum Data {
    String(String),
    Number(i32),
}

pub struct Expression {
    data: Data,
}

impl Expression {
    pub fn new(data: Data) -> Expression {
        Expression {
            data
        }
    }

    pub fn evaluate(&self) -> &Data {
        &self.data
    }
}