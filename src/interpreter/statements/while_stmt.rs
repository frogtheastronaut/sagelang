use crate::parser::ast::{Expr, Stmt};
use crate::interpreter::{Env, Value};

pub fn eval_while_stmt(env: &mut Env, condition: &Expr, body: &[Stmt]) -> Value {
    let mut last = Value::Null;
    while is_truthy(super::super::expressions::eval_expr(env, condition)) {
        last = super::super::statements::block::eval_block(env, body);
    }
    last
}

fn is_truthy(val: Value) -> bool {
    match val {
        Value::Bool(b) => b,
        Value::Number(n) => n != 0.0,
        Value::Null => false,
        Value::List(ref l) => !l.is_empty(),
        Value::String(ref s) => !s.is_empty(),
        _ => true,
    }
}
