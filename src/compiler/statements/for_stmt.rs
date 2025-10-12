use crate::parser::ast::{Expr, Stmt};
use crate::lexer::tokens::Token;
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn for_stmt(compiler: &mut Compiler, var: &String, iterable: &Expr, body: &Vec<Stmt>) {
    if let Expr::BinaryOp { left, op, right } = iterable {
        if *op == Token::DotDot {
            compiler.compile_expr(left);
            compiler.bytecode.instructions.push(Instruction::Store(var.clone()));
            let cond_pos = compiler.bytecode.instructions.len();
            compiler.bytecode.instructions.push(Instruction::Load(var.clone()));
            compiler.compile_expr(right);
            compiler.bytecode.instructions.push(Instruction::Less);
            let jump_if_false_pos = compiler.bytecode.instructions.len();
            compiler.bytecode.instructions.push(Instruction::JumpIfFalse(0));
            compiler.compile_stmts(body);
            compiler.bytecode.instructions.push(Instruction::Load(var.clone()));
            compiler.bytecode.instructions.push(Instruction::Const(1.0));
            compiler.bytecode.instructions.push(Instruction::Add);
            compiler.bytecode.instructions.push(Instruction::Store(var.clone()));
            compiler.bytecode.instructions.push(Instruction::Jump(cond_pos));
            let after_body = compiler.bytecode.instructions.len();
            compiler.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(after_body);
            return;
        }
    }
    compiler.bytecode.instructions.push(Instruction::Const(0.0));
    compiler.bytecode.instructions.push(Instruction::Store(var.clone()));
    let cond_pos = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions.push(Instruction::Load(var.clone()));
    compiler.compile_expr(iterable);
    compiler.bytecode.instructions.push(Instruction::Less);
    let jump_if_false_pos = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions.push(Instruction::JumpIfFalse(0));
    compiler.compile_stmts(body);
    compiler.bytecode.instructions.push(Instruction::Load(var.clone()));
    compiler.bytecode.instructions.push(Instruction::Const(1.0));
    compiler.bytecode.instructions.push(Instruction::Add);
    compiler.bytecode.instructions.push(Instruction::Store(var.clone()));
    compiler.bytecode.instructions.push(Instruction::Jump(cond_pos));
    let after_body = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(after_body);
}
