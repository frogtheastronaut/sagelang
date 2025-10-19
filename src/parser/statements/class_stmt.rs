/*
 * Parser for class declarations
 */
use crate::parser::Parser;
use crate::parser::ast::{Stmt, Method, Param, Field, AccessModifier};
use crate::lexer::tokens::Token;
use crate::error::errormsg;

impl<'a> Parser<'a> {
    pub fn class_declaration(&mut self) -> Stmt {
        let line = self.current.line;
        // class keyword already consumed
        
        // Get class name
        let name = if let Token::Identifier(n) = &self.current.token {
            let name = n.clone();
            self.advance();
            name
        } else {
            errormsg::parser_error("Expected class name", self.current.line);
        };
        
        // Check for superclass
        let superclass = if matches!(self.current.token, Token::Less) {
            self.advance(); // consume '<'
            if let Token::Identifier(super_name) = &self.current.token {
                let super_name = super_name.clone();
                self.advance();
                Some(super_name)
            } else {
                errormsg::parser_error("Expected superclass name", self.current.line);
            }
        } else {
            None
        };
        
        // Expect opening brace
        if !matches!(self.current.token, Token::OpenBrace) {
            errormsg::parser_error("Expected '{' after class name", self.current.line);
        }
        self.advance();
        
        // Parse fields and methods
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        
        while !matches!(self.current.token, Token::CloseBrace) && !matches!(self.current.token, Token::EOF) {
            // Check for access modifier (default is public)
            let access = if matches!(self.current.token, Token::PrivateKw) {
                self.advance();
                AccessModifier::Private
            } else if matches!(self.current.token, Token::ProtectedKw) {
                self.advance();
                AccessModifier::Protected
            } else if matches!(self.current.token, Token::PublicKw) {
                self.advance();
                AccessModifier::Public
            } else {
                AccessModifier::Public // Default
            };
            
            // Check for static
            let is_static = if matches!(self.current.token, Token::StaticKw) {
                self.advance();
                true
            } else {
                false
            };
            
            // Check if it's a field or method
            if matches!(self.current.token, Token::Fn) {
                // It's a method
                self.advance();
            
            // Get method name
            let method_name = match &self.current.token {
                Token::Identifier(n) => {
                    let name = n.clone();
                    self.advance();
                    name
                }
                Token::PrintKw | Token::Return | Token::StrKw | Token::NumKw | 
                Token::BoolKw | Token::ListKw | Token::StaticKw | Token::If | 
                Token::Else | Token::ElseIfKw | Token::WhileKw | Token::ForKw |
                Token::Let | Token::Fn | Token::InKw | Token::ClassKw | 
                Token::ThisKw | Token::SuperKw | Token::NewKw | Token::PrivateKw |
                Token::PublicKw | Token::ProtectedKw => {
                    errormsg::parser_error(
                        &format!("Cannot use keyword '{:?}' as method name", self.current.token),
                        self.current.line
                    );
                }
                _ => {
                    errormsg::parser_error("Expected method name", self.current.line);
                }
            };
            
            // Expect opening paren
            if !matches!(self.current.token, Token::LParen) {
                errormsg::parser_error("Expected '(' after method name", self.current.line);
            }
            self.advance();
            
            // Parse parameters (without type annotations)
            let mut params = Vec::new();
            while !matches!(self.current.token, Token::RParen) {
                // Get parameter name directly
                let param_name = if let Token::Identifier(n) = &self.current.token {
                    let name = n.clone();
                    self.advance();
                    name
                } else {
                    errormsg::parser_error("Expected parameter name", self.current.line);
                };
                
                // Type is no longer required - use empty string as placeholder
                let param_type = String::new();
                
                params.push(Param { param_name, param_type });
                
                if matches!(self.current.token, Token::Comma) {
                    self.advance();
                }
            }
            
            // Consume closing paren
            if !matches!(self.current.token, Token::RParen) {
                errormsg::parser_error("Expected ')' after parameters", self.current.line);
            }
            self.advance();
            
            // Parse method body
            if !matches!(self.current.token, Token::OpenBrace) {
                errormsg::parser_error("Expected '{' before method body", self.current.line);
            }
            self.advance();
            
            let mut body = Vec::new();
            while !matches!(self.current.token, Token::CloseBrace) && !matches!(self.current.token, Token::EOF) {
                body.push(self.statement());
            }
            
            // Consume closing brace
            if !matches!(self.current.token, Token::CloseBrace) {
                errormsg::parser_error("Expected '}' after method body", self.current.line);
            }
            self.advance();
            
            methods.push(Method {
                name: method_name,
                params,
                body,
                is_static,
                access,
                });
            } else if let Token::Identifier(field_name) = &self.current.token {
                // It's a field declaration
                let field_name = field_name.clone();
                self.advance();
                
                // Expect semicolon
                if !matches!(self.current.token, Token::Semicolon) {
                    errormsg::parser_error("Expected ';' after field declaration", self.current.line);
                }
                self.advance();
                
                fields.push(Field {
                    name: field_name,
                    access,
                });
            } else {
                errormsg::parser_error("Expected 'function' or field name in class body", self.current.line);
            }
        }
        
        // Consume closing brace
        if !matches!(self.current.token, Token::CloseBrace) {
            errormsg::parser_error("Expected '}' after class body", self.current.line);
        }
        self.advance();
        
        Stmt::Class {
            name,
            superclass,
            fields,
            methods,
            line,
        }
    }
}
