mod lexer;

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

    // run the tokenizer loop
    loop {
        let tok = tokenizer.next_token();
        println!("{:?}", tok);
        if tok == lexer::Token::EOF {
            break;
        }
    }
}
