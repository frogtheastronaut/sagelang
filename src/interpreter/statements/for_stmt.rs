use crate::parser::ast::{Expr, Stmt};
use crate::interpreter::{Env, Value};

pub fn eval_for_stmt(env: &mut Env, var: &str, iterable: &Expr, body: &[Stmt]) -> Value {
    let iter_val = super::super::expressions::eval_expr(env, iterable);
    let mut last = Value::Null;
    match iter_val {
        Value::List(items) => {
            for item in items {
                env.set(var.to_string(), item);
                last = super::super::statements::block::eval_block(env, body);
            }
        }
        Value::Number(n) => {
            for i in 0..(n as i64) {
                env.set(var.to_string(), Value::Number(i as f64));
                last = super::super::statements::block::eval_block(env, body);
            }
        }
        _ => {}
    }
    last
}
