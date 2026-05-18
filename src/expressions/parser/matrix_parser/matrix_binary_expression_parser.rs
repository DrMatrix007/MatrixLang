use crate::expressions::ops::Op;

use std::collections::HashSet;

pub struct MatrixBinaryExpressionParser {
    _ops: HashSet<Op>,
}

impl MatrixBinaryExpressionParser {
    pub fn new(_ops: HashSet<Op>) -> Self {
        Self { _ops }
    }
}
