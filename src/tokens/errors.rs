use crate::errors::LanguageError;

use super::Token;

#[derive(Debug, Clone, Copy)]
pub enum TokenErrorType {
    UnexpectedChar(char),
}

pub type TokenError = LanguageError<TokenErrorType>;

pub type TokenResult<'string> = Result<Token<'string>, TokenError>;
