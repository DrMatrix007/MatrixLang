use std::str::CharIndices;

use lazy_static::lazy_static;
use regex::Regex;

use crate::tokens::errors;

use super::MatrixToken;

use super::MatrixTokenType;

pub(crate) struct Tokenizer<'a> {
    pub(crate) chars: CharIndices<'a>,
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

impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            chars: code.char_indices(),
        }
    }

    pub fn match_string(
        &mut self,
        string: &str,
        token_type: MatrixTokenType,
    ) -> Option<errors::MatrixTokenResult<'a>> {
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
            Ok(MatrixToken {
                token_type,
                data: string,
            })
        })
    }

    pub fn match_regex(
        &mut self,
        regex: &Regex,
        token_type: MatrixTokenType,
    ) -> Option<errors::MatrixTokenResult<'a>> {
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

                return Some(Ok(MatrixToken {
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

    pub fn next_token(&mut self) -> Option<errors::MatrixTokenResult<'a>> {
        lazy_static! {
            pub static ref IDENTIFIER_RE: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();
            pub static ref FUNCTION_RE: Regex = Regex::new(r"^func").unwrap();
            pub static ref INTEGER_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
            pub static ref FLOAT_RE: Regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
            pub static ref STRING_RE: Regex =
                Regex::new(r#"^(?:"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')"#).unwrap();
        }

        println!("current: {}", self.chars.as_str());

        try_tokenize_tokenizer_from_regex!(
            self,
            &FUNCTION_RE => MatrixTokenType::Function,
            &IDENTIFIER_RE => MatrixTokenType::Identifier,
            &INTEGER_RE => MatrixTokenType::Integer,
            &FLOAT_RE => MatrixTokenType::Float,
            &STRING_RE => MatrixTokenType::String
        );

        try_tokenize_tokenizer_from_string!(
            self,
            "if" => MatrixTokenType::If,
            "else" => MatrixTokenType::Else,
            "return" => MatrixTokenType::Return,
            "+" => MatrixTokenType::Plus,
            "-" => MatrixTokenType::Minus,
            "*" => MatrixTokenType::Multiply,
            "/" => MatrixTokenType::Divide,
            "(" => MatrixTokenType::LeftParen,
            ")" => MatrixTokenType::RightParen,
            "{" => MatrixTokenType::LeftBrace,
            "}" => MatrixTokenType::RightBrace,
            "," => MatrixTokenType::Comma,
            ";" => MatrixTokenType::Semicolon,
            "=" => MatrixTokenType::Assign
        );

        let (current_index, current_char) = self.chars.next()?;

        match current_char {
            ' ' | '\n' | '\r' => return self.next_token(),
            _ => {}
        }

        return Some(Err(errors::MatrixTokenError {
            index: current_index,
            error: errors::MatrixTokenErrorType::UnexpectedChar(current_char),
        }));
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = errors::MatrixTokenResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
