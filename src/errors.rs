#[derive(Debug, Clone, Copy)]
pub struct LanguageError<ErrorType> {
    pub index: usize,
    pub error_type: ErrorType,
}

impl<ErrorType> LanguageError<ErrorType> {
    pub fn map_to<ErrorType2>(
        self,
        f: impl FnOnce(ErrorType) -> ErrorType2,
    ) -> LanguageError<ErrorType2> {
        LanguageError {
            index: self.index,
            error_type: f(self.error_type),
        }
    }
}
