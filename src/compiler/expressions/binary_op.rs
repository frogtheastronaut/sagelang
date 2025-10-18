use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::lexer::tokens::Token;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_binary_op(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<(), String> {
        // handle and/or
        match op {
            Token::And => {
                self.compile_expr(left)?;
                self.chunk.write(OpCode::Dup);
                let end_jump = self.emit_jump(OpCode::JumpIfFalse(0));
                self.chunk.write(OpCode::Pop);
                self.compile_expr(right)?;
                self.patch_jump(end_jump);
                return Ok(());
            }
            Token::Or => {
                self.compile_expr(left)?;
                self.chunk.write(OpCode::Dup);
                let end_jump = self.emit_jump(OpCode::JumpIfTrue(0));
                self.chunk.write(OpCode::Pop);
                self.compile_expr(right)?;
                self.patch_jump(end_jump);
                return Ok(());
            }
            _ => {}
        }
        
        // regular binary ops
        self.compile_expr(left)?;
        self.compile_expr(right)?;
        
        match op {
            Token::Plus => self.chunk.write(OpCode::Add),
            Token::Minus => self.chunk.write(OpCode::Subtract),
            Token::Star => self.chunk.write(OpCode::Multiply),
            Token::Slash => self.chunk.write(OpCode::Divide),
            Token::Percent => self.chunk.write(OpCode::Modulo),
            Token::EqEq => self.chunk.write(OpCode::Equal),
            Token::NotEq => self.chunk.write(OpCode::NotEqual),
            Token::Greater => self.chunk.write(OpCode::Greater),
            Token::GreaterEq => self.chunk.write(OpCode::GreaterEqual),
            Token::Less => self.chunk.write(OpCode::Less),
            Token::LessEq => self.chunk.write(OpCode::LessEqual),
            Token::DotDot => self.chunk.write(OpCode::BuildRange),
            _ => return Err(format!("Unsupported binary operator: {:?}", op)),
        }
        
        Ok(())
    }
}
