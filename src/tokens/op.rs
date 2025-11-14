#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,

    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,

    ParenthesisLeft,
    ParenthesisRight,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),

            "+=" => Ok(Op::AddAssign),
            "-=" => Ok(Op::SubAssign),
            "*=" => Ok(Op::MulAssign),
            "/=" => Ok(Op::DivAssign),

            "(" => Ok(Op::ParenthesisLeft),
            ")" => Ok(Op::ParenthesisRight),
            _ => Err(()),
        }
    }
}
