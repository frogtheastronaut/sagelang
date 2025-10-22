use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_expr_stmt(&mut self, expr: &Expr) -> Result<(), String> {
        // compile the expression
        self.compile_expr(expr)?;
        // pop the result off the stack
        self.chunk.write(OpCode::Pop, self.current_line);
        Ok(())
    }
}
