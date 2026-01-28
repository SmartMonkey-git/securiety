#[derive(Debug)]
pub enum CurieParsingError {
    InvalidCurie(String),
    UnparsableCurie(String),
}

impl std::fmt::Display for CurieParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurieParsingError::InvalidCurie(curie_string) => {
                write!(f, "Invalid CURIE: {}", curie_string)
            }
            CurieParsingError::UnparsableCurie(curie_string) => {
                write!(f, "Unparsable CURIE: {}", curie_string)
            }
        }
    }
}

impl std::error::Error for CurieParsingError {}
#[derive(Debug)]
pub struct InvalidRegexError(pub(crate) regex::Error);

impl std::fmt::Display for InvalidRegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid CURIE regex: {}", self.0)
    }
}

impl std::error::Error for InvalidRegexError {}
