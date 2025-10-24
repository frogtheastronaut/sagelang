use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_expr_stmt(&mut self, expr: &Expr) -> Result<(), String> {
        self.compile_expr(expr)?;
        self.chunk.write(OpCode::Pop, self.current_line);
        Ok(())
    }
}
