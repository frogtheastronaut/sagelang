pub mod let_stmt;
pub mod function_stmt;
pub mod if_stmt;
pub mod return_stmt;
pub mod block_stmt;
pub mod while_stmt;
pub mod for_stmt;
pub mod print_stmt;
pub mod class_stmt;

use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token};
use crate::error::errormsg;

impl<'a> Parser<'a> {
    pub fn statement(&mut self) -> Stmt {
        match &self.current.token {
            Token::Let => self.let_stmt(),
            Token::Fn => self.function_stmt(),
            Token::If => self.if_stmt(),
            Token::Return => self.return_stmt(),
            Token::OpenBrace => {
                let line = self.current.line;
                Stmt::Block { stmts: self.block_stmt(), line }
            }
            Token::WhileKw => self.while_stmt(),
            Token::ForKw => self.for_stmt(),
            Token::PrintKw => self.print_stmt(),
            Token::ClassKw => {
                self.advance();
                self.class_declaration()
            }
            Token::Identifier(_) | Token::ThisKw => {
                // Parse the left side as an expression
                let expr = self.call();
                
                // Check if it's followed by assignment
                if matches!(self.current.token, Token::Assign) {
                    let line = self.current.line;
                    self.advance(); // consume '='
                    let value = self.expr();
                    self.eat(Token::Semicolon);
                    
                    // Check if it's property assignment or variable assignment
                    match expr {
                        crate::parser::ast::Expr::Get { object, name, .. } => {
                            // Convert Get to Set
                            Stmt::ExprStmt {
                                expr: crate::parser::ast::Expr::Set {
                                    object,
                                    name,
                                    value: Box::new(value),
                                    line,
                                },
                                line,
                            }
                        }
                        crate::parser::ast::Expr::Identifier { name, .. } => {
                            // Simple variable assignment
                            Stmt::Assign { name, value, line }
                        }
                        _ => errormsg::parser_error("Invalid assignment target", self.current.line),
                    }
                } else {
                    let line = expr.line();
                    // It's just an expression statement
                    self.eat(Token::Semicolon);
                    Stmt::ExprStmt { expr, line }
                }
            }
            _ => {
                let expr = self.expr();
                let line = expr.line();
                self.eat(Token::Semicolon);
                Stmt::ExprStmt { expr, line }
            }
        }
    }
}
