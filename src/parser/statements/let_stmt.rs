/*
 * parser for let statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn let_stmt(&mut self) -> Stmt {
        self.eat(Token::Let);

        let name = match &self.current {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected identifier after let"),
        };
        self.advance();

        self.eat(Token::Assign);

        let value = self.expr();

        self.eat(Token::Semicolon);

        Stmt::VarDecl { name, value }
    }
}
