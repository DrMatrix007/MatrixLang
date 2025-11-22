use unic_emoji_char::is_emoji;

use crate::{
    errors::{LangError, TokenError},
    tokens::{
        cursor::Cursor,
        lexer::TokenTypeResult,
        op::Op,
        token_type::{ImmediateType, NumberType, TokenType},
    },
};

pub trait TokenSubLexer {
    fn parse<'a>(peeked_char: char, data: &mut Cursor<'a>) -> TokenTypeResult;

    fn is_relevant(c: char) -> bool;
}

pub struct WordParser;

impl TokenSubLexer for WordParser {
    fn parse<'a>(_: char, cursor: &mut Cursor<'a>) -> TokenTypeResult {
        cursor.advance_while(|c| is_ident_char(c, false));
        Ok(TokenType::Identifier)
    }

    fn is_relevant(c: char) -> bool {
        is_ident_char(c, true)
    }
}

fn is_ident_char(c: char, start: bool) -> bool {
    c.is_alphabetic() || c == '_' || (is_emoji(c) && !c.is_ascii()) || (!start && c.is_numeric())
}

pub struct NumberParser;

impl TokenSubLexer for NumberParser {
    fn parse<'a>(_: char, data: &mut Cursor<'a>) -> TokenTypeResult {
        let mut dot_counter = 0;
        let mut is_valid = true;
        data.advance_while(|ch| match ch {
            '.' => {
                dot_counter += 1;
                true
            }
            ch if Self::is_relevant(ch) => true,
            _ => {
                is_valid = false;
                false
            }
        });

        Ok(TokenType::Immediate(ImmediateType::Number(
            match dot_counter {
                0 => NumberType::I32,
                1 => NumberType::F32,
                _ => return Err(LangError::TokenError(TokenError::NotValidNumber)),
            },
        )))
    }

    fn is_relevant(c: char) -> bool {
        c.is_ascii_digit() || c == '.'
    }
}

pub struct OpCodeParser;

impl TokenSubLexer for OpCodeParser {
    fn parse<'a>(_: char, cursor: &mut Cursor<'a>) -> TokenTypeResult {
        let mut res = None;

        cursor.advance_while_inc_str(|string| match Op::try_from(string) {
            Ok(op) => {
                res = Some(op);
                true
            }
            Err(_) => false,
        });

        match res {
            Some(op) => Ok(TokenType::Op(op)),
            None => Err(LangError::TokenError(TokenError::NotValidOp)),
        }
    }

    fn is_relevant(c: char) -> bool {
        let c = c.to_ascii_uppercase();
        !c.is_whitespace() && !c.is_ascii_alphanumeric()
    }
}

pub struct WhitespaceParser;

impl TokenSubLexer for WhitespaceParser {
    fn parse<'a>(_: char, data: &mut Cursor<'a>) -> TokenTypeResult {
        data.advance_while(char::is_whitespace);
        Ok(TokenType::Whitespace)
    }

    fn is_relevant(c: char) -> bool {
        c.is_whitespace()
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
