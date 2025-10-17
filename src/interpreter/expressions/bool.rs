use crate::interpreter::Value;

pub fn eval_bool(b: bool) -> Value {
    Value::Bool(b)
}
