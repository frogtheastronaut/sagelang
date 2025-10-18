mod lexer;
mod parser;
mod interpreter;
mod compiler;
mod vm;
mod error;

use std::fs;
use std::env;
use crate::error::errormsg;

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [--debug]", args[0]);
        return;
    }
    let filename = &args[1];
    let debug = args.len() > 2 && args[2] == "--debug";

    // read the file into a string
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // create the lexer and tokenizer
    let mut lexer = lexer::Lexer::new(&contents);
    let mut tokenizer = lexer::Tokenizer::new(&mut lexer);
    let mut parser = parser::Parser::new(&mut tokenizer);

    let ast = parser.parse();
    
    if debug {
        // write AST to ast.txt for debugging
        let ast_str = format!("{:?}", ast);
        fs::write("ast.txt", ast_str).expect("[ERR] Failed to write AST");
        print!("[SAGE] Wrote AST")
    }

    // compile to bytecode
    let mut compiler = compiler::Compiler::new();
    let chunk = match compiler.compile(&ast) {
        Ok(chunk) => chunk,
        Err(e) => {
            errormsg::error(&format!("Compilation error: {}", e));
        }
    };

    // run in VM
    let mut vm = vm::VM::new();
    vm.debug = debug;
    
    if let Err(e) = vm.run(chunk) {
        errormsg::runtime_error(&e);
    }
}
