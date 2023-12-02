use std::collections::HashMap;

use crate::tokens::Operator;

pub struct Type(u128, Box<dyn InnerType>);

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Type {}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub trait InnerType {
    fn type_name(&self) -> &str;
}

pub struct TypeRegistry {
    ops: HashMap<(Type, Operator, Type), ()>,
}
