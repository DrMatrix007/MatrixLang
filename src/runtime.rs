use crate::error::MLangError;

pub struct Runtime;

impl Runtime {
    pub fn run(_code: &str) -> Result<(), MLangError> {
        Ok(())
    }
}
