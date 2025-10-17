use crate::parser::ast::{Expr, Stmt};
use crate::interpreter::{Env, Value};

pub fn eval_for_stmt(env: &mut Env, var: &str, iterable: &Expr, body: &[Stmt]) -> Value {
    let iter_val = super::super::expressions::eval_expr(env, iterable);
    let mut last = Value::Null;
    match iter_val {
        Value::List(items) => {
            for item in items {
                env.set(var.to_string(), item);
                // Evaluate statements directly in the current environment
                for stmt in body {
                    last = super::eval_stmt(env, stmt).unwrap_or(Value::Null);
                    // Check if we hit a return statement
                    if let Value::Return(_) = last {
                        return last;
                    }
                }
            }
        }
        Value::Number(n) => {
            for i in 0..(n as i64) {
                env.set(var.to_string(), Value::Number(i as f64));
                // Evaluate statements directly in the current environment
                for stmt in body {
                    last = super::eval_stmt(env, stmt).unwrap_or(Value::Null);
                    // Check if we hit a return statement
                    if let Value::Return(_) = last {
                        return last;
                    }
                }
            }
        }
        _ => {}
    }
    last
}
