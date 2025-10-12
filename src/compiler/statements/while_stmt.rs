use crate::parser::ast::{Expr, Stmt};
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn while_stmt(compiler: &mut Compiler, condition: &Expr, body: &Vec<Stmt>) {
    let cond_pos = compiler.bytecode.instructions.len();
    compiler.compile_expr(condition);
    let jump_if_false_pos = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions.push(Instruction::JumpIfFalse(0));
    compiler.compile_stmts(body);
    compiler.bytecode.instructions.push(Instruction::Jump(cond_pos));
    let after_body = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(after_body);
}
