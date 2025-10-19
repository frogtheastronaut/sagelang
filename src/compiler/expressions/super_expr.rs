use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;

impl Compiler {
    pub fn compile_super(&mut self, method: &str) -> Result<(), String> {
        // Check if we're in a class context
        let superclass_name = self.current_superclass.as_ref()
            .ok_or("Cannot use 'super' outside of a class with a superclass")?;
        
        // Push 'this' (local 0) onto the stack first
        self.chunk.write(OpCode::GetLocal(0), self.current_line);
        
        // Push the superclass onto the stack second
        // The superclass is stored in globals
        let superclass_idx = self.chunk.add_constant(Value::String(superclass_name.clone()));
        self.chunk.write(OpCode::GetGlobal(superclass_idx), self.current_line);
        
        // Get the superclass method
        // Stack order: [this, superclass] -> GetSuper pops superclass then this
        let method_idx = self.chunk.add_constant(Value::String(method.to_string()));
        self.chunk.write(OpCode::GetSuper(method_idx), self.current_line);
        
        Ok(())
    }
}
