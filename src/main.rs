
mod lexer;
mod parser;
mod interpreter;
mod compiler;
mod vm;
mod error;
mod gpu;

use std::{env, fs};
use crate::error::errormsg;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [--debug]", args[0]);
        return;
    }
    let filename = &args[1];
    let debug = args.get(2).map_or(false, |f| f == "--debug");

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("[ERR] Failed to read file: {}", filename);
            return;
        }
    };

    let mut lexer = lexer::Lexer::new(&contents);
    let mut tokenizer = lexer::Tokenizer::new(&mut lexer);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let ast = parser.parse();

    if debug {
        let ast_str = format!("{:?}", ast);
        if fs::write("ast.txt", ast_str).is_err() {
            eprintln!("[ERR] Failed to write AST");
        }
    }

    let mut compiler = compiler::Compiler::new();
    compiler.debug = debug;
    let chunk = match compiler.compile(&ast) {
        Ok(chunk) => chunk,
        Err(e) => {
            errormsg::error(&format!("Compilation error: {}", e));
        }
    };

    let mut vm = vm::VM::new();
    vm.debug = debug;
    if let Err(e) = vm.run(chunk) {
        let line = e.rsplit("[line ").next().and_then(|s| s.trim_end_matches(']').parse::<usize>().ok()).unwrap_or(0);
        let message = e.split(" [line ").next().unwrap_or(&e);
        errormsg::runtime_error(message, line);
    }
}
