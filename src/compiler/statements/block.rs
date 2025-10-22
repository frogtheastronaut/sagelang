use crate::compiler::Compiler;
use crate::parser::ast::Stmt;

impl Compiler {
    pub fn compile_block(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        // begin a new scope
        self.begin_scope();

        // compile each statement in the block
        for stmt in stmts {
            self.compile_stmt(stmt)?;
        }
        // end the scope
        self.end_scope();
        Ok(())
    }
}
