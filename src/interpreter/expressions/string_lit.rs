use crate::interpreter::Value;

pub fn eval_string_lit(s: &str) -> Value {
    Value::String(s.to_string())
}
