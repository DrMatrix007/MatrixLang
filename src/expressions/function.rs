use crate::{tokens::Identifier};

use super::Expression;

pub struct FunctionDecleration {
    pub name: Identifier,
    pub vals: Vec<Expression>,
}
