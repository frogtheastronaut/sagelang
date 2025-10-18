use crate::lexer::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
}

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
    Get {
        object: Box<Expr>,
        name: String,
    },
    Set {
        object: Box<Expr>,
        name: String,
        value: Box<Expr>,
    },
    This,
    Super {
        method: String,
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
    Class {
        name: String,
        superclass: Option<String>,
        fields: Vec<Field>,
        methods: Vec<Method>,
    },
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub is_static: bool,
    pub access: AccessModifier,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Field {
    pub name: String,
    pub access: AccessModifier,
}
