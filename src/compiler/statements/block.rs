use crate::parser::ast::Stmt;
use crate::compiler::Compiler;

pub fn block(compiler: &mut Compiler, stmts: &Vec<Stmt>) {
    compiler.compile_stmts(stmts);
}
