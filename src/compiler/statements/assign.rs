use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_assign(&mut self, name: &str, value: &Expr) -> Result<(), String> {
        self.compile_expr(value)?;
        
        // Check if it's a local variable first
        if let Some(&idx) = self.locals.get(name) {
            self.chunk.write(OpCode::SetLocal(idx));
        } else {
            // Otherwise it's global
            let name_idx = self.chunk.add_constant(Value::String(name.to_string()));
            self.chunk.write(OpCode::SetGlobal(name_idx));
        }
        
        Ok(())
    }
}
