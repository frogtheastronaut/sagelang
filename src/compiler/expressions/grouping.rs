use crate::compiler::Compiler;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_grouping(&mut self, expr: &Expr) -> Result<(), String> {
        self.compile_expr(expr)
    }
}
