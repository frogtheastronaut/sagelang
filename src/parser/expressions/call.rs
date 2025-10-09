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
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Expr::Number(num)
            }
            Token::StringLit(s) => {
                let val = s.clone();
                self.advance();
                Expr::StringLit(val)
            }
            Token::Bool(b) => {
                let val = *b;
                self.advance();
                Expr::Bool(val)
            }
            Token::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while self.current != Token::RBracket && self.current != Token::EOF {
                    items.push(self.expr());
                    if self.current == Token::Comma {
                        self.advance();
                    }
                }
                self.eat(Token::RBracket);
                Expr::List(items)
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
