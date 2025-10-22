use crate::parser::Parser;
use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;
use crate::error::errormsg;

impl<'a> Parser<'a> {
    pub fn call(&mut self) -> Expr {
        let mut expr = match &self.current.token {
            Token::NewKw => {
                self.advance();
                // expect class name
                if let Token::Identifier(class_name) = &self.current.token {
                    let name = class_name.clone();
                    let id_line = self.current.line;
                    self.advance();
                    Expr::Identifier { name, line: id_line }
                } else {
                    errormsg::parser_error("Expected class name after 'new'", self.current.line);
                }
            }
            Token::ThisKw => {
                let this_line = self.current.line;
                self.advance();
                Expr::This { line: this_line }
            }
            Token::SuperKw => {
                let super_line = self.current.line;
                self.advance();
                self.eat(Token::Dot);
                if let Token::Identifier(method) = &self.current.token {
                    let method_name = method.clone();
                    self.advance();
                    Expr::Super { method: method_name, line: super_line }
                } else {
                    errormsg::parser_error("Expected method name after 'super.'", self.current.line);
                }
            }
            Token::Identifier(name) => {
                let id = name.clone();
                let id_line = self.current.line;
                self.advance();
                Expr::Identifier { name: id, line: id_line }
            }
            Token::LParen => self.grouping(),
            Token::Number(n) => {
                let num = *n;
                let num_line = self.current.line;
                self.advance();
                Expr::Number { value: num, line: num_line }
            }
            Token::StringLit(s) => {
                let val = s.clone();
                let str_line = self.current.line;
                self.advance();
                Expr::StringLit { value: val, line: str_line }
            }
            Token::Bool(b) => {
                let val = *b;
                let bool_line = self.current.line;
                self.advance();
                Expr::Bool { value: val, line: bool_line }
            }
            Token::List(items) => {
                let list_line = self.current.line;
                // convert Vec<Token> to Vec<Expr>
                let expr_items = items.iter().map(|tok| match tok {
                    Token::Number(n) => Expr::Number { value: *n, line: list_line },
                    Token::StringLit(s) => Expr::StringLit { value: s.clone(), line: list_line },
                    Token::Bool(b) => Expr::Bool { value: *b, line: list_line },
                    Token::Identifier(id) => Expr::Identifier { name: id.clone(), line: list_line },
                    _ => {
                        errormsg::parser_error(&format!("Unsupported list element: {:?}", tok), 0);
                    }
                }).collect();
                self.advance();
                Expr::List { items: expr_items, line: list_line }
            }
            Token::LBracket => {
                let bracket_line = self.current.line;
                self.advance();
                let mut items = Vec::new();
                while self.current.token != Token::RBracket && self.current.token != Token::EOF {
                    items.push(self.expr());
                    if self.current.token == Token::Comma {
                        self.advance();
                    }
                }
                self.eat(Token::RBracket);
                Expr::List { items, line: bracket_line }
            }
            Token::ElseIfKw | Token::If | Token::Let | Token::Fn | Token::Return | Token::WhileKw | Token::ForKw | Token::PrintKw | Token::Else => {
                errormsg::parser_error(
                    &format!("Unexpected statement keyword in expression: {:?}", self.current.token),
                    self.current.line
                );
            }
            _ => {
                errormsg::parser_error(
                    &format!("Unexpected token in call: {:?}", self.current.token),
                    self.current.line
                );
            }
        };

        loop {
            match &self.current.token {
                Token::LParen => {
                    let line = self.current.line;
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
                        line,
                    };
                }
                Token::Dot => {
                    let line = self.current.line;
                    self.advance();
                    if let Token::Identifier(name) = &self.current.token {
                        let prop_name = name.clone();
                        self.advance();
                        expr = Expr::Get {
                            object: Box::new(expr),
                            name: prop_name,
                            line,
                        };
                    } else {
                        errormsg::parser_error("Expected property name after '.'", self.current.line);
                    }
                }
                _ => break,
            }
        }

        expr
    }
}
