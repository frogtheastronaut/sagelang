use crate::parser::ast::Expr;
use crate::interpreter::{Env, Value};

pub fn eval_return_stmt(env: &mut Env, value: &Option<Expr>) -> Value {
    match value {
        Some(expr) => Value::Return(Box::new(super::super::expressions::eval_expr(env, expr))),
        None => Value::Return(Box::new(Value::Null)),
    }
}
