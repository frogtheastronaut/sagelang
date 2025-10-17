mod lexer;
mod parser;
mod interpreter;

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
    // write AST to ast.txt
    let ast_str = format!("{:?}", ast);
    fs::write("ast.txt", ast_str).expect("Failed to write AST");

    // run interpreter
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(&ast);
}
