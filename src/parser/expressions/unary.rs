/*
 * parser for unary expressions.
 */
use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn unary(&mut self) -> Expr {
        if matches!(self.current, Token::Plus | Token::Minus) {
            let op = self.current.clone();
            self.advance();
            let right = self.unary();
            return Expr::UnaryOp {
                op,
                right: Box::new(right),
            };
        }
        self.call()
    }
}
