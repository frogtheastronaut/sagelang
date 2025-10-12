use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn identifier(compiler: &mut Compiler, name: &String) {
    compiler.bytecode.instructions.push(Instruction::Load(name.clone()));
}
