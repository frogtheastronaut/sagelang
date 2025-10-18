/*
 * parser for let statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token};
use crate::error::errormsg;

impl<'a> Parser<'a> {
    pub fn let_stmt(&mut self) -> Stmt {
        self.eat(Token::Let);

        let name = match &self.current.token {
            Token::Identifier(id) => id.clone(),
            _ => errormsg::parser_error("Expected identifier after let", self.current.line),
        };
        self.advance();

        self.eat(Token::Assign);

        let value = self.expr();

        self.eat(Token::Semicolon);

        Stmt::VarDecl { name, value }
    }
}
