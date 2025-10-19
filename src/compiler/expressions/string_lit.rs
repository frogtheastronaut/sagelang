use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;

impl Compiler {
    pub fn compile_string_lit(&mut self, s: &str) -> Result<(), String> {
        let idx = self.chunk.add_constant(Value::String(s.to_string()));
        self.chunk.write(OpCode::LoadConst(idx), self.current_line);
        Ok(())
    }
}
