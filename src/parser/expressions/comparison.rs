/*
 * Parser for comparison expressions (<, >, <=, >=)
 */
use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn comparison(&mut self) -> Expr {
        let mut node = self.term();

        while matches!(self.current, Token::Less | Token::LessEq | Token::Greater | Token::GreaterEq) {
            let op = self.current.clone();
            self.advance();
            let right = self.term();
            node = Expr::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }

        node
    }
}
