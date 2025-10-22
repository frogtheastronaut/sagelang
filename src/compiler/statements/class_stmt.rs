use crate::compiler::Compiler;
use crate::vm::OpCode;
use crate::interpreter::Value;
use crate::parser::ast::{Method, Field};
use std::collections::HashMap;

impl Compiler {
    pub fn compile_class_stmt(&mut self, name: &str, superclass: &Option<String>, fields: &[Field], methods: &[Method]) -> Result<(), String> {
        // create the class first (will be stored in globals)
        let name_idx = self.chunk.add_constant(Value::String(name.to_string()));
        self.chunk.write(OpCode::DefineClass(name_idx), self.current_line);
        
        // compile methods
        let mut instance_method_map = HashMap::new();
        let mut static_method_map = HashMap::new();
        
        for method in methods {
            // compile method body
            let mut method_compiler = Compiler::new();
            method_compiler.chunk.name = format!("{}::{}", name, method.name);
            
            // set class context for super keyword
            method_compiler.current_class = Some(name.to_string());
            method_compiler.current_superclass = superclass.clone();

            // set up parameters (including 'this' as local 0 for instance methods)
            if !method.is_static {
                method_compiler.locals.insert("this".to_string(), 0);
                method_compiler.local_count = 1;
            }
            
            for (i, param) in method.params.iter().enumerate() {
                let local_idx = if method.is_static { i } else { i + 1 };
                method_compiler.locals.insert(param.param_name.clone(), local_idx);
                method_compiler.local_count = local_idx + 1;
            }
            
            // compile method body
            for stmt in &method.body {
                method_compiler.compile_stmt(stmt)?;
            }

            // ensure method returns something
            // for constructors, return 'this'. for regular methods, return null.
            if method.name == "constructor" {
                method_compiler.chunk.write(OpCode::GetLocal(0), 0); // Get 'this'
            } else {
                method_compiler.chunk.write(OpCode::LoadNull, 0);
            }
            method_compiler.chunk.write(OpCode::Return, 0);

            // create method value
            let method_value = Value::Function {
                name: method.name.clone(),
                param_count: method.params.len(),
                chunk: method_compiler.chunk,
            };

            // store in appropriate map based on static flag
            if method.is_static {
                static_method_map.insert(method.name.clone(), method_value);
            } else {
                instance_method_map.insert(method.name.clone(), method_value);
            }
        }

        // build field and method access maps
        let mut field_access_map = HashMap::new();
        for field in fields {
            field_access_map.insert(field.name.clone(), field.access.clone());
        }
        
        let mut method_access_map = HashMap::new();
        for method in methods {
            method_access_map.insert(method.name.clone(), method.access.clone());
        }

        // store class with methods as a constant
        let class_value = Value::Class {
            name: name.to_string(),
            superclass: None,
            field_access: field_access_map.clone(),
            method_access: method_access_map.clone(),
            methods: instance_method_map,
            static_methods: static_method_map,
        };
        
        let class_idx = self.chunk.add_constant(class_value);
        self.chunk.write(OpCode::LoadConst(class_idx), self.current_line);

        // store class in global variable
        let name_idx = self.chunk.add_constant(Value::String(name.to_string()));

        self.chunk.write(OpCode::SetGlobal(name_idx), self.current_line);

        // handle inheritance if there's a superclass
        if let Some(super_name) = superclass {
            // load superclass
            self.compile_identifier(super_name)?;
            
            // load subclass name
            let class_name_const = self.chunk.add_constant(Value::String(name.to_string()));
            self.chunk.write(OpCode::LoadConst(class_name_const), self.current_line);

            // inherit from superclass
            self.chunk.write(OpCode::Inherit, self.current_line);
        }
        
        Ok(())
    }
}
