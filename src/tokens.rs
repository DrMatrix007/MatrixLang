use std::str::Chars;

use regex::Regex;

mod tokens_regex {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref IDENTIFIER_RE: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();
        pub static ref FUNCTION_RE: Regex = Regex::new(r"^func").unwrap();
        pub static ref INTEGER_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
        pub static ref FLOAT_RE: Regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
        pub static ref STRING_RE: Regex = Regex::new(r#"^"((?:[^"\\]|\\.)*)""#).unwrap();
        pub static ref BOOLEAN_RE: Regex = Regex::new(r"^(true|false)").unwrap();
        pub static ref PLUS_RE: Regex = Regex::new(r"^\+").unwrap();
        pub static ref MINUS_RE: Regex = Regex::new(r"^-").unwrap();
        pub static ref MULTIPLY_RE: Regex = Regex::new(r"^\*").unwrap();
        pub static ref DIVIDE_RE: Regex = Regex::new(r"^/").unwrap();
        pub static ref LPAREN_RE: Regex = Regex::new(r"^\(").unwrap();
        pub static ref RPAREN_RE: Regex = Regex::new(r"^\)").unwrap();
        pub static ref LBRACE_RE: Regex = Regex::new(r"^\{").unwrap();
        pub static ref RBRACE_RE: Regex = Regex::new(r"^\}").unwrap();
        pub static ref COMMA_RE: Regex = Regex::new(r"^,").unwrap();
        pub static ref SEMICOLON_RE: Regex = Regex::new(r"^;").unwrap();
        pub static ref ASSIGN_RE: Regex = Regex::new(r"^=").unwrap();
        pub static ref IF_RE: Regex = Regex::new(r"^if").unwrap();
        pub static ref ELSE_RE: Regex = Regex::new(r"^else").unwrap();
        pub static ref RETURN_RE: Regex = Regex::new(r"^return").unwrap();
        pub static ref WHITESPACE_RE: Regex = Regex::new(r"^\s+").unwrap();
    }
}

#[derive(Debug)]
pub enum MatrixTokenType {
    Function,
    Identifier,
    Integer,
    Float,
    String,
    Boolean,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Assign,
    If,
    Else,
    Return,
    Whitespace,
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
                panic!("regex should only match the beggining of the string once!!! values: {:?}, regex: {}", values, regex.as_str());
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
        &tokens_regex::FUNCTION_RE=>MatrixTokenType::Function,
        &tokens_regex::IDENTIFIER_RE=>MatrixTokenType::Identifier,
        &tokens_regex::INTEGER_RE=>MatrixTokenType::Integer,
        &tokens_regex::FLOAT_RE=>MatrixTokenType::Float,
        &tokens_regex::STRING_RE=>MatrixTokenType::String,
        &tokens_regex::BOOLEAN_RE=>MatrixTokenType::Boolean,
        &tokens_regex::PLUS_RE=>MatrixTokenType::Plus,
        &tokens_regex::MINUS_RE=>MatrixTokenType::Minus,
        &tokens_regex::MULTIPLY_RE=>MatrixTokenType::Multiply,
        &tokens_regex::DIVIDE_RE=>MatrixTokenType::Divide,
        &tokens_regex::LPAREN_RE=>MatrixTokenType::LeftParen,
        &tokens_regex::RPAREN_RE=>MatrixTokenType::RightParen,
        &tokens_regex::LBRACE_RE=>MatrixTokenType::LeftBrace,
        &tokens_regex::RBRACE_RE=>MatrixTokenType::RightBrace,
        &tokens_regex::COMMA_RE=>MatrixTokenType::Comma,
        &tokens_regex::SEMICOLON_RE=>MatrixTokenType::Semicolon,
        &tokens_regex::ASSIGN_RE=>MatrixTokenType::Assign,
        &tokens_regex::IF_RE=>MatrixTokenType::If,
        &tokens_regex::ELSE_RE=>MatrixTokenType::Else,
        &tokens_regex::RETURN_RE=>MatrixTokenType::Return,
        &tokens_regex::WHITESPACE_RE=>MatrixTokenType::Whitespace
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
