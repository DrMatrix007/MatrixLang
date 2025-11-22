use crate::tokens::token_type::TokenType;

pub mod cursor;
pub mod lexer;
pub mod op;
pub mod sub_parsers;
pub mod token_type;

pub type Token = Spanned<TokenType>;

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T> {
    pub value: T,
    pub len: usize,
}
