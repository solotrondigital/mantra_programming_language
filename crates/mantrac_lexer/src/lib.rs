pub mod span;
pub mod token;

use span::Span;
use token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
    pub code: &'static str,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<(usize, char)>,
    cursor: usize,
    line: usize,
    col: usize,
    indent_stack: Vec<usize>,
    paren_depth: usize,
    pending_tokens: Vec<Token>,
    at_line_start: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        Lexer {
            source,
            chars,
            cursor: 0,
            line: 1,
            col: 1,
            indent_stack: vec![0],
            paren_depth: 0,
            pending_tokens: Vec::new(),
            at_line_start: true,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        if !self.pending_tokens.is_empty() {
            return Ok(self.pending_tokens.remove(0));
        }

        // Handle line beginning indentation if at line start
        if self.at_line_start && self.cursor < self.chars.len() {
            let line_start_pos = self.current_pos();
            let mut indent_spaces = 0;
            let mut blank_or_comment = false;

            while self.cursor < self.chars.len() {
                let (_, ch) = self.chars[self.cursor];
                if ch == ' ' {
                    indent_spaces += 1;
                    self.advance();
                } else if ch == '\t' {
                    return Err(LexerError {
                        message: "Tabs in indentation are rejected. Use spaces.".to_string(),
                        span: Span::new(self.line, self.col, line_start_pos, self.current_pos() + 1),
                        code: "E0001",
                    });
                } else if ch == '#' {
                    // Comment until end of line
                    while self.cursor < self.chars.len() && self.chars[self.cursor].1 != '\n' {
                        self.advance();
                    }
                    blank_or_comment = true;
                    break;
                } else if ch == '\r' || ch == '\n' {
                    blank_or_comment = true;
                    break;
                } else {
                    break;
                }
            }

            self.at_line_start = false;

            if !blank_or_comment && self.cursor < self.chars.len() && self.paren_depth == 0 {
                let current_indent = *self.indent_stack.last().unwrap_or(&0);
                if indent_spaces > current_indent {
                    self.indent_stack.push(indent_spaces);
                    return Ok(Token::new(
                        TokenKind::Indent,
                        Span::new(self.line, 1, line_start_pos, self.current_pos()),
                    ));
                } else if indent_spaces < current_indent {
                    while let Some(&top) = self.indent_stack.last() {
                        if indent_spaces < top {
                            self.indent_stack.pop();
                            self.pending_tokens.push(Token::new(
                                TokenKind::Dedent,
                                Span::new(self.line, 1, line_start_pos, self.current_pos()),
                            ));
                        } else if indent_spaces == top {
                            break;
                        } else {
                            return Err(LexerError {
                                message: format!(
                                    "Inconsistent indentation: unindent does not match any outer indentation level"
                                ),
                                span: Span::new(self.line, 1, line_start_pos, self.current_pos()),
                                code: "E0002",
                            });
                        }
                    }
                    if !self.pending_tokens.is_empty() {
                        return Ok(self.pending_tokens.remove(0));
                    }
                }
            }
        }

        self.skip_inline_whitespace();

        if self.cursor >= self.chars.len() {
            // End of file: emit remaining dedents
            if self.indent_stack.len() > 1 {
                self.indent_stack.pop();
                return Ok(Token::new(
                    TokenKind::Dedent,
                    Span::new(self.line, self.col, self.source.len(), self.source.len()),
                ));
            }
            return Ok(Token::new(
                TokenKind::Eof,
                Span::new(self.line, self.col, self.source.len(), self.source.len()),
            ));
        }

        let start_pos = self.current_pos();
        let start_line = self.line;
        let start_col = self.col;
        let ch = self.chars[self.cursor].1;

        // Comments
        if ch == '#' {
            while self.cursor < self.chars.len() && self.chars[self.cursor].1 != '\n' {
                self.advance();
            }
            return self.next_token();
        }

        // Newline
        if ch == '\n' || ch == '\r' {
            if ch == '\r' && self.peek_char() == Some('\n') {
                self.advance();
            }
            self.advance();
            self.line += 1;
            self.col = 1;
            self.at_line_start = true;

            if self.paren_depth == 0 {
                return Ok(Token::new(
                    TokenKind::Newline,
                    Span::new(start_line, start_col, start_pos, self.current_pos()),
                ));
            } else {
                return self.next_token();
            }
        }

        // Parentheses / Brackets
        match ch {
            '(' => {
                self.advance();
                self.paren_depth += 1;
                return Ok(Token::new(TokenKind::LParen, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            ')' => {
                self.advance();
                if self.paren_depth > 0 { self.paren_depth -= 1; }
                return Ok(Token::new(TokenKind::RParen, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '[' => {
                self.advance();
                self.paren_depth += 1;
                return Ok(Token::new(TokenKind::LBracket, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            ']' => {
                self.advance();
                if self.paren_depth > 0 { self.paren_depth -= 1; }
                return Ok(Token::new(TokenKind::RBracket, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '{' => {
                self.advance();
                self.paren_depth += 1;
                return Ok(Token::new(TokenKind::LBrace, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '}' => {
                self.advance();
                if self.paren_depth > 0 { self.paren_depth -= 1; }
                return Ok(Token::new(TokenKind::RBrace, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            ':' => {
                self.advance();
                return Ok(Token::new(TokenKind::Colon, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            ',' => {
                self.advance();
                return Ok(Token::new(TokenKind::Comma, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '.' => {
                self.advance();
                return Ok(Token::new(TokenKind::Dot, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            ';' => {
                self.advance();
                return Ok(Token::new(TokenKind::Semicolon, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '@' => {
                self.advance();
                return Ok(Token::new(TokenKind::At, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '+' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::PlusEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Ok(Token::new(TokenKind::Plus, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '-' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '>' {
                    self.advance();
                    return Ok(Token::new(TokenKind::Arrow, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::MinusEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Ok(Token::new(TokenKind::Minus, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '*' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::StarEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Ok(Token::new(TokenKind::Star, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '/' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::SlashEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Ok(Token::new(TokenKind::Slash, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '%' => {
                self.advance();
                return Ok(Token::new(TokenKind::Percent, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '=' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::EqEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Ok(Token::new(TokenKind::Eq, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '!' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::NotEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Err(LexerError {
                    message: "Unexpected character '!'. Did you mean '!='?".to_string(),
                    span: Span::new(start_line, start_col, start_pos, self.current_pos()),
                    code: "E0003",
                });
            }
            '<' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::LtEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Ok(Token::new(TokenKind::Lt, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '>' => {
                self.advance();
                if self.cursor < self.chars.len() && self.chars[self.cursor].1 == '=' {
                    self.advance();
                    return Ok(Token::new(TokenKind::GtEq, Span::new(start_line, start_col, start_pos, self.current_pos())));
                }
                return Ok(Token::new(TokenKind::Gt, Span::new(start_line, start_col, start_pos, self.current_pos())));
            }
            '"' => {
                return self.read_string();
            }
            _ => {}
        }

        // Numbers
        if ch.is_ascii_digit() {
            return self.read_number();
        }

        // Identifiers & Keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.read_identifier_or_keyword();
        }

        Err(LexerError {
            message: format!("Unexpected character: '{}'", ch),
            span: Span::new(start_line, start_col, start_pos, start_pos + 1),
            code: "E0004",
        })
    }

    fn read_string(&mut self) -> Result<Token, LexerError> {
        let start_pos = self.current_pos();
        let start_line = self.line;
        let start_col = self.col;
        self.advance(); // skip opening quote

        let mut content = String::new();
        while self.cursor < self.chars.len() {
            let (_, ch) = self.chars[self.cursor];
            if ch == '"' {
                self.advance();
                return Ok(Token::new(
                    TokenKind::StringLit(content),
                    Span::new(start_line, start_col, start_pos, self.current_pos()),
                ));
            } else if ch == '\\' {
                self.advance();
                if self.cursor >= self.chars.len() {
                    break;
                }
                let (_, escaped) = self.chars[self.cursor];
                match escaped {
                    'n' => content.push('\n'),
                    't' => content.push('\t'),
                    'r' => content.push('\r'),
                    '\\' => content.push('\\'),
                    '"' => content.push('"'),
                    _ => {
                        content.push('\\');
                        content.push(escaped);
                    }
                }
                self.advance();
            } else if ch == '\n' {
                return Err(LexerError {
                    message: "Unterminated string literal at end of line".to_string(),
                    span: Span::new(start_line, start_col, start_pos, self.current_pos()),
                    code: "E0005",
                });
            } else {
                content.push(ch);
                self.advance();
            }
        }

        Err(LexerError {
            message: "Unterminated string literal at end of file".to_string(),
            span: Span::new(start_line, start_col, start_pos, self.current_pos()),
            code: "E0005",
        })
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let start_pos = self.current_pos();
        let start_line = self.line;
        let start_col = self.col;

        let mut num_str = String::new();
        let mut is_float = false;

        while self.cursor < self.chars.len() {
            let (_, ch) = self.chars[self.cursor];
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float && self.peek_char().map_or(false, |c| c.is_ascii_digit()) {
                is_float = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let span = Span::new(start_line, start_col, start_pos, self.current_pos());
        if is_float {
            match num_str.parse::<f64>() {
                Ok(val) => Ok(Token::new(TokenKind::Float(val), span)),
                Err(_) => Err(LexerError {
                    message: format!("Invalid float literal: {}", num_str),
                    span,
                    code: "E0006",
                }),
            }
        } else {
            match num_str.parse::<i64>() {
                Ok(val) => Ok(Token::new(TokenKind::Integer(val), span)),
                Err(_) => Err(LexerError {
                    message: format!("Invalid integer literal: {}", num_str),
                    span,
                    code: "E0006",
                }),
            }
        }
    }

    fn read_identifier_or_keyword(&mut self) -> Result<Token, LexerError> {
        let start_pos = self.current_pos();
        let start_line = self.line;
        let start_col = self.col;

        let mut ident = String::new();
        while self.cursor < self.chars.len() {
            let (_, ch) = self.chars[self.cursor];
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let span = Span::new(start_line, start_col, start_pos, self.current_pos());
        let kind = match ident.as_str() {
            "module" => TokenKind::Module,
            "import" => TokenKind::Import,
            "from" => TokenKind::From,
            "as" => TokenKind::As,
            "def" => TokenKind::Def,
            "return" => TokenKind::Return,
            "class" => TokenKind::Class,
            "interface" => TokenKind::Interface,
            "extends" => TokenKind::Extends,
            "implements" => TokenKind::Implements,
            "abstract" => TokenKind::Abstract,
            "override" => TokenKind::Override,
            "static" => TokenKind::Static,
            "pub" => TokenKind::Pub,
            "private" => TokenKind::Private,
            "protected" => TokenKind::Protected,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "while" => TokenKind::While,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "True" => TokenKind::True,
            "False" => TokenKind::False,
            "None" => TokenKind::None,
            "self" => TokenKind::SelfKw,
            "super" => TokenKind::Super,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            _ => TokenKind::Ident(ident),
        };

        Ok(Token::new(kind, span))
    }

    fn advance(&mut self) {
        if self.cursor < self.chars.len() {
            self.cursor += 1;
            self.col += 1;
        }
    }

    fn current_pos(&self) -> usize {
        if self.cursor < self.chars.len() {
            self.chars[self.cursor].0
        } else {
            self.source.len()
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.cursor + 1 < self.chars.len() {
            Some(self.chars[self.cursor + 1].1)
        } else {
            None
        }
    }

    fn skip_inline_whitespace(&mut self) {
        while self.cursor < self.chars.len() {
            let (_, ch) = self.chars[self.cursor];
            if ch == ' ' || ch == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }
}

