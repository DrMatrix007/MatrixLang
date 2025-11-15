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
                    Some(Err(err)) => return Err(err.clone()),
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

fn parse_scope_expression<T: Iterator<Item = TokenResult>>(
    tokens: &mut Peekable<T>,
    consume_left_brace: bool,
) {
    // match token {}
    todo!();
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
        Some(Ok(Token::Op(Op::SquiglyParenthesesLeft))) => {}
        Some(Ok(tok)) => {
            return Err(LangError::FunctionError(FunctionError::FunctionTokenHere {
                got: tok,
                should_be: Token::Op(Op::SquiglyParenthesesLeft),
            }));
        }
        Some(Err(err)) => return Err(err),
        None => return Err(LangError::UnexpectedEOF),
    };

    parse_expression(tokens)?;

    match tokens.next() {
        Some(Ok(Token::Op(Op::SquiglyParenthesesRight))) => {}
        Some(Ok(tok)) => {
            return Err(LangError::FunctionError(FunctionError::FunctionTokenHere {
                got: tok,
                should_be: Token::Op(Op::SquiglyParenthesesRight),
            }));
        }
        Some(Err(err)) => return Err(err),
        None => return Err(LangError::UnexpectedEOF),
    };

    Ok(FunctionDeclerationExpression { ident: fn_name })
}
