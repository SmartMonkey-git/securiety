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