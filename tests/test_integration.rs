use securiety::curie_parser::CurieParser;
use securiety::traits::CurieValidator;
use securiety::validators::general_validator::GeneralCurieValidator;
use securiety::validators::regex_validator::CurieRegexValidator;

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
fn test_integration_general_validation() {
    let validator = GeneralCurieValidator;

    assert!(validator.validate("HP:0000054"));
    assert!(validator.validate("MONDO:0000054"));
}
