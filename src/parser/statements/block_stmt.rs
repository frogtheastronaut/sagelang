/*
 * parser for block {} statements
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn block_stmt(&mut self) -> Vec<Stmt> {
        self.eat(Token::OpenBrace);

        let mut stmts = Vec::new();
        while self.current != Token::CloseBrace && self.current != Token::EOF {
            // skip any extra semicolons between statements
            while self.current == Token::Semicolon {
                self.advance();
            }
            if self.current == Token::CloseBrace || self.current == Token::EOF {
                break;
            }
            stmts.push(self.statement());
        }

        self.eat(Token::CloseBrace);
        stmts
    }
}
