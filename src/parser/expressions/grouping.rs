/*
 * parser for grouping () expressions
 */
use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn grouping(&mut self) -> Expr {
        let line = self.current.line;
        self.eat(Token::LParen);
        let expr = self.expr();
        self.eat(Token::RParen);
        Expr::Grouping { expr: Box::new(expr), line }
    }
}
