/*
 * parser for + and -
 */
use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn term(&mut self) -> Expr {
        let mut node = self.factor();

        while matches!(self.current, Token::Plus | Token::Minus | Token::DotDot) {
            let op = self.current.clone();
            self.advance();
            let right = self.factor();
            node = Expr::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }

        node
    }
}
