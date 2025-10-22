/*
 * parser for function statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token};
use crate::error::errormsg;

impl<'a> Parser<'a> {
    pub fn function_stmt(&mut self) -> Stmt {
        let line = self.current.line;
        // eat function
        self.eat(Token::Fn);

        // this is the function name
        let name = match &self.current.token {
            Token::Identifier(id) => id.clone(),
            _ => errormsg::parser_error("Expected function name", self.current.line),
        };
        self.advance();

        // eat (
        self.eat(Token::LParen);

        // this is the parameter list
        let mut params = Vec::new();
        if self.current.token != Token::RParen {
            loop {
                match &self.current.token {
                    Token::Identifier(id) => {
                        params.push(crate::parser::ast::Param { param_name: id.clone() });
                        self.advance();
                    }
                    _ => errormsg::parser_error("Expected identifier in function parameters", self.current.line),
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
        Stmt::Function { name, params, body, line }
    }
}
