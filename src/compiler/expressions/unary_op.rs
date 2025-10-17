use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::lexer::tokens::Token;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_unary_op(&mut self, op: &Token, right: &Expr) -> Result<(), String> {
        self.compile_expr(right)?;
        match op {
            Token::Minus => self.chunk.write(OpCode::Negate),
            _ => return Err(format!("Unsupported unary operator: {:?}", op)),
        }
        Ok(())
    }
}
