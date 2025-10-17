use crate::parser::ast::Expr;
use crate::interpreter::{Env, Value};

pub fn eval_print(env: &mut Env, expr: &Expr) -> Value {
    let val = super::super::expressions::eval_expr(env, expr);
    println!("{:?}", val);
    Value::Null
}
