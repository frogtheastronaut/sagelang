use crate::interpreter::Value;
use super::opcode::OpCode;

/// A chunk of bytecode with its associated constant pool
#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub name: String,
}

impl Chunk {
    pub fn new(name: String) -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            name,
        }
    }
    
    /// Add an instruction to the chunk
    pub fn write(&mut self, op: OpCode) {
        self.code.push(op);
    }
    
    /// Add a constant to the constant pool and return its index
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
    
    /// Disassemble the chunk for debugging
    pub fn disassemble(&self) {
        println!("== {} ==", self.name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    
    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        
        match &self.code[offset] {
            OpCode::LoadConst(idx) => {
                println!("LoadConst {} ({:?})", idx, self.constants.get(*idx));
                offset + 1
            }
            OpCode::LoadTrue => {
                println!("LoadTrue");
                offset + 1
            }
            OpCode::LoadFalse => {
                println!("LoadFalse");
                offset + 1
            }
            OpCode::LoadNull => {
                println!("LoadNull");
                offset + 1
            }
            OpCode::GetGlobal(idx) => {
                println!("GetGlobal {}", idx);
                offset + 1
            }
            OpCode::SetGlobal(idx) => {
                println!("SetGlobal {}", idx);
                offset + 1
            }
            OpCode::GetLocal(idx) => {
                println!("GetLocal {}", idx);
                offset + 1
            }
            OpCode::SetLocal(idx) => {
                println!("SetLocal {}", idx);
                offset + 1
            }
            OpCode::Add => {
                println!("Add");
                offset + 1
            }
            OpCode::Subtract => {
                println!("Subtract");
                offset + 1
            }
            OpCode::Multiply => {
                println!("Multiply");
                offset + 1
            }
            OpCode::Divide => {
                println!("Divide");
                offset + 1
            }
            OpCode::Modulo => {
                println!("Modulo");
                offset + 1
            }
            OpCode::Negate => {
                println!("Negate");
                offset + 1
            }
            OpCode::Equal => {
                println!("Equal");
                offset + 1
            }
            OpCode::NotEqual => {
                println!("NotEqual");
                offset + 1
            }
            OpCode::Greater => {
                println!("Greater");
                offset + 1
            }
            OpCode::GreaterEqual => {
                println!("GreaterEqual");
                offset + 1
            }
            OpCode::Less => {
                println!("Less");
                offset + 1
            }
            OpCode::LessEqual => {
                println!("LessEqual");
                offset + 1
            }
            OpCode::Jump(addr) => {
                println!("Jump -> {}", addr);
                offset + 1
            }
            OpCode::JumpIfFalse(addr) => {
                println!("JumpIfFalse -> {}", addr);
                offset + 1
            }
            OpCode::JumpIfTrue(addr) => {
                println!("JumpIfTrue -> {}", addr);
                offset + 1
            }
            OpCode::Loop(addr) => {
                println!("Loop -> {}", addr);
                offset + 1
            }
            OpCode::Call(arg_count) => {
                println!("Call {}", arg_count);
                offset + 1
            }
            OpCode::Return => {
                println!("Return");
                offset + 1
            }
            OpCode::MakeList(count) => {
                println!("MakeList {}", count);
                offset + 1
            }
            OpCode::BuildRange => {
                println!("BuildRange");
                offset + 1
            }
            OpCode::GetIndex => {
                println!("GetIndex");
                offset + 1
            }
            OpCode::Pop => {
                println!("Pop");
                offset + 1
            }
            OpCode::Print => {
                println!("Print");
                offset + 1
            }
            OpCode::Dup => {
                println!("Dup");
                offset + 1
            }
        }
    }
}
