use std::iter::Peekable;

use crate::{
    errors::{FunctionError, LangError},
    expressions::{
        Expression,
        binary_expression_parser::parse_binary_expr,
        expression_types::{
            CallExpression, FunctionDeclerationExpression, IdentifierExpression,
            ImmediateExpression, UnaryExpression,
        },
        parse_expression,
    },
    tokens::{Token, TokenResult, keyword::Keyword, op::Op},
};
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
    type SubLayer = SimpleExpressionParser;

    fn parse_layered<T: Iterator<Item = TokenResult>>(
        tokens: &mut Peekable<T>,
    ) -> ExpressionResult {
        parse_binary_expr(tokens, &[Op::Mul, Op::Div], Self::parse_sub)
    }
}

pub struct SimpleExpressionParser;
impl ExpressionParser for SimpleExpressionParser {
    fn parse<T: Iterator<Item = TokenResult>>(
        data: &mut Peekable<T>,
    ) -> Result<Expression, LangError> {
        if let Some(tok) = data.next() {
            let tok = match tok {
                Err(tok) => return Err(tok),
                Ok(tok) => tok,
            };
            match tok {
                Token::Op(op) => Ok(Expression::UnaryExpression(UnaryExpression {
                    op,
                    expr: Box::new(parse_expression(data)?),
                })),
                Token::Immediate(imm) => Ok(Expression::ImmediateExpression(ImmediateExpression {
                    value: imm,
                })),
                Token::Identifier(ident) => {
                    Ok(Expression::IdentifierExpression(IdentifierExpression {
                        ident,
                    }))
                }
                Token::Keyword(keyword) => parse_keyword(data, keyword),
            }
        } else {
            Err(LangError::UnexpectedEOF)
        }
    }
}

fn parse_keyword<T: Iterator<Item = TokenResult>>(
    tokens: &mut Peekable<T>,
    keyword: Keyword,
) -> ExpressionResult {
    match keyword {
        Keyword::Fn => parse_func_decl(tokens).map(Expression::FunctionDeclerationExpression),
        Keyword::Extern => todo!(),
    }
}

fn parse_func_decl<T: Iterator<Item = TokenResult>>(
    tokens: &mut Peekable<T>,
) -> Result<FunctionDeclerationExpression, LangError> {
    let fn_name = match tokens.next() {
        Some(Ok(Token::Identifier(ident))) => ident,
        Some(Ok(tok)) => {
            return Err(LangError::FunctionError(
                FunctionError::FunctionNameShouldBeIdentifier(tok),
            ));
        }
        Some(Err(err)) => return Err(err),
        None => return Err(LangError::UnexpectedEOF),
    };

    match tokens.next() {
        Some(Ok(Token::Op(Op::ParenthesesLeft))) => {}
        Some(Ok(tok)) => {
            return Err(LangError::FunctionError(FunctionError::FunctionTokenHere {
                got: tok,
                should_be: Token::Op(Op::ParenthesesLeft),
            }));
        }
        Some(Err(err)) => return Err(err),
        None => return Err(LangError::UnexpectedEOF),
    };
    // parse paremeters

    match tokens.next() {
        Some(Ok(Token::Op(Op::ParenthesesRight))) => {}
        Some(Ok(tok)) => {
            return Err(LangError::FunctionError(FunctionError::FunctionTokenHere {
                got: tok,
                should_be: Token::Op(Op::ParenthesesRight),
            }));
        }
        Some(Err(err)) => return Err(err),
        None => return Err(LangError::UnexpectedEOF),
    };

    match tokens.next() {
        Some(Ok(Token::Op(Op::SquiglyParenthesisLeft))) => {}
        Some(Ok(tok)) => {
            return Err(LangError::FunctionError(FunctionError::FunctionTokenHere {
                got: tok,
                should_be: Token::Op(Op::SquiglyParenthesisLeft),
            }));
        }
        Some(Err(err)) => return Err(err),
        None => return Err(LangError::UnexpectedEOF),
    };
    
    parse_expression(tokens);

    match tokens.next() {
        Some(Ok(Token::Op(Op::SquiglyParenthesisRight))) => {}
        Some(Ok(tok)) => {
            return Err(LangError::FunctionError(FunctionError::FunctionTokenHere {
                got: tok,
                should_be: Token::Op(Op::SquiglyParenthesisRight),
            }));
        }
        Some(Err(err)) => return Err(err),
        None => return Err(LangError::UnexpectedEOF),
    };

    Ok(FunctionDeclerationExpression { ident: fn_name })
}
