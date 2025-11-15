pub mod binary_expression_parser;
pub mod expression_types;
pub mod parsers;

use std::{fmt::Display, iter::Peekable};

use crate::{
    expressions::{
        binary_expression_parser::{
            AssignsBinaryExpressionParser, ExpressionParser, ExpressionResult,
        },
        expression_types::{
            BinaryExpression, CallExpression, FunctionDeclerationExpression, IdentifierExpression, ImmediateExpression, ScopeExpressions, UnaryExpression
        },
    },
    tokens::TokenResult,
};

#[derive(Debug)]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    CallExpressin(CallExpression),
    ImmediateExpression(ImmediateExpression),
    IdentifierExpression(IdentifierExpression),
    FunctionDeclerationExpression(FunctionDeclerationExpression),
    ScopeExpressions(ScopeExpressions),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BinaryExpression(expr) => write!(f, "{}", expr),
            Expression::UnaryExpression(expr) => write!(f, "{}", expr),
            Expression::CallExpressin(expr) => write!(f, "{}", expr),
            Expression::ImmediateExpression(expr) => write!(f, "{}", expr),
            Expression::IdentifierExpression(expr) => write!(f, "{}", expr),
            Expression::FunctionDeclerationExpression(expr) => write!(f, "{}", expr),
            Expression::ScopeExpressions(expr) => write!(f, "{}", expr),
        }
    }
}

type TopLayerExpression = AssignsBinaryExpressionParser;

pub fn parse_expression(
    iter: &mut Peekable<impl Iterator<Item = TokenResult>>,
) -> ExpressionResult {
    TopLayerExpression::parse(iter)
}
