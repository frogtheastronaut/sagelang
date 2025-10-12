use crate::parser::ast::Expr;
use crate::compiler::instruction::Instruction;
use crate::compiler::Compiler;

pub fn call(compiler: &mut Compiler, callee: &Expr, args: &Vec<Expr>) {
    for arg in args {
        compiler.compile_expr(arg);
    }
    if let Expr::Identifier(name) = callee {
        compiler.bytecode.instructions.push(Instruction::CallFunc(name.clone()));
    }
}
