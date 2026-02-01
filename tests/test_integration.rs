use securiety::{CurieParser, CurieParsing, CurieRegexValidator, CurieValidation};

#[test]
fn test_integration_parsing() {
    let raw_hp_curie = "HP:0000054";
    let parser = CurieParser::hp();

    let curie = parser.parse(raw_hp_curie).unwrap();

    assert_eq!(curie.prefix(), "HP");
    assert_eq!(curie.reference(), "0000054");
    assert_eq!(curie.to_string(), raw_hp_curie);

    assert!(parser.parse("NotACurie").is_err());
    assert!(parser.parse("MONDO:0000054").is_err());
}

#[test]
fn test_integration_parsing_from_prefix() {
    let raw_hp_curie = "HP:0000054";
    let parser = CurieParser::from_prefix("HP").unwrap();

    let curie = parser.parse(raw_hp_curie).unwrap();

    assert_eq!(curie.prefix(), "HP");
    assert_eq!(curie.reference(), "0000054");
    assert_eq!(curie.to_string(), raw_hp_curie);

    assert!(parser.parse("NotACurie").is_err());
    assert!(parser.parse("MONDO:0000054").is_err());
}

#[test]
fn test_integration_regex_validation_from_prefix() {
    let validator = CurieRegexValidator::from_prefix("HP").unwrap();

    assert!(validator.validate("HP:0000054"));
    assert!(!validator.validate("MONDO:0000054"));
}

#[test]
fn test_integration_regex_validation() {
    let validator = CurieRegexValidator::hp();

    assert!(validator.validate("HP:0000054"));
    assert!(!validator.validate("MONDO:0000054"));
}

#[test]
fn test_integration_regex_validation_general() {
    let validator = CurieRegexValidator::general();

    // --- Happy Path (Standard) ---
    assert!(validator.validate("HP:0000054")); // Standard OBO
    assert!(validator.validate("MONDO:0000054")); // Standard OBO
    assert!(validator.validate("LOINC:0000054-6")); // Hyphens in reference
    assert!(validator.validate("FOO:123")); // Simple alphanumeric
    assert!(validator.validate("ICD9CM:123")); // Simple alphanumeric

    // --- Complex References (The "Reference" part) ---
    // Colons inside the reference (common in URNs or oddly namespaced IDs)
    assert!(validator.validate("urn:uuid:1234-5678"));
    // Slashes (common in DOIs treated as CURIEs)
    assert!(validator.validate("doi:10.1038/nphys1170"));
    // Dots and underscores
    assert!(validator.validate("refseq:NC_000001.11"));
    // Query params or fragments (rare but technically allowed in some specs)
    assert!(validator.validate("wikipedia:Page#Section"));

    // Underscores/Dots in prefix
    assert!(validator.validate("my.prop:123"));
    assert!(validator.validate("my_prop:123"));

    // --- Minimal Cases ---
    // Single character prefix/reference
    assert!(validator.validate("a:b"));

    // --- Invalid Cases ---
    // Missing Delimiter
    assert!(!validator.validate("LOINC0000054-6"));

    // Missing Prefix
    assert!(!validator.validate(":0000054-6"));
    assert!(!validator.validate("0000054-6")); // No colon at all

    // Missing Reference (Empty Reference)
    assert!(!validator.validate("LOINC:"));
    assert!(!validator.validate("LOINC:   ")); // Empty but with whitespace

    // Whitespace Issues
    assert!(!validator.validate(" HP:0000054")); // Leading space
    assert!(!validator.validate("HP:0000054 ")); // Trailing space
    assert!(!validator.validate("HP: 0000054")); // Space after colon
    assert!(!validator.validate("HP :0000054")); // Space before colon

    // Illegal Characters (Control chars, pipes, etc - unless explicitly allowed)
    assert!(!validator.validate("HP:00\n054"));
}
