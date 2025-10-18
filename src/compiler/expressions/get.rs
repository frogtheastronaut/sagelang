use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_get(&mut self, object: &Expr, name: &str) -> Result<(), String> {
        // Compile the object expression
        self.compile_expr(object)?;
        
        // Get the property
        let name_idx = self.chunk.add_constant(Value::String(name.to_string()));
        self.chunk.write(OpCode::GetProperty(name_idx));
        
        Ok(())
    }
}
