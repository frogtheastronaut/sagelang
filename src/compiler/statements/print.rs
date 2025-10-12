use crate::parser::ast::Expr;
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn print(compiler: &mut Compiler, expr: &Expr) {
    compiler.compile_expr(expr);
    compiler.bytecode.instructions.push(Instruction::Print);
}
