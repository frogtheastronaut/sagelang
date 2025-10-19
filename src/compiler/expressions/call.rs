use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::parser::ast::Expr;

impl Compiler {
    pub fn compile_call(&mut self, callee: &Expr, args: &[Expr]) -> Result<(), String> {
        // compile the function expression
        self.compile_expr(callee)?;
        
        // compile arguments
        for arg in args {
            self.compile_expr(arg)?;
        }
        
        // call with argument count
        self.chunk.write(OpCode::Call(args.len()), self.current_line);
        
        Ok(())
    }
}
