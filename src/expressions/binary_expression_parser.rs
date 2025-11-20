use std::iter::Peekable;

use crate::{
    errors::LangError,
    expressions::{
        Expression, expression_types::BinaryExpression, parsers::FunctionCallExpressionParser,
    },
    tokens::{Token, TokenResult, op::Op},
};

pub fn parse_binary_expr<T: Iterator<Item = TokenResult>>(
    tokens: &mut Peekable<T>,
    ops: &[Op],
    mut sub: impl FnMut(&mut Peekable<T>) -> ExpressionResult,
) -> Result<Expression, LangError> {
    assert!(ops.iter().all(|op| op.can_be_binary()));
    
    let left = sub(tokens)?;
    if let Some(tok) = tokens.peek() {
        let tok = match tok {
            Ok(tok) => tok,
            Err(_) => {
                return Err(tokens
                    .next()
                    .unwrap()
                    .expect_err("this was peeked as an error"));
            }
        };

        if let Token::Op(op) = tok {
            let op = *op;

            if ops.contains(&op) {
                tokens.next();
                let right = sub(tokens)?;
                return Ok(Expression::BinaryExpression(BinaryExpression {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }));
            }
        }
    }
    Ok(left)
}

pub type ExpressionResult = Result<Expression, LangError>;
pub trait ExpressionParser {
    fn parse<T: Iterator<Item = TokenResult>>(tokens: &mut Peekable<T>) -> ExpressionResult;
}

pub trait ExpressionParserWithSubLayer {
    type SubLayer: ExpressionParser;

    fn parse_layered<T: Iterator<Item = TokenResult>>(tokens: &mut Peekable<T>)
    -> ExpressionResult;

    fn parse_sub<T: Iterator<Item = TokenResult>>(tokens: &mut Peekable<T>) -> ExpressionResult {
        Self::SubLayer::parse(tokens)
    }
}
impl<P: ExpressionParserWithSubLayer> ExpressionParser for P {
    fn parse<T: Iterator<Item = TokenResult>>(tokens: &mut Peekable<T>) -> ExpressionResult {
        Self::parse_layered(tokens)
    }
}

pub struct AssignsBinaryExpressionParser;
impl ExpressionParserWithSubLayer for AssignsBinaryExpressionParser {
    type SubLayer = AddSubBinaryExpressionParser;

    fn parse_layered<T: Iterator<Item = TokenResult>>(
        tokens: &mut Peekable<T>,
    ) -> ExpressionResult {
        parse_binary_expr(
            tokens,
            &[Op::AddAssign, Op::SubAssign, Op::MulAssign, Op::DivAssign],
            Self::parse_sub,
        )
    }
}

pub struct AddSubBinaryExpressionParser;
impl ExpressionParserWithSubLayer for AddSubBinaryExpressionParser {
    type SubLayer = MulDivBinaryExpressionParser;

    fn parse_layered<T: Iterator<Item = TokenResult>>(
        tokens: &mut Peekable<T>,
    ) -> ExpressionResult {
        parse_binary_expr(tokens, &[Op::Add, Op::Sub], Self::parse_sub)
    }
}

pub struct MulDivBinaryExpressionParser;
impl ExpressionParserWithSubLayer for MulDivBinaryExpressionParser {
    type SubLayer = FunctionCallExpressionParser;

    fn parse_layered<T: Iterator<Item = TokenResult>>(
        tokens: &mut Peekable<T>,
    ) -> ExpressionResult {
        parse_binary_expr(tokens, &[Op::Mul, Op::Div], Self::parse_sub)
    }
}
