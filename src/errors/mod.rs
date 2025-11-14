#[derive(Debug)]
pub enum TokenError {
    UnexpectedChar(char),
    NotValidNumber(String),
    NotValidOp(String),
}