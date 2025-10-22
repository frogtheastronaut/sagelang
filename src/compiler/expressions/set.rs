use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_set(&mut self, object: &Expr, name: &str, value: &Expr) -> Result<(), String> {
        // compile the object expression
        self.compile_expr(object)?;

        // compile the value to set
        self.compile_expr(value)?;

        // set the property
        let name_idx = self.chunk.add_constant(Value::String(name.to_string()));
        self.chunk.write(OpCode::SetProperty(name_idx), self.current_line);
        
        Ok(())
    }
}
