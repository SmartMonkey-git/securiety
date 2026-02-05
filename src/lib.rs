//! A library for parsing and validating CURIEs.
//!
//! CURIEs provide a compact syntax for representing URIs by using a prefix that maps
//! to a namespace IRI. For example, `HP:0000738`.
//!
//! # Examples
//!
//! Parsing a CURIE ensures that whenever the [`Curie`] type is encountered in your code,
//! it has been validated. The [`Curie`] type cannot be constructed directly—only through parsing.
//!
//! ```
//! use securiety::{Curie, CurieParser, CurieParsing, CurieParsingError};
//! # fn main() -> Result<(), CurieParsingError> {
//! // Using a general parser
//! let parser = CurieParser::general();
//! let curie = parser.parse("HP:0000738")?;
//!
//! // Using a specific parser
//! let mondo_parser = CurieParser::mondo();
//! let mondo_curie = mondo_parser.parse("MONDO:0006007")?;
//! # Ok(())
//! # }
//! ```
//! ## Dynamic parser instantiation
//!
//! When the ontology is only known at runtime, a parser can be instantiated via a prefix:
//!
//! ```
//! use securiety::{CurieParser, CurieParsing, CurieParsingError};
//! # fn main() -> Result<(), CurieParsingError> {
//! let parser = CurieParser::from_prefix("HP").unwrap();
//! let curie = parser.parse("HP:0000738")?;
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - Parse CURIE strings into structured [`Curie`] objects
//! - Validate CURIEs against various formats
//! - Support for specific ontology parsers (HP, MONDO, etc.)
//!
//! # Modules
//!
//! - [`curie`] - Core CURIE data structure
//! - [`curie_parser`] - Parsing and expansion logic
//! - [`validators`] - CURIE validation implementations
//! - [`traits`] - Common traits for extensibility
//! - [`error`] - Error types

pub mod curie;
pub use curie::Curie;
pub mod curie_parser;
pub use curie_parser::CurieParser;
pub mod error;
pub use error::*;
pub mod traits;
pub use traits::*;
pub mod validators;
pub use validators::regex_validator::CurieRegexValidator;
