pub mod matrix_binary_expression_parser;
pub mod matrix_unary_expression_parser;
pub mod parsers;

use crate::expressions::Expression;

use crate::expressions::literals::DoubleLiteral;
use crate::expressions::literals::IntegerLiteral;
use crate::tokens::TokenType;

use crate::expressions::errors::ExpressionErrorType;

use crate::expressions::errors::ExpressionResult;
use crate::tokens::tokenizer::Tokenizer;

pub struct MatrixParser<'string> {
    tokens: Box<dyn Tokenizer<'string>>,
}

impl<'string> MatrixParser<'string> {
    pub fn new(tokens: Box<dyn Tokenizer<'string>>) -> Self {
        Self { tokens }
    }

    pub fn parse_literal(&mut self) -> Option<ExpressionResult<'string>> {
        let curr_tok = match self.tokens.tokenize_next()? {
            Err(error) => return Some(Err(error.map_to(ExpressionErrorType::TokenError))),
            Ok(token) => token,
        };

        Some(match curr_tok.token_type {
            TokenType::Double => Ok(Expression::DoubleLiteral(DoubleLiteral(curr_tok.data))),
            TokenType::Integer => Ok(Expression::IntegerLiteral(IntegerLiteral(curr_tok.data))),
            t => panic!("not supported yet: {}", t),
        })
    }

    pub fn parse_next(&mut self) -> Option<ExpressionResult<'string>> {
        self.parse_literal()
    }
}

impl<'string> Iterator for MatrixParser<'string> {
    type Item = ExpressionResult<'string>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next()
    }
}
