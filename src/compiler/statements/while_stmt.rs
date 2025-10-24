use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::{Expr, Stmt};

impl Compiler {
    pub fn compile_while_stmt(&mut self, condition: &Expr, body: &[Stmt]) -> Result<(), String> {
        let loop_start = self.chunk.code.len();
        
        self.compile_expr(condition)?;
        
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.chunk.write(OpCode::Pop, self.current_line);
        
        self.begin_scope();
        for stmt in body {
            self.compile_stmt(stmt)?;
        }
        self.end_scope();
        
        self.chunk.write(OpCode::Loop(loop_start), self.current_line);
        
        self.patch_jump(exit_jump);
        self.chunk.write(OpCode::Pop, self.current_line);
        
        Ok(())
    }
}
