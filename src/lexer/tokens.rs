use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // literals
    Number(f64),
    Identifier(String),
    StringLit(String),
    Bool(bool), // literal value: true/false
    List(Vec<Token>),

    // ops
    Plus,
    Minus,
    Star,
    Slash,
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
	Assign, // =
    LBracket, // [
    RBracket, // ]

    // keywords
    Let,
    Fn,
    If,
    Else,
    Return,
    NumKw,    // num keyword
    BoolKw,   // bool keyword
    ListKw,   // list keyword
    StrKw,    // str keyword
    WhileKw,  // while keyword
    ForKw,    // for keyword
    InKw,     // in keyword

	// other
	Comma, // ,
	Dot,   // .

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
	map.insert('=', Token::Assign);
	map.insert('<', Token::Less);
	map.insert('>', Token::Greater);
	map.insert(';', Token::Semicolon);
	map.insert('{', Token::OpenBrace);
	map.insert('}', Token::CloseBrace);
    map.insert('[', Token::LBracket);
    map.insert(']', Token::RBracket);
	map.insert(',', Token::Comma);
	map.insert('.', Token::Dot);
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
    map.insert("function", Token::Fn);
    map.insert("if", Token::If);
    map.insert("else", Token::Else);
    map.insert("return", Token::Return);
    map.insert("num", Token::NumKw);
    map.insert("bool", Token::BoolKw);
    map.insert("list", Token::ListKw);
    map.insert("str", Token::StrKw);
    map.insert("while", Token::WhileKw);
    map.insert("for", Token::ForKw);
    map.insert("in", Token::InKw);
    map.insert("true", Token::Bool(true));
    map.insert("false", Token::Bool(false));
    map
}
