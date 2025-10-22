// bytecode instructions
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // variables
    LoadConst(usize),       // load constant from constant pool
    LoadTrue,               // load boolean true
    LoadFalse,              // load boolean false
    LoadNull,               // load null value
    
    // variables
    GetGlobal(usize),       // get global variable by name index
    SetGlobal(usize),       // set global variable by name index
    GetLocal(usize),        // get local variable by stack index
    SetLocal(usize),        // set local variable by stack index
    
    // arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,
    
    // comparison ops
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // flow control
    Jump(usize),            // unconditional jump to instruction
    JumpIfFalse(usize),     // jump if top of stack is falsy
    JumpIfTrue(usize),      // jump if top of stack is truthy
    Loop(usize),            // jump backwards (for loops)
    
    // functions
    Call(usize),            // call function
    Return,                 // return from function
    
    // collections
    MakeList(usize),        // create list from N stack items
    BuildRange,             // build range from two numbers on stack
    GetIndex,               // get item from list by index
    
    // OOP-related
    DefineClass(usize),     // define a class with name index
    GetProperty(usize),     // get property from object (name index)
    SetProperty(usize),     // set property on object (name index)
    GetSuper(usize),        // get method from superclass (name index)
    Inherit,                // set up inheritance

    // Other
    Pop,                    // pop and discard top of stack
    Print,                  // print top of stack
    Dup,                    // duplicate top of stack
}
