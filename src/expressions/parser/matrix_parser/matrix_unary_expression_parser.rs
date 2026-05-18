use crate::expressions::ops::Op;

use std::collections::HashSet;

pub struct MatrixUnaryExpressionParser {
    ops: HashSet<Op>,
}
