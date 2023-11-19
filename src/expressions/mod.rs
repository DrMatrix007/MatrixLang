pub mod parse;

use std::fmt::Display;

use crate::tokens::{
    literals::{NumberLiteral, StringLiteral},
    Identifier, Operator,
};

pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    Number(NumberLiteral),
    String(StringLiteral),
    Identifier(Identifier),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BinaryExpression(b) => {
                write!(f, "({} {} {})", b.left, b.op, b.right)
            }
            Expression::UnaryExpression(unary) => write!(f, "({} {})", unary.op, unary.val),
            Expression::Number(n) => write!(f, "({})", n.num),
            Expression::String(s) => write!(f, "(\"{}\")", s.inner),
            Expression::Identifier(i) => write!(f, "({})", i.name),
        }
    }
}

pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub op: Operator,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(right: Box<Expression>, op: Operator, left: Box<Expression>) -> Self {
        Self { right, op, left }
    }
}

pub struct UnaryExpression {
    pub op: Operator,
    pub val: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(op: Operator, val: Box<Expression>) -> Self {
        Self { op, val }
    }
}
