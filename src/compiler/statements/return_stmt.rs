use crate::parser::ast::Expr;
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn return_stmt(compiler: &mut Compiler, opt_expr: &Option<Expr>) {
    if let Some(expr) = opt_expr {
        compiler.compile_expr(expr);
    }
    compiler.bytecode.instructions.push(Instruction::Return);
}
