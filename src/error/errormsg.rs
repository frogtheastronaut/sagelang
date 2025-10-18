use std::process;

pub fn parser_error(message: &str, line: usize) -> ! {
    eprintln!("[ERR] Parse error: {} at line {}", message, line);
    process::exit(1);
}

pub fn lexer_error(message: &str) -> ! {
    eprintln!("[ERR] Lexer error: {}", message);
    process::exit(1);
}

pub fn runtime_error(message: &str) -> ! {
    eprintln!("[ERR] Runtime error: {}", message);
    process::exit(1);
}

pub fn error(message: &str) -> ! {
    eprintln!("[ERR] {}", message);
    process::exit(1);
}
