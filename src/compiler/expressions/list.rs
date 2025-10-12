use crate::parser::ast::Expr;
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn list(compiler: &mut Compiler, items: &Vec<Expr>) {
    let mut vals = Vec::new();
    for item in items {
        if let Expr::Number(n) = item {
            vals.push(*n);
        }
    }
    compiler.bytecode.instructions.push(Instruction::PushList(vals));
}
