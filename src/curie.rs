use std::fmt::{Display, Formatter};

/// A parsed CURIE (Compact URI) representation.
///
/// A CURIE consists of a prefix and a reference separated by a colon (`:`),
/// providing a compact way to represent URIs. For example, `HP:0000054` where
/// `HP` is the prefix and `0000054` is the reference.
///
/// This struct stores the CURIE as a single string internally for efficiency,
/// tracking the prefix length to enable zero-copy access to both components.
///
/// # Examples
///
/// ```
/// # use securiety::{Curie, CurieParser, CurieParsing};
/// let parser = CurieParser::general();
/// let curie = parser.parse("prefix:reference").unwrap();
///
/// assert_eq!(curie.prefix(), "prefix");
/// assert_eq!(curie.reference(), "reference");
/// assert_eq!(curie.to_string(), "prefix:reference");
/// ```
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Curie {
    inner: String,
    prefix_len: usize,
}

impl Curie {
    /// Creates a new `Curie` from a prefix and reference.
    ///
    /// This is an internal constructor used by the parser. Users should typically
    /// obtain `Curie` instances through a [`CurieParser`].
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
