/*
 * parser for for-in statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token};
use crate::error::errormsg;

// for statements like:
// for (i in 1 .. 10) {  }, or
// for (i in list) { }
impl<'a> Parser<'a> {
    pub fn for_stmt(&mut self) -> Stmt {
        let line = self.current.line;
        // eat for 
        self.eat(Token::ForKw);
        // eat (
        self.eat(Token::LParen);
        // this is the variable
        let var = match &self.current.token {
            Token::Identifier(id) => id.clone(),
            _ => errormsg::parser_error("Expected variable name in for loop", self.current.line),
        };
        self.advance();
        // eat in
        self.eat(Token::InKw);
        // this is the iterable
        let iterable = self.expr();
        // eat )
        self.eat(Token::RParen);
        // this is the block {}
        let body = self.block_stmt();

        // return for loop
        Stmt::For { var, iterable, body, line }
    }
}
