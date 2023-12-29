use anyhow::*;
use std::rc::Rc;

pub struct Scanner {
    start: usize,
    current: usize,
    source: Rc<String>,
    pub line: usize,
}

pub struct Token {
    pub typ: TokenType,
    pub source: Rc<String>,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: Rc<String>) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            source,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        self.skip_whitespace();

        match self.advance() {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::Semicolon),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.peek2() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.scan_token()
                } else {
                    self.make_token(TokenType::Slash)
                }
            }
            '"' => self.string(),
            other => self.error_token(format!("Unexpected character: {}", other)),
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source
            .as_str()
            .chars()
            .nth(self.current)
            .unwrap_or_else(|| panic!("Could not get {}th  character", self.current))
    }

    fn peek2(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source
            .as_str()
            .chars()
            .nth(self.current + 1)
            .unwrap_or_else(|| panic!("Could not get {}th  character", self.current + 1))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self
            .source
            .as_str()
            .chars()
            .nth(self.current - 1)
            .unwrap_or_else(|| panic!("Could not get {}th  character", self.current - 1));
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while let ' ' | '\r' | '\t' = self.peek() {
            self.advance();
        }
    }

    fn make_token(&self, typ: TokenType) -> Token {
        Token {
            typ,
            source: self.source.clone(),
            start: self.start,
            end: self.current,
            line: self.line,
        }
    }

    fn error_token(&self, msg: String) -> Token {
        Token {
            typ: TokenType::Error,
            start: 0,
            end: msg.len() - 1,
            source: Rc::new(msg),
            line: self.line,
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token(String::from("Unterminated string"));
        }

        // The closing quote
        self.advance();

        self.make_token(TokenType::String)
    }
}

#[derive(strum_macros::Display)]
pub enum TokenType {
    // Single char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two chars
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    EOF,
}
