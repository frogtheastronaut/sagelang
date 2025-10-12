use crate::parser::ast::Expr;
use crate::lexer::tokens::Token;
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn binary_op(compiler: &mut Compiler, left: &Expr, op: &Token, right: &Expr) {
    compiler.compile_expr(left);
    compiler.compile_expr(right);
    match op {
        Token::Plus => compiler.bytecode.instructions.push(Instruction::Add),
        Token::Minus => compiler.bytecode.instructions.push(Instruction::Sub),
        Token::Star => compiler.bytecode.instructions.push(Instruction::Mul),
        Token::Slash => compiler.bytecode.instructions.push(Instruction::Div),
        Token::Percent => compiler.bytecode.instructions.push(Instruction::Modulo),
        Token::EqEq => compiler.bytecode.instructions.push(Instruction::EqEq),
        Token::NotEq => compiler.bytecode.instructions.push(Instruction::NotEq),
        Token::Less => compiler.bytecode.instructions.push(Instruction::Less),
        Token::LessEq => compiler.bytecode.instructions.push(Instruction::LessEq),
        Token::Greater => compiler.bytecode.instructions.push(Instruction::Greater),
        Token::GreaterEq => compiler.bytecode.instructions.push(Instruction::GreaterEq),
        Token::And => compiler.bytecode.instructions.push(Instruction::And),
        Token::Or => compiler.bytecode.instructions.push(Instruction::Or),
        _ => {}
    }
}
