use std::fmt::Display;

use super::MatrixToken;

#[derive(Debug)]
pub enum MatrixTokenErrorType {
    UnexpectedChar(char),
}

#[derive(Debug)]
pub struct MatrixTokenError {
    pub index: usize,
    pub error: MatrixTokenErrorType,
}

pub type MatrixTokenResult<'a> = Result<MatrixToken<'a>, MatrixTokenError>;
