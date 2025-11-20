#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Extern,
}

impl Keyword {
    pub fn len(&self) -> usize {
        String::from(*self).len()
    }
}
impl From<Keyword> for String {
    fn from(val: Keyword) -> Self {
        match val {
            Keyword::Fn => "fn".into(),
            Keyword::Extern => "extern".into(),
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "fn" => Ok(Keyword::Fn),
            "extern" => Ok(Keyword::Extern),
            _ => Err(()),
        }
    }
}
