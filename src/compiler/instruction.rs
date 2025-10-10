#[derive(Debug)]
pub enum Instruction {
    Const(f64),      // Push constant onto the stack
    Add,              // Pop two values, add them
    Sub,
    Mul,
    Div,
    Store(String),    // Store value into variable
    Load(String),     // Load variable value onto stack
    JumpIfFalse(usize), // Conditional jump
    Jump(usize),      // Unconditional jump
    Call(String),     // Call function
    Return,
    Print,
    PushString(String),
    PushBool(bool),
    PushList(Vec<f64>),
    DefFunc { name: String, params: Vec<String>, body: Vec<Instruction> },
    CallFunc(String),
    EqEq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    And,
    Or,
}

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
}
