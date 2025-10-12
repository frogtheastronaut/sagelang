use crate::parser::ast::{Expr, Stmt};
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn if_stmt(
    compiler: &mut Compiler,
    condition: &Expr,
    then_branch: &Vec<Stmt>,
    else_branch: &Option<Vec<Stmt>>,
    elseif_branches: &Vec<(Expr, Vec<Stmt>)>
) {
    compiler.compile_expr(condition);
    let jump_if_false_pos = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions.push(Instruction::JumpIfFalse(0));
    compiler.compile_stmts(then_branch);
    let after_then_jump_pos = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions.push(Instruction::Jump(0));
    let elseif_start = compiler.bytecode.instructions.len();
    compiler.bytecode.instructions[jump_if_false_pos] = Instruction::JumpIfFalse(elseif_start);
    for (cond, branch) in elseif_branches {
        compiler.compile_expr(cond);
        let elseif_jump_if_false = compiler.bytecode.instructions.len();
        compiler.bytecode.instructions.push(Instruction::JumpIfFalse(0));
        compiler.compile_stmts(branch);
        let elseif_after_jump = compiler.bytecode.instructions.len();
        compiler.bytecode.instructions.push(Instruction::Jump(0));
        let elseif_next = compiler.bytecode.instructions.len();
        compiler.bytecode.instructions[elseif_jump_if_false] = Instruction::JumpIfFalse(elseif_next);
        compiler.bytecode.instructions[elseif_after_jump] = Instruction::Jump(compiler.bytecode.instructions.len());
    }
    if let Some(else_stmts) = else_branch {
        compiler.compile_stmts(else_stmts);
    }
    compiler.bytecode.instructions[after_then_jump_pos] = Instruction::Jump(compiler.bytecode.instructions.len());
}
