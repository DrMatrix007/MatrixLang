use crate::{expressions::errors::ExpressionResult, tokens::tokenizer::Tokenizer};

pub trait MatrixExpressionParser {
    fn parse<'string>(
        &self,
        tokens: &mut impl Tokenizer<'string>,
    ) -> Option<ExpressionResult<'string>>;
}

pub trait LayeredMatrixExpressionParser {
    fn parse<'string, 'expr_ctx>(
        &self,
        next: &impl MatrixExpressionParser,
        tokens: &mut impl Tokenizer<'string>,
    ) -> Option<ExpressionResult<'string>>;
}

impl<Layered: LayeredMatrixExpressionParser, Rest: MatrixExpressionParser> MatrixExpressionParser
    for (Layered, Rest)
{
    fn parse<'string>(
        &self,
        tokens: &mut impl Tokenizer<'string>,
    ) -> Option<ExpressionResult<'string>> {
        self.0.parse(&self.1, tokens)
    }
}
