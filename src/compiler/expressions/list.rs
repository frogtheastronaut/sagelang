use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_list(&mut self, items: &[Expr]) -> Result<(), String> {
        // compile each item
        for item in items {
            self.compile_expr(item)?;
        }
        // create list from stack items
        self.chunk.write(OpCode::MakeList(items.len()), self.current_line);
        Ok(())
    }
}
