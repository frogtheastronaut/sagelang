/*
 * parser for print statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn print_stmt(&mut self) -> Stmt {
        self.eat(Token::PrintKw);
        let expr = self.expr();
        self.eat(Token::Semicolon);
        Stmt::Print(expr)
    }
}
