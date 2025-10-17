use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::{Expr, Stmt};

impl Compiler {
    pub fn compile_for_stmt(&mut self, var: &str, iterable: &Expr, body: &[Stmt]) -> Result<(), String> {
        // Compile iterable
        self.compile_expr(iterable)?;
        
        // For simplicity, we'll compile this as:
        // 1. Store list/range in a temp local
        // 2. Initialize counter to 0
        // 3. Loop: check if counter < length, get item, run body, increment counter
        
        self.begin_scope();
        
        // Store iterable
        let iterable_idx = self.local_count;
        self.chunk.write(OpCode::SetLocal(iterable_idx));
        self.local_count += 1;
        
        // Initialize counter to 0
        let zero_const = self.chunk.add_constant(Value::Number(0.0));
        self.chunk.write(OpCode::LoadConst(zero_const));
        let counter_idx = self.local_count;
        self.chunk.write(OpCode::SetLocal(counter_idx));
        self.local_count += 1;
        
        // Loop start
        let loop_start = self.chunk.code.len();
        
        // Check if we've iterated over all items
        // For now, we'll use a simpler approach:
        // Just iterate through the list using GetIndex
        
        // Get current item from list
        self.chunk.write(OpCode::GetLocal(iterable_idx));
        self.chunk.write(OpCode::GetLocal(counter_idx));
        self.chunk.write(OpCode::GetIndex);
        
        // Check if we got null (end of list)
        self.chunk.write(OpCode::Dup);
        self.chunk.write(OpCode::LoadNull);
        self.chunk.write(OpCode::Equal);
        let exit_jump = self.emit_jump(OpCode::JumpIfTrue(0));
        self.chunk.write(OpCode::Pop);
        
        // Store current item in loop variable
        let var_idx = self.local_count;
        self.locals.insert(var.to_string(), var_idx);
        self.chunk.write(OpCode::SetLocal(var_idx));
        self.local_count += 1;
        
        // Compile body
        for stmt in body {
            self.compile_stmt(stmt)?;
        }
        
        // Increment counter
        self.chunk.write(OpCode::GetLocal(counter_idx));
        let one_const = self.chunk.add_constant(Value::Number(1.0));
        self.chunk.write(OpCode::LoadConst(one_const));
        self.chunk.write(OpCode::Add);
        self.chunk.write(OpCode::SetLocal(counter_idx));
        self.chunk.write(OpCode::Pop);
        
        // Loop back
        self.chunk.write(OpCode::Loop(loop_start));
        
        // Patch exit
        self.patch_jump(exit_jump);
        self.chunk.write(OpCode::Pop); // Pop the comparison result
        
        self.end_scope();
        
        Ok(())
    }
}
