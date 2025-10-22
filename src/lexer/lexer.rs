pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
    pub line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    pub fn current_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    pub fn peek_char(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            if c == '\n' {
                self.line += 1;
            }
        }
        self.pos += 1;
    }

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
