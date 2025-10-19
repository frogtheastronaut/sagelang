use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::{Param, Stmt};

impl Compiler {
    pub fn compile_function_stmt(&mut self, name: &str, params: &[Param], body: &[Stmt]) -> Result<(), String> {
        // compile the function body into a separate chunk
        let mut func_compiler = Compiler::new();
        func_compiler.chunk.name = name.to_string();
        
        // set up parameters as local variables
        for (i, param) in params.iter().enumerate() {
            func_compiler.locals.insert(param.param_name.clone(), i);
            func_compiler.local_count = i + 1;
        }
        
        // compile function body
        for stmt in body {
            func_compiler.compile_stmt(stmt)?;
        }
        
        // ensure function returns something
        func_compiler.chunk.write(OpCode::LoadNull, 0);
        func_compiler.chunk.write(OpCode::Return, 0);
        
        // create compiled function value
        let func_value = Value::Function {
            name: name.to_string(),
            param_count: params.len(),
            chunk: func_compiler.chunk,
        };
        
        let const_idx = self.chunk.add_constant(func_value);
        self.chunk.write(OpCode::LoadConst(const_idx), self.current_line);
        
        // store function in variable
        if self.scope_depth > 0 {
            let idx = self.local_count;
            self.locals.insert(name.to_string(), idx);
            self.chunk.write(OpCode::SetLocal(idx), self.current_line);
            self.local_count += 1;
        } else {
            let name_idx = self.chunk.add_constant(Value::String(name.to_string()));
            self.chunk.write(OpCode::SetGlobal(name_idx), self.current_line);
        }
        
        Ok(())
    }
}
