use std::fmt;

pub mod literals;
pub mod parse;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Operator(Operator),
    Identifier(Identifier),
    StringLiteral(literals::StringLiteral),
    NumberLiteral(literals::NumberLiteral),
    Keyword(Keyword),
}
impl Into<Token> for Operator {
    fn into(self) -> Token {
        Token::Operator(self)
    }
}
impl Into<Token> for Identifier {
    fn into(self) -> Token {
        Token::Identifier(self)
    }
}

impl Into<Token> for literals::StringLiteral {
    fn into(self) -> Token {
        Token::StringLiteral(self)
    }
}

impl Into<Token> for literals::NumberLiteral {
    fn into(self) -> Token {
        Token::NumberLiteral(self)
    }
}

impl Into<Token> for Keyword {
    fn into(self) -> Token {
        Token::Keyword(self)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
    Comma,
    Dot,
    Minus,
    Plus,
    Div,
    Mul,
    Semicolon,
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEq,
    Smaller,
    SmallerEq,
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::ParenLeft => write!(f, "("),
            Operator::ParenRight => write!(f, ")"),
            Operator::BraceLeft => write!(f, "{{"),
            Operator::BraceRight => write!(f, "}}"),
            Operator::Comma => write!(f, ","),
            Operator::Dot => write!(f, "."),
            Operator::Minus => write!(f, "-"),
            Operator::Plus => write!(f, "+"),
            Operator::Div => write!(f, "/"),
            Operator::Mul => write!(f, "*"),
            Operator::Semicolon => write!(f, ";"),
            Operator::Not => write!(f, "!"),
            Operator::NotEqual => write!(f, "!="),
            Operator::Equal => write!(f, "="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEq => write!(f, ">="),
            Operator::Smaller => write!(f, "<"),
            Operator::SmallerEq => write!(f, "<="),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    If,
    Null,
    Or,
    Return,
    Super,
    True,
    Let,
    While,
    Fn,
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "and" => Ok(Keyword::And),
            "class" => Ok(Keyword::Class),
            "else" => Ok(Keyword::Else),
            "false" => Ok(Keyword::False),
            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "null" => Ok(Keyword::Null),
            "or" => Ok(Keyword::Or),
            "return" => Ok(Keyword::Return),
            "super" => Ok(Keyword::Super),
            "true" => Ok(Keyword::True),
            "let" => Ok(Keyword::Let),
            "while" => Ok(Keyword::While),
            "fn" => Ok(Keyword::Fn),
            _ => Err("Invalid keyword"),
        }
    }
}
