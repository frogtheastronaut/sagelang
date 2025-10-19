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
                self.chunk.write(OpCode::Dup, self.current_line);
                let end_jump = self.emit_jump(OpCode::JumpIfFalse(0));
                self.chunk.write(OpCode::Pop, self.current_line);
                self.compile_expr(right)?;
                self.patch_jump(end_jump);
                return Ok(());
            }
            Token::Or => {
                self.compile_expr(left)?;
                self.chunk.write(OpCode::Dup, self.current_line);
                let end_jump = self.emit_jump(OpCode::JumpIfTrue(0));
                self.chunk.write(OpCode::Pop, self.current_line);
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
            Token::Plus => self.chunk.write(OpCode::Add, self.current_line),
            Token::Minus => self.chunk.write(OpCode::Subtract, self.current_line),
            Token::Star => self.chunk.write(OpCode::Multiply, self.current_line),
            Token::Slash => self.chunk.write(OpCode::Divide, self.current_line),
            Token::Percent => self.chunk.write(OpCode::Modulo, self.current_line),
            Token::EqEq => self.chunk.write(OpCode::Equal, self.current_line),
            Token::NotEq => self.chunk.write(OpCode::NotEqual, self.current_line),
            Token::Greater => self.chunk.write(OpCode::Greater, self.current_line),
            Token::GreaterEq => self.chunk.write(OpCode::GreaterEqual, self.current_line),
            Token::Less => self.chunk.write(OpCode::Less, self.current_line),
            Token::LessEq => self.chunk.write(OpCode::LessEqual, self.current_line),
            Token::DotDot => self.chunk.write(OpCode::BuildRange, self.current_line),
            _ => return Err(format!("Unsupported binary operator: {:?}", op)),
        }
        
        Ok(())
    }
}
