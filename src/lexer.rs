// The lexical analysis module
#[derive(Debug)]
pub enum Token {
    // keywords
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,
    // +       -       *       /       %       ^       #
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Len,
    // &       ~       |       <<      >>      //
    BitAnd,
    BitXor,
    BitOr,
    ShiftLeft,
    ShiftRight,
    Idiv,
    // ==       ~=     <=      >=      <       >        =
    Eq,
    NotEq,
    LessEq,
    GreaterEq,
    Less,
    Greater,
    Assign,
    // (       )       {       }       [       ]       ::
    ParLeft,
    ParRight,
    CurlyLeft,
    CurlyRight,
    SqurLeft,
    SqurRight,
    DoubColon,
    SemiColon,
    Colon,
    Comma,
    Dot,
    Concat,
    Dots,
    // const values
    ConstInteger(i64),
    ConstFloat(f64),
    ConstString(String),
    // name of variables or table keys
    Name(String),
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

        if let Some(token) = self.parse_keyword(c) {
            return token;
        }

        if let Some(token) = self.parse_number(c) {
            return token;
        }

        if let Some(token) = self.parse_string(c) {
            return token;
        }

        // ---------- Operators & punctuation ----------
        macro_rules! next_if {
            ($ch:expr) => {
                if self.peek_next() == Some($ch) {
                    self.next_char();
                    true
                } else {
                    false
                }
            };
        }

        let ch = self.next_char().unwrap();
        match ch {
            '+' => Token::Add,
            '-' => {
                if next_if!('-') {
                    // comment (skip line)
                    while let Some(c) = self.next_char() {
                        if c == '\n' {
                            break;
                        }
                    }
                    return self.next();
                }
                Token::Sub
            }
            '*' => Token::Mul,
            '/' => {
                if next_if!('/') {
                    Token::Idiv
                } else {
                    Token::Div
                }
            }
            '%' => Token::Mod,
            '^' => Token::Pow,
            '#' => Token::Len,
            '&' => Token::BitAnd,
            '~' => {
                if next_if!('=') {
                    Token::NotEq
                } else {
                    Token::BitXor
                }
            }
            '|' => Token::BitOr,
            '<' => {
                if next_if!('<') {
                    Token::ShiftLeft
                } else if next_if!('=') {
                    Token::LessEq
                } else {
                    Token::Less
                }
            }
            '>' => {
                if next_if!('>') {
                    Token::ShiftRight
                } else if next_if!('=') {
                    Token::GreaterEq
                } else {
                    Token::Greater
                }
            }
            '=' => {
                if next_if!('=') {
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '(' => Token::ParLeft,
            ')' => Token::ParRight,
            '{' => Token::CurlyLeft,
            '}' => Token::CurlyRight,
            '[' => Token::SqurLeft,
            ']' => Token::SqurRight,
            ':' => {
                if next_if!(':') {
                    Token::DoubColon
                } else {
                    Token::Colon
                }
            }
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            '.' => {
                if next_if!('.') {
                    if next_if!('.') {
                        Token::Dots
                    } else {
                        Token::Concat
                    }
                } else {
                    Token::Dot
                }
            }
            _ => self.next(), // skip unknowns
        }
    }

    fn parse_keyword(&mut self, c: char) -> Option<Token> {
        if c.is_ascii_alphabetic() || c == '_' {
            let mut name = String::new();

            while let Some(ch) = self.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    name.push(ch);
                    self.next_char();
                } else {
                    break;
                }
            }

            let token = match name.as_str() {
                "and" => Token::And,
                "break" => Token::Break,
                "do" => Token::Do,
                "else" => Token::Else,
                "elseif" => Token::Elseif,
                "end" => Token::End,
                "false" => Token::False,
                "for" => Token::For,
                "function" => Token::Function,
                "goto" => Token::Goto,
                "if" => Token::If,
                "in" => Token::In,
                "local" => Token::Local,
                "nil" => Token::Nil,
                "not" => Token::Not,
                "or" => Token::Or,
                "repeat" => Token::Repeat,
                "return" => Token::Return,
                "then" => Token::Then,
                "true" => Token::True,
                "until" => Token::Until,
                "while" => Token::While,
                _ => Token::Name(name),
            };

            return Some(token);
        }

        None
    }

    fn parse_number(&mut self, c: char) -> Option<Token> {
        if c.is_ascii_digit() {
            let mut number = String::new();
            let mut is_float = false;

            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    number.push(ch);
                    self.next_char();
                } else if ch == '.' {
                    if is_float {
                        break;
                    }
                    is_float = true;
                    number.push(ch);
                    self.next_char();
                } else {
                    break;
                }
            }

            if is_float {
                if let Ok(v) = number.parse::<f64>() {
                    return Some(Token::ConstFloat(v));
                }
            } else if let Ok(v) = number.parse::<i64>() {
                return Some(Token::ConstInteger(v));
            }
        }

        None
    }

    fn parse_string(&mut self, c: char) -> Option<Token> {
        if c == '"' || c == '\'' {
            let quote = c;
            self.next_char(); // consume opening quote
            let mut s = String::new();

            while let Some(ch) = self.next_char() {
                if ch == quote {
                    return Some(Token::ConstString(s));
                } else {
                    s.push(ch);
                }
            }

            // Unterminated string (still return)
            return Some(Token::ConstString(s));
        }

        None
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut iter = self.src[self.pos..].chars();
        iter.next()?; // skip current char
        iter.next()
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
