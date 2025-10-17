use crate::parser::ast::{Param, Stmt};
use crate::interpreter::{Env, Value};

pub fn eval_function_stmt(env: &mut Env, name: &str, params: &[Param], body: &[Stmt]) -> Value {
    let func = Value::Function {
        params: params.to_vec(),
        body: body.to_vec(),
        env: env.clone(),
    };
    env.set(name.to_string(), func.clone());
    func
}
