use crate::compiler::Compiler;
use crate::vm::OpCode;

impl Compiler {
    pub fn compile_bool(&mut self, b: bool) -> Result<(), String> {
        // compile bools
        if b {
            self.chunk.write(OpCode::LoadTrue);
        } else {
            self.chunk.write(OpCode::LoadFalse);
        }
        Ok(())
    }
}
