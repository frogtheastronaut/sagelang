/*
 * Parser for function calls
 */
use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn call(&mut self) -> Expr {
        let mut expr = match &self.current {
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Expr::Identifier(id)
            }
            Token::LParen => self.grouping(),
            Token::Number(_) => {
                let num = match &self.current {
                    Token::Number(n) => *n,
                    _ => unreachable!(),
                };
                self.advance();
                Expr::Number(num)
            }
            _ => panic!("Unexpected token in call: {:?}", self.current),
        };

        while self.current == Token::LParen {
            self.advance();
            let mut args = Vec::new();
            if self.current != Token::RParen {
                args.push(self.expr());
                while self.current == Token::Comma {
                    self.advance();
                    args.push(self.expr());
                }
            }
            self.eat(Token::RParen);
            expr = Expr::Call {
                callee: Box::new(expr),
                args,
            };
        }

        expr
    }
}
