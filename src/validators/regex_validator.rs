use crate::error::InvalidRegexError;
use crate::traits::CurieValidation;
use regex::Regex;

/// A CURIE validator that uses regular expressions to validate CURIE strings.
///
/// This validator allows flexible validation rules by accepting any regular expression
/// pattern. It's useful when you need custom validation logic beyond simple format checks.
///
/// # Examples
///
/// ```
/// use securiety::{CurieRegexValidator, CurieValidation};
/// use regex::Regex;
/// // Create a validator that requires lowercase prefixes
/// let validator = CurieRegexValidator::general();
///
/// assert!(validator.validate("rdf:type"));
/// ```
///
/// ```
///  use securiety::{CurieRegexValidator, CurieValidation};
/// // Create validator for specific resource
/// let validator = CurieRegexValidator::hp();
/// assert!(validator.validate("HP:0000054"));
/// ```
#[derive(Debug, Clone)]
pub struct CurieRegexValidator {
    regex: Regex,
}

impl CurieValidation for CurieRegexValidator {
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
