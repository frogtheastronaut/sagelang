/*
 * parser for if statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn if_stmt(&mut self) -> Stmt {
        self.eat(Token::If);

        let condition = self.expr();

        let then_branch = self.block_stmt();

        let else_branch = if self.current == Token::Else {
            self.advance();
            Some(self.block_stmt())
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }
}
