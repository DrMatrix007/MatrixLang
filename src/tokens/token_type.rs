use std::fmt::Display;

use crate::tokens::op::Op;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Immediate(ImmediateType),
    Op(Op),
    Whitespace,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Tok(Identifier)"),
            TokenType::Immediate(im) => write!(f, "Tok({im})"),
            TokenType::Op(op) => write!(f, "Tok({})", op),
            TokenType::Whitespace => write!(f, "Tok( )"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImmediateType {
    String,
    Number(NumberType),
}

impl Display for ImmediateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmediateType::String => write!(f, "ImmediateString()"),
            ImmediateType::Number(number) => write!(f, "ImmediateNumber({:?})", number),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    I32,
    F32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Extern,
}

impl From<Keyword> for String {
    fn from(val: Keyword) -> Self {
        match val {
            Keyword::Fn => "fn".into(),
            Keyword::Extern => "extern".into(),
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "fn" => Ok(Keyword::Fn),
            "extern" => Ok(Keyword::Extern),
            _ => Err(()),
        }
    }
}
