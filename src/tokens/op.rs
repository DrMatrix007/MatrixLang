use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,

    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,

    ParenthesesLeft,
    ParenthesesRight,

    SquareParenthesisLeft,   
    SquareParenthesisRight,  

    SquiglyParenthesisLeft,     
    SquiglyParenthesisRight,    
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Op({})", Into::<String>::into(*self))
    }
}

impl Into<String> for Op {
    fn into(self) -> String {
        match self {
            Op::Add => "+".into(),
            Op::Sub => "-".into(),
            Op::Mul => "*".into(),
            Op::Div => "/".into(),

            Op::AddAssign => "+=".into(),
            Op::SubAssign => "-=".into(),
            Op::MulAssign => "*=".into(),
            Op::DivAssign => "/=".into(),

            Op::ParenthesesLeft => "(".into(),
            Op::ParenthesesRight => ")".into(),

            Op::SquareParenthesisLeft => "[".into(),
            Op::SquareParenthesisRight => "]".into(),
            Op::SquiglyParenthesisLeft => "{".into(),
            Op::SquiglyParenthesisRight => "}".into(),
        }
    }
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

            "(" => Ok(Op::ParenthesesLeft),
            ")" => Ok(Op::ParenthesesRight),

            "[" => Ok(Op::SquareParenthesisLeft),
            "]" => Ok(Op::SquareParenthesisRight),

            "{" => Ok(Op::SquiglyParenthesisLeft),
            "}" => Ok(Op::SquiglyParenthesisRight),

            _ => Err(()),
        }
    }
}
