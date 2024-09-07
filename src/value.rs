#![allow(dead_code)]
use rug::{Assign, Float};

#[derive(Debug, Clone)]
pub enum Value {
    Number(Float),
    Variable(char),
    Function(String, Box<Value>),
    Operation(Box<Value>, String, Box<Value>),
    Negation(Box<Value>),
}

pub fn apply_operation<F>(l: &Value, r: &Value, op: &str, operation: F) -> Value
where
    F: Fn(&Float, &Float) -> Float,
{
    match (l, r) {
        (Value::Number(lhs), Value::Number(rhs)) => {
            let mut float = Float::new(256);
            float.assign(operation(lhs, rhs));
            Value::Number(float)
        }
        _ => Value::Operation(Box::new(l.clone()), op.to_string(), Box::new(r.clone())),
    }
}
