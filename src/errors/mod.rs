use crate::{expressions::Expression, tokens::Token};

#[derive(Debug)]
pub enum LangError {
    UnexpectedEOF,
    TokenError(TokenError),
    FunctionError(FunctionError),
    UnexpectedToken(Token),
    CantCompile(String),
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
    FunctionTokenHere {
        should_be: Token,
        got: Token
    }
}
