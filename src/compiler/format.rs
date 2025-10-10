use crate::compiler::instruction::Instruction;

pub fn format_instruction(instr: &Instruction) -> String {
    match instr {
        Instruction::Const(n) => format!("PUSH {}", n),
        Instruction::Add => "ADD".to_string(),
        Instruction::Sub => "SUB".to_string(),
        Instruction::Mul => "MUL".to_string(),
        Instruction::Div => "DIV".to_string(),
        Instruction::Modulo => "MODULO".to_string(),
        Instruction::Store(name) => format!("STORE {}", name),
        Instruction::Load(name) => format!("LOAD {}", name),
        Instruction::JumpIfFalse(addr) => format!("JUMP_IF_FALSE {}", addr),
        Instruction::Jump(addr) => format!("JUMP {}", addr),
        Instruction::Return => "RETURN".to_string(),
        Instruction::Print => "PRINT".to_string(),
        Instruction::PushString(s) => format!("PUSH_STR {}", s),
        Instruction::PushBool(b) => format!("PUSH_BOOL {}", b),
        Instruction::PushList(v) => format!("PUSH_LIST {:?}", v),
        Instruction::DefFunc { name, params, body } => {
            let body_str = body.iter().map(|i| format_instruction(i)).collect::<Vec<_>>().join("\n    ");
            format!("DEF_FUNC {} {:?}\n    {}\nEND_DEF", name, params, body_str)
        },
        Instruction::CallFunc(name) => format!("CALL_FUNC {}", name),
        Instruction::EqEq => "EQEQ".to_string(),
        Instruction::NotEq => "NOTEQ".to_string(),
        Instruction::Less => "LT".to_string(),
        Instruction::LessEq => "LTE".to_string(),
        Instruction::Greater => "GT".to_string(),
        Instruction::GreaterEq => "GTE".to_string(),
        Instruction::And => "AND".to_string(),
        Instruction::Or => "OR".to_string(),
    }
}