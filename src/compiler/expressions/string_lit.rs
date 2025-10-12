use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn string_lit(compiler: &mut Compiler, s: &String) {
    compiler.bytecode.instructions.push(Instruction::PushString(s.clone()));
}
