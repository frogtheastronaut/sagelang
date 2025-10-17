/// Bytecode instructions for the SageLang VM
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Literal values
    LoadConst(usize),       // Load constant from constant pool
    LoadTrue,               // Load boolean true
    LoadFalse,              // Load boolean false
    LoadNull,               // Load null value
    
    // Variables
    GetGlobal(usize),       // Get global variable by name index
    SetGlobal(usize),       // Set global variable by name index
    GetLocal(usize),        // Get local variable by stack index
    SetLocal(usize),        // Set local variable by stack index
    
    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,
    
    // Comparison operations
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // Control flow
    Jump(usize),            // Unconditional jump to instruction
    JumpIfFalse(usize),     // Jump if top of stack is falsy
    JumpIfTrue(usize),      // Jump if top of stack is truthy
    Loop(usize),            // Jump backwards (for loops)
    
    // Functions
    Call(usize),            // Call function with N arguments
    Return,                 // Return from function
    
    // Collections
    MakeList(usize),        // Create list from N stack items
    BuildRange,             // Build range from two numbers on stack
    GetIndex,               // Get item from list by index
    
    // Other
    Pop,                    // Pop and discard top of stack
    Print,                  // Print top of stack
    Dup,                    // Duplicate top of stack
}
