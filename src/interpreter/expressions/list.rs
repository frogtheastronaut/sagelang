use crate::parser::ast::Expr;
use crate::interpreter::{Env, Value};

pub fn eval_list(env: &mut Env, exprs: &[Expr]) -> Value {
    Value::List(exprs.iter().map(|e| super::eval_expr(env, e)).collect())
}
