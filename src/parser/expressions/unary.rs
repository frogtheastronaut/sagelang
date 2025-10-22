use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn unary(&mut self) -> Expr {
        if matches!(self.current.token, Token::Plus | Token::Minus) {
            let line = self.current.line;
            let op = self.current.token.clone();
            self.advance();
            let right = self.unary();
            return Expr::UnaryOp {
                op,
                right: Box::new(right),
                line,
            };
        }
        self.call()
    }
}
