use crate::tokens::errors::TokenResult;

pub trait Tokenizer<'string> {
    fn tokenize_next(&mut self) -> Option<TokenResult<'string>>;
}

impl<'string> Iterator for dyn Tokenizer<'string> {
    type Item = TokenResult<'string>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize_next()
    }
}
