use crate::parser::ast::Expr;
use crate::compiler::Compiler;

pub fn grouping(compiler: &mut Compiler, expr: &Expr) {
    compiler.compile_expr(expr);
}
