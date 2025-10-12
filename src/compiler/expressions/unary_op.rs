use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn unary_op(compiler: &mut Compiler, op: &Token, right: &Expr) {
    compiler.compile_expr(right);
    match op {
        Token::Minus => compiler.bytecode.instructions.push(Instruction::Sub),
        Token::NotEq => compiler.bytecode.instructions.push(Instruction::NotEq),
        _ => {}
    }
}
