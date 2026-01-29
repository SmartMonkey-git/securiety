use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Curie {
    inner: String,
    prefix_len: usize,
}

impl Curie {
    pub(crate) fn new(prefix: &str, reference: &str) -> Curie {
        let inner = format!("{prefix}:{reference}");
        let prefix_len = prefix.len();
        Curie { inner, prefix_len }
    }

    pub fn prefix(&self) -> &str {
        &self.inner[..self.prefix_len]
    }

    pub fn reference(&self) -> &str {
        &self.inner[self.prefix_len + 1..]
    }
}

impl Display for Curie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl AsRef<str> for Curie {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_curie_with_correct_format() {
        let curie = Curie::new("ex", "123");
        assert_eq!(curie.inner, "ex:123");
        assert_eq!(curie.prefix_len, 2);
    }

    #[test]
    fn test_prefix_returns_correct_prefix() {
        let curie = Curie::new("example", "reference");
        assert_eq!(curie.prefix(), "example");
    }

    #[test]
    fn test_reference_returns_correct_reference() {
        let curie = Curie::new("example", "reference");
        assert_eq!(curie.reference(), "reference");
    }

    #[test]
    fn test_reference_with_colons() {
        let curie = Curie::new("prefix", "ref:with:colons");
        assert_eq!(curie.prefix(), "prefix");
        assert_eq!(curie.reference(), "ref:with:colons");
    }

    #[test]
    fn test_display_implementation() {
        let curie = Curie::new("example", "123");
        assert_eq!(format!("{}", curie), "example:123");
    }

    #[test]
    fn test_as_ref_returns_full_string() {
        let curie = Curie::new("ex", "ref");
        let s: &str = curie.as_ref();
        assert_eq!(s, "ex:ref");
    }

    #[test]
    fn test_long_prefix_and_reference() {
        let long_prefix = "a".repeat(1000);
        let long_reference = "b".repeat(1000);
        let curie = Curie::new(&long_prefix, &long_reference);

        assert_eq!(curie.prefix(), long_prefix);
        assert_eq!(curie.reference(), long_reference);
    }
}
