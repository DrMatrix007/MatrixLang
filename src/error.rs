#[derive(Debug)]
pub struct Error {
    line: usize,
    error: MLangError,
}

#[derive(Debug)]
pub enum MLangError {
    TokenError(TokenError),
}

#[derive(Debug)]
pub enum TokenError {
    UnexpectedToken(String),
}
