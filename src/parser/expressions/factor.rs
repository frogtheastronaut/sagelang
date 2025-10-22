use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn factor(&mut self) -> Expr {
        let mut node = self.unary();

        while matches!(self.current.token, Token::Star | Token::Slash | Token::Percent) {
            let line = self.current.line;
            let op = self.current.token.clone();
            self.advance();
            let right = self.unary();
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
