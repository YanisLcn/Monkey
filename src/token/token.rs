use std::{fmt, mem};

#[derive(Clone, Debug, Eq, Hash)]
pub enum Token {
    ILLEGAL(String),
    EOF,

    // Identifiers & literals
    IDENT(String), // foo, bar, x, y, ...
    INT(String),
    STRING(String),

    // Operators
    ASSIGN,
    EQ,
    NE,
    BANG,
    PLUS,
    SUB,
    DIV,
    MUL,
    GT,
    LT,

    // Delimiters
    COMMA,
    SEMICOLON,
    COLON,
    QUOTE,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    // Keywords
    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ILLEGAL(s) => write!(f, "ILLEGAL : {}", s),
            Token::EOF => write!(f, "EOF"),
            Token::IDENT(s) => write!(f, "{}", s),
            Token::INT(s) => write!(f, "{}", s),
            Token::STRING(s) => write!(f, "{}", s),
            Token::ASSIGN => write!(f, "="),
            Token::EQ => write!(f, "=="),
            Token::NE => write!(f, "!="),
            Token::BANG => write!(f, "!"),
            Token::PLUS => write!(f, "+"),
            Token::SUB => write!(f, "-"),
            Token::DIV => write!(f, "/"),
            Token::MUL => write!(f, "*"),
            Token::GT => write!(f, ">"),
            Token::LT => write!(f, "<"),
            Token::COMMA => write!(f, ","),
            Token::SEMICOLON => write!(f, ";"),
            Token::COLON => write!(f, ":"),
            Token::QUOTE => write!(f, "\""),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::LBRACKET => write!(f, "["),
            Token::RBRACKET => write!(f, "]"),
            Token::FUNCTION => write!(f, "FUNCTION"),
            Token::LET => write!(f, "LET"),
            Token::IF => write!(f, "IF"),
            Token::ELSE => write!(f, "ELSE"),
            Token::RETURN => write!(f, "RETURN"),
            Token::TRUE => write!(f, "TRUE"),
            Token::FALSE => write!(f, "FALSE"),
        }
    }
}
