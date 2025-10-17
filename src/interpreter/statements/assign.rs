use crate::parser::ast::Expr;
use crate::interpreter::{Env, Value};

pub fn eval_assign(env: &mut Env, name: &str, value: &Expr) -> Value {
    let val = super::super::expressions::eval_expr(env, value);
    // Try to update existing variable first, otherwise create new one
    if !env.update(name, val.clone()) {
        env.set(name.to_string(), val.clone());
    }
    val
}
