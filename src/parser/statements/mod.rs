pub mod let_stmt;
pub mod function_stmt;
pub mod if_stmt;
pub mod return_stmt;
pub mod block_stmt;
pub mod while_stmt;
pub mod for_stmt;

use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn statement(&mut self) -> Stmt {
        match &self.current {
            Token::Let => self.let_stmt(),
            Token::Fn => self.function_stmt(),
            Token::If => self.if_stmt(),
            Token::Return => self.return_stmt(),
            Token::OpenBrace => Stmt::Block(self.block_stmt()),
            Token::WhileKw => self.while_stmt(),
            Token::ForKw => self.for_stmt(),
            Token::Identifier(_) => {
                // assignment: identifier = expr;
                let name = match &self.current {
                    Token::Identifier(id) => id.clone(),
                    _ => unreachable!(),
                };
                self.advance();
                self.eat(Token::Assign);
                let value = self.expr();
                self.eat(Token::Semicolon);
                Stmt::Assign { name, value }
            }
            _ => Stmt::ExprStmt(self.expr()),
        }
    }
}
