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
                // accept type keyword before parameter name
                match &self.current {
                    Token::NumKw | Token::BoolKw | Token::StrKw | Token::ListKw => {
                        self.advance(); // skip type
                        match &self.current {
                            Token::Identifier(id) => params.push(id.clone()),
                            _ => panic!("Expected identifier after type in function parameters"),
                        }
                        self.advance();
                    }
                    Token::Identifier(id) => {
                        params.push(id.clone());
                        self.advance();
                    }
                    _ => panic!("Expected identifier in function parameters"),
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
