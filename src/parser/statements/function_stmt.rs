/*
 * parser for function statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn function_stmt(&mut self) -> Stmt {
        self.eat(Token::Fn);

        let name = match &self.current {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected function name"),
        };
        self.advance();

        self.eat(Token::LParen);

        let mut params = Vec::new();
        if self.current != Token::RParen {
            loop {
                let param_type = match &self.current {
                    Token::NumKw => "num",
                    Token::BoolKw => "bool",
                    Token::StrKw => "str",
                    Token::ListKw => "list",
                    _ => "",
                };
                if param_type != "" {
                    self.advance(); // skip type
                    match &self.current {
                        Token::Identifier(id) => {
                            params.push(crate::parser::ast::Param { param_name: id.clone(), param_type: param_type.to_string() });
                        }
                        _ => panic!("Expected identifier after type in function parameters"),
                    }
                    self.advance();
                } else {
                    match &self.current {
                        Token::Identifier(id) => {
                            params.push(crate::parser::ast::Param { param_name: id.clone(), param_type: String::new() });
                        }
                        _ => panic!("Expected identifier in function parameters"),
                    }
                    self.advance();
                }

                if self.current == Token::Comma {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            }
        }

        self.eat(Token::RParen);
        let body = self.block_stmt();

        Stmt::Function { name, params, body }
    }
}
