use crate::parser::ast::Expr;
use crate::interpreter::{Env, Value};

pub fn eval_expr_stmt(env: &mut Env, expr: &Expr) -> Value {
    super::super::expressions::eval_expr(env, expr)
}
