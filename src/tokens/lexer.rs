use crate::{
    errors::{LangError, TokenError},
    tokens::{
        Token,
        cursor::Cursor,
        sub_parsers::{NumberParser, OpCodeParser, TokenSubLexer, WhitespaceParser, WordParser},
        token_type::TokenType,
    },
};

macro_rules! match_parse {
    ($char:expr, $cursor: expr, $($t:ident),*) => {
        {
            match $char {
                $(ch if $t::is_relevant(ch) => {
                    $t::parse(ch, $cursor)
                }),*
                _ => Err(LangError::TokenError(TokenError::UnexpectedChar($char)))
            }
        }
    };
}

pub type TokenTypeResult = Result<TokenType, LangError>;
pub type TokenResult = Result<Token, LangError>;
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    is_valid: bool,
}
impl<'a> Iterator for Lexer<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_token()
    }
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            cursor: Cursor::new(data),
            is_valid: true,
        }
    }

    pub fn advance_token(&mut self) -> Option<Result<Token, LangError>> {
        if !self.is_valid {
            return None;
        }

        let ch = self.cursor.peek_first()?;

        self.cursor.reset_token_start();
        let token = match_parse!(
            ch,
            &mut self.cursor,
            WhitespaceParser,
            WordParser,
            NumberParser,
            OpCodeParser
        );

        self.is_valid = token.is_ok();

        Some(token.map(|value| Token {
            value,
            len: self.cursor.get_current_token_len() as usize,
        }))
    }
}
