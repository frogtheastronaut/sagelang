
use crate::parser::ast::{Expr, Stmt};
use crate::compiler::instruction::Bytecode;
use crate::compiler::statements::{
    var_decl,
    assign,
    expr_stmt,
    print,
    block,
    return_stmt,
    if_stmt,
    while_stmt,
    for_stmt,
    function_stmt,
};
use crate::compiler::expressions::{
    number,
    bool as bool_lit,
    identifier,
    binary_op,
    unary_op,
    grouping,
    call,
    string_lit,
    list,
};

pub struct Compiler {
    pub bytecode: Bytecode,
}

impl Compiler {
    pub fn compile_stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
    }

    pub fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl { name, value } => var_decl::var_decl(self, name, value),
            Stmt::Assign { name, value } => assign::assign(self, name, value),
            Stmt::ExprStmt(expr) => expr_stmt::expr_stmt(self, expr),
            Stmt::Print(expr) => print::print(self, expr),
            Stmt::Block(stmts) => block::block(self, stmts),
            Stmt::Return(opt_expr) => return_stmt::return_stmt(self, opt_expr),
            Stmt::If { condition, then_branch, else_branch, elseif_branches } => if_stmt::if_stmt(self, condition, then_branch, else_branch, elseif_branches),
            Stmt::While { condition, body } => while_stmt::while_stmt(self, condition, body),
            Stmt::For { var, iterable, body } => for_stmt::for_stmt(self, var, iterable, body),
            Stmt::Function { name, params, body } => function_stmt::function_stmt(self, name, params, body),
        }
    }

    pub fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => number::number(self, *n),
            Expr::Bool(b) => bool_lit::bool_lit(self, *b),
            Expr::Identifier(name) => identifier::identifier(self, name),
            Expr::BinaryOp { left, op, right } => binary_op::binary_op(self, left, op, right),
            Expr::UnaryOp { op, right } => unary_op::unary_op(self, op, right),
            Expr::Grouping(expr) => grouping::grouping(self, expr),
            Expr::Call { callee, args } => call::call(self, callee, args),
            Expr::StringLit(s) => string_lit::string_lit(self, s),
            Expr::List(items) => list::list(self, items),
        }
    }
}
