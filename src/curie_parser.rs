use crate::curie::Curie;
use crate::error::CurieParsingError;
use crate::traits::{CurieParsing, CurieValidation};

#[derive(Debug, Clone)]
pub struct CurieParser<Validator: CurieValidation> {
    pub(crate) validator: Validator,
}

impl<Validator: CurieValidation> CurieParsing for CurieParser<Validator> {
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
