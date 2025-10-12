use crate::parser::ast::Expr;
use crate::compiler::instruction::{Instruction};
use super::super::Compiler;

pub fn var_decl(compiler: &mut Compiler, name: &String, value: &Expr) {
    compiler.compile_expr(value);
    compiler.bytecode.instructions.push(Instruction::Store(name.clone()));
}
