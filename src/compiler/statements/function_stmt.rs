use crate::parser::ast::{Stmt, Param};
use crate::compiler::instruction::{Instruction, Bytecode};
use crate::compiler::Compiler;

pub fn function_stmt(
    compiler: &mut Compiler,
    name: &String,
    params: &Vec<Param>,
    body: &Vec<Stmt>
) {
    let mut func_compiler = Compiler { bytecode: Bytecode { instructions: vec![] } };
    func_compiler.compile_stmts(body);
    let param_names = params.iter().map(|p| p.param_name.clone()).collect();
    compiler.bytecode.instructions.push(Instruction::DefFunc {
        name: name.clone(),
        params: param_names,
        body: func_compiler.bytecode.instructions,
    });
}
