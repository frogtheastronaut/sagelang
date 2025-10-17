use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_list(&mut self, items: &[Expr]) -> Result<(), String> {
        // Compile each item
        for item in items {
            self.compile_expr(item)?;
        }
        // Create list from stack items
        self.chunk.write(OpCode::MakeList(items.len()));
        Ok(())
    }
}
