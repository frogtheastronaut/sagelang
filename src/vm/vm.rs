use crate::interpreter::Value;
use super::chunk::Chunk;
use super::opcode::OpCode;
use std::collections::HashMap;

/// Call frame for function calls
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack_offset: usize,
}

/// Virtual Machine for executing bytecode
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
    
    pub fn run(&mut self, chunk: Chunk) -> Result<(), String> {
        if self.debug {
            chunk.disassemble();
        }
        
        let frame = CallFrame {
            chunk,
            ip: 0,
            stack_offset: 0,
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
                // End of chunk
                self.frames.pop();
                if self.frames.is_empty() {
                    return Ok(());
                }
                continue;
            }
            
            let instruction = self.frames[frame_idx].chunk.code[ip].clone();
            self.frames[frame_idx].ip += 1;
            
            if self.debug {
                println!("Stack before {:?}: {:?}", instruction, self.stack);
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
                        return Err("Invalid global variable name".to_string());
                    };
                    
                    let value = self.globals.get(&name).cloned().unwrap_or(Value::Null);
                    self.stack.push(value);
                }
                
                OpCode::SetGlobal(idx) => {
                    let name = if let Value::String(s) = &self.frames[frame_idx].chunk.constants[idx] {
                        s.clone()
                    } else {
                        return Err("Invalid global variable name".to_string());
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
                        // Extend stack if necessary
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
                    // Stack layout: [..., func, arg1, arg2, ...argN]
                    // The function is at position: stack.len() - arg_count - 1
                    let func_index = self.stack.len() - arg_count - 1;
                    let function = self.stack[func_index].clone();
                    
                    match function {
                        Value::Function { name: _, param_count, chunk } => {
                            // Verify argument count
                            if arg_count != param_count {
                                return Err(format!("Expected {} arguments but got {}", param_count, arg_count));
                            }
                            
                            // Remove the function from the stack, keeping arguments
                            // Stack layout after: [..., arg1, arg2, ...argN]
                            self.stack.remove(func_index);
                            
                            // Set up call frame with arguments on stack
                            // stack_offset points to where arg1 now is
                            let stack_offset = self.stack.len() - arg_count;
                            
                            let new_frame = CallFrame {
                                chunk,
                                ip: 0,
                                stack_offset,
                            };
                            
                            self.frames.push(new_frame);
                        }
                        _ => return Err("Attempted to call non-function".to_string()),
                    }
                }
                
                OpCode::Return => {
                    let return_value = self.stack.pop().unwrap_or(Value::Null);
                    
                    // Pop the current frame
                    let frame = self.frames.pop().ok_or("Frame stack underflow")?;
                    
                    // Clean up the stack to the point before the function call
                    self.stack.truncate(frame.stack_offset);
                    
                    // Push the return value
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
