use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn bool_lit(compiler: &mut Compiler, b: bool) {
    compiler.bytecode.instructions.push(Instruction::PushBool(b));
}
