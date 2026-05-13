use std::str::Chars;

use regex::Regex;

mod tokens_regex {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref IDENTIFIER_RE: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();
        pub static ref FUNCTION_RE: Regex = Regex::new(r"^func").unwrap();
        pub static ref INTEGER_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
    }
}

#[derive(Debug)]
pub enum MatrixTokenType {
    Function,
    Identifier,
    Integer,
}

#[derive(Debug)]
pub struct MatrixToken<'a> {
    pub _token_type: MatrixTokenType,
    pub _data: &'a str,
}

impl<'a> MatrixToken<'a> {
    pub fn new(_token_type: MatrixTokenType, _data: &'a str) -> Self {
        Self { _token_type, _data }
    }
}

pub(crate) struct Tokenizer<'a> {
    pub(crate) chars: Chars<'a>,
}

macro_rules! try_tokenize_tokenizer_from_regex {
    ($tokenizer: expr, $regex: expr => $token: expr) => {
        if let Some(value) = $tokenizer.match_regex($regex) {
            return Some(MatrixToken::new($token, value));
        }
    };

    ($tokenizer: expr, $($regex: expr => $token: expr),+ $(,)?) => {
        $(try_tokenize_tokenizer_from_regex!($tokenizer, $regex => $token));+
    }

}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            chars: code.chars(),
        }
    }

    pub fn match_regex(&mut self, regex: &Regex) -> Option<&'a str> {
        if let Some(values) = regex.captures(self.chars.as_str()) {
            if values.len() > 1 {
                panic!("regex should only match the beggining of the string once!!!");
            }

            if let Some(value) = values.get(0) {
                if value.start() > 0 {
                    panic!("regex should match only the beggining of the string!!!");
                }

                let str = value.as_str();

                for _ in str.chars() {
                    self.chars.next();
                }

                return Some(str);
            }
        }
        None
    }

    pub fn next_token(&mut self) -> Option<MatrixToken<'a>> {
        try_tokenize_tokenizer_from_regex!(
            self,
            &tokens_regex::FUNCTION_RE => MatrixTokenType::Function,
            &tokens_regex::IDENTIFIER_RE => MatrixTokenType::Identifier,
            &tokens_regex::INTEGER_RE => MatrixTokenType::Integer
        );

        let current_char = self.chars.next()?;

        if current_char == ' ' {
            return self.next_token();
        }

        None
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = MatrixToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
