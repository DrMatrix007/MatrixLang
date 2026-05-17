#[derive(Debug)]
pub struct IntegerLiteral<'a>(pub &'a str);

#[derive(Debug)]
pub struct DoubleLiteral<'a>(pub &'a str);

#[derive(Debug)]
pub struct StringLiteral<'a>(pub &'a str);
