/*
 * parser for block {} statements
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token};

impl<'a> Parser<'a> {
    pub fn block_stmt(&mut self) -> Vec<Stmt> {
        // eat {
        self.eat(Token::OpenBrace);

        // parse statements until we hit }
        let mut stmts = Vec::new();
        while self.current.token != Token::CloseBrace && self.current.token != Token::EOF {
            // skip any extra semicolons between statements
            while self.current.token == Token::Semicolon {
                self.advance();
            }
            if self.current.token == Token::CloseBrace || self.current.token == Token::EOF {
                break;
            }
            stmts.push(self.statement());
        }
        // eat }
        self.eat(Token::CloseBrace);

        // return the parsed statements
        stmts
    }
}
