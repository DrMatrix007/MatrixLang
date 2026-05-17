use crate::{
    errors::LanguageError,
    expressions::{
        Expression,
        errors::{ExpressionErrorType, ExpressionResult},
        literals::{DoubleLiteral, IntegerLiteral},
    },
    tokens::{TokenType, errors::TokenResult},
};

pub struct Parser<'a, Tokenizer: Iterator<Item = TokenResult<'a>>> {
    tokens: Tokenizer,
}

impl<'a, Tokenizer: Iterator<Item = TokenResult<'a>>> Parser<'a, Tokenizer> {
    pub fn new(tokens: Tokenizer) -> Self {
        Self { tokens }
    }

    pub fn parse_next(&mut self) -> Option<ExpressionResult<'a>> {
        let curr_tok = match self.tokens.next()? {
            Err(error) => return Some(Err(error.map_to(ExpressionErrorType::TokenError))),
            Ok(token) => token,
        };

        Some(match curr_tok.token_type {
            TokenType::Double => Ok(Expression::DoubleLiteral(DoubleLiteral(curr_tok.data))),
            TokenType::Integer => Ok(Expression::IntegerLiteral(IntegerLiteral(curr_tok.data))),
            t => panic!("not supported yet: {}", t),
        })
    }
}

impl<'a, Tokenizer: Iterator<Item = TokenResult<'a>>> Iterator for Parser<'a, Tokenizer> {
    type Item = ExpressionResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next()
    }
}
