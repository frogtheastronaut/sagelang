/*
 * parser for for-in statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn for_stmt(&mut self) -> Stmt {
        self.eat(Token::ForKw);
        self.eat(Token::LParen);
        let var = match &self.current.token {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected variable name in for loop at line {}", self.current.line),
        };
        self.advance();
        self.eat(Token::InKw);
        let iterable = self.expr();
        self.eat(Token::RParen);
        let body = self.block_stmt();
        Stmt::For { var, iterable, body }
    }
}
