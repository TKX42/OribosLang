use dyn_clone::DynClone;

use crate::expression::{Data, evaluate, Expression};

dyn_clone::clone_trait_object!(Operator);
pub trait Operator: DynClone {
    fn new() -> Box<dyn Operator> where Self: Sized;
    fn evaluate(&self, left: &Expression, right: &Expression) -> Data;
}

// region "Add"
#[derive(Clone)]
pub struct Add {}

impl Operator for Add {
    fn new() -> Box<dyn Operator> where Self: Sized {
        Box::new(Add {})
    }

    fn evaluate(&self, left: &Expression, right: &Expression) -> Data {
        let left_val = evaluate(left);
        let right_val = evaluate(right);
        match left_val {
            Data::String(ls) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot add type 'string' and 'string'") }
                    Data::Number(rn) => { panic!("Error: Cannot add type 'string' and 'number'") }
                }
            }
            Data::Number(ln) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot add type 'number' and 'string'") }
                    Data::Number(rn) => { return Data::Number(ln + rn); }
                }
            }
        }
    }
}
// endregion

// region "Sub"
#[derive(Clone)]
pub struct Sub {}

impl Operator for Sub {
    fn new() -> Box<dyn Operator> where Self: Sized {
        Box::new(Sub {})
    }

    fn evaluate(&self, left: &Expression, right: &Expression) -> Data {
        let left_val = evaluate(left);
        let right_val = evaluate(right);
        match left_val {
            Data::String(ls) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot sub type 'string' and 'string'") }
                    Data::Number(rn) => { panic!("Error: Cannot sub type 'string' and 'number'") }
                }
            }
            Data::Number(ln) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot sub type 'number' and 'string'") }
                    Data::Number(rn) => { return Data::Number(ln - rn); }
                }
            }
        }
    }
}
// endregion

// region "Mul"
#[derive(Clone)]
pub struct Mul {}

impl Operator for Mul {
    fn new() -> Box<dyn Operator> where Self: Sized {
        Box::new(Mul {})
    }

    fn evaluate(&self, left: &Expression, right: &Expression) -> Data {
        let left_val = evaluate(left);
        let right_val = evaluate(right);
        match left_val {
            Data::String(ls) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot mul type 'string' and 'string'") }
                    Data::Number(rn) => { panic!("Error: Cannot mul type 'string' and 'number'") }
                }
            }
            Data::Number(ln) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot mul type 'number' and 'string'") }
                    Data::Number(rn) => { return Data::Number(ln * rn); }
                }
            }
        }
    }
}
// endregion

// region "Div"
#[derive(Clone)]
pub struct Div {}

impl Operator for Div {
    fn new() -> Box<dyn Operator> where Self: Sized {
        Box::new(Div {})
    }

    fn evaluate(&self, left: &Expression, right: &Expression) -> Data {
        let left_val = evaluate(left);
        let right_val = evaluate(right);
        match left_val {
            Data::String(ls) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot div type 'string' and 'string'") }
                    Data::Number(rn) => { panic!("Error: Cannot div type 'string' and 'number'") }
                }
            }
            Data::Number(ln) => {
                match right_val {
                    Data::String(rs) => { panic!("Error: Cannot div type 'number' and 'string'") }
                    Data::Number(rn) => { return Data::Number(ln / rn); }
                }
            }
        }
    }
}
// endregion