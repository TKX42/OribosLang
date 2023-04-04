use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::expression::{Data, evaluate, Expression};
use crate::instruction::Scope;
use crate::interpreter::Interpreter;

dyn_clone::clone_trait_object!(Operator);
pub trait Operator: DynClone + Debug {
    fn create() -> Box<dyn Operator> where Self: Sized;
    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data;
}

// region "Add"
#[derive(Clone, Debug)]
pub struct Add {
    name: String,
}

impl Operator for Add {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Add {
            name: "addition".to_string()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { Data::Number(left_x + right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }
    }
}
// endregion

// region "Sub"
#[derive(Clone, Debug)]
pub struct Sub {
    name: String,
}

impl Operator for Sub {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Sub {
            name: "subtraction".to_string()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { Data::Number(left_x - right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }
    }
}
// endregion

// region "Mul"
#[derive(Clone, Debug)]
pub struct Mul {
    name: String,
}

impl Operator for Mul {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Mul {
            name: "multiplication".parse().unwrap()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(right_x) => { Data::Number(left_x * right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }
    }
}
// endregion

// region "Div"
#[derive(Clone, Debug)]
pub struct Div {
    name: String,
}

impl Operator for Div {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Div {
            name: "division".parse().unwrap()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { Data::Number(left_x / right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }
    }
}
// endregion

// region "Equals"
#[derive(Clone, Debug)]
pub struct Equals {
    name: String,
}

impl Operator for Equals {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Equals {
            name: "equals".parse().unwrap()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::String(ref left_x) => {
                match right_val {
                    Data::String(ref right_x) => { Data::Bool(left_x == right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { Data::Bool(left_x == right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Bool(ref left_x) => {
                match right_val {
                    Data::Bool(ref right_x) => { Data::Bool(left_x == right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
        }
    }
}
// endregion

// region "Modulo"
#[derive(Clone, Debug)]
pub struct Modulo {
    name: String,
}

impl Operator for Modulo {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Modulo {
            name: "modulo".parse().unwrap()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(right_x) => { Data::Number(left_x % right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            _ => operator_error_single(&self.name, left_val.to_string())
        }
    }
}
// endregion

// region "NotEquals"
#[derive(Clone, Debug)]
pub struct NotEquals {
    name: String,
}

impl Operator for NotEquals {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(NotEquals {
            name: "not_equals".parse().unwrap()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::String(ref left_x) => {
                match right_val {
                    Data::String(ref right_x) => { Data::Bool(left_x != right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { Data::Bool(left_x != right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Bool(ref left_x) => {
                match right_val {
                    Data::Bool(ref right_x) => { Data::Bool(left_x != right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
        }
    }
}
// endregion

// region "Greater"
#[derive(Clone, Debug)]
pub struct Greater {
    name: String,
}

impl Operator for Greater {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Greater {
            name: "greater".parse().unwrap()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::String(ref left_x) => {
                match right_val {
                    Data::String(ref right_x) => { Data::Bool(left_x > right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { Data::Bool(left_x > right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Bool(ref left_x) => {
                match right_val {
                    Data::Bool(ref right_x) => { Data::Bool(left_x > right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
        }
    }
}
// endregion

// region "Lesser"
#[derive(Clone, Debug)]
pub struct Lesser {
    name: String,
}

impl Operator for Lesser {
    fn create() -> Box<dyn Operator> where Self: Sized {
        Box::new(Lesser {
            name: "lesser".parse().unwrap()
        })
    }

    fn evaluate(&self, left: &Expression, right: &Expression, interpreter: &mut Interpreter, scope: &mut Scope) -> Data {
        let left_val = evaluate(left, interpreter, scope);
        let right_val = evaluate(right, interpreter, scope);
        match left_val {
            Data::String(ref left_x) => {
                match right_val {
                    Data::String(ref right_x) => { Data::Bool(left_x < right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Number(ref left_x) => {
                match right_val {
                    Data::Number(ref right_x) => { Data::Bool(left_x < right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
            Data::Bool(ref left_x) => {
                match right_val {
                    Data::Bool(ref right_x) => { Data::Bool(left_x < right_x) }
                    _ => operator_error(&self.name, left_val.to_string(), right_val.to_string())
                }
            }
        }
    }
}
// endregion

fn operator_error(operation: &String, type_left: String, type_right: String) -> Data {
    panic!("Error: Cannot perform '{operation}' on '{type_left}' and '{type_right}'");
}

fn operator_error_single(operation: &String, type_left: String) -> Data {
    panic!("Error: Cannot perform '{operation}' on '{type_left}'");
}