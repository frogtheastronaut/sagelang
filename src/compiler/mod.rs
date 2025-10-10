pub mod format;
pub mod instruction;
pub mod compiler;

pub use instruction::{Bytecode};
pub use compiler::Compiler;
pub use format::format_instruction;