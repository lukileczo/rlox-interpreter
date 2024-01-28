use core::f64;
use std::{any::Any, collections::HashMap};

use crate::{
    error::ErrorReporter,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn from(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn add_token(&mut self, token: TokenType<'a>) {
        let txt = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token, txt, self.line))
    }

    fn compare_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> char {
        let ret = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ret
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // fractional part
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // consume '.'
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let n = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::Number { literal: n });
    }

    fn identifier(&mut self) {
        while self.peek().is_alphabetic() {
            self.advance();
        }

        use TokenType::*;

        let keywords = HashMap::from([
            ("and", And),
            ("class", Class),
            ("else", Else),
            ("false", False),
            ("for", For),
            ("fun", Fun),
            ("if", If),
            ("nil", Nil),
            ("or", Or),
            ("print", Print),
            ("return", Return),
            ("super", Super),
            ("this", This),
            ("true", True),
            ("var", Var),
            ("while", While),
        ]);

        let txt = &self.source[self.start..self.current];
        let tk = keywords.get(txt).unwrap_or(&Identifier);

        self.add_token(*tk);
    }

    fn string(&mut self, reporter: &mut ErrorReporter) {
        while self.peek() != '"' && !self.is_at_end() {
            if (self.peek() == '\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            reporter.report();
            return;
        }

        self.advance();

        let val = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String { literal: val });
    }

    fn scan_token(&mut self, reporter: &mut ErrorReporter) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.compare_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.compare_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.compare_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.compare_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok);
            }
            '/' => {
                if self.compare_char('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(reporter),
            //  words
            'o' => {
                if self.compare_char('r') {
                    self.add_token(TokenType::Or)
                }
            }

            // numbers, reserved & default
            _ => {
                if (c.is_digit(10)) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    reporter.report()
                }
            }
        };
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self, reporter: &mut ErrorReporter) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(reporter);
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.line));

        &self.tokens
    }
}
