use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_return_stmt(&mut self, expr: &Option<Expr>) -> Result<(), String> {
        if let Some(e) = expr {
            self.compile_expr(e)?;
        } else {
            self.chunk.write(OpCode::LoadNull);
        }
        self.chunk.write(OpCode::Return);
        Ok(())
    }
}
