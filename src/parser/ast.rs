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
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expr {
    Number {
        value: f64,
        line: usize,
    },

    Identifier {
        name: String,
        line: usize,
    },

    StringLit {
        value: String,
        line: usize,
    },

    Bool {
        value: bool,
        line: usize,
    },

    List {
        items: Vec<Expr>,
        line: usize,
    },

    UnaryOp {
        op: Token,
        right: Box<Expr>,
        line: usize,
    },

    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
        line: usize,
    },
    Grouping {
        expr: Box<Expr>,
        line: usize,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        line: usize,
    },
    Get {
        object: Box<Expr>,
        name: String,
        line: usize,
    },
    Set {
        object: Box<Expr>,
        name: String,
        value: Box<Expr>,
        line: usize,
    },
    This {
        line: usize,
    },
    Super {
        method: String,
        line: usize,
    },
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Stmt {
    VarDecl {
        name: String,
        value: Expr,
        line: usize,
    },
    Assign {
        name: String,
        value: Expr,
        line: usize,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    Function {
        name: String,
        params: Vec<Param>,
        body: Vec<Stmt>,
        line: usize,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        elseif_branches: Vec<(Expr, Vec<Stmt>)>,
        line: usize,
    },
    Print {
        expr: Expr,
        line: usize,
    },
    Return {
        value: Option<Expr>,
        line: usize,
    },
    ExprStmt {
        expr: Expr,
        line: usize,
    },
    Block {
        stmts: Vec<Stmt>,
        line: usize,
    },
    Class {
        name: String,
        superclass: Option<String>,
        fields: Vec<Field>,
        methods: Vec<Method>,
        line: usize,
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

impl Expr {
    pub fn line(&self) -> usize {
        match self {
            Expr::Number { line, .. } => *line,
            Expr::Identifier { line, .. } => *line,
            Expr::StringLit { line, .. } => *line,
            Expr::Bool { line, .. } => *line,
            Expr::List { line, .. } => *line,
            Expr::UnaryOp { line, .. } => *line,
            Expr::BinaryOp { line, .. } => *line,
            Expr::Grouping { line, .. } => *line,
            Expr::Call { line, .. } => *line,
            Expr::Get { line, .. } => *line,
            Expr::Set { line, .. } => *line,
            Expr::This { line } => *line,
            Expr::Super { line, .. } => *line,
        }
    }
}

impl Stmt {
    pub fn line(&self) -> usize {
        match self {
            Stmt::VarDecl { line, .. } => *line,
            Stmt::Assign { line, .. } => *line,
            Stmt::While { line, .. } => *line,
            Stmt::For { line, .. } => *line,
            Stmt::Function { line, .. } => *line,
            Stmt::If { line, .. } => *line,
            Stmt::Print { line, .. } => *line,
            Stmt::Return { line, .. } => *line,
            Stmt::ExprStmt { line, .. } => *line,
            Stmt::Block { line, .. } => *line,
            Stmt::Class { line, .. } => *line,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Field {
    pub name: String,
    pub access: AccessModifier,
}
