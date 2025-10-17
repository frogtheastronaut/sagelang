use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;
use crate::interpreter::{Env, Value};

pub fn eval_binary_op(env: &mut Env, left: &Expr, op: &Token, right: &Expr) -> Value {
    let l = super::eval_expr(env, left);
    let r = super::eval_expr(env, right);
    match op {
        Token::Plus => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            _ => Value::Null,
        },
        Token::Minus => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Value::Null,
        },
        Token::Star => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Value::Null,
        },
        Token::Slash => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Value::Null,
        },
        Token::Percent => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => Value::Null,
        },
        Token::And => match (&l, &r) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(*a && *b),
            _ => Value::Null,
        },
        Token::Or => match (&l, &r) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(*a || *b),
            _ => Value::Null,
        },
        Token::DotDot => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => {
                let mut list = Vec::new();
                let start = *a as i64;
                let end = *b as i64;
                for i in start..=end {
                    list.push(Value::Number(i as f64));
                }
                Value::List(list)
            }
            _ => Value::Null,
        },
        Token::EqEq => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a == b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
            (Value::String(a), Value::String(b)) => Value::Bool(a == b),
            _ => Value::Bool(false),
        },
        Token::NotEq => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a != b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
            (Value::String(a), Value::String(b)) => Value::Bool(a != b),
            _ => Value::Bool(true),
        },
        Token::Greater => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a > b),
            _ => Value::Null,
        },
        Token::GreaterEq => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a >= b),
            _ => Value::Null,
        },
        Token::Less => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a < b),
            _ => Value::Null,
        },
        Token::LessEq => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a <= b),
            _ => Value::Null,
        },
        _ => Value::Null,
    }
}