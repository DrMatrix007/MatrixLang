use crate::expressions::ops::Op;

use std::collections::HashSet;

pub struct MatrixBinaryExpressionParser {
    ops: HashSet<Op>,
}
