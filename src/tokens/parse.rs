use crate::error::{MLangError, TokenError};

use super::{
    literals::{NumberLiteral, StringLiteral},
    Identifier, Keyword, Operator, Token,
};

pub fn parse_tokens(data: &str) -> Result<Vec<Token>, MLangError> {
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
            ';' => Operator::Semicolon.into(),
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
                        let current = iter.peek().copied();
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
                        let current = iter.peek().copied();
                        if let Some(c @ ('a'..='z' | 'A'..='Z' | '0'..='9')) = current {
                            iter.next();
                            Some(c)
                        } else {
                            None
                        }
                    }))
                    .collect::<String>();
                match Keyword::try_from(s.as_str()) {
                    Ok(key) => key.into(),
                    Err(_) => Identifier::new(s).into(),
                }
            }
            '"' => {
                let s = std::iter::from_fn(|| {
                    let current = iter.next();
                    current.filter(|&c| c != '"')
                })
                .collect::<String>();

                match Keyword::try_from(s.as_str()) {
                    Ok(data) => Token::Keyword(data),
                    Err(_) => StringLiteral::new(s).into(),
                }
            }
            ' ' | '\n' => {
                continue;
            }
            c => return Err(MLangError::TokenError(TokenError::NotValidToken(c.into()))),
        };
        ans.push(current_token);
    }
    Ok(ans)
}
