use crate::parser::ast::Stmt;
use super::{Env, Value};
use super::statements;

pub struct Interpreter {
    pub env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Env::new() }
    }
    pub fn interpret(&mut self, stmts: &[Stmt]) -> Option<Value> {
        let mut last = None;
        for stmt in stmts {
            last = statements::eval_stmt(&mut self.env, stmt);
        }
        last
    }
}
