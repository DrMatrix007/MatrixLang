use std::iter::Peekable;

use crate::{
    errors::{LangError, TokenError},
    tokens::{
        Token,
        sub_parsers::{NumberParser, OpCodeParser, TokenSubParser, WordParser},
    },
};

macro_rules! match_parse {
    ($iter:expr, $curr: expr, $($t:ident),*) => {
        {
            let curr = { $curr };
            match curr {
                $(ch if $t::is_relevant(ch) => {
                    $t::parse($iter)
                }),*

                ch => Err(LangError::TokenError(TokenError::UnexpectedChar(ch)))
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ContexedChar {
    pub value: char,
    pub pos: usize,
}

#[derive(Debug, Clone)]
pub struct ContexedToken {
    pub value: Token,
    pub pos: usize
}

pub type TokenResult = Result<ContexedToken, (usize, LangError)>;
pub struct TokenParserIter<T: Iterator<Item = ContexedChar>> {
    data: Peekable<T>,
    finished: bool,
}
impl<T: Iterator<Item = ContexedChar>> Iterator for TokenParserIter<T> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let ctx = loop {
            if let Some(ctx) = self.data.peek().copied() {
                if ctx.value.is_whitespace() {
                    self.data.next();
                    continue;
                }
                break ctx;
            } else {
                return None;
            }
        };

        let token = match_parse!(&mut self.data, curr, WordParser, NumberParser, OpCodeParser);
        if token.is_err() {
            self.finished = true;
        }

        Some(token)
    }
}

impl<T: Iterator<Item = char>> TokenParserIter<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: data.peekable(),
            finished: false,
        }
    }
}
