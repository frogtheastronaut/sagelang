use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::{Expr, Stmt};

impl Compiler {
    pub fn compile_while_stmt(&mut self, condition: &Expr, body: &[Stmt]) -> Result<(), String> {
        let loop_start = self.chunk.code.len();
        
        // compile condition
        self.compile_expr(condition)?;
        
        // exit loop if condition is false
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.chunk.write(OpCode::Pop, self.current_line);
        
        // compile body
        self.begin_scope();
        for stmt in body {
            self.compile_stmt(stmt)?;
        }
        self.end_scope();
        
        // loop back to condition
        self.chunk.write(OpCode::Loop(loop_start), self.current_line);
        
        // patch exit jump
        self.patch_jump(exit_jump);
        self.chunk.write(OpCode::Pop, self.current_line);
        
        Ok(())
    }
}
