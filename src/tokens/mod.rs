use std::fmt::Display;

pub mod errors;
pub mod tokenizer;

#[derive(Debug)]
pub enum MatrixTokenType {
    Function,
    Identifier,
    Integer,
    Float,
    String,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Assign,
    If,
    Else,
    Return,
}

impl Display for MatrixTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct MatrixToken<'a> {
    pub token_type: MatrixTokenType,
    pub data: &'a str,
}

impl<'a> Display for MatrixToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({}, \"{}\")", self.token_type, self.data)
    }
}

impl<'a> MatrixToken<'a> {
    pub fn new(token_type: MatrixTokenType, data: &'a str) -> Self {
        Self { token_type, data }
    }
}
