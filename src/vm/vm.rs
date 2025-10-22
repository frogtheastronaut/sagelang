use crate::interpreter::Value;
use super::chunk::Chunk;
use super::opcode::OpCode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack_offset: usize,
    pub class_context: Option<String>,
}

pub struct VM {
    pub stack: Vec<Value>,
    pub frames: Vec<CallFrame>,
    pub globals: HashMap<String, Value>,
    pub debug: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            frames: Vec::new(),
            globals: HashMap::new(),
            debug: false,
        }
    }
    
    fn get_current_line(&self) -> usize {
        if let Some(frame) = self.frames.last() {
            let ip = if frame.ip > 0 { frame.ip - 1 } else { 0 };
            frame.chunk.lines.get(ip).copied().unwrap_or(0)
        } else {
            0
        }
    }
    
    fn runtime_error(&self, message: &str) -> String {
        let line = self.get_current_line();
        format!("{} [line {}]", message, line)
    }
    
    pub fn run(&mut self, chunk: Chunk) -> Result<(), String> {
        if self.debug {
            chunk.disassemble();
        }
        
        let frame = CallFrame {
            chunk,
            ip: 0,
            stack_offset: 0,
            class_context: None,
        };
        self.frames.push(frame);
        
        self.execute()
    }
    
    fn execute(&mut self) -> Result<(), String> {
        loop {
            if self.frames.is_empty() {
                return Ok(());
            }
            
            let frame_idx = self.frames.len() - 1;
            let ip = self.frames[frame_idx].ip;
            
            if ip >= self.frames[frame_idx].chunk.code.len() {
                self.frames.pop();
                if self.frames.is_empty() {
                    return Ok(());
                }
                continue;
            }
            
            let instruction = self.frames[frame_idx].chunk.code[ip].clone();
            self.frames[frame_idx].ip += 1;
            
            if self.debug {
                println!("[DEBUG] Stack before {:?}: {:?}", instruction, self.stack);
            }
            
            match instruction {
                OpCode::LoadConst(idx) => {
                    let constant = self.frames[frame_idx].chunk.constants[idx].clone();
                    self.stack.push(constant);
                }
                
                OpCode::LoadTrue => {
                    self.stack.push(Value::Bool(true));
                }
                
                OpCode::LoadFalse => {
                    self.stack.push(Value::Bool(false));
                }
                
                OpCode::LoadNull => {
                    self.stack.push(Value::Null);
                }
                
                OpCode::GetGlobal(idx) => {
                    let name = if let Value::String(s) = &self.frames[frame_idx].chunk.constants[idx] {
                        s.clone()
                    } else {
                        return Err(self.runtime_error("Invalid global variable name"));
                    };
                    
                    let value = self.globals.get(&name).cloned().unwrap_or(Value::Null);
                    self.stack.push(value);
                }
                
                OpCode::SetGlobal(idx) => {
                    let name = if let Value::String(s) = &self.frames[frame_idx].chunk.constants[idx] {
                        s.clone()
                    } else {
                        return Err(self.runtime_error("Invalid global variable name"));
                    };
                    
                    let value = self.stack.last().cloned().unwrap_or(Value::Null);
                    self.globals.insert(name, value);
                }
                
                OpCode::GetLocal(idx) => {
                    let stack_offset = self.frames[frame_idx].stack_offset;
                    let value = self.stack.get(stack_offset + idx).cloned().unwrap_or(Value::Null);
                    self.stack.push(value);
                }
                
                OpCode::SetLocal(idx) => {
                    let stack_offset = self.frames[frame_idx].stack_offset;
                    let value = self.stack.last().cloned().unwrap_or(Value::Null);
                    if stack_offset + idx < self.stack.len() {
                        self.stack[stack_offset + idx] = value;
                    } else {
                        while self.stack.len() <= stack_offset + idx {
                            self.stack.push(Value::Null);
                        }
                        self.stack[stack_offset + idx] = value;
                    }
                }
                
                OpCode::Add => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Number(x + y));
                        }
                        (Value::String(x), Value::String(y)) => {
                            self.stack.push(Value::String(format!("{}{}", x, y)));
                        }
                        (Value::String(x), _) => {
                            let b_str = match &b {
                                Value::Number(n) => n.to_string(),
                                Value::Bool(b) => b.to_string(),
                                Value::Null => "null".to_string(),
                                Value::String(s) => s.clone(),
                                _ => format!("{:?}", b),
                            };
                            self.stack.push(Value::String(format!("{}{}", x, b_str)));
                        }
                        (_, Value::String(y)) => {
                            let a_str = match &a {
                                Value::Number(n) => n.to_string(),
                                Value::Bool(b) => b.to_string(),
                                Value::Null => "null".to_string(),
                                Value::String(s) => s.clone(),
                                _ => format!("{:?}", a),
                            };
                            self.stack.push(Value::String(format!("{}{}", a_str, y)));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Subtract => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Number(x - y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Multiply => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Number(x * y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Divide => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            if *y == 0.0 {
                                return Err(self.runtime_error("Division by zero"));
                            }
                            self.stack.push(Value::Number(x / y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Modulo => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Number(x % y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Negate => {
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match a {
                        Value::Number(x) => self.stack.push(Value::Number(-x)),
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Equal => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(Value::Bool(self.values_equal(&a, &b)));
                }
                
                OpCode::NotEqual => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(Value::Bool(!self.values_equal(&a, &b)));
                }
                
                OpCode::Greater => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Bool(x > y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::GreaterEqual => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Bool(x >= y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Less => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Bool(x < y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::LessEqual => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.stack.push(Value::Bool(x <= y));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::Jump(addr) => {
                    let frame_idx = self.frames.len() - 1;
                    self.frames[frame_idx].ip = addr;
                }
                
                OpCode::JumpIfFalse(addr) => {
                    let condition = self.stack.last().cloned().unwrap_or(Value::Null);
                    if !self.is_truthy(&condition) {
                        let frame_idx = self.frames.len() - 1;
                        self.frames[frame_idx].ip = addr;
                    }
                }
                
                OpCode::JumpIfTrue(addr) => {
                    let condition = self.stack.last().cloned().unwrap_or(Value::Null);
                    if self.is_truthy(&condition) {
                        let frame_idx = self.frames.len() - 1;
                        self.frames[frame_idx].ip = addr;
                    }
                }
                
                OpCode::Loop(addr) => {
                    let frame_idx = self.frames.len() - 1;
                    self.frames[frame_idx].ip = addr;
                }
                
                OpCode::Call(arg_count) => {
                    let func_index = self.stack.len() - arg_count - 1;
                    let function = self.stack[func_index].clone();
                    
                    match function {
                        Value::Function { name: _, param_count, chunk } => {
                            if arg_count != param_count {
                                return Err(self.runtime_error(&format!("Expected {} arguments but got {}", param_count, arg_count)));
                            }
                            
                            self.stack.remove(func_index);
                            let stack_offset = self.stack.len() - arg_count;
                            
                            let new_frame = CallFrame {
                                chunk,
                                ip: 0,
                                stack_offset,
                                class_context: None,
                            };
                            
                            self.frames.push(new_frame);
                        }
                        Value::Class { name, methods, field_access, method_access, static_methods, .. } => {
                            use std::rc::Rc;
                            use std::cell::RefCell;
                            
                            let instance = Value::Instance {
                                class_name: name.clone(),
                                fields: Rc::new(RefCell::new(std::collections::HashMap::new())),
                                field_access: field_access.clone(),
                                methods: methods.clone(),
                                method_access: method_access.clone(),
                                static_methods: static_methods.clone(),
                            };
                            
                            self.stack.remove(func_index);
                            
                            if let Some(constructor) = methods.get("constructor") {
                                let args_start = self.stack.len() - arg_count;
                                self.stack.insert(args_start, instance.clone());
                                
                                if let Value::Function { param_count, chunk, .. } = constructor {
                                    if arg_count != *param_count {
                                        return Err(self.runtime_error(&format!("Expected {} arguments but got {}", param_count, arg_count)));
                                    }
                                    
                                    let stack_offset = args_start;
                                    let new_frame = CallFrame {
                                        chunk: chunk.clone(),
                                        ip: 0,
                                        stack_offset,
                                        class_context: Some(name.clone()),
                                    };
                                    self.frames.push(new_frame);
                                }
                            } else {
                                self.stack.push(instance);
                            }
                        }
                        Value::BoundMethod { receiver, method } => {
                            let class_name = if let Value::Instance { class_name, .. } = &*receiver {
                                Some(class_name.clone())
                            } else {
                                None
                            };
                            
                            self.stack.remove(func_index);
                            self.stack.insert(self.stack.len() - arg_count, *receiver);
                            
                            if let Value::Function { param_count, chunk, .. } = *method {
                                if arg_count != param_count {
                                    return Err(self.runtime_error(&format!("Expected {} arguments but got {}", param_count, arg_count)));
                                }
                                
                                let stack_offset = self.stack.len() - arg_count - 1;
                                let new_frame = CallFrame {
                                    chunk,
                                    ip: 0,
                                    stack_offset,
                                    class_context: class_name,
                                };
                                self.frames.push(new_frame);
                            } else {
                                return Err(self.runtime_error("Bound method must wrap a function"));
                            }
                        }
                        _ => return Err(self.runtime_error("Attempted to call non-callable")),
                    }
                }
                
                OpCode::Return => {
                    let return_value = self.stack.pop().unwrap_or(Value::Null);
                    let frame = self.frames.pop().ok_or("Frame stack underflow")?;
                    self.stack.truncate(frame.stack_offset);
                    self.stack.push(return_value);
                }
                
                OpCode::MakeList(count) => {
                    let mut items = Vec::new();
                    for _ in 0..count {
                        items.push(self.stack.pop().ok_or("Stack underflow")?);
                    }
                    items.reverse();
                    self.stack.push(Value::List(items));
                }
                
                OpCode::BuildRange => {
                    let end = self.stack.pop().ok_or("Stack underflow")?;
                    let start = self.stack.pop().ok_or("Stack underflow")?;
                    
                    match (&start, &end) {
                        (Value::Number(a), Value::Number(b)) => {
                            let mut items = Vec::new();
                            let start_i = *a as i64;
                            let end_i = *b as i64;
                            for i in start_i..=end_i {
                                items.push(Value::Number(i as f64));
                            }
                            self.stack.push(Value::List(items));
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::GetIndex => {
                    let index = self.stack.pop().ok_or("Stack underflow")?;
                    let list = self.stack.pop().ok_or("Stack underflow")?;
                    
                    match (&list, &index) {
                        (Value::List(items), Value::Number(idx)) => {
                            let i = *idx as usize;
                            let value = items.get(i).cloned().unwrap_or(Value::Null);
                            self.stack.push(value);
                        }
                        _ => self.stack.push(Value::Null),
                    }
                }
                
                OpCode::DefineClass(_name_idx) => {
                    self.stack.pop();
                }
                
                OpCode::GetProperty(name_idx) => {
                    let instance = self.stack.pop().ok_or("Stack underflow")?;
                    let name_value = self.frames[frame_idx].chunk.constants.get(name_idx).ok_or("Invalid constant index")?;
                    let prop_name = if let Value::String(n) = name_value {
                        n.clone()
                    } else {
                        return Err("Property name must be a string".to_string());
                    };
                    
                    match instance {
                        Value::Instance { fields, methods, field_access, method_access, class_name, .. } => {
                            let current_context = self.frames.last().and_then(|f| f.class_context.clone());
                            
                            if let Some(field_value) = fields.borrow().get(&prop_name) {
                                if let Some(access) = field_access.get(&prop_name) {
                                    use crate::parser::ast::AccessModifier;
                                    match access {
                                        AccessModifier::Private => {
                                            if current_context.as_ref() != Some(&class_name) {
                                                return Err(self.runtime_error(&format!("Cannot access private field '{}' from outside class", prop_name)));
                                            }
                                        }
                                        AccessModifier::Protected => {
                                            if current_context.is_none() {
                                                return Err(self.runtime_error(&format!("Cannot access protected field '{}' from outside class hierarchy", prop_name)));
                                            }
                                        }
                                        AccessModifier::Public => {}
                                    }
                                }
                                self.stack.push(field_value.clone());
                            }
                            else if let Some(method) = methods.get(&prop_name) {
                                if let Some(access) = method_access.get(&prop_name) {
                                    use crate::parser::ast::AccessModifier;
                                    match access {
                                        AccessModifier::Private => {
                                            if current_context.as_ref() != Some(&class_name) {
                                                return Err(self.runtime_error(&format!("Cannot access private method '{}' from outside class", prop_name)));
                                            }
                                        }
                                        AccessModifier::Protected => {
                                            if current_context.is_none() {
                                                return Err(self.runtime_error(&format!("Cannot access protected method '{}' from outside class hierarchy", prop_name)));
                                            }
                                        }
                                        AccessModifier::Public => {}
                                    }
                                }
                                self.stack.push(Value::BoundMethod {
                                    receiver: Box::new(Value::Instance {
                                        class_name: "".to_string(),
                                        fields: fields.clone(),
                                        field_access: field_access.clone(),
                                        methods: methods.clone(),
                                        method_access: method_access.clone(),
                                        static_methods: HashMap::new(),
                                    }),
                                    method: Box::new(method.clone()),
                                });
                            } else {
                                return Err(self.runtime_error(&format!("Undefined property '{}'", prop_name)));
                            }
                        }
                        Value::Class { static_methods, .. } => {
                            if let Some(static_method) = static_methods.get(&prop_name) {
                                self.stack.push(static_method.clone());
                            } else {
                                return Err(self.runtime_error(&format!("Undefined static method '{}'", prop_name)));
                            }
                        }
                        _ => return Err(self.runtime_error("Only instances and classes have properties")),
                    }
                }
                
                OpCode::SetProperty(name_idx) => {
                    let value = self.stack.pop().ok_or("Stack underflow")?;
                    let instance = self.stack.pop().ok_or("Stack underflow")?;
                    let name_value = self.frames[frame_idx].chunk.constants.get(name_idx).ok_or("Invalid constant index")?;
                    let prop_name = if let Value::String(n) = name_value {
                        n.clone()
                    } else {
                        return Err(self.runtime_error("Property name must be a string"));
                    };
                    
                    match instance {
                        Value::Instance { fields, field_access, class_name, .. } => {
                            let current_context = self.frames.last().and_then(|f| f.class_context.clone());
                            
                            if let Some(access) = field_access.get(&prop_name) {
                                use crate::parser::ast::AccessModifier;
                                match access {
                                    AccessModifier::Private => {
                                        // private: only accessible within same class
                                        if current_context.as_ref() != Some(&class_name) {
                                            return Err(self.runtime_error(&format!("Cannot set private field '{}' from outside class", prop_name)));
                                        }
                                    }
                                    AccessModifier::Protected => {
                                        // protected: accessible within class and subclasses
                                        if current_context.is_none() {
                                            return Err(self.runtime_error(&format!("Cannot set protected field '{}' from outside class hierarchy", prop_name)));
                                        }
                                    }
                                    AccessModifier::Public => {
                                        // public: accessible from anywhere
                                        // don't do anything here
                                    }
                                }
                            }
                            fields.borrow_mut().insert(prop_name, value.clone());
                            self.stack.push(value);
                        }
                        _ => return Err(self.runtime_error("Only instances have fields")),
                    }
                }
                
                OpCode::GetSuper(name_idx) => {
                    let name_value = self.frames[frame_idx].chunk.constants.get(name_idx).ok_or("Invalid constant index")?;
                    let method_name = if let Value::String(n) = name_value {
                        n.clone()
                    } else {
                        return Err(self.runtime_error("Method name must be a string"));
                    };
                    
                    let superclass = self.stack.pop().ok_or("Stack underflow")?;
                    let instance = self.stack.pop().ok_or("Stack underflow")?;
                    
                    match superclass {
                        Value::Class { methods, .. } => {
                            if let Some(method) = methods.get(&method_name) {
                                if let Value::Instance { class_name, fields, field_access, methods: inst_methods, method_access, static_methods } = instance {
                                    self.stack.push(Value::BoundMethod {
                                        receiver: Box::new(Value::Instance {
                                            class_name,
                                            fields,
                                            field_access,
                                            methods: inst_methods,
                                            method_access,
                                            static_methods,
                                        }),
                                        method: Box::new(method.clone()),
                                    });
                                } else {
                                    return Err(self.runtime_error("Super can only be used with instances"));
                                }
                            } else {
                                return Err(self.runtime_error(&format!("Undefined method '{}' in superclass", method_name)));
                            }
                        }
                        _ => return Err(self.runtime_error("Superclass must be a class")),
                    }
                }
                
                OpCode::Inherit => {
                    let subclass_name_val = self.stack.pop().ok_or("Stack underflow")?;
                    let superclass = self.stack.pop().ok_or("Stack underflow")?;
                    
                    let subclass_name = if let Value::String(n) = subclass_name_val {
                        n
                    } else {
                        return Err(self.runtime_error("Class name must be a string"));
                    };
                    
                    if let Value::Class { methods: super_methods, field_access: super_field_access, method_access: super_method_access, static_methods: super_static_methods, .. } = superclass {
                        if let Some(Value::Class { name, superclass: _, methods: subclass_methods, field_access: subclass_field_access, method_access: subclass_method_access, static_methods: subclass_static_methods }) = self.globals.get(&subclass_name).cloned() {
                            let mut merged_methods = super_methods.clone();
                            for (method_name, method_value) in subclass_methods {
                                merged_methods.insert(method_name, method_value);
                            }
                            
                            // merge static methods
                            let mut merged_static_methods = super_static_methods.clone();
                            for (method_name, method_value) in subclass_static_methods {
                                merged_static_methods.insert(method_name, method_value);
                            }
                            
                            // merge access modifiers
                            let mut merged_field_access = super_field_access.clone();
                            for (field_name, access) in subclass_field_access {
                                merged_field_access.insert(field_name, access);
                            }
                            
                            let mut merged_method_access = super_method_access.clone();
                            for (method_name, access) in subclass_method_access {
                                merged_method_access.insert(method_name, access);
                            }
                            
                            let new_class = Value::Class {
                                name: name.clone(),
                                superclass: Some(Box::new(Value::Class {
                                    name: "".to_string(),
                                    superclass: None,
                                    field_access: super_field_access.clone(),
                                    method_access: super_method_access.clone(),
                                    methods: super_methods.clone(),
                                    static_methods: super_static_methods.clone(),
                                })),
                                field_access: merged_field_access,
                                method_access: merged_method_access,
                                methods: merged_methods,
                                static_methods: merged_static_methods,
                            };
                            
                            self.globals.insert(subclass_name, new_class);
                        }
                    } else {
                        return Err(self.runtime_error("Superclass must be a class"));
                    }
                }
                
                OpCode::Pop => {
                    self.stack.pop();
                }
                
                OpCode::Print => {
                    let value = self.stack.pop().ok_or("Stack underflow")?;
                    println!("{:?}", value);
                }
                
                OpCode::Dup => {
                    let value = self.stack.last().cloned().ok_or("Stack underflow")?;
                    self.stack.push(value);
                }
            }
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Null => false,
            Value::List(l) => !l.is_empty(),
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
