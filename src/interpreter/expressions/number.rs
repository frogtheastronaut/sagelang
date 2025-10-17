use crate::interpreter::Value;

pub fn eval_number(n: f64) -> Value {
    Value::Number(n)
}
