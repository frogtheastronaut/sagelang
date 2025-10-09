/*
 * parser for return statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn return_stmt(&mut self) -> Stmt {
        self.eat(Token::Return);

        let value = if self.current != Token::Semicolon {
            Some(self.expr())
        } else {
            None
        };

        self.eat(Token::Semicolon);

        Stmt::Return(value)
    }
}
