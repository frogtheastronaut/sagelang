use crate::parser::ast::Expr;
use crate::interpreter::{Env, Value};

pub fn eval_call(env: &mut Env, callee: &Expr, args: &[Expr]) -> Value {
    use crate::interpreter::Value;
    match callee {
        Expr::Identifier(name) => {
            let func_val = env.get(name);
            match func_val {
                Some(Value::Function { params, body, env: func_env }) => {
                    let mut call_env = Env::with_parent(func_env.clone());
                    for (i, param) in params.iter().enumerate() {
                        let arg_val = args.get(i).map(|e| super::eval_expr(env, e)).unwrap_or(Value::Null);
                        call_env.set(param.param_name.clone(), arg_val.clone());
                    }
                    for stmt in &body {
                        if let Some(val) = crate::interpreter::statements::eval_stmt(&mut call_env, stmt) {
                            // If we hit a return, propagate it immediately
                            if let Value::Return(ret_val) = val {
                                return *ret_val;
                            }
                        }
                    }
                    Value::Null
                }
                _ => Value::Null,
            }
        }
        _ => Value::Null,
    }
}
