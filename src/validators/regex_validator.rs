use crate::error::InvalidRegexError;
use crate::traits::CurieValidator;
use regex::Regex;

pub struct CurieRegexValidator {
    regex: Regex,
}

impl CurieValidator for CurieRegexValidator {
    fn validate(&self, curie: &str) -> bool {
        self.regex.is_match(curie)
    }
}

impl From<Regex> for CurieRegexValidator {
    fn from(value: Regex) -> Self {
        Self { regex: value }
    }
}

impl TryFrom<&str> for CurieRegexValidator {
    type Error = InvalidRegexError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(value).map_err(InvalidRegexError)?;
        Ok(Self { regex })
    }
}

impl TryFrom<String> for CurieRegexValidator {
    type Error = InvalidRegexError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let regex = Regex::new(&value).map_err(InvalidRegexError)?;
        Ok(Self { regex })
    }
}
