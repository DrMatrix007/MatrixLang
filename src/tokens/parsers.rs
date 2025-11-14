use std::iter::Peekable;

use unic_emoji_char::is_emoji;

use crate::{
    errors::{LangError, TokenError},
    tokens::{
        Token, TokenResult,
        identifier::Identifier,
        immediate::{Immediate, Number},
        keyword::Keyword,
        op::Op,
    },
};

pub trait TokenParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> TokenResult;

    fn is_relevant(c: char) -> bool;
}

pub struct WordParser;

impl TokenParser for WordParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> TokenResult {
        let mut string = String::new();
        while let Some(c) = data.peek() {
            if is_ident_char(*  c, false) {
                string.push(*c);
                data.next();
            } else {
                break;
            }
        }
        match Keyword::try_from(string.as_str()) {
            Ok(keyword) => Ok(Token::Keyword(keyword)),
            Err(_) => Ok(Token::Identifier(Identifier { name: string })),
        }
    }

    fn is_relevant(c: char) -> bool {
        is_ident_char(c, true)
    }
}

fn is_ident_char(c: char, start: bool) -> bool {
    c.is_alphabetic() || c == '_' || (is_emoji(c) && c.is_ascii()) || (!start && c.is_numeric())
}

pub struct NumberParser;

impl TokenParser for NumberParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> TokenResult {
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
                return Err(LangError::TokenError(TokenError::UnexpectedChar(c)));
            }
        }

        Ok(Token::Immediate(Immediate::Number(if is_float {
            Number::F32(
                num.parse()
                    .map_err(move |_| LangError::TokenError(TokenError::NotValidNumber(num)))?,
            )
        } else {
            Number::I32(
                num.parse()
                    .map_err(move |_| LangError::TokenError(TokenError::NotValidNumber(num)))?,
            )
        })))
    }

    fn is_relevant(c: char) -> bool {
        c.is_ascii_digit() || c == '.'
    }
}

pub struct OpCodeParser;

impl TokenParser for OpCodeParser {
    fn parse(data: &mut Peekable<impl Iterator<Item = char>>) -> TokenResult {
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
            None => Err(LangError::TokenError(TokenError::NotValidOp(string))),
        }
    }

    fn is_relevant(c: char) -> bool {
        let c = c.to_ascii_uppercase();
        !c.is_whitespace() && !c.is_ascii_alphanumeric()
    }
}

#[cfg(test)]
mod tests {
    use unic_emoji_char::is_emoji;

    #[test]
    fn test_char() {
        fn is_ident_char(c: char) -> bool {
            c.is_alphabetic() || c == '_' || (is_emoji(c) && !c.is_ascii())
        }
        assert!(!is_ident_char('*'));
        assert!(is_ident_char('a'));
        assert!(is_ident_char('💀'));
    }
}
