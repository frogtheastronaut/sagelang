use crate::parser::ast::Stmt;
use crate::interpreter::{Env, Value};

pub fn eval_block(env: &mut Env, stmts: &[Stmt]) -> Value {
    let mut local_env = Env::with_parent(env.clone());
    for s in stmts {
        let val = super::super::statements::eval_stmt(&mut local_env, s).unwrap_or(Value::Null);
        if let Value::Return(_) = val {
            return val;
        }
    }
    Value::Null
}