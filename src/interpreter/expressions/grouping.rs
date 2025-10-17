use crate::parser::ast::Expr;
use crate::interpreter::{Env, Value};

pub fn eval_grouping(env: &mut Env, expr: &Expr) -> Value {
    super::eval_expr(env, expr)
}
