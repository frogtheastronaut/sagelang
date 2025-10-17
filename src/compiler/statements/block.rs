use crate::compiler::Compiler;
use crate::parser::ast::Stmt;

impl Compiler {
    pub fn compile_block(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        self.begin_scope();
        for stmt in stmts {
            self.compile_stmt(stmt)?;
        }
        self.end_scope();
        Ok(())
    }
}
