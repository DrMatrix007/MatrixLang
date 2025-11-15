use std::fmt::Display;

use crate::{
    expressions::Expression,
    tokens::{identifier::Identifier, immediate::Immediate, op::Op},
};

#[derive(Debug)]
pub struct ImmediateExpression {
    pub value: Immediate,
}

impl Display for ImmediateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub op: Op,
    pub right: Box<Expression>,
}

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BinExpr({}, {}, {})", self.left, self.op, self.right)
    }
}
#[derive(Debug)]
pub struct UnaryExpression {
    pub op: Op,
    pub expr: Box<Expression>,
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnaryExpr({}, {})", self.op, self.expr)
    }
}
#[derive(Debug)]
pub struct CallExpression {
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallExpr({}, [", self.func)?;
        for arg in &self.args {
            write!(f, "{},", arg)?;
        }
        write!(f, "])")
    }
}
#[derive(Debug)]
pub struct IdentifierExpression {
    pub ident: Identifier,
}

impl Display for IdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IdentExpr({})", self.ident.name)
    }
}

#[derive(Debug)]
pub struct FunctionDeclerationExpression {
    pub ident: Identifier,
    // pub args
    // pub expressions
}

impl Display for FunctionDeclerationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionDecleration({})", self.ident.name)
    }
}

#[derive(Debug)]
pub struct ScopeExpressions {
    pub exprs: Vec<Expression>,
}

impl Display for ScopeExpressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScopeExprs({})",
            self.exprs
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
