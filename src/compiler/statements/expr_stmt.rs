use crate::parser::ast::Expr;
use crate::compiler::Compiler;

pub fn expr_stmt(compiler: &mut Compiler, expr: &Expr) {
    compiler.compile_expr(expr);
}
