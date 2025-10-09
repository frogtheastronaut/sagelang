use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // literals
    Number(f64),
    Identifier(String),

    // ops
    Plus,
    Minus,
    Star,
    Slash,
    Eq,          // =
    EqEq,        // ==
    NotEq,       // !=
    Less,        // <
    LessEq,      // <=
    Greater,     // >
    GreaterEq,   // >=
    Semicolon,   // ;
	OpenBrace,   // {
	CloseBrace,  // }

    // symbols
    LParen,
    RParen,
    Colon,
    Quote,

    // keywords
    Let,
    Fn,
    If,
    Else,
    Return,

    // EOF
    EOF,
}

/// single-character symbols
pub fn default_symbol_map() -> HashMap<char, Token> {
    let mut map = HashMap::new();
    map.insert('+', Token::Plus);
    map.insert('-', Token::Minus);
    map.insert('*', Token::Star);
    map.insert('/', Token::Slash);
    map.insert('(', Token::LParen);
    map.insert(')', Token::RParen);
    map.insert(':', Token::Colon);
    map.insert('\'', Token::Quote);
	map.insert('=', Token::Eq);
	map.insert('<', Token::Less);
	map.insert('>', Token::Greater);
	map.insert(';', Token::Semicolon);
	map.insert('{', Token::OpenBrace);
	map.insert('}', Token::CloseBrace);
    map
}

/// multi-character operators
pub fn multi_char_ops() -> HashMap<&'static str, Token> {
    let mut map = HashMap::new();
    map.insert("==", Token::EqEq);
    map.insert("!=", Token::NotEq);
    map.insert(">=", Token::GreaterEq);
    map.insert("<=", Token::LessEq);
    map
}

/// keywords
pub fn keywords() -> HashMap<&'static str, Token> {
    let mut map = HashMap::new();
    map.insert("let", Token::Let);
    map.insert("fn", Token::Fn);
    map.insert("if", Token::If);
    map.insert("else", Token::Else);
    map.insert("return", Token::Return);
    map
}
