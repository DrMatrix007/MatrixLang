use crate::{
    error::{MLangError, TokenError},
    literals::{self, NumberLiteral, StringLiteral},
};

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Identifier(Identifier),
    StringLiteral(literals::StringLiteral),
    NumberLiteral(literals::NumberLiteral),
    Keyword(Keyword),
}
impl Into<Token> for Operator {
    fn into(self) -> Token {
        Token::Operator(self)
    }
}
impl Into<Token> for Identifier {
    fn into(self) -> Token {
        Token::Identifier(self)
    }
}

impl Into<Token> for literals::StringLiteral {
    fn into(self) -> Token {
        Token::StringLiteral(self)
    }
}

impl Into<Token> for literals::NumberLiteral {
    fn into(self) -> Token {
        Token::NumberLiteral(self)
    }
}

impl Into<Token> for Keyword {
    fn into(self) -> Token {
        Token::Keyword(self)
    }
}
#[derive(Debug)]
pub enum Operator {
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
    Comma,
    Dot,
    Minus,
    Plus,
    Div,
    Mul,
    Smicolon,
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEq,
    Smaller,
    SmallerEq,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[derive(Debug)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    Function,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

pub fn tokenize(data: &str) -> Result<Vec<Token>, MLangError> {
    let mut ans = vec![];
    let mut iter = data.chars().peekable();
    let mut current_token;
    while let Some(current_char) = iter.next() {
        current_token = match current_char {
            '(' => Operator::ParenLeft.into(),
            ')' => Operator::ParenRight.into(),
            '{' => Operator::BraceLeft.into(),
            '}' => Operator::BraceRight.into(),
            ',' => Operator::Comma.into(),
            '.' => Operator::Dot.into(),
            '-' => Operator::Minus.into(),
            '+' => Operator::Plus.into(),
            '*' => Operator::Mul.into(),
            '/' => Operator::Div.into(),
            '!' => {
                if let Some(&'=') = iter.peek() {
                    iter.next();
                    Operator::NotEqual.into()
                } else {
                    Operator::Not.into()
                }
            }
            '=' => {
                if let Some(&'=') = iter.peek() {
                    iter.next();
                    Operator::EqualEqual.into()
                } else {
                    Operator::Equal.into()
                }
            }
            '>' => {
                if let Some(&'=') = iter.peek() {
                    iter.next();
                    Operator::GreaterEq.into()
                } else {
                    Operator::Greater.into()
                }
            }
            '<' => {
                if let Some(&'=') = iter.peek() {
                    iter.next();
                    Operator::SmallerEq.into()
                } else {
                    Operator::Smaller.into()
                }
            }
            c @ ('0'..='9') => {
                let s = core::iter::once(c)
                    .chain(std::iter::from_fn(|| {
                        let current = iter.peek().map(|c| *c);
                        if let Some(c @ ('0'..='9' | '.')) = current {
                            iter.next();
                            Some(c)
                        } else {
                            None
                        }
                    }))
                    .collect();
                NumberLiteral::new(s).into()
            }
            c @ ('a'..='z' | 'A'..='Z') => {
                let s = core::iter::once(c)
                    .chain(std::iter::from_fn(|| {
                        let current = iter.peek().map(|c| *c);
                        if let Some(c @ ('a'..='z' | 'A'..='Z' | '0'..='9')) = current {
                            iter.next();
                            Some(c)
                        } else {
                            None
                        }
                    }))
                    .collect();
                Identifier::new(s).into()
            }
            '"' => {
                let s = std::iter::from_fn(|| {
                    let current = iter.next();
                    if let Some(c) = current {
                        if c != '"' {
                            Some(c)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

                StringLiteral::new(s).into()
            }
            ' ' | '\n' => {
                continue;
            }
            c => {
                return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                    c.into(),
                )))
            }
        };
        ans.push(current_token);
    }
    Ok(ans)
}
