/*
 * parser for while statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token};

impl<'a> Parser<'a> {
    pub fn while_stmt(&mut self) -> Stmt {
        let line = self.current.line;
        self.eat(Token::WhileKw);
        self.eat(Token::LParen);
        let condition = self.expr();
        self.eat(Token::RParen);
        let body = self.block_stmt();
        Stmt::While { condition, body, line }
    }
}
