use crate::tokens::Token;

#[derive(Debug, Clone)]
pub enum LangError {
    UnexpectedEOF,
    TokenError(TokenError),
    FunctionError(FunctionError)
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
