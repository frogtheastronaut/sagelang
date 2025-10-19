use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_print(&mut self, expr: &Expr) -> Result<(), String> {
        self.compile_expr(expr)?;
        self.chunk.write(OpCode::Print, self.current_line);
        Ok(())
    }
}
