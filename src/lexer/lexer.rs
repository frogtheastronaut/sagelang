pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }
	// get char at current position
    pub fn current_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }
	// peek at next char without advancing
    pub fn peek_char(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }
	// advance
    pub fn advance(&mut self) {
        self.pos += 1;
    }
	// skip whitespace
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}
