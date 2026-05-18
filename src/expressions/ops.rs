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
pub struct BinaryExpression<'string> {
    pub left: Box<Expression<'string>>,
    pub op: Op,
    pub right: Box<Expression<'string>>,
}

impl<'string> BinaryExpression<'string> {
    pub fn new(left: Expression<'string>, op: Op, right: Expression<'string>) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression<'string> {
    pub op: Op,
    pub expr: Box<Expression<'string>>,
}

impl<'string> UnaryExpression<'string> {
    pub fn new(op: Op, expr: Expression<'string>) -> Self {
        Self {
            op,
            expr: Box::new(expr),
        }
    }
}
