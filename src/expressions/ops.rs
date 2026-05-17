use crate::expressions::Expression;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Bang,
    Assign,
    Equals,
}

#[derive(Debug)]
pub struct BinaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub op: Op,
    pub right: Box<Expression<'a>>,
}

impl<'a> BinaryExpression<'a> {
    pub fn new(left: Expression<'a>, op: Op, right: Expression<'a>) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression<'a> {
    pub op: Op,
    pub expr: Box<Expression<'a>>,
}

impl<'a> UnaryExpression<'a> {
    pub fn new(op: Op, expr: Expression<'a>) -> Self {
        Self {
            op,
            expr: Box::new(expr),
        }
    }
}
