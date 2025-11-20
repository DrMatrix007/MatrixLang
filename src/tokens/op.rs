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

    SquareParenthesesLeft,
    SquareParenthesesRight,

    SquiglyParenthesesLeft,
    SquiglyParenthesesRight,

    Comma,
    SemiColon,
}

impl Op {
    pub fn get_closing_parentheses(&self) -> Option<Op> {
        match self {
            Op::ParenthesesLeft => Some(Op::ParenthesesRight),
            Op::SquareParenthesesLeft => Some(Op::SquareParenthesesRight),
            Op::SquiglyParenthesesLeft => Some(Op::SquiglyParenthesesRight),
            _ => None,
        }
    }

    pub fn can_be_unary(&self) -> bool {
        matches!(self, Op::Add | Op::Sub)
    }

    pub fn can_be_binary(&self) -> bool {
        matches!(
            self,
            Op::Add
                | Op::Sub
                | Op::Mul
                | Op::Div
                | Op::AddAssign
                | Op::SubAssign
                | Op::MulAssign
                | Op::DivAssign
        )
    }

    pub fn len(&self) -> usize {
        String::from(*self).len()
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Op({})", Into::<String>::into(*self))
    }
}

impl From<Op> for String {
    fn from(val: Op) -> Self {
        match val {
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

            Op::SquareParenthesesLeft => "[".into(),
            Op::SquareParenthesesRight => "]".into(),
            Op::SquiglyParenthesesLeft => "{".into(),
            Op::SquiglyParenthesesRight => "}".into(),

            Op::Comma => ",".into(),
            Op::SemiColon => ";".into(),
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

            "[" => Ok(Op::SquareParenthesesLeft),
            "]" => Ok(Op::SquareParenthesesRight),

            "{" => Ok(Op::SquiglyParenthesesLeft),
            "}" => Ok(Op::SquiglyParenthesesRight),

            "," => Ok(Op::Comma),
            ";" => Ok(Op::SemiColon),

            _ => Err(()),
        }
    }
}
