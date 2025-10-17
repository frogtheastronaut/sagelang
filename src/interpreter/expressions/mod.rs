pub mod number;
pub mod bool;
pub mod string_lit;
pub mod identifier;
pub mod list;
pub mod unary_op;
pub mod binary_op;
pub mod grouping;
pub mod call;

use crate::parser::ast::{Expr};
use crate::interpreter::{Env, Value};

pub fn eval_expr(env: &mut Env, expr: &Expr) -> Value {
    match expr {
        Expr::Number(n) => number::eval_number(*n),
        Expr::Bool(b) => bool::eval_bool(*b),
        Expr::StringLit(s) => string_lit::eval_string_lit(s),
        Expr::Identifier(name) => identifier::eval_identifier(env, name),
        Expr::List(exprs) => list::eval_list(env, exprs),
        Expr::UnaryOp { op, right } => unary_op::eval_unary_op(env, op, right),
        Expr::BinaryOp { left, op, right } => binary_op::eval_binary_op(env, left, op, right),
        Expr::Grouping(e) => grouping::eval_grouping(env, e),
        Expr::Call { callee, args } => call::eval_call(env, callee, args),
    }
}
