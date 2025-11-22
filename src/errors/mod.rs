use crate::{
    tokens::{token_type::TokenType},
};

#[derive(Debug)]
pub enum LangErrorKind {
    UnexpectedEOF,
    TokenError(TokenError),
    FunctionError(FunctionError),
    UnexpectedToken(TokenType),
    TokenShouldBe {
        should_be: TokenType,
        got: TokenType,
    },
}

#[derive(Debug, Clone)]
pub enum TokenError {
    UnexpectedChar(char),
    NotValidNumber,
    NotValidOp,
}

#[derive(Debug, Clone)]
pub enum FunctionError {
    FunctionNameShouldBeIdentifier(TokenType),
}
