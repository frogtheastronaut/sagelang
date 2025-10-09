/*
 * Abstract Syntax Tree
*/
use crate::lexer::tokens::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    UnaryOp {
        op: Token,
        right: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl {
        name: String,
        value: Expr,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    Return(Option<Expr>),
    ExprStmt(Expr),
    Block(Vec<Stmt>),
}
