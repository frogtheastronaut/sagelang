use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::{Expr, Stmt};

impl Compiler {
    pub fn compile_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &[Stmt],
        else_branch: &Option<Vec<Stmt>>,
        elseif_branches: &[(Expr, Vec<Stmt>)]
    ) -> Result<(), String> {
        // Compile condition
        self.compile_expr(condition)?;
        
        // Jump to else if condition is false
        let then_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.chunk.write(OpCode::Pop); // Pop condition
        
        // Compile then branch
        self.begin_scope();
        for stmt in then_branch {
            self.compile_stmt(stmt)?;
        }
        self.end_scope();
        
        // Jump over else branch
        let else_jump = self.emit_jump(OpCode::Jump(0));
        
        // Patch then jump to point here
        self.patch_jump(then_jump);
        self.chunk.write(OpCode::Pop); // Pop condition
        
        // Handle elseif branches
        let mut elseif_jumps = Vec::new();
        for (elseif_cond, elseif_body) in elseif_branches {
            self.compile_expr(elseif_cond)?;
            let elseif_then_jump = self.emit_jump(OpCode::JumpIfFalse(0));
            self.chunk.write(OpCode::Pop);
            
            self.begin_scope();
            for stmt in elseif_body {
                self.compile_stmt(stmt)?;
            }
            self.end_scope();
            
            elseif_jumps.push(self.emit_jump(OpCode::Jump(0)));
            self.patch_jump(elseif_then_jump);
            self.chunk.write(OpCode::Pop);
        }
        
        // Compile else branch if it exists
        if let Some(else_stmts) = else_branch {
            self.begin_scope();
            for stmt in else_stmts {
                self.compile_stmt(stmt)?;
            }
            self.end_scope();
        }
        
        // Patch all jumps to end
        self.patch_jump(else_jump);
        for jump in elseif_jumps {
            self.patch_jump(jump);
        }
        
        Ok(())
    }
}
