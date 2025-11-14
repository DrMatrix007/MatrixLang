#[derive(Debug)]
pub enum Keyword {
    Fn,
    Extern,
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
