use crate::parser::ast::{Expr, Stmt};
use crate::interpreter::{Env, Value};

pub fn eval_if_stmt(env: &mut Env, condition: &Expr, then_branch: &[Stmt], else_branch: &Option<Vec<Stmt>>, elseif_branches: &[(Expr, Vec<Stmt>)]) -> Value {
    if is_truthy(super::super::expressions::eval_expr(env, condition)) {
        // Evaluate then branch directly in current environment
        let mut last = Value::Null;
        for stmt in then_branch {
            last = super::eval_stmt(env, stmt).unwrap_or(Value::Null);
            if let Value::Return(_) = last {
                return last;
            }
        }
        last
    } else {
        for (cond, block) in elseif_branches {
            if is_truthy(super::super::expressions::eval_expr(env, cond)) {
                let mut last = Value::Null;
                for stmt in block {
                    last = super::eval_stmt(env, stmt).unwrap_or(Value::Null);
                    if let Value::Return(_) = last {
                        return last;
                    }
                }
                return last;
            }
        }
        if let Some(else_block) = else_branch {
            let mut last = Value::Null;
            for stmt in else_block {
                last = super::eval_stmt(env, stmt).unwrap_or(Value::Null);
                if let Value::Return(_) = last {
                    return last;
                }
            }
            last
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
