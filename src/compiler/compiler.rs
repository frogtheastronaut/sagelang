use crate::parser::ast::{Expr, Stmt};
use crate::lexer::tokens::Token;
use crate::compiler::instruction::{Instruction, Bytecode};

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
            Stmt::VarDecl { name, value } => {
                self.compile_expr(value);
                self.bytecode.instructions.push(Instruction::Store(name.clone()));
            }
            Stmt::Assign { name, value } => {
                self.compile_expr(value);
                self.bytecode.instructions.push(Instruction::Store(name.clone()));
            }
            Stmt::ExprStmt(expr) => self.compile_expr(expr),
            Stmt::Print(expr) => {
                self.compile_expr(expr);
                self.bytecode.instructions.push(Instruction::Print);
            }
            Stmt::Block(stmts) => self.compile_stmts(stmts),
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.compile_expr(expr);
                }
                self.bytecode.instructions.push(Instruction::Return);
            }
            Stmt::If { condition, then_branch, else_branch, elseif_branches } => {
                // compile condition
                self.compile_expr(condition);

                // insert JumpIfFalse to else/elseif/after
                let jump_if_false_pos = self.bytecode.instructions.len();
                self.bytecode.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder, will patch

                // compile then branch
                self.compile_stmts(then_branch);
                let after_then_jump_pos = self.bytecode.instructions.len();
                self.bytecode.instructions.push(Instruction::Jump(0)); // Placeholder, will patch

                // patch JumpIfFalse to start of else/elseif
                let elseif_start = self.bytecode.instructions.len();
                self.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(elseif_start);

                // compile elseif branches
                for (cond, branch) in elseif_branches {
                    self.compile_expr(cond);
                    let elseif_jump_if_false = self.bytecode.instructions.len();
                    self.bytecode.instructions.push(Instruction::JumpIfFalse(0));
                    self.compile_stmts(branch);
                    let elseif_after_jump = self.bytecode.instructions.len();
                    self.bytecode.instructions.push(Instruction::Jump(0));
                    let elseif_next = self.bytecode.instructions.len();
                    self.bytecode.instructions[elseif_jump_if_false] = Instruction::JumpIfFalse(elseif_next);
                    self.bytecode.instructions[elseif_after_jump] = Instruction::Jump(self.bytecode.instructions.len());
                }

                // compile else branch
                if let Some(else_stmts) = else_branch {
                    self.compile_stmts(else_stmts);
                }
                // patch after then branch jump
                self.bytecode.instructions[after_then_jump_pos] = Instruction::Jump(self.bytecode.instructions.len());
            }
            Stmt::While { condition, body } => {
                let cond_pos = self.bytecode.instructions.len();

                // condition
                self.compile_expr(condition);

                let jump_if_false_pos = self.bytecode.instructions.len();
                self.bytecode.instructions.push(Instruction::JumpIfFalse(0));

                // loop body
                self.compile_stmts(body);

                // jump back to condition
                self.bytecode.instructions.push(Instruction::Jump(cond_pos));

                // patch jump target
                let after_body = self.bytecode.instructions.len();
                self.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(after_body);
            }
            Stmt::For { var, iterable, body } => {
                // handle .. operator for ranges
                if let Expr::BinaryOp { left, op, right } = iterable {
                    if *op == Token::DotDot {
                        // range: for (i in a .. b)
                        self.compile_expr(left);
                        self.bytecode.instructions.push(Instruction::Store(var.clone()));
                        let cond_pos = self.bytecode.instructions.len();
                        self.bytecode.instructions.push(Instruction::Load(var.clone()));
                        self.compile_expr(right);
                        self.bytecode.instructions.push(Instruction::Less);
                        let jump_if_false_pos = self.bytecode.instructions.len();
                        self.bytecode.instructions.push(Instruction::JumpIfFalse(0));
                        self.compile_stmts(body);
                        self.bytecode.instructions.push(Instruction::Load(var.clone()));
                        self.bytecode.instructions.push(Instruction::Const(1.0));
                        self.bytecode.instructions.push(Instruction::Add);
                        self.bytecode.instructions.push(Instruction::Store(var.clone()));
                        self.bytecode.instructions.push(Instruction::Jump(cond_pos));
                        let after_body = self.bytecode.instructions.len();
                        self.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(after_body);
                        return;
                    }
                }
                // normal iterable
                self.bytecode.instructions.push(Instruction::Const(0.0));
                self.bytecode.instructions.push(Instruction::Store(var.clone()));
                let cond_pos = self.bytecode.instructions.len();
                self.bytecode.instructions.push(Instruction::Load(var.clone()));
                self.compile_expr(iterable);
                self.bytecode.instructions.push(Instruction::Less);
                let jump_if_false_pos = self.bytecode.instructions.len();
                self.bytecode.instructions.push(Instruction::JumpIfFalse(0));
                self.compile_stmts(body);
                self.bytecode.instructions.push(Instruction::Load(var.clone()));
                self.bytecode.instructions.push(Instruction::Const(1.0));
                self.bytecode.instructions.push(Instruction::Add);
                self.bytecode.instructions.push(Instruction::Store(var.clone()));
                self.bytecode.instructions.push(Instruction::Jump(cond_pos));
                let after_body = self.bytecode.instructions.len();
                self.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(after_body);
            }
            Stmt::Function { name, params, body } => {
                let mut func_compiler = Compiler { bytecode: Bytecode { instructions: vec![] } };
                func_compiler.compile_stmts(body);
                let param_names = params.iter().map(|p| p.param_name.clone()).collect();
                self.bytecode.instructions.push(Instruction::DefFunc {
                    name: name.clone(),
                    params: param_names,
                    body: func_compiler.bytecode.instructions,
                });
            }
        }
    }

    pub fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => self.bytecode.instructions.push(Instruction::Const(*n)),
            Expr::Bool(b) => self.bytecode.instructions.push(Instruction::PushBool(*b)),
            Expr::Identifier(name) => self.bytecode.instructions.push(Instruction::Load(name.clone())),
            Expr::BinaryOp { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    Token::Plus => self.bytecode.instructions.push(Instruction::Add),
                    Token::Minus => self.bytecode.instructions.push(Instruction::Sub),
                    Token::Star => self.bytecode.instructions.push(Instruction::Mul),
                    Token::Slash => self.bytecode.instructions.push(Instruction::Div),
                    Token::Percent => self.bytecode.instructions.push(Instruction::Modulo),
                    Token::EqEq => self.bytecode.instructions.push(Instruction::EqEq),
                    Token::NotEq => self.bytecode.instructions.push(Instruction::NotEq),
                    Token::Less => self.bytecode.instructions.push(Instruction::Less),
                    Token::LessEq => self.bytecode.instructions.push(Instruction::LessEq),
                    Token::Greater => self.bytecode.instructions.push(Instruction::Greater),
                    Token::GreaterEq => self.bytecode.instructions.push(Instruction::GreaterEq),
                    Token::And => self.bytecode.instructions.push(Instruction::And),
                    Token::Or => self.bytecode.instructions.push(Instruction::Or),
                    _ => {}
                }
            }
            Expr::UnaryOp { op, right } => {
                self.compile_expr(right);
                match op {
                    Token::Minus => self.bytecode.instructions.push(Instruction::Sub),
                    Token::NotEq => self.bytecode.instructions.push(Instruction::NotEq),
                    _ => {}
                }
            }
            Expr::Grouping(expr) => self.compile_expr(expr),
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_expr(arg);
                }
                if let Expr::Identifier(name) = &**callee {
                    self.bytecode.instructions.push(Instruction::CallFunc(name.clone()));
                }
            }
            Expr::StringLit(s) => self.bytecode.instructions.push(Instruction::PushString(s.clone())),
            Expr::List(items) => {
                let mut vals = Vec::new();
                for item in items {
                    if let Expr::Number(n) = item {
                        vals.push(*n);
                    }
                }
                self.bytecode.instructions.push(Instruction::PushList(vals));
            }
        }
    }
}
