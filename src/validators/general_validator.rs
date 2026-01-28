use crate::traits::CurieValidator;

pub struct GeneralCurieValidator;

impl GeneralCurieValidator {
    fn is_valid_prefix(prefix: &str) -> bool {
        #[allow(clippy::unnecessary_map_or)]
        if prefix
            .chars()
            .next()
            .map_or(true, |c| !c.is_alphabetic() && c != '_')
        {
            return false;
        }

        prefix
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    }
}

impl CurieValidator for GeneralCurieValidator {
    fn validate(&self, curie: &str) -> bool {
        let Some((prefix, reference)) = curie.split_once(':') else {
            return false;
        };

        if prefix.is_empty() || !Self::is_valid_prefix(prefix) {
            return false;
        }

        if reference.is_empty() || reference.contains(char::is_whitespace) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::CurieValidator;

    #[test]
    fn test_valid_curies() {
        let validator = GeneralCurieValidator;

        let valid_cases = vec![
            "GO:0000001",       // Standard alphabetic prefix
            "R_A:123",          // Underscore in prefix
            "my.ontology:term", // Dot in prefix
            "My-Ontology:term", // Hyphen in prefix
            "_:blank_node",     // Prefix starting with underscore
            "OBO:123:456",      // Multiple colons (reference contains ':')
            "P:valid/path",     // Reference with special chars (non-whitespace)
            "P:valid+more",     // Reference with plus
            "P:AbCd",           // Mixed case reference
            "LOINC:92039-7",    // Reference contains -
        ];

        for curie in valid_cases {
            assert!(
                validator.validate(curie),
                "Expected '{}' to be valid, but it failed.",
                curie
            );
        }
    }

    #[test]
    fn test_invalid_curies_bad_structure() {
        let validator = GeneralCurieValidator;

        let invalid_cases = vec![
            "",             // Empty string
            "noprefix",     // No separator
            ":noreference", // Empty prefix
            "noprefix:",    // Empty reference
            ":",            // Only separator
        ];

        for curie in invalid_cases {
            assert!(
                !validator.validate(curie),
                "Expected '{}' to be invalid due to structure, but it passed.",
                curie
            );
        }
    }

    #[test]
    fn test_invalid_curies_bad_prefix() {
        let validator = GeneralCurieValidator;

        let invalid_cases = vec![
            "1GO:001",  // Starts with number
            "-GO:001",  // Starts with hyphen
            ".GO:001",  // Starts with dot
            "GO :001",  // Space in prefix
            "GO/A:001", // Invalid char (/) in prefix
            "GO!:001",  // Invalid char (!) in prefix
        ];

        for curie in invalid_cases {
            assert!(
                !validator.validate(curie),
                "Expected '{}' to be invalid due to prefix, but it passed.",
                curie
            );
        }
    }

    #[test]
    fn test_invalid_curies_bad_reference() {
        let validator = GeneralCurieValidator;

        let invalid_cases = vec![
            "GO: 001",    // Leading space in reference
            "GO:001 ",    // Trailing space in reference
            "GO:00 1",    // Inner space in reference
            "GO:term\t1", // Tab in reference
            "GO:term\n1", // Newline in reference
        ];

        for curie in invalid_cases {
            assert!(
                !validator.validate(curie),
                "Expected '{}' to be invalid due to reference whitespace, but it passed.",
                curie
            );
        }
    }

    #[test]
    fn test_is_valid_prefix_internal_logic() {
        assert!(GeneralCurieValidator::is_valid_prefix("GO"));
        assert!(GeneralCurieValidator::is_valid_prefix("_GO"));
        assert!(GeneralCurieValidator::is_valid_prefix("G-O"));
        assert!(GeneralCurieValidator::is_valid_prefix("G.O"));
        assert!(GeneralCurieValidator::is_valid_prefix("G0"));

        assert!(!GeneralCurieValidator::is_valid_prefix(""));
        assert!(!GeneralCurieValidator::is_valid_prefix("1GO"));
        assert!(!GeneralCurieValidator::is_valid_prefix("-GO"));
        assert!(!GeneralCurieValidator::is_valid_prefix("G O"));
        assert!(!GeneralCurieValidator::is_valid_prefix("G*O"));
    }
}
