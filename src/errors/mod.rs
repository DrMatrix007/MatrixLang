use crate::{expressions::Expression, tokens::Token};

#[derive(Debug)]
pub enum LangError {
    UnexpectedEOF,
    TokenError(TokenError),
    FunctionError(FunctionError),
    UnexpectedToken(Token),
    CantCompile(String),
    TokenShouldBe {
        should_be: Token,
        got: Token
    }
}

#[derive(Debug, Clone)]
pub enum TokenError {
    UnexpectedChar(char),
    NotValidNumber(String),
    NotValidOp(String),
}

#[derive(Debug, Clone)]
pub enum FunctionError {
    FunctionNameShouldBeIdentifier(Token),
}
