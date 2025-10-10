/*
 * parser for elseif statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn elseif_stmt(&mut self) -> Stmt {
        self.eat(Token::ElseIfKw);
        let condition = self.expr();
        let then_branch = self.block_stmt();
        Stmt::If {
            condition,
            then_branch,
            else_branch: None,
            elseif_branches: vec![],
        }
    }
}
