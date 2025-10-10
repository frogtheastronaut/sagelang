mod lexer;
mod parser;
mod compiler;

use std::fs;
use std::env;

fn main() {
    // get command line args
    // we're assuming its first argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];

    // read the file into a string
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // create the lexer and tokenizer
    let mut lexer = lexer::Lexer::new(&contents);
    let mut tokenizer = lexer::Tokenizer::new(&mut lexer);
    let mut parser = parser::Parser::new(&mut tokenizer);

    let ast = parser.parse();
    let mut compiler = compiler::Compiler { bytecode: compiler::Bytecode { instructions: vec![] } };
    compiler.compile_stmts(&ast);

    // write bytecode to bytecode.txt
    let bytecode_str = compiler.bytecode.instructions.iter()
        .map(|instr| compiler::format_instruction(instr))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write("bytecode.txt", bytecode_str).expect("Failed to write bytecode");

    // VM logic removed. Only output bytecode to bytecode.txt
}
