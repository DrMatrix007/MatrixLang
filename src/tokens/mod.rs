use std::iter::Peekable;

use crate::{
    errors::TokenError,
    tokens::{
        immediate::Immediate, keyword::Keyword, op::Op, parser::{NumberParser, OpCodeParser, TokenParser, WordParser}
    },
};

pub mod immediate;
pub mod keyword;
pub mod parser;
pub mod op;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Immediate(Immediate),
    Keyword(Keyword),
    Op(Op)
}

macro_rules! match_parse {
    ($iter:expr, $curr: expr, $($t:ident),*) => {
        {
            let curr = { $curr };
            match curr {
                $(ch if $t::is_relevant(ch) => {
                    $t::parse($iter)
                }),*

                ch => Err(TokenError::UnexpectedChar(ch))
            }
        }
    };
}

pub struct TokenParserIter<T: Iterator<Item = char>> {
    data: Peekable<T>,
    finished: bool,
}

impl<T: Iterator<Item = char>> Iterator for TokenParserIter<T> {
    type Item = Result<Token, TokenError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let curr = loop {
            if let Some(curr) = self.data.peek().copied() {
                if curr.is_whitespace() {
                    self.data.next();
                    continue;
                }
                break curr;
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

pub fn parse_tokens(
    data: impl Iterator<Item = char>,
) -> TokenParserIter<impl Iterator<Item = char>> {
    TokenParserIter::new(data)
}
