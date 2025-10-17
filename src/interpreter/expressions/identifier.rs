use crate::interpreter::{Env, Value};

pub fn eval_identifier(env: &Env, name: &str) -> Value {
    env.get(name).unwrap_or(Value::Null)
}
