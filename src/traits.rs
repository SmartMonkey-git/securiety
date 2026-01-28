pub trait CurieValidator {
    fn validate(&self, curie: &str) -> bool;
}
