use std::{iter::Peekable, marker::PhantomData};

use crate::{
    error::{MLangError, TokenError},
    tokens::{Keyword, Operator, Token},
};

use super::{BinaryExpression, Expression, UnaryExpression, VariableDecleration};

pub fn parse_file<'a>(
    tokens: impl Iterator<Item = &'a Token>,
) -> Result<Vec<Expression>, MLangError> {
    let mut tokens = tokens.peekable();
    let mut ans = Vec::new();
    while tokens.peek().is_some() {
        ans.push(_parse(&mut tokens)?);
    }
    Ok(ans)
}

pub fn parse_expression<'a>(
    tokens: impl Iterator<Item = &'a Token>,
) -> Result<Expression, MLangError> {
    let mut tokens = tokens.peekable();
    let r = _parse(&mut tokens);
    if let Some(t) = tokens.next() {
        return Err(MLangError::TokenError(TokenError::UnexpectedToken(
            t.clone(),
        )));
    }
    r
}

fn _parse<'a, I: Iterator<Item = &'a Token>>(
    i: &mut core::iter::Peekable<I>,
) -> Result<Expression, MLangError> {
    Equality::parse(i)
}
struct ParserWrapper<T: Parser>(PhantomData<T>);
impl<T: Parser> Parser for ParserWrapper<T> {
    fn parse<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        T::parse(tokens)
    }
}

trait Parser {
    fn parse<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError>;
}

trait ParseLayer {
    type NextLayer: Parser;

    fn parse_layer<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError>;
}

impl<P: ParseLayer> Parser for P {
    fn parse<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        Self::parse_layer(tokens)
    }
}

struct Equality;

impl ParseLayer for Equality {
    type NextLayer = Comparison;
    fn parse_layer<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        parse_binary_expression(
            tokens,
            &[Operator::EqualEqual, Operator::NotEqual],
            Self::NextLayer::parse,
        )
    }
}

struct Comparison;

impl ParseLayer for Comparison {
    type NextLayer = Term;

    fn parse_layer<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        parse_binary_expression(
            tokens,
            &[
                Operator::Greater,
                Operator::GreaterEq,
                Operator::Smaller,
                Operator::SmallerEq,
            ],
            Self::NextLayer::parse,
        )
    }
}

struct Term;

impl ParseLayer for Term {
    type NextLayer = Factor;

    fn parse_layer<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        parse_binary_expression(
            tokens,
            &[Operator::Plus, Operator::Minus],
            Self::NextLayer::parse,
        )
    }
}
struct Factor;
impl ParseLayer for Factor {
    type NextLayer = Unary; // You can set the next layer to be a default one or an empty one

    fn parse_layer<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        parse_binary_expression(
            tokens,
            &[Operator::Mul, Operator::Div],
            Self::NextLayer::parse,
        )
    }
}
struct Unary;

impl ParseLayer for Unary {
    type NextLayer = Primary;

    fn parse_layer<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        match tokens.peek() {
            Some(Token::Operator(op @ (Operator::Minus | Operator::Not))) => {
                tokens.next();
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op: *op,
                    val: Box::new(Self::NextLayer::parse(tokens)?),
                }))
            }
            Some(_) => Self::NextLayer::parse(tokens),
            None => Err(MLangError::TokenError(TokenError::MissingToken)),
        }
    }
}

struct Primary;

impl Parser for Primary {
    fn parse<'a, I: Iterator<Item = &'a Token>>(
        tokens: &mut core::iter::Peekable<I>,
    ) -> Result<Expression, MLangError> {
        match tokens.next() {
            Some(Token::Identifier(ident)) => Ok(Expression::Identifier(ident.clone())),
            Some(Token::NumberLiteral(number)) => Ok(Expression::Number(number.clone())),
            Some(Token::StringLiteral(string)) => Ok(Expression::String(string.clone())),
            Some(Token::Operator(Operator::ParenLeft)) => {
                let v = _parse(tokens);
                if let Some(Token::Operator(Operator::ParenRight)) = tokens.next() {
                } else {
                    return Err(MLangError::TokenError(TokenError::MissingToken));
                }
                v
            }
            Some(Token::Keyword(Keyword::Let)) => {
                let name = match tokens.next() {
                    Some(Token::Identifier(var_name)) => var_name,
                    Some(t) => {
                        return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                            t.clone(),
                        )))
                    }
                    None => return Err(MLangError::TokenError(TokenError::MissingToken)),
                };
                let _eq = match tokens.next() {
                    Some(Token::Operator(op @ Operator::Equal)) => op,
                    Some(t) => {
                        return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                            t.clone(),
                        )))
                    }
                    None => return Err(MLangError::TokenError(TokenError::MissingToken)),
                };
                let v = _parse(tokens)?;
                Ok(Expression::VariableDecleration(VariableDecleration {
                    var_name: name.clone(),
                    value: Box::new(v),
                }))
            }
            Some(Token::Keyword(Keyword::Fn)) => {
                let name = match tokens.next() {
                    Some(Token::Identifier(name)) => name,
                    Some(t) => {
                        return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                            t.clone(),
                        )))
                    }
                    None => return Err(MLangError::TokenError(TokenError::MissingToken)),
                };

                // (
                match tokens.next() {
                    Some(Token::Operator(Operator::ParenLeft)) => name,
                    Some(t) => {
                        return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                            t.clone(),
                        )))
                    }
                    None => return Err(MLangError::TokenError(TokenError::MissingToken)),
                };
                //TODO: read args

                // )
                match tokens.next() {
                    Some(Token::Operator(Operator::ParenRight)) => name,
                    Some(t) => {
                        return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                            t.clone(),
                        )))
                    }
                    None => return Err(MLangError::TokenError(TokenError::MissingToken)),
                };

                // {
                match tokens.next() {
                    Some(Token::Operator(Operator::BraceLeft)) => name,
                    Some(t) => {
                        return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                            t.clone(),
                        )))
                    }
                    None => return Err(MLangError::TokenError(TokenError::MissingToken)),
                };
                let mut vec = Vec::new();
                while tokens.peek() != Some(&&Token::Operator(Operator::BraceRight)) {
                    vec.push(_parse(tokens)?);
                    match tokens.next() {
                        Some(Token::Operator(Operator::Semicolon)) => {}

                        Some(t) => {
                            return Err(MLangError::TokenError(TokenError::UnexpectedToken(
                                t.clone(),
                            )))
                        }
                        None => return Err(MLangError::TokenError(TokenError::MissingToken)),
                    }
                }
                tokens.next();
                Ok(Expression::FunctionDecleration(
                    super::function::FunctionDecleration {
                        name: name.clone(),
                        vals: vec,
                    },
                ))
            }
            Some(Token::Keyword(Keyword::Return)) => {
                Ok(Expression::Return)
            }
            Some(t) => Err(MLangError::TokenError(TokenError::UnexpectedToken(
                t.clone(),
            ))),
            None => Err(MLangError::TokenError(TokenError::MissingToken)),
        }
    }
}

fn parse_binary_expression<'a, I: Iterator<Item = &'a Token>>(
    i: &mut core::iter::Peekable<I>,
    ops: &[Operator],
    next: fn(&mut Peekable<I>) -> Result<Expression, MLangError>,
) -> Result<Expression, MLangError> {
    let v = next(i)?;
    match i.peek() {
        Some(Token::Operator(op)) if ops.contains(op) => {
            i.next();
            Ok(Expression::BinaryExpression(BinaryExpression {
                left: Box::new(v),
                op: *op,
                right: Box::new(parse_binary_expression(i, ops, next)?),
            }))
        }
        Some(_) | None => Ok(v),
    }
}
