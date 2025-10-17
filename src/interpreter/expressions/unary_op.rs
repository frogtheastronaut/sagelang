use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;
use crate::interpreter::{Env, Value};

pub fn eval_unary_op(env: &mut Env, op: &Token, right: &Expr) -> Value {
    let val = super::eval_expr(env, right);
    match op {
        Token::Minus => match val {
            Value::Number(n) => Value::Number(-n),
            _ => Value::Null,
        },
        _ => Value::Null,
    }
}
