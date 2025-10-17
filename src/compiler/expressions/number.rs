use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;

impl Compiler {
    pub fn compile_number(&mut self, n: f64) -> Result<(), String> {
        let idx = self.chunk.add_constant(Value::Number(n));
        self.chunk.write(OpCode::LoadConst(idx));
        Ok(())
    }
}
