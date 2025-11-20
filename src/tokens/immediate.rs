use std::{fmt::Display, string};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Immediate {
    String(String),
    Number(Number),
}
impl Immediate {
    pub(crate) fn len(&self) -> usize {
        match self {
            Immediate::String(string) => string.len() + 2, // 2 is for the ""
            Immediate::Number(number) => number.len(),
        }
    }
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immediate::String(string) => write!(f, "ImmediateString({})", string),
            Immediate::Number(number) => write!(f, "ImmediateNumber({:?})", number),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    I32(String),
    F32(String),
}
impl Number {
    fn len(&self) -> usize {
        match self {
            Number::I32(string) => string.len(),
            Number::F32(string) => string.len(),
        }
    }
}

impl Eq for Number {}
