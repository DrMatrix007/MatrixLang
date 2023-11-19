use crate::tokens::Token;

#[derive(Debug)]
pub enum MLangError {
    TokenError(TokenError),
}

#[derive(Debug)]
pub enum TokenError {
    NotValidToken(String),
    UnexpectedToken(Token),
    MissingToken,
}
