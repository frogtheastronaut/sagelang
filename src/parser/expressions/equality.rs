use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn equality(&mut self) -> Expr {
        let mut node = self.comparison();

        while matches!(self.current.token, Token::EqEq | Token::NotEq | Token::And | Token::Or) {
            let line = self.current.line;
            let op = self.current.token.clone();
            self.advance();
            let right = self.comparison();
            node = Expr::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(right),
                line,
            };
        }

        node
    }
}
