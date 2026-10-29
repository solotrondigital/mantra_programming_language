#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Name(String),
    Int(i64),
    Float(f64),
    String(String),
    Exclamation,     // !
    Dollar,          // $
    LeftParen,       // (
    RightParen,      // )
    Colon,           // :
    Equals,          // =
    At,              // @
    LeftBracket,     // [
    RightBracket,    // ]
    LeftBrace,       // {
    RightBrace,      // }
    Pipe,            // |
    Ellipsis,        // ...
    Eof,
}

pub struct Lexer<'a> {
    chars: Vec<char>,
    pos: usize,
    _source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
            _source: source,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() || ch == ',' {
                self.advance();
            } else if ch == '#' {
                // Comment until end of line
                while let Some(c) = self.advance() {
                    if c == '\n' || c == '\r' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();

        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        match ch {
            '!' => { self.advance(); Ok(Token::Exclamation) }
            '$' => { self.advance(); Ok(Token::Dollar) }
            '(' => { self.advance(); Ok(Token::LeftParen) }
            ')' => { self.advance(); Ok(Token::RightParen) }
            ':' => { self.advance(); Ok(Token::Colon) }
            '=' => { self.advance(); Ok(Token::Equals) }
            '@' => { self.advance(); Ok(Token::At) }
            '[' => { self.advance(); Ok(Token::LeftBracket) }
            ']' => { self.advance(); Ok(Token::RightBracket) }
            '{' => { self.advance(); Ok(Token::LeftBrace) }
            '}' => { self.advance(); Ok(Token::RightBrace) }
            '|' => { self.advance(); Ok(Token::Pipe) }
            '.' => {
                if self.pos + 2 < self.chars.len()
                    && self.chars[self.pos + 1] == '.'
                    && self.chars[self.pos + 2] == '.'
                {
                    self.pos += 3;
                    Ok(Token::Ellipsis)
                } else {
                    Err(format!("Unexpected character '.' at position {}", self.pos))
                }
            }
            '"' => self.lex_string(),
            '-' | '0'..='9' => self.lex_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_name(),
            other => Err(format!("Unexpected character '{}' at position {}", other, self.pos)),
        }
    }

    fn lex_name(&mut self) -> Result<Token, String> {
        let mut name = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Ok(Token::Name(name))
    }

    fn lex_number(&mut self) -> Result<Token, String> {
        let mut s = String::new();
        if self.peek() == Some('-') {
            s.push('-');
            self.advance();
        }
        let mut is_float = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                s.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            s.parse::<f64>()
                .map(Token::Float)
                .map_err(|e| format!("Invalid float '{}': {}", s, e))
        } else {
            s.parse::<i64>()
                .map(Token::Int)
                .map_err(|e| format!("Invalid integer '{}': {}", s, e))
        }
    }

    fn lex_string(&mut self) -> Result<Token, String> {
        self.advance(); // consume opening '"'
        let mut s = String::new();
        while let Some(ch) = self.advance() {
            match ch {
                '"' => return Ok(Token::String(s)),
                '\\' => {
                    if let Some(escaped) = self.advance() {
                        match escaped {
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            '/' => s.push('/'),
                            'n' => s.push('\n'),
                            'r' => s.push('\r'),
                            't' => s.push('\t'),
                            other => s.push(other),
                        }
                    }
                }
                other => s.push(other),
            }
        }
        Err("Unterminated string literal in GraphQL query".to_string())
    }
}

