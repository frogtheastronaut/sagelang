use crate::compiler::instruction::{Instruction};
use super::super::Compiler;

pub fn number(compiler: &mut Compiler, n: f64) {
    compiler.bytecode.instructions.push(Instruction::Const(n));
}
