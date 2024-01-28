use std::{any::Any, fmt};

#[derive(Debug)]
pub struct Token<'b> {
    token_type: TokenType<'b>,
    lexeme: &'b str,
    line: usize,
}

impl<'b> Token<'b> {
    pub fn new(token_type: TokenType<'b>, lexeme: &'b str, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl<'b> fmt::Display for Token<'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType<'b> {
    // Single character tokens
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

    // One or two character tokens
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
    String { literal: &'b str },
    Number { literal: f64 },

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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

    Eof,
}
