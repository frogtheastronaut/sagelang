use crate::compiler::Compiler;
use crate::parser::ast::{Stmt, Expr};
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::lexer::tokens::Token;

impl Compiler {
    pub fn compile_use_metal(&mut self, kernel_code: &str, body: &[Stmt]) -> Result<(), String> {
        // If user provided raw Metal code, use it
        if !kernel_code.is_empty() {
            self.chunk.write(OpCode::MetalInit, self.current_line);
            let kernel_idx = self.chunk.constants.len();
            self.chunk.constants.push(Value::String(kernel_code.to_string()));
            self.chunk.write(OpCode::MetalLoadKernel(kernel_idx), self.current_line);
        } else {
            // Generate Metal code from Sage code
            let metal_code = self.generate_metal_from_sage(body)?;
            
            // Print generated Metal code in debug mode
            if self.chunk.name == "main" {
                println!("\n[Generated Metal Kernel]:");
                println!("{}", metal_code);
                println!();
            }
            
            // Initialize Metal with generated code
            self.chunk.write(OpCode::MetalInit, self.current_line);
            let kernel_idx = self.chunk.constants.len();
            self.chunk.constants.push(Value::String(metal_code));
            self.chunk.write(OpCode::MetalLoadKernel(kernel_idx), self.current_line);
        }
        
        // Execute the body normally - code inside use_metal runs like regular Sage code
        // The Metal kernel generation is just for reference/future GPU acceleration
        for stmt in body {
            self.compile_stmt(stmt)?;
        }
        
        Ok(())
    }
    
    fn generate_metal_from_sage(&self, body: &[Stmt]) -> Result<String, String> {
        let mut metal = String::new();
        metal.push_str("#include <metal_stdlib>\n");
        metal.push_str("using namespace metal;\n\n");
        metal.push_str("kernel void compute_kernel(\n");
        metal.push_str("    device float* input [[buffer(0)]],\n");
        metal.push_str("    device float* output [[buffer(1)]],\n");
        metal.push_str("    uint id [[thread_position_in_grid]]\n");
        metal.push_str(") {\n");
        
        // Track local variables for Metal kernel
        let mut metal_vars: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        
        // Analyze Sage code to generate GPU operations
        for stmt in body {
            match stmt {
                Stmt::VarDecl { name, value, .. } => {
                    if let Some(op) = self.extract_operation(value) {
                        metal.push_str(&format!("    float {} = {};\n", name, op));
                        metal_vars.insert(name.clone(), op);
                    }
                }
                Stmt::Assign { name, value, .. } => {
                    if let Some(op) = self.extract_operation(value) {
                        metal.push_str(&format!("    {} = {};\n", name, op));
                        metal_vars.insert(name.clone(), op);
                    }
                }
                Stmt::Print { expr, .. } => {
                    // For print statements, store the result in output buffer
                    if let Expr::Identifier { name, .. } = expr {
                        if metal_vars.contains_key(name) {
                            metal.push_str(&format!("    output[id] = {};\n", name));
                        }
                    } else if let Some(op) = self.extract_operation(expr) {
                        metal.push_str(&format!("    output[id] = {};\n", op));
                    }
                }
                _ => {}
            }
        }
        
        metal.push_str("}\n");
        Ok(metal)
    }
    
    fn extract_operation(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::BinaryOp { left, op, right, .. } => {
                let left_str = self.expr_to_metal(left)?;
                let right_str = self.expr_to_metal(right)?;
                let op_str = match op {
                    Token::Plus => "+",
                    Token::Minus => "-",
                    Token::Star => "*",
                    Token::Slash => "/",
                    Token::Percent => "%",
                    _ => return None,
                };
                Some(format!("{} {} {}", left_str, op_str, right_str))
            }
            _ => self.expr_to_metal(expr),
        }
    }
    
    fn expr_to_metal(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Number { value, .. } => Some(format!("{}", value)),
            Expr::Identifier { name, .. } => {
                if name == "data" || name.starts_with("data") {
                    Some("input[id]".to_string())
                } else {
                    Some(name.clone())
                }
            }
            Expr::BinaryOp { left, op, right, .. } => {
                let left_str = self.expr_to_metal(left)?;
                let right_str = self.expr_to_metal(right)?;
                let op_str = match op {
                    Token::Plus => "+",
                    Token::Minus => "-",
                    Token::Star => "*",
                    Token::Slash => "/",
                    Token::Percent => "%",
                    _ => return None,
                };
                Some(format!("({} {} {})", left_str, op_str, right_str))
            }
            _ => None,
        }
    }
}
