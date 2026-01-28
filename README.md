[![Crates.io](https://img.shields.io/crates/v/securiety.svg)](https://crates.io/crates/ontology-registry)
[![Docs.rs](https://docs.rs/securiety/badge.svg)](https://docs.rs/ontology-registry)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![RobisonGroup](https://img.shields.io/badge/Robinson%20Group-blue)](https://robinsongroup.github.io/)

# Securiety
A robust Rust crate for parsing and validating Compact Uniform Resource Identifiers (CURIEs). It provides standard syntax validation as well as specific, regex-based validation for hundreds of biological and biomedical ontologies (e.g., GO, MONDO, CHEBI) generated directly from Bioregistry.

### Features
General Validation: Parse and validate CURIEs based on standard syntactic rules (W3C-style prefix/reference separation).

Ontology-Specific Validation: precise Regex validation for over 100+ supported ontologies (including GO, CHEBI, NCIT, etc.).

Auto-Generated Patterns: Validation logic is generated from upstream Bioregistry metadata, ensuring compliance with current standards.

Dynamic Lookup: Instantiate validators dynamically using string prefixes (e.g., from_prefix("go")).

Lightweight: Core dependencies are minimal (primarily regex).

### Installation
Add this to your Cargo.toml:

```toml
[dependencies]
curie_validator = "0.1.0"
```
### Usage
1. General Parsing
   If you need to validate that a string is simply a well-formed CURIE (has a valid prefix and reference structure) without enforcing specific ontology patterns:

```Rust
use curie_validator::curie_parser::CurieParser;

fn main() {
// Create a general parser
let parser = CurieParser::general();

    // Valid syntax
    let curie = parser.parse("AnyPrefix:12345").unwrap();
    println!("Prefix: {}, Reference: {}", curie.prefix(), curie.reference());

    // Invalid syntax (e.g., no separator, invalid characters)
    let result = parser.parse("InvalidString");
    assert!(result.is_err());
}
```
2. Specific Ontology Validation
   You can use strict, pre-compiled regex validators for specific ontologies. This ensures that a GO term actually looks like a GO term (e.g., GO:0001234).
```Rust
use curie_validator::curie_parser::CurieParser;

fn main() {
// strict validator for Gene Ontology
let go_parser = CurieParser::go();

    // Passes: Matches ^GO:\d{7}$
    let valid = go_parser.parse("GO:0006915"); 
    assert!(valid.is_ok());

    // Fails: Syntax is okay, but pattern matches invalid GO ID
    let invalid = go_parser.parse("GO:ABC"); 
    assert!(invalid.is_err());
}
```
3. Dynamic Prefix Lookup
   If you are processing data where the ontology prefix is determined at runtime, you can look up the validator dynamically:
```Rust

use curie_validator::curie_parser::CurieParser;
use curie_validator::validators::regex_validator::CurieRegexValidator;

fn main() {
let input_prefix = "mondo";
let input_curie = "MONDO:0012345";

    // Attempt to find a validator for the given prefix
    if let Some(parser) = CurieParser::from_prefix(input_prefix) {
        match parser.parse(input_curie) {
            Ok(curie) => println!("Valid {} term: {}", input_prefix, curie),
            Err(e) => println!("Invalid term: {}", e),
        }
    } else {
        println!("No validator found for prefix: {}", input_prefix);
    }
}
```

### Supported Ontologies
This crate includes generated validators for a wide range of biological ontologies found in the Bioregistry, including but not limited to:

GO (Gene Ontology)

MONDO (Mondo Disease Ontology)

CHEBI (Chemical Entities of Biological Interest)

NCIT (NCI Thesaurus)

HP (Human Phenotype Ontology)

UBERON (Uber Anatomy Ontology)

Note: The patterns are generated using the create.rs utility which fetches metadata from the Bioregistry API.

### Error Handling
The parser returns a CurieParsingError enum to distinguish between structural failures and validation failures:

InvalidCurie(String): The string failed the specific validation logic (e.g., Regex mismatch).

UnparsableCurie(String): The string lacked the basic structure of a CURIE (e.g., missing a colon).