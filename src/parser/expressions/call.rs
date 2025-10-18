/*
 * parser for function calls
 */
use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn call(&mut self) -> Expr {
        let mut expr = match &self.current.token {
            Token::ThisKw => {
                self.advance();
                Expr::This
            }
            Token::SuperKw => {
                self.advance();
                self.eat(Token::Dot);
                if let Token::Identifier(method) = &self.current.token {
                    let method_name = method.clone();
                    self.advance();
                    Expr::Super { method: method_name }
                } else {
                    panic!("Expected method name after 'super.'");
                }
            }
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
            Token::List(items) => {
                // convert Vec<Token> to Vec<Expr>
                let expr_items = items.iter().map(|tok| match tok {
                    Token::Number(n) => Expr::Number(*n),
                    Token::StringLit(s) => Expr::StringLit(s.clone()),
                    Token::Bool(b) => Expr::Bool(*b),
                    Token::Identifier(id) => Expr::Identifier(id.clone()),
                    _ => panic!("[ERR] Unsupported list element: {:?}", tok),
                }).collect();
                self.advance();
                Expr::List(expr_items)
            }
            Token::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while self.current.token != Token::RBracket && self.current.token != Token::EOF {
                    items.push(self.expr());
                    if self.current.token == Token::Comma {
                        self.advance();
                    }
                }
                self.eat(Token::RBracket);
                Expr::List(items)
            }
            Token::ElseIfKw | Token::If | Token::Let | Token::Fn | Token::Return | Token::WhileKw | Token::ForKw | Token::PrintKw | Token::Else => {
                panic!("[ERR] Unexpected statement keyword in expression: {:?} at line {}", self.current.token, self.current.line)
            }
            _ => panic!("[ERR] Unexpected token in call: {:?} at line {}", self.current.token, self.current.line),
        };

        loop {
            match &self.current.token {
                Token::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if self.current.token != Token::RParen {
                        args.push(self.expr());
                        while self.current.token == Token::Comma {
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
                Token::Dot => {
                    self.advance();
                    if let Token::Identifier(name) = &self.current.token {
                        let prop_name = name.clone();
                        self.advance();
                        expr = Expr::Get {
                            object: Box::new(expr),
                            name: prop_name,
                        };
                    } else {
                        panic!("Expected property name after '.'");
                    }
                }
                _ => break,
            }
        }

        expr
    }
}
