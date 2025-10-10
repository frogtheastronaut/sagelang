/*
 * Abstract Syntax Tree
*/
use crate::lexer::tokens::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Param {
    pub param_name: String,
    pub param_type: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expr {
    Number(f64),

    Identifier(String),

    StringLit(String),

    Bool(bool),

    List(Vec<Expr>),

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
#[allow(dead_code)]
pub enum Stmt {
    VarDecl {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<Param>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        elseif_branches: Vec<(Expr, Vec<Stmt>)>,
    },
    Print(Expr),
    Return(Option<Expr>),
    ExprStmt(Expr),
    Block(Vec<Stmt>),
}
