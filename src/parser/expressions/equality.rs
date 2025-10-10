/*
 * Parser for == and != expressions
 */
use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn equality(&mut self) -> Expr {
        let mut node = self.comparison();

        while matches!(self.current, Token::EqEq | Token::NotEq | Token::And | Token::Or) {
            let op = self.current.clone();
            self.advance();
            let right = self.comparison();
            node = Expr::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }

        node
    }
}
