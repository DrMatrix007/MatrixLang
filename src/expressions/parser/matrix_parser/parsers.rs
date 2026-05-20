use crate::{expressions::errors::ExpressionResult, tokens::tokenizer::Tokenizer};

pub trait ExpressionParser {
    fn parse<'string>(
        &self,
        tokens: &mut impl Tokenizer<'string>,
    ) -> Option<ExpressionResult<'string>>;
}

pub trait LayeredExpressionParser {
    fn parse<'string>(
        &self,
        next: &impl ExpressionParser,
        tokens: &mut impl Tokenizer<'string>,
    ) -> Option<ExpressionResult<'string>>;

    #[must_use]
    fn chain<T>(self, sub_layer: T) -> ChainedExpressionParsers<Self, T>
    where
        Self: Sized,
    {
        ChainedExpressionParsers(self, sub_layer)
    }
}

pub struct ChainedExpressionParsers<A, B>(pub(self) A, pub(self) B);

impl<Layered: LayeredExpressionParser, Rest: ExpressionParser> ExpressionParser
    for ChainedExpressionParsers<Layered, Rest>
{
    fn parse<'string>(
        &self,
        tokens: &mut impl Tokenizer<'string>,
    ) -> Option<ExpressionResult<'string>> {
        self.0.parse(&self.1, tokens)
    }
}
