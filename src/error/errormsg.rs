use std::process;
use colored::Colorize;

fn error_title() -> colored::ColoredString {
    "[ERR]".bold().red()
}

pub fn parser_error(message: &str, line: usize) -> ! {
    eprintln!("{} Parse error: {} at line {}", error_title(), message, line);
    process::exit(1);
}
pub fn lexer_error(message: &str) -> ! {
    eprintln!("{} Lexer error: {}", error_title(), message);
    process::exit(1);
}

pub fn runtime_error(message: &str, line: usize) -> ! {
    eprintln!("{} Runtime error: {} at line {}", error_title(), message, line);
    process::exit(1);
}
pub fn error(message: &str) -> ! {
    eprintln!("{} {}", error_title(), message);
    process::exit(1);
}
