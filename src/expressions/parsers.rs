use std::iter::Peekable;

use crate::{
    errors::{FunctionError, LangError},
    expressions::{
        Expression,
        binary_expression_parser::{
            ExpressionParser, ExpressionParserWithSubLayer, ExpressionResult,
        },
        expression_types::{
            CallExpression, FunctionDeclerationExpression, IdentifierExpression,
            ImmediateExpression, UnaryExpression,
        },
        parse_expression,
    },
    tokens::{Token, TokenResult, keyword::Keyword, op::Op},
};

macro_rules! match_token {
    ($curr: expr, $should_be: expr) => {{
        let curr = { $curr };
        match curr {
            Some(Ok(tok)) if (tok == $should_be) => Ok(()),
            Some(Ok(tok)) => Err(LangError::TokenShouldBe {
                got: tok,
                should_be: $should_be,
            }),
            Some(Err(err)) => Err(err),
            None => Err(LangError::UnexpectedEOF),
        }
    }};
}

pub struct FunctionCallExpressionParser;
impl ExpressionParserWithSubLayer for FunctionCallExpressionParser {
    type SubLayer = SimpleExpressionParser;

    fn parse_layered<T: Iterator<Item = TokenResult>>(
        tokens: &mut Peekable<T>,
    ) -> ExpressionResult {
        let potential_func = Self::parse_sub(tokens)?;

        if let Some(Ok(Token::Op(Op::ParenthesesLeft))) = tokens.peek() {
            let _ = tokens.next().unwrap();
            let mut args = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(Token::Op(Op::ParenthesesRight))) => {
                        tokens.next();
                        break;
                    }
                    Some(Ok(Token::Op(Op::Comma))) => {
                        tokens.next();
                    }
                    Some(Ok(_)) => {
                        args.push(parse_expression(tokens)?);
                    }
                    Some(Err(_)) => {
                        return Err(tokens
                            .next()
                            .expect("already peeked")
                            .expect_err("already peeked"));
                    }
                    None => return Err(LangError::UnexpectedEOF),
                }
            }
            Ok(Expression::CallExpressin(CallExpression {
                func: Box::new(potential_func),
                args,
            }))
        } else {
            Ok(potential_func)
        }
    }
}

pub struct SimpleExpressionParser;
impl ExpressionParser for SimpleExpressionParser {
    fn parse<T: Iterator<Item = TokenResult>>(
        tokens: &mut Peekable<T>,
    ) -> Result<Expression, LangError> {
        if let Some(tok) = tokens.next() {
            let tok = tok?;

            let potential_closing_parenthesis = match tok {
                Token::Op(op) => op.get_closing_parentheses(),
                _ => None,
            };

            match tok {
                Token::Op(op) => match op {
                    op if potential_closing_parenthesis.is_some() => {
                        let potential_closing_parenthesis = potential_closing_parenthesis.unwrap();
                        todo!();
                    }
                    op if op.can_be_unary() => Ok(Expression::UnaryExpression(UnaryExpression {
                        op,
                        expr: Box::new(parse_expression(tokens)?),
                    })),
                    op => Err(LangError::UnexpectedToken(Token::Op(op))),
                },
                Token::Immediate(imm) => Ok(Expression::ImmediateExpression(ImmediateExpression {
                    value: imm,
                })),
                Token::Identifier(ident) => {
                    Ok(Expression::IdentifierExpression(IdentifierExpression {
                        ident,
                    }))
                }
                Token::Keyword(keyword) => parse_keyword(tokens, keyword),
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

    parse_function_args(tokens)?;

    let body = parse_body_expressions(tokens)?;

    Ok(FunctionDeclerationExpression {
        ident: fn_name,
        body,
    })
}

fn parse_function_args<T: Iterator<Item = TokenResult>>(
    tokens: &mut Peekable<T>,
) -> Result<(), LangError> {
    match_token!(tokens.next(), Token::Op(Op::ParenthesesLeft))?;

    // parse paremeters

    match_token!(tokens.next(), Token::Op(Op::ParenthesesRight))?;

    Ok(())
}

fn parse_body_expressions<T: Iterator<Item = TokenResult>>(
    tokens: &mut Peekable<T>,
) -> Result<Vec<Expression>, LangError> {
    let mut body = Vec::new();
    match_token!(tokens.next(), Token::Op(Op::SquiglyParenthesesLeft))?;

    loop {
        match tokens.peek() {
            Some(Ok(Token::Op(Op::SquiglyParenthesesRight))) => break,
            Some(Ok(_)) => {
                body.push(parse_expression(tokens)?);
                match tokens.peek() {
                    Some(Ok(Token::Op(Op::SquiglyParenthesesLeft))) => {
                        tokens.next();
                    }
                    Some(Ok(tok)) => {
                        return Err(LangError::TokenShouldBe {
                            got: tok.clone(),
                            should_be: Token::Op(Op::SquiglyParenthesesLeft),
                        });
                    }
                    Some(Err(_)) => {
                        return Err(tokens
                            .next()
                            .expect("already peeked")
                            .expect_err("already peeked"));
                    }
                    None => return Err(LangError::UnexpectedEOF),
                }
            }
            Some(Err(_)) => {
                return Err(tokens
                    .next()
                    .expect("already peeked")
                    .expect_err("already peeked"));
            }
            None => return Err(LangError::UnexpectedEOF),
        };
    }

    tokens.next();

    Ok(body)
}
