use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Curie {
    prefix: String,
    reference: String,
}

impl Curie {
    pub(crate) fn new(prefix: &str, reference: &str) -> Curie {
        Curie {
            prefix: prefix.to_string(),
            reference: reference.to_string(),
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

impl Display for Curie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}:{}", self.prefix, self.reference))
    }
}
