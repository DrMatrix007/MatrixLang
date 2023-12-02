use crate::tokens::Token;

#[derive(Debug)]
pub enum MLangError {
    TokenError(TokenError),
    CompilerError(CompilerError),
}

#[derive(Debug)]
pub enum TokenError {
    NotValidToken(String),
    UnexpectedToken(Token),
    MissingToken,
}

#[derive(Debug)]
pub enum CompilerError {}
