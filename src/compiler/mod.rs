mod statements;
mod expressions;

use crate::parser::ast::{Expr, Stmt};
use crate::vm::{Chunk, OpCode};
use std::collections::HashMap;

pub struct Compiler {
    pub chunk: Chunk,
    pub locals: HashMap<String, usize>,
    pub local_count: usize,
    pub scope_depth: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new("main".to_string()),
            locals: HashMap::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }
    
    pub fn compile(&mut self, stmts: &[Stmt]) -> Result<Chunk, String> {
        for stmt in stmts {
            self.compile_stmt(stmt)?;
        }
        
        Ok(self.chunk.clone())
    }
    
    pub fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::VarDecl { name, value } => self.compile_var_decl(name, value),
            Stmt::Assign { name, value } => self.compile_assign(name, value),
            Stmt::Print(expr) => self.compile_print(expr),
            Stmt::ExprStmt(expr) => self.compile_expr_stmt(expr),
            Stmt::Block(stmts) => self.compile_block(stmts),
            Stmt::If { condition, then_branch, else_branch, elseif_branches } => {
                self.compile_if_stmt(condition, then_branch, else_branch, elseif_branches)
            }
            Stmt::While { condition, body } => self.compile_while_stmt(condition, body),
            Stmt::For { var, iterable, body } => self.compile_for_stmt(var, iterable, body),
            Stmt::Function { name, params, body } => self.compile_function_stmt(name, params, body),
            Stmt::Return(expr) => self.compile_return_stmt(expr),
        }
    }
    
    pub fn compile_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Number(n) => self.compile_number(*n),
            Expr::Bool(b) => self.compile_bool(*b),
            Expr::StringLit(s) => self.compile_string_lit(s),
            Expr::Identifier(name) => self.compile_identifier(name),
            Expr::List(items) => self.compile_list(items),
            Expr::UnaryOp { op, right } => self.compile_unary_op(op, right),
            Expr::BinaryOp { left, op, right } => self.compile_binary_op(left, op, right),
            Expr::Grouping(expr) => self.compile_grouping(expr),
            Expr::Call { callee, args } => self.compile_call(callee, args),
        }
    }
    
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    
    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        
        // Remove locals that went out of scope
        let mut to_remove = Vec::new();
        for (name, &idx) in &self.locals {
            if idx >= self.local_count {
                to_remove.push(name.clone());
            }
        }
        for name in to_remove {
            self.locals.remove(&name);
        }
    }
    
    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.chunk.write(instruction);
        self.chunk.code.len() - 1
    }
    
    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len();
        
        match &mut self.chunk.code[offset] {
            OpCode::Jump(addr) |
            OpCode::JumpIfFalse(addr) |
            OpCode::JumpIfTrue(addr) => {
                *addr = jump;
            }
            _ => {}
        }
    }
}
