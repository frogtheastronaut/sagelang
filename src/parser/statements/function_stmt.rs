/*
 * parser for function statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token};

impl<'a> Parser<'a> {
    pub fn function_stmt(&mut self) -> Stmt {
        // eat function
        self.eat(Token::Fn);

        // this is the function name
        let name = match &self.current.token {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected function name at line {}", self.current.line),
        };
        self.advance();

        // eat (
        self.eat(Token::LParen);

        // this is the parameter list
        let mut params = Vec::new();
        if self.current.token != Token::RParen {
            loop {
                let param_type = match &self.current.token {
                    Token::NumKw => "num",
                    Token::BoolKw => "bool",
                    Token::StrKw => "str",
                    Token::ListKw => "list",
                    _ => "",
                };
                if param_type != "" {
                    self.advance(); // skip type
                    match &self.current.token {
                        Token::Identifier(id) => {
                            params.push(crate::parser::ast::Param { param_name: id.clone(), param_type: param_type.to_string() });
                        }
                        _ => panic!("Expected identifier after type in function parameters at line {}", self.current.line),
                    }
                    self.advance();
                } else {
                    match &self.current.token {
                        Token::Identifier(id) => {
                            params.push(crate::parser::ast::Param { param_name: id.clone(), param_type: String::new() });
                        }
                        _ => panic!("Expected identifier in function parameters at line {}", self.current.line),
                    }
                    self.advance();
                }

                if self.current.token == Token::Comma {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            }
        }
        // eat )
        self.eat(Token::RParen);

        // this is the block {}
        let body = self.block_stmt();

        // return function
        Stmt::Function { name, params, body }
    }
}
