use crate::expressions::ops::Op;

use std::collections::HashSet;

pub struct MatrixUnaryExpressionParser {
    _ops: HashSet<Op>,
}

impl MatrixUnaryExpressionParser {
    pub fn new(_ops: HashSet<Op>) -> Self {
        Self { _ops }
    }
}
