use crate::compiler::Compiler;
use crate::vm::OpCode;

impl Compiler {
    pub fn compile_this(&mut self) -> Result<(), String> {
        // 'this' is always stored as local variable 0 in methods
        self.chunk.write(OpCode::GetLocal(0), self.current_line);
        Ok(())
    }
}
