use crate::compiler::Compiler;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_grouping(&mut self, expr: &Expr) -> Result<(), String> {
        // compile the inner expression
        self.compile_expr(expr)
    }
}
