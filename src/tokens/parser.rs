use std::iter::Peekable;

use unic_emoji_char::is_emoji;

use crate::{
    errors::TokenError,
    tokens::{
        Token,
        immediate::{Immediate, Number},
        keyword::Keyword,
        op::Op,
    },
};

pub trait TokenParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> Result<Token, TokenError>;

    fn is_relevant(c: char) -> bool;
}

pub struct WordParser;

impl TokenParser for WordParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> Result<Token, TokenError> {
        let mut string = String::new();
        while let Some(c) = data.peek() {
            if c.is_alphabetic() || is_emoji(*c) || *c == '_' {
                string.push(*c);
                data.next();
            } else if c.is_whitespace() {
                break;
            } else {
                return Err(TokenError::UnexpectedChar(*c));
            }
        }
        match Keyword::try_from(string.as_str()) {
            Ok(keyword) => Ok(Token::Keyword(keyword)),
            Err(_) => Ok(Token::Identifier(string)),
        }
    }

    fn is_relevant(c: char) -> bool {
        (c.is_alphabetic() || c == '_' || is_emoji(c)) && !c.is_numeric()
    }
}

pub struct NumberParser;

impl TokenParser for NumberParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> Result<Token, TokenError> {
        let mut num = String::new();
        let mut is_float = false;
        for c in data {
            if Self::is_relevant(c) {
                num.push(c);
                if c == '.' {
                    is_float = true;
                }
            } else if c.is_whitespace() {
                break;
            } else {
                return Err(TokenError::UnexpectedChar(c));
            }
        }

        Ok(Token::Immediate(Immediate::Number(if is_float {
            Number::F32(
                num.parse()
                    .map_err(move |_| TokenError::NotValidNumber(num))?,
            )
        } else {
            Number::I32(
                num.parse()
                    .map_err(move |_| TokenError::NotValidNumber(num))?,
            )
        })))
    }

    fn is_relevant(c: char) -> bool {
        c.is_ascii_digit() || c == '.'
    }
}

pub struct OpCodeParser;

impl TokenParser for OpCodeParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> Result<Token, TokenError> {
        let mut string = String::new();
        let mut res = None;

        while let Some(c) = data.peek().copied() {
            if Self::is_relevant(c) {
                string.push(c);
                if let Ok(curr) = Op::try_from(string.as_str()) {
                    res = Some(curr);
                } else {
                    break;
                }
                data.next();

            } else {
                break;
            }
        }

        match res {
            Some(op) => Ok(Token::Op(op)),
            None => Err(TokenError::NotValidOp(string)),
        }
    }

    fn is_relevant(c: char) -> bool {
        let c = c.to_ascii_uppercase();
        !c.is_whitespace() && !c.is_ascii_alphanumeric()
    }
}
