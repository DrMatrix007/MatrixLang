use std::fmt::Display;

pub mod errors;
pub mod tokenizer;

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Integer,
    Double,
    String,

    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Equals,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    Function,
    If,
    Else,
    Return,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub data: &'a str,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({}, \"{}\")", self.token_type, self.data)
    }
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, data: &'a str) -> Self {
        Self { token_type, data }
    }
}
