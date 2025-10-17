use crate::parser::ast::{Param, Stmt};
use super::env::Env;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Function {
        params: Vec<Param>,
        body: Vec<Stmt>,
        env: Env,
    },
    Return(Box<Value>),
    Null,
}
