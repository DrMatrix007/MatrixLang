use crate::{errors::LanguageError, expressions::Expression, tokens::errors::TokenErrorType};

#[derive(Debug, Clone, Copy)]
pub enum ExpressionErrorType {
    TokenError(TokenErrorType),
}

pub type ExpressionError = LanguageError<ExpressionErrorType>;

pub type ExpressionResult<'a> = Result<Expression<'a>, ExpressionError>;
