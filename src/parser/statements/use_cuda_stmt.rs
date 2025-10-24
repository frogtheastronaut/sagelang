use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;
use crate::error::errormsg;

impl<'a> Parser<'a> {
    pub fn use_cuda_stmt(&mut self) -> Stmt {
        let line = self.current.line;
        self.advance();
        
        if !matches!(self.current.token, Token::OpenBrace) {
            errormsg::parser_error("Expected '{' after use_cuda", self.current.line);
        }
        self.advance();
        
        let mut kernel_code = String::new();
        
        // Check if first thing is a string literal (raw CUDA code)
        if let Token::StringLit(s) = &self.current.token {
            kernel_code = s.clone();
            self.advance();
            
            if !matches!(self.current.token, Token::Semicolon) {
                errormsg::parser_error("Expected ';' after kernel code string", self.current.line);
            }
            self.advance();
        }
        // Otherwise, it's Sage code that will be converted to CUDA
        
        let mut body = Vec::new();
        while !matches!(self.current.token, Token::CloseBrace) && !matches!(self.current.token, Token::EOF) {
            body.push(self.statement());
        }
        
        if !matches!(self.current.token, Token::CloseBrace) {
            errormsg::parser_error("Expected '}' after use_cuda block", self.current.line);
        }
        self.advance();
        
        Stmt::UseCuda {
            kernel_code,
            body,
            line,
        }
    }
}
