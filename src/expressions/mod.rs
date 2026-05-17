use crate::expressions::{
    identifier::Identifier,
    literals::{DoubleLiteral, IntegerLiteral, StringLiteral},
    ops::{BinaryExpression, UnaryExpression},
};

pub mod identifier;
pub mod literals;
pub mod ops;
pub mod parser;
pub mod errors;

#[derive(Debug)]
pub enum Expression<'a> {
    DoubleLiteral(DoubleLiteral<'a>),
    IntegerLiteral(IntegerLiteral<'a>),
    String(StringLiteral<'a>),
    Identifier(Identifier<'a>),

    BinaryExpression(BinaryExpression<'a>),
    UnaryExpression(UnaryExpression<'a>),
}
