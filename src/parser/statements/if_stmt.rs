/*
 * parser for if, else and elseif statements.
 */
use crate::parser::Parser;
use crate::parser::ast::Stmt;
use crate::lexer::tokens::Token;

impl<'a> Parser<'a> {
    pub fn if_stmt(&mut self) -> Stmt {
        self.eat(Token::If);

        let condition = self.expr();

        let then_branch = self.block_stmt();

        let mut elseif_branches = Vec::new();
        while self.current == Token::ElseIfKw {
            self.advance();
            let elseif_cond = self.expr();
            let elseif_block = self.block_stmt();
            elseif_branches.push((elseif_cond, elseif_block));
        }

        let else_branch = if self.current == Token::Else {
            self.advance();
            Some(self.block_stmt())
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
            elseif_branches,
        }
    }
}
