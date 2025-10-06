// The lexical analysis module
#[derive(Debug)]
pub enum Token {
    Name(String),
    String(String),
    Eos,
}

#[derive(Debug)]
pub struct Lexer {
    src: String,
    pos: usize,
}

impl Lexer {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let Some(c) = self.peek() else {
            return Token::Eos;
        };

        if c.is_ascii_alphabetic() || c == '_' {
            // Parse identifier
            let mut name = String::new();

            while let Some(c) = self.peek() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    name.push(c);
                    self.next_char();
                } else {
                    break;
                }
            }

            return Token::Name(name);
        }

        if c == '"' {
            // Parse string literal
            self.next_char(); // consume opening quote
            let mut s = String::new();

            while let Some(ch) = self.next_char() {
                if ch == '"' {
                    return Token::String(s);
                } else {
                    s.push(ch);
                }
            }

            // Unterminated string
            return Token::String(s);
        }

        // Unknown character — skip it for now
        self.next_char();
        self.next()
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }
}
