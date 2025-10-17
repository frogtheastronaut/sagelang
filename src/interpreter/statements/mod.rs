pub mod var_decl;
pub mod assign;
pub mod print;
pub mod expr_stmt;
pub mod block;
pub mod if_stmt;
pub mod while_stmt;
pub mod for_stmt;
pub mod function_stmt;
pub mod return_stmt;

use crate::parser::ast::{Stmt};
use crate::interpreter::{Env, Value};

pub fn eval_stmt(env: &mut Env, stmt: &Stmt) -> Option<Value> {
    let result = match stmt {
        Stmt::VarDecl { name, value } => var_decl::eval_var_decl(env, name, value),
        Stmt::Assign { name, value } => assign::eval_assign(env, name, value),
        Stmt::Print(expr) => print::eval_print(env, expr),
        Stmt::ExprStmt(expr) => expr_stmt::eval_expr_stmt(env, expr),
        Stmt::Block(stmts) => {
            for s in stmts {
                let val = eval_stmt(env, s);
                if let Some(Value::Return(_)) = val {
                    return val;
                }
            }
            Value::Null
        },
        Stmt::If { condition, then_branch, else_branch, elseif_branches } => {
            let val = if_stmt::eval_if_stmt(env, condition, then_branch, else_branch, elseif_branches);
            if let Value::Return(_) = val {
                return Some(val);
            }
            val
        },
        Stmt::While { condition, body } => {
            let val = while_stmt::eval_while_stmt(env, condition, body);
            if let Value::Return(_) = val {
                return Some(val);
            }
            val
        },
        Stmt::For { var, iterable, body } => {
            let val = for_stmt::eval_for_stmt(env, var, iterable, body);
            if let Value::Return(_) = val {
                return Some(val);
            }
            val
        },
        Stmt::Function { name, params, body } => function_stmt::eval_function_stmt(env, name, params, body),
        Stmt::Return(value) => return_stmt::eval_return_stmt(env, value),
    };
    Some(result)
}