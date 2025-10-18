use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_var_decl(&mut self, name: &str, value: &Expr) -> Result<(), String> {
        self.compile_expr(value)?;
        
        // store in local if we're in a local, otherwise global
        if self.scope_depth > 0 {
            let idx = self.local_count;
            self.locals.insert(name.to_string(), idx);
            self.chunk.write(OpCode::SetLocal(idx));
            self.local_count += 1;
        } else {
            let name_idx = self.chunk.add_constant(Value::String(name.to_string()));
            self.chunk.write(OpCode::SetGlobal(name_idx));
        }
        
        Ok(())
    }
}
