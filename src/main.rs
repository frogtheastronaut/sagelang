mod lexer;
mod parser;
mod interpreter;
mod compiler;
mod vm;
mod error;

use std::fs;
use std::env;
use crate::error::errormsg;
use colored::Colorize;

fn debug_print(message: &str) {
    println!("{} {}", "[DEBUG]".bright_blue(), message);
}

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
        debug_print("AST written to ast.txt");
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
        // Try to extract line number from error message
        if let Some(line_str) = e.strip_suffix(']').and_then(|s| s.rsplit("[line ").next()) {
            if let Ok(line) = line_str.parse::<usize>() {
                let message = e.split(" [line ").next().unwrap_or(&e);
                errormsg::runtime_error(message, line);
            } else {
                errormsg::runtime_error(&e, 0);
            }
        } else {
            errormsg::runtime_error(&e, 0);
        }
    }
}
