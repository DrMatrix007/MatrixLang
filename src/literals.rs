#[derive(Debug)]
pub struct NumberLiteral {
    pub num: std::string::String,
}

impl NumberLiteral {
    pub fn new(num: std::string::String) -> Self {
        Self { num }
    }
}
#[derive(Debug)]
pub struct StringLiteral {
    pub inner: std::string::String,
}

impl StringLiteral {
    pub fn new(inner: std::string::String) -> Self {
        Self { inner }
    }
}
