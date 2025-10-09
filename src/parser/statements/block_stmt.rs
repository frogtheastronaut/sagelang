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
            stmts.push(self.statement());
        }

        self.eat(Token::CloseBrace);
        stmts
    }
}
