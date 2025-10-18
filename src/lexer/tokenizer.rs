use crate::lexer::Lexer;
use crate::lexer::tokens::{Token, CurrentToken, default_symbol_map, multi_char_ops, keywords};
use std::collections::HashMap;
use crate::error::errormsg;

pub struct Tokenizer<'a> {
    lexer: &'a mut Lexer,
    symbols: HashMap<char, Token>,
    multi_ops: HashMap<&'static str, Token>,
    keywords: HashMap<&'static str, Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer,
            symbols: default_symbol_map(),
            multi_ops: multi_char_ops(),
            keywords: keywords(),
        }
    }
	// parse numbers
    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(c) = self.lexer.current_char() {
            if c.is_ascii_digit() {
                num_str.push(c);
            } else if c == '.' && !has_dot {
				// number has decimal point
                has_dot = true;
                num_str.push(c);
            } else {
                break;
            }
            self.lexer.advance();
        }

        Token::Number(num_str.parse::<f64>().unwrap())
    }
    // skip comments
    fn skip_comment(&mut self) {
        // advance past both slashes
        self.lexer.advance();
        self.lexer.advance();
        // skip until end of line or EOF
        while let Some(c) = self.lexer.current_char() {
            if c == '\n' {
                break;
            }
            self.lexer.advance();
        }
    }

	// parse identifiers and keywords
    fn identifier(&mut self) -> Token {
        let mut id = String::new();

        while let Some(c) = self.lexer.current_char() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.lexer.advance();
            } else {
                break;
            }
        }

        // check if it's a keyword
        if let Some(keyword_token) = self.keywords.get(id.as_str()) {
            keyword_token.clone()
        } else if let Some(op_token) = self.multi_ops.get(id.as_str()) {
            op_token.clone()
        } else {
            // it's a regular identifier
            Token::Identifier(id)
        }
    }
	// parse multi-character operators
    fn multi_char_op(&mut self) -> Option<Token> {
        let c1 = self.lexer.current_char()?;
        let c2 = self.lexer.peek_char()?;
        let op_str = format!("{}{}", c1, c2);

        if let Some(token) = self.multi_ops.get(op_str.as_str()) {
            self.lexer.advance();
            self.lexer.advance();
            Some(token.clone())
        } else {
            None
        }
    }
    // get next token
    pub fn next_token(&mut self) -> CurrentToken {
        self.lexer.skip_whitespace();
        let line = self.lexer.line;

        // handle comments
        if let Some('/') = self.lexer.current_char() {
            if let Some('/') = self.lexer.peek_char() {
                self.skip_comment();
                return self.next_token(); // get the next token
            }
        }

        if let Some(tok) = self.multi_char_op() {
            return CurrentToken { token: tok, line };
        }
        let token = match self.lexer.current_char() {
            Some('"') => self.string_lit(),
            Some('t') | Some('f') => {
                let id = self.identifier();
                if let Token::Identifier(ref s) = id {
                    if s == "true" {
                        return CurrentToken { token: Token::Bool(true), line };
                    } else if s == "false" {
                        return CurrentToken { token: Token::Bool(false), line };
                    }
                }
                id
            }
            Some('[') => self.list_lit(),
            Some(c) if c.is_ascii_digit() => self.number(),
            Some(c) if c.is_alphabetic() || c == '_' => self.identifier(),
            Some(c) => {
                if let Some(tok) = self.symbols.get(&c) {
                    self.lexer.advance();
                    tok.clone()
                } else {
                    errormsg::lexer_error(&format!("Unexpected character: '{}'", c));
                }
            }
            None => Token::EOF,
        };
        CurrentToken { token, line }
    }

    // parse string literals
    fn string_lit(&mut self) -> Token {
        self.lexer.advance(); // skip opening quote
        let mut s = String::new();
        while let Some(c) = self.lexer.current_char() {
            if c == '"' {
                self.lexer.advance();
                break;
            } else if c == '\\' {
                self.lexer.advance();
                if let Some(esc) = self.lexer.current_char() {
                    s.push(esc);
                    self.lexer.advance();
                }
            } else {
                s.push(c);
                self.lexer.advance();
            }
        }
        Token::StringLit(s)
    }

    // parse list literals
    fn list_lit(&mut self) -> Token {
        self.lexer.advance(); // skip [
        let mut items = Vec::new();
        loop {
            self.lexer.skip_whitespace();
            if let Some(c) = self.lexer.current_char() {
                if c == ']' {
                    self.lexer.advance();
                    break;
                }
                // parse item (number, string, bool, identifier)
                let item = self.next_token();
                if item.token != Token::RBracket && item.token != Token::EOF {
                    items.push(item.token);
                }
                self.lexer.skip_whitespace();
                if let Some(',') = self.lexer.current_char() {
                    self.lexer.advance();
                }
            } else {
                break;
            }
        }
        Token::List(items)
    }
}
