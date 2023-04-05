use std::fmt;

#[derive(Clone, Debug)]
pub enum Data {
    String(String),
    Number(f64),
    Bool(bool),
    Address(i64),
    None
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}