use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::{Expr, Stmt};

impl Compiler {
    pub fn compile_for_stmt(&mut self, var: &str, iterable: &Expr, body: &[Stmt]) -> Result<(), String> {
        // compile iterable
        self.compile_expr(iterable)?;
        
        // we'll compile this as:
        // 1. store list/range in a temp local
        // 2. initialize counter to 0
        // 3. loop: check if counter < length, get item, run body, increment counter
        
        self.begin_scope();
        
        // store iterable
        let iterable_idx = self.local_count;
        self.chunk.write(OpCode::SetLocal(iterable_idx));
        self.local_count += 1;
        
        // initialize counter to 0
        let zero_const = self.chunk.add_constant(Value::Number(0.0));
        self.chunk.write(OpCode::LoadConst(zero_const));
        let counter_idx = self.local_count;
        self.chunk.write(OpCode::SetLocal(counter_idx));
        self.local_count += 1;
        
        // loop start
        let loop_start = self.chunk.code.len();
        
        // get current item from list
        self.chunk.write(OpCode::GetLocal(iterable_idx));
        self.chunk.write(OpCode::GetLocal(counter_idx));
        self.chunk.write(OpCode::GetIndex);
        
        // check if we got null (end of list)
        self.chunk.write(OpCode::Dup);
        self.chunk.write(OpCode::LoadNull);
        self.chunk.write(OpCode::Equal);
        let exit_jump = self.emit_jump(OpCode::JumpIfTrue(0));
        self.chunk.write(OpCode::Pop);
        
        // store current item in loop variable
        let var_idx = self.local_count;
        self.locals.insert(var.to_string(), var_idx);
        self.chunk.write(OpCode::SetLocal(var_idx));
        self.local_count += 1;
        
        // compile body
        for stmt in body {
            self.compile_stmt(stmt)?;
        }
        
        // increment counter
        self.chunk.write(OpCode::GetLocal(counter_idx));
        let one_const = self.chunk.add_constant(Value::Number(1.0));
        self.chunk.write(OpCode::LoadConst(one_const));
        self.chunk.write(OpCode::Add);
        self.chunk.write(OpCode::SetLocal(counter_idx));
        self.chunk.write(OpCode::Pop);
        
        // loop back
        self.chunk.write(OpCode::Loop(loop_start));
        
        // patch exit
        self.patch_jump(exit_jump);
        self.chunk.write(OpCode::Pop); // Pop the comparison result
        
        self.end_scope();
        
        Ok(())
    }
}
