use crate::expressions::{
    identifier::Identifier,
    literals::{DoubleLiteral, IntegerLiteral, StringLiteral},
    ops::{BinaryExpression, UnaryExpression},
};

pub mod errors;
pub mod identifier;
pub mod literals;
pub mod ops;
pub mod parser;

#[derive(Debug)]
pub enum Expression<'string> {
    DoubleLiteral(DoubleLiteral<'string>),
    IntegerLiteral(IntegerLiteral<'string>),
    String(StringLiteral<'string>),
    Identifier(Identifier<'string>),

    BinaryExpression(BinaryExpression<'string>),
    UnaryExpression(UnaryExpression<'string>),
}
