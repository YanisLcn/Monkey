use crate::token::token::Token;
use lazy_static::lazy_static;
use std::{char, collections::HashMap, usize};

pub struct Lexer {
    input: String,
    index: usize,
    read_index: usize,
    ch: char,
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, Token> = {
        let mut keywords = HashMap::new();
        keywords.insert(String::from("let"), Token::LET);
        keywords.insert(String::from("fn"), Token::FUNCTION);
        keywords.insert(String::from("if"), Token::IF);
        keywords.insert(String::from("else"), Token::ELSE);
        keywords.insert(String::from("true"), Token::TRUE);
        keywords.insert(String::from("false"), Token::FALSE);
        keywords.insert(String::from("return"), Token::RETURN);
        keywords
    };
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        let mut lexer = Self {
            input: input.into(),
            index: 0,
            read_index: 0,
            ch: '\0',
        };
        lexer.next_token();
        lexer
    }

    // TODO!: Support UNICODE
    pub fn read_char(&mut self) {
        if self.read_index >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_index).unwrap();
        }
        self.index = self.read_index;
        self.read_index += 1;
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_index >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_index).unwrap()
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let start_index = self.index;
        while Self::is_ident_letter(self.ch) {
            self.read_char();
        }
        self.input[start_index..self.index].to_string()
    }

    pub fn read_number(&mut self) -> String {
        let start_index = self.index;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[start_index..self.index].to_string()
    }

    pub fn read_string(&mut self) -> String {
        let start_index = self.index + 1;
        loop {
            self.read_char();
            match self.ch {
                '\"' | '\0' => break,
                _ => continue,
            }
        }
        self.input[start_index..self.index].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            '+' => Token::PLUS,
            '-' => Token::SUB,
            '/' => Token::DIV,
            '*' => Token::MUL,
            '>' => Token::GT,
            '<' => Token::LT,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NE
                } else {
                    Token::BANG
                }
            }

            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            ':' => Token::COLON,
            '"' => Token::STRING(self.read_string()),

            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '[' => Token::LBRACKET,
            ']' => Token::RBRACKET,

            '\0' => Token::EOF,
            c => {
                if Self::is_ident_letter(c) {
                    return Self::is_keyword(self.read_identifier());
                } else if c.is_digit(10) {
                    return Token::INT(self.read_number());
                } else {
                    Token::ILLEGAL(c.to_string())
                }
            }
        };
        self.read_char();
        tok
    }

    fn is_ident_letter(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_keyword(ident: String) -> Token {
        KEYWORDS
            .get(&ident)
            .unwrap_or(&Token::IDENT(String::from(ident)))
            .clone()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}
