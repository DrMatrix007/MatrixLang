use std::{
    iter::{Enumerate, Map, Peekable},
    str::Chars,
};

use crate::{
    errors::{LangError, TokenError},
    layers::Layer,
    tokens::{
        identifier::Identifier,
        immediate::Immediate,
        keyword::Keyword,
        op::Op,
        parsers::TokenParserIter,
        sub_parsers::{NumberParser, OpCodeParser, TokenSubParser, WordParser},
    },
};

pub mod identifier;
pub mod immediate;
pub mod keyword;
pub mod op;
pub mod parsers;
pub mod sub_parsers;

pub struct Contexed<T> {
    pub value: T,
    pub pos: usize,
}

impl<T> From<(usize, T)> for Contexed<T> {
    fn from(value: (usize, T)) -> Self {
        Self {
            value: value.1,
            pos: value.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(Identifier),
    Immediate(Immediate),
    Keyword(Keyword),
    Op(Op),
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::Identifier(identifier) => identifier.name.len(),
            Token::Immediate(immediate) => immediate.len(),
            Token::Keyword(keyword) => keyword.len(),
            Token::Op(op) => op.len(),
        }
    }
}

pub struct StringPreparer;

impl<'a> Layer<&'a String, Map<Enumerate<Chars<'a>>, fn((usize, char)) -> Contexed<char>>>
    for StringPreparer
{
    fn run_layer(
        &mut self,
        data: &'a String,
    ) -> Map<Enumerate<Chars<'a>>, fn((usize, char)) -> Contexed<char>> {
        data.chars().enumerate().map(Contexed::from)
    }
}

pub struct Tokenizer;

impl<'a, T: Iterator<Item = Contexed<char>>> Layer<T, TokenParserIter<T>>
    for Tokenizer
{
    fn run_layer(
        &mut self,
        data: Enumerate<Chars<'a>>,
    ) -> Map<Enumerate<Chars<'a>>, fn((usize, char)) -> Contexed<char>> {
        data
    }
}

pub fn parse_tokens(
    data: impl Iterator<Item = char>,
) -> TokenParserIter<impl Iterator<Item = char>> {
    TokenParserIter::new(data)
}
