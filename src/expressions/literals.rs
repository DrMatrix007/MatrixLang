#[derive(Debug)]
pub struct IntegerLiteral<'string>(pub &'string str);

#[derive(Debug)]
pub struct DoubleLiteral<'string>(pub &'string str);

#[derive(Debug)]
pub struct StringLiteral<'string>(pub &'string str);
