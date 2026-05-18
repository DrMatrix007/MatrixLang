use lazy_static::lazy_static;
use regex::Regex;

use crate::tokens::{
    Token, TokenType,
    errors::{TokenError, TokenErrorType, TokenResult},
    tokenizer::Tokenizer,
};

use std::str::CharIndices;

pub struct MatrixTokenizer<'string> {
    pub(crate) chars: CharIndices<'string>,
}

macro_rules! try_tokenize_tokenizer_from_string {
    ($tokenizer: expr, $string: expr => $token_type: expr) => {
        if let Some(value) = $tokenizer.match_string($string, $token_type) {
            return Some(value);
        }
    };

    ($tokenizer: expr, $($string: expr => $token_type: expr),+ $(,)?) => {
        $(try_tokenize_tokenizer_from_string!($tokenizer, $string => $token_type));+
    }
}

macro_rules! try_tokenize_tokenizer_from_regex {
    ($tokenizer: expr, $regex: expr => $token_type: expr) => {
        if let Some(value) = $tokenizer.match_regex($regex, $token_type) {
            return Some(value);
        }
    };

    ($tokenizer: expr, $($regex: expr => $token_type: expr),+ $(,)?) => {
        $(try_tokenize_tokenizer_from_regex!($tokenizer, $regex => $token_type));+
    }
}

impl<'string> MatrixTokenizer<'string> {
    pub fn new(code: &'string str) -> Self {
        Self {
            chars: code.char_indices(),
        }
    }

    pub fn match_string(
        &mut self,
        string: &str,
        token_type: TokenType,
    ) -> Option<TokenResult<'string>> {
        let string = self
            .chars
            .as_str()
            .get(0..string.len())
            .and_then(|sub_str| {
                if sub_str == string {
                    Some(sub_str)
                } else {
                    None
                }
            });

        if let Some(string) = string {
            self.skip_string(string);
        }

        string.map(|string| {
            Ok(Token {
                token_type,
                data: string,
            })
        })
    }

    pub fn match_regex(
        &mut self,
        regex: &Regex,
        token_type: TokenType,
    ) -> Option<TokenResult<'string>> {
        if let Some(values) = regex.captures(self.chars.as_str()) {
            if values.len() > 1 {
                panic!(
                    "regex should only match the beggining of the string once (by regex first character)!!! values: {:?}, regex: {}",
                    values,
                    regex.as_str()
                );
            }

            if let Some(value) = values.get(0) {
                if value.start() > 0 {
                    panic!("regex should match only the beggining of the string!!!");
                }

                let str = value.as_str();

                self.skip_string(str);

                return Some(Ok(Token {
                    token_type,
                    data: str,
                }));
            }
        }
        None
    }

    pub(crate) fn skip_string(&mut self, str: &str) {
        for char in str.chars() {
            assert!(matches!(self.chars.next(), Some((_, current)) if char == current));
        }
    }

    pub fn next_token(&mut self) -> Option<TokenResult<'string>> {
        lazy_static! {
            pub static ref IDENTIFIER_RE: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();
            pub static ref INTEGER_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
            pub static ref DOUBLE_RE: Regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
            pub static ref STRING_RE: Regex =
                Regex::new(r#"^(?:"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')"#).unwrap();
        }

        try_tokenize_tokenizer_from_string!(
            self,
            "func" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            "==" => TokenType::Equals,
            "+" => TokenType::Add,
            "-" => TokenType::Sub,
            "*" => TokenType::Mul,
            "/" => TokenType::Div,
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            "=" => TokenType::Assign,
        );

        try_tokenize_tokenizer_from_regex!(
            self,
            &IDENTIFIER_RE => TokenType::Identifier,
            &DOUBLE_RE => TokenType::Double,
            &INTEGER_RE => TokenType::Integer,
            &STRING_RE => TokenType::String
        );

        let (current_index, current_char) = self.chars.next()?;

        match current_char {
            ' ' | '\n' | '\r' => return self.next_token(),
            _ => {}
        }

        Some(Err(TokenError {
            index: current_index,
            error_type: TokenErrorType::UnexpectedChar(current_char),
        }))
    }
}

impl<'string> Tokenizer<'string> for MatrixTokenizer<'string> {
    fn tokenize_next(&mut self) -> Option<TokenResult<'string>> {
        self.next_token()
    }
}
