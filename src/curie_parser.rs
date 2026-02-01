use crate::curie::Curie;
use crate::error::CurieParsingError;
use crate::traits::{CurieParsing, CurieValidation};

/// A parser for CURIE (Compact URI) strings that validates input using a configurable validator.
///
/// CURIEs are compact representations of URIs in the form `prefix:reference`, commonly used
/// in semantic web applications and knowledge graphs.
///
/// # Type Parameters
///
/// * `Validator` - A type implementing [`CurieValidation`] used to validate CURIE strings
///   before parsing.
///
/// # Examples
///
/// ```
/// use securiety::{CurieParser, CurieParsing};
///
/// let parser = CurieParser::general();
/// let curie = parser.parse("prefix:reference").unwrap();
/// assert_eq!(curie.prefix(), "prefix");
/// assert_eq!(curie.reference(), "reference");
/// ```
#[derive(Debug, Clone)]
pub struct CurieParser<Validator: CurieValidation> {
    pub(crate) validator: Validator,
}

impl<Validator: CurieValidation> CurieParsing for CurieParser<Validator> {
    /// Parses a CURIE string into a [`Curie`] instance.
    ///
    /// The parsing process:
    /// 1. Validates the input string using the configured validator
    /// 2. Splits the string on the colon (`:`) separator
    /// 3. Returns a [`Curie`] with the prefix and reference components
    ///
    /// # Arguments
    ///
    /// * `curie` - A string slice containing the CURIE to parse (e.g., "prefix:reference")
    ///
    /// # Returns
    ///
    /// * `Ok(Curie)` - Successfully parsed CURIE
    /// * `Err(CurieParsingError::InvalidCurie)` - The input failed validation
    /// * `Err(CurieParsingError::UnparsableCurie)` - The input passed validation but
    ///   couldn't be split into prefix and reference (no colon found)
    ///
    /// # Examples
    ///
    /// ```
    /// use securiety::{CurieParser, CurieParsing};
    /// let parser = CurieParser::hp();
    ///
    /// // Valid CURIE
    /// let result = parser.parse("HP:0000054");
    /// assert!(result.is_ok());
    ///
    /// // Invalid CURIE (no colon)
    /// let result = parser.parse("invalid");
    /// assert!(result.is_err());
    /// ```
    fn parse(&self, curie: &str) -> Result<Curie, CurieParsingError> {
        match self.validator.validate(curie) {
            true => {
                if let Some((prefix, reference)) = curie.split_once(':') {
                    Ok(Curie::new(prefix, reference))
                } else {
                    Err(CurieParsingError::UnparsableCurie(curie.to_string()))
                }
            }
            false => Err(CurieParsingError::InvalidCurie(curie.to_string())),
        }
    }
}

impl<Validator: CurieValidation> CurieParser<Validator> {
    pub fn new(validators: Validator) -> CurieParser<Validator> {
        CurieParser {
            validator: validators,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        pub MyValidator {}

        impl CurieValidation for MyValidator {
            fn validate(&self, curie: &str) -> bool;
        }
    }

    #[test]
    fn test_parse_success() {
        let curie_str = "GO:001";
        let mut mock = MockMyValidator::new();

        mock.expect_validate()
            .with(eq(curie_str))
            .times(1)
            .return_const(true);

        let parser = CurieParser::new(mock);

        let result = parser.parse(curie_str);

        assert!(result.is_ok());
        let curie = result.unwrap();
        assert_eq!(curie.prefix(), "GO");
        assert_eq!(curie.reference(), "001");
    }

    #[test]
    fn test_parse_fails_validation() {
        let curie_str = "INVALID:XYZ";
        let mut mock = MockMyValidator::new();

        mock.expect_validate()
            .with(eq(curie_str))
            .times(1)
            .return_const(false);

        let parser = CurieParser::new(mock);

        let result = parser.parse(curie_str);

        match result {
            Err(CurieParsingError::InvalidCurie(s)) => assert_eq!(s, curie_str),
            _ => panic!("Expected InvalidCurie error, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_passes_validation_but_fails_structure() {
        let curie_str = "NoColonHere";
        let mut mock = MockMyValidator::new();

        mock.expect_validate()
            .with(eq(curie_str))
            .times(1)
            .return_const(true);

        let parser = CurieParser::new(mock);

        let result = parser.parse(curie_str);

        match result {
            Err(CurieParsingError::UnparsableCurie(s)) => assert_eq!(s, curie_str),
            _ => panic!("Expected UnparsableCurie error, got {:?}", result),
        }
    }
}
