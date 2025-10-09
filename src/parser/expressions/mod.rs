pub mod equality;
pub mod comparison;
pub mod term;
pub mod factor;
pub mod unary;
pub mod grouping;
pub mod call;

use crate::parser::Parser;
use crate::parser::ast::Expr;

impl<'a> Parser<'a> {
    pub fn expr(&mut self) -> Expr {
        self.equality()
    }
}
