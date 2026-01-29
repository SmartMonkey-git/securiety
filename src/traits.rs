use crate::curie::Curie;
use crate::error::CurieParsingError;

pub trait CurieParsing {
    fn parse(&self, curie: &str) -> Result<Curie, CurieParsingError>;
}

pub trait CurieValidation {
    fn validate(&self, curie: &str) -> bool;
}
