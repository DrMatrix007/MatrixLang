use crate::{expressions::errors::ExpressionResult, tokens::tokenizer::Tokenizer};

mod matrix_parser;

pub trait ExpressionParser {
    fn parse_next<'string>(
        &self,
        tokens: &mut impl Tokenizer<'string>,
    ) -> Option<ExpressionResult<'string>>;
}
