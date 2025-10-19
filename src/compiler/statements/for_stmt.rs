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
        self.chunk.write(OpCode::SetLocal(iterable_idx), self.current_line);
        self.local_count += 1;
        
        // initialize counter to 0
        let zero_const = self.chunk.add_constant(Value::Number(0.0));
        self.chunk.write(OpCode::LoadConst(zero_const), self.current_line);
        let counter_idx = self.local_count;
        self.chunk.write(OpCode::SetLocal(counter_idx), self.current_line);
        self.local_count += 1;
        
        // loop start
        let loop_start = self.chunk.code.len();
        
        // get current item from list
        self.chunk.write(OpCode::GetLocal(iterable_idx), self.current_line);
        self.chunk.write(OpCode::GetLocal(counter_idx), self.current_line);
        self.chunk.write(OpCode::GetIndex, self.current_line);
        
        // check if we got null (end of list)
        self.chunk.write(OpCode::Dup, self.current_line);
        self.chunk.write(OpCode::LoadNull, self.current_line);
        self.chunk.write(OpCode::Equal, self.current_line);
        let exit_jump = self.emit_jump(OpCode::JumpIfTrue(0));
        self.chunk.write(OpCode::Pop, self.current_line);
        
        // store current item in loop variable
        let var_idx = self.local_count;
        self.locals.insert(var.to_string(), var_idx);
        self.chunk.write(OpCode::SetLocal(var_idx), self.current_line);
        self.local_count += 1;
        
        // compile body
        for stmt in body {
            self.compile_stmt(stmt)?;
        }
        
        // increment counter
        self.chunk.write(OpCode::GetLocal(counter_idx), self.current_line);
        let one_const = self.chunk.add_constant(Value::Number(1.0));
        self.chunk.write(OpCode::LoadConst(one_const), self.current_line);
        self.chunk.write(OpCode::Add, self.current_line);
        self.chunk.write(OpCode::SetLocal(counter_idx), self.current_line);
        self.chunk.write(OpCode::Pop, self.current_line);
        
        // loop back
        self.chunk.write(OpCode::Loop(loop_start), self.current_line);
        
        // patch exit
        self.patch_jump(exit_jump);
        self.chunk.write(OpCode::Pop, self.current_line); // Pop the comparison result
        
        self.end_scope();
        
        Ok(())
    }
}
