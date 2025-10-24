use crate::compiler::Compiler;
use crate::parser::ast::{Stmt, Expr};
use crate::vm::OpCode;
use crate::interpreter::Value;

impl Compiler {
    pub fn compile_use_cuda(&mut self, kernel_code: &str, body: &[Stmt]) -> Result<(), String> {
        // If user provided raw CUDA code, use it
        if !kernel_code.is_empty() {
            self.chunk.write(OpCode::CudaInit, self.current_line);
            let kernel_idx = self.chunk.constants.len();
            self.chunk.constants.push(Value::String(kernel_code.to_string()));
            self.chunk.write(OpCode::CudaLoadKernel(kernel_idx), self.current_line);
        } else {
            // Generate CUDA code from Sage code
            let cuda_code = self.generate_cuda_from_sage(body)?;
            
            // Print generated CUDA code in debug mode
            if self.debug {
                println!("\n[Generated CUDA Kernel]:");
                println!("{}", cuda_code);
                println!();
            }
            
            // Initialize CUDA with generated code
            self.chunk.write(OpCode::CudaInit, self.current_line);
            let kernel_idx = self.chunk.constants.len();
            self.chunk.constants.push(Value::String(cuda_code));
            self.chunk.write(OpCode::CudaLoadKernel(kernel_idx), self.current_line);
            
            // Execute kernel on GPU
            self.chunk.write(OpCode::CudaExecute, self.current_line);
        }
        
        // Execute the body normally - code inside use_cuda runs like regular Sage code
        // The CUDA kernel generation is just for reference/future GPU acceleration
        for stmt in body {
            self.compile_stmt(stmt)?;
        }
        
        Ok(())
    }
    
    fn generate_cuda_from_sage(&self, body: &[Stmt]) -> Result<String, String> {
        let mut cuda = String::new();
        cuda.push_str("extern \"C\" __global__\n");
        cuda.push_str("void compute_kernel(float* input, float* output, int n) {\n");
        cuda.push_str("    int idx = blockIdx.x * blockDim.x + threadIdx.x;\n");
        cuda.push_str("    if (idx < n) {\n");
        
        // Track local variables for CUDA kernel
        let mut cuda_vars: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        
        // Analyze Sage code to generate GPU operations
        for stmt in body {
            match stmt {
                Stmt::VarDecl { name, value, .. } => {
                    if let Some(op) = self.gpu_extract_operation(value, "input[idx]") {
                        cuda.push_str(&format!("        float {} = {};\n", name, op));
                        cuda_vars.insert(name.clone(), op);
                    }
                }
                Stmt::Assign { name, value, .. } => {
                    if let Some(op) = self.gpu_extract_operation(value, "input[idx]") {
                        cuda.push_str(&format!("        {} = {};\n", name, op));
                        cuda_vars.insert(name.clone(), op);
                    }
                }
                Stmt::Print { expr, .. } => {
                    // For print statements, store the result in output buffer
                    if let Expr::Identifier { name, .. } = expr {
                        if cuda_vars.contains_key(name) {
                            cuda.push_str(&format!("        output[idx] = {};\n", name));
                        }
                    } else if let Some(op) = self.gpu_extract_operation(expr, "input[idx]") {
                        cuda.push_str(&format!("        output[idx] = {};\n", op));
                    }
                }
                _ => {}
            }
        }
        
        cuda.push_str("    }\n");
        cuda.push_str("}\n");
        Ok(cuda)
    }
}
