use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;

impl Compiler {
    pub fn compile_super(&mut self, method: &str) -> Result<(), String> {
        // Get 'this' (local 0)
        self.chunk.write(OpCode::GetLocal(0));
        
        // Get the superclass method
        let method_idx = self.chunk.add_constant(Value::String(method.to_string()));
        self.chunk.write(OpCode::GetSuper(method_idx));
        
        Ok(())
    }
}
