use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_call(&mut self, callee: &Expr, args: &[Expr]) -> Result<(), String> {
        // Compile the function expression
        self.compile_expr(callee)?;
        
        // Compile arguments
        for arg in args {
            self.compile_expr(arg)?;
        }
        
        // Call with argument count
        self.chunk.write(OpCode::Call(args.len()));
        
        Ok(())
    }
}
