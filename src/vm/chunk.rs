use crate::interpreter::Value;
use super::opcode::OpCode;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
    pub name: String,
}

impl Chunk {
    pub fn new(name: String) -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            name,
        }
    }
    
    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
    
    pub fn disassemble(&self) {
        let mut output = String::new();
        output.push_str(&format!("== {} ==\n", self.name));
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction_to_string(offset, &mut output);
        }
        
        if let Ok(mut file) = File::create("bytecode.txt") {
            let _ = file.write_all(output.as_bytes());
        } else {
        }
    }
    
    fn disassemble_instruction_to_string(&self, offset: usize, output: &mut String) -> usize {
        output.push_str(&format!("{:04} ", offset));
        
        match &self.code[offset] {
            OpCode::LoadConst(idx) => {
                output.push_str(&format!("LoadConst {} ({:?})\n", idx, self.constants.get(*idx)));
                offset + 1
            }
            OpCode::LoadTrue => {
                output.push_str("LoadTrue\n");
                offset + 1
            }
            OpCode::LoadFalse => {
                output.push_str("LoadFalse\n");
                offset + 1
            }
            OpCode::LoadNull => {
                output.push_str("LoadNull\n");
                offset + 1
            }
            OpCode::GetGlobal(idx) => {
                output.push_str(&format!("GetGlobal {}\n", idx));
                offset + 1
            }
            OpCode::SetGlobal(idx) => {
                output.push_str(&format!("SetGlobal {}\n", idx));
                offset + 1
            }
            OpCode::GetLocal(idx) => {
                output.push_str(&format!("GetLocal {}\n", idx));
                offset + 1
            }
            OpCode::SetLocal(idx) => {
                output.push_str(&format!("SetLocal {}\n", idx));
                offset + 1
            }
            OpCode::Add => {
                output.push_str("Add\n");
                offset + 1
            }
            OpCode::Subtract => {
                output.push_str("Subtract\n");
                offset + 1
            }
            OpCode::Multiply => {
                output.push_str("Multiply\n");
                offset + 1
            }
            OpCode::Divide => {
                output.push_str("Divide\n");
                offset + 1
            }
            OpCode::Modulo => {
                output.push_str("Modulo\n");
                offset + 1
            }
            OpCode::Negate => {
                output.push_str("Negate\n");
                offset + 1
            }
            OpCode::Equal => {
                output.push_str("Equal\n");
                offset + 1
            }
            OpCode::NotEqual => {
                output.push_str("NotEqual\n");
                offset + 1
            }
            OpCode::Greater => {
                output.push_str("Greater\n");
                offset + 1
            }
            OpCode::GreaterEqual => {
                output.push_str("GreaterEqual\n");
                offset + 1
            }
            OpCode::Less => {
                output.push_str("Less\n");
                offset + 1
            }
            OpCode::LessEqual => {
                output.push_str("LessEqual\n");
                offset + 1
            }
            OpCode::Jump(addr) => {
                output.push_str(&format!("Jump -> {}\n", addr));
                offset + 1
            }
            OpCode::JumpIfFalse(addr) => {
                output.push_str(&format!("JumpIfFalse -> {}\n", addr));
                offset + 1
            }
            OpCode::JumpIfTrue(addr) => {
                output.push_str(&format!("JumpIfTrue -> {}\n", addr));
                offset + 1
            }
            OpCode::Loop(addr) => {
                output.push_str(&format!("Loop -> {}\n", addr));
                offset + 1
            }
            OpCode::Call(arg_count) => {
                output.push_str(&format!("Call {}\n", arg_count));
                offset + 1
            }
            OpCode::Return => {
                output.push_str("Return\n");
                offset + 1
            }
            OpCode::MakeList(count) => {
                output.push_str(&format!("MakeList {}\n", count));
                offset + 1
            }
            OpCode::BuildRange => {
                output.push_str("BuildRange\n");
                offset + 1
            }
            OpCode::GetIndex => {
                output.push_str("GetIndex\n");
                offset + 1
            }
            OpCode::DefineClass(name_idx) => {
                output.push_str(&format!("DefineClass {}\n", name_idx));
                offset + 1
            }
            OpCode::GetProperty(name_idx) => {
                output.push_str(&format!("GetProperty {}\n", name_idx));
                offset + 1
            }
            OpCode::SetProperty(name_idx) => {
                output.push_str(&format!("SetProperty {}\n", name_idx));
                offset + 1
            }
            OpCode::GetSuper(method_idx) => {
                output.push_str(&format!("GetSuper {}\n", method_idx));
                offset + 1
            }
            OpCode::Inherit => {
                output.push_str("Inherit\n");
                offset + 1
            }
            OpCode::Pop => {
                output.push_str("Pop\n");
                offset + 1
            }
            OpCode::Print => {
                output.push_str("Print\n");
                offset + 1
            }
            OpCode::Dup => {
                output.push_str("Dup\n");
                offset + 1
            }
            OpCode::MetalInit => {
                output.push_str("MetalInit\n");
                offset + 1
            }
            OpCode::MetalLoadKernel(idx) => {
                output.push_str(&format!("MetalLoadKernel {}\n", idx));
                offset + 1
            }
            OpCode::MetalExecute => {
                output.push_str("MetalExecute\n");
                offset + 1
            }
            OpCode::CudaInit => {
                output.push_str("CudaInit\n");
                offset + 1
            }
            OpCode::CudaLoadKernel(idx) => {
                output.push_str(&format!("CudaLoadKernel {}\n", idx));
                offset + 1
            }
            OpCode::CudaExecute => {
                output.push_str("CudaExecute\n");
                offset + 1
            }
        }
    }
}
