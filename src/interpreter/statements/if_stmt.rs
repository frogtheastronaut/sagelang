use crate::parser::ast::{Expr, Stmt};
use crate::interpreter::{Env, Value};

pub fn eval_if_stmt(env: &mut Env, condition: &Expr, then_branch: &[Stmt], else_branch: &Option<Vec<Stmt>>, elseif_branches: &[(Expr, Vec<Stmt>)]) -> Value {
    if is_truthy(super::super::expressions::eval_expr(env, condition)) {
        super::super::statements::block::eval_block(env, then_branch)
    } else {
        for (cond, block) in elseif_branches {
            if is_truthy(super::super::expressions::eval_expr(env, cond)) {
                return super::super::statements::block::eval_block(env, block);
            }
        }
        if let Some(else_block) = else_branch {
            super::super::statements::block::eval_block(env, else_block)
        } else {
            Value::Null
        }
    }
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
