use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;

impl Compiler {
    pub fn compile_identifier(&mut self, name: &str) -> Result<(), String> {
        // Check if it's a local variable
        if let Some(&idx) = self.locals.get(name) {
            self.chunk.write(OpCode::GetLocal(idx));
        } else {
            // Otherwise it's global
            let name_idx = self.chunk.add_constant(Value::String(name.to_string()));
            self.chunk.write(OpCode::GetGlobal(name_idx));
        }
        Ok(())
    }
}
