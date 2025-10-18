/*
 * Parser for class declarations
 */
use crate::parser::Parser;
use crate::parser::ast::{Stmt, Method, Param, Field, AccessModifier};
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn class_declaration(&mut self) -> Stmt {
        // class keyword already consumed
        
        // Get class name
        let name = if let Token::Identifier(n) = &self.current.token {
            let name = n.clone();
            self.advance();
            name
        } else {
            panic!("Expected class name");
        };
        
        // Check for superclass
        let superclass = if matches!(self.current.token, Token::Less) {
            self.advance(); // consume '<'
            if let Token::Identifier(super_name) = &self.current.token {
                let super_name = super_name.clone();
                self.advance();
                Some(super_name)
            } else {
                panic!("Expected superclass name");
            }
        } else {
            None
        };
        
        // Expect opening brace
        if !matches!(self.current.token, Token::OpenBrace) {
            panic!("Expected '{{' after class name");
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
            let method_name = if let Token::Identifier(n) = &self.current.token {
                let name = n.clone();
                self.advance();
                name
            } else {
                panic!("Expected method name");
            };
            
            // Expect opening paren
            if !matches!(self.current.token, Token::LParen) {
                panic!("Expected '(' after method name");
            }
            self.advance();
            
            // Parse parameters
            let mut params = Vec::new();
            while !matches!(self.current.token, Token::RParen) {
                // Get parameter type
                let param_type = if let Token::Identifier(t) = &self.current.token {
                    let type_name = t.clone();
                    self.advance();
                    type_name
                } else {
                    panic!("Expected parameter type");
                };
                
                // Get parameter name
                let param_name = if let Token::Identifier(n) = &self.current.token {
                    let name = n.clone();
                    self.advance();
                    name
                } else {
                    panic!("Expected parameter name");
                };
                
                params.push(Param { param_name, param_type });
                
                if matches!(self.current.token, Token::Comma) {
                    self.advance();
                }
            }
            
            // Consume closing paren
            if !matches!(self.current.token, Token::RParen) {
                panic!("Expected ')' after parameters");
            }
            self.advance();
            
            // Parse method body
            if !matches!(self.current.token, Token::OpenBrace) {
                panic!("Expected '{{' before method body");
            }
            self.advance();
            
            let mut body = Vec::new();
            while !matches!(self.current.token, Token::CloseBrace) && !matches!(self.current.token, Token::EOF) {
                body.push(self.statement());
            }
            
            // Consume closing brace
            if !matches!(self.current.token, Token::CloseBrace) {
                panic!("Expected '}}' after method body");
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
                    panic!("Expected ';' after field declaration");
                }
                self.advance();
                
                fields.push(Field {
                    name: field_name,
                    access,
                });
            } else {
                panic!("Expected 'function' or field name in class body");
            }
        }
        
        // Consume closing brace
        if !matches!(self.current.token, Token::CloseBrace) {
            panic!("Expected '}}' after class body");
        }
        self.advance();
        
        Stmt::Class {
            name,
            superclass,
            fields,
            methods,
        }
    }
}
