use std::iter::Peekable;

use crate::{
    errors::LangError,
    expressions::{Expression, expression_types::BinaryExpression, parsers::ExpressionResult},
    tokens::{Token, TokenResult, op::Op},
};

pub fn parse_binary_expr<T: Iterator<Item = TokenResult>>(
    tokens: &mut Peekable<T>,
    ops: &[Op],
    mut sub: impl FnMut(&mut Peekable<T>) -> ExpressionResult,
) -> Result<Expression, LangError> {
    let left = sub(tokens)?;
    if let Some(tok) = tokens.peek() {
        let tok = match tok {
            Ok(tok) => tok,
            Err(err) => return Err(err.clone()),
        };

        if let Token::Op(op) = tok {
            let op = *op;

            if ops.contains(&op) {
                tokens.next();
                let right = sub(tokens)?;
                return Ok(Expression::BinaryExpression(BinaryExpression {
                    left: Box::new(left),
                    op: op,
                    right: Box::new(right),
                }));
            }
        }
    }
    Ok(left)
}
