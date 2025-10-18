pub mod expressions;
pub mod statements;
pub mod ast;

use crate::lexer::tokenizer::Tokenizer;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::{Token, CurrentToken};

pub struct Parser<'a> {
    pub tokenizer: &'a mut Tokenizer<'a>,
    pub current: CurrentToken,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
        let current = tokenizer.next_token();
        Self { tokenizer, current }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current.token != Token::EOF {
            stmts.push(self.statement());
        }
        stmts
    }

    pub fn advance(&mut self) {
        self.current = self.tokenizer.next_token();
    }

    pub fn eat(&mut self, expected: Token) {
        if std::mem::discriminant(&self.current.token) == std::mem::discriminant(&expected) {
            self.advance();
        } else {
            panic!("[ERR] Expected {:?}, got {:?} at line {}", expected, self.current.token, self.current.line);
        }
    }
}
