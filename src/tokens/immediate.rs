use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Immediate {
    String(String),
    Number(Number),
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immediate::String(string) => write!(f, "ImmediateString({})", string),
            Immediate::Number(number) => write!(f, "ImmediateNumber({:?})", number),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    I32(i32),
    F32(f32),
}
