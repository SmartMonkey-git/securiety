/*use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write; // Allows using write! macro on Strings
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OntologyRegistryError {
    #[error("Unable to provide Metadata: {reason}")]
    ProvidingMetadata { reason: String },
    #[error("Unable to register ontology: {reason}")]
    UnableToRegister { reason: String },
}

// 1. Defined Structs
type Registry = HashMap<String, ResourceMetaData>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ResourceMetaData {
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub name: Option<String>,
    pub mappings: Option<MappingsDetail>,
    #[serde(default)]
    pub pattern: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MappingsDetail {
    bioportal: Option<String>,
    ontobee: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BioRegistryMetadataProvider {
    api_url: String,
    client: Client,
}

impl BioRegistryMetadataProvider {
    pub fn new(api_url: &str) -> Self {
        // reqwest handles trailing slashes fairly well, but clean construction is better.
        BioRegistryMetadataProvider {
            api_url: api_url.to_string(),
            client: Client::new(),
        }
    }

    fn provide_metadata(&self) -> Result<Registry, OntologyRegistryError> {
        let url = format!("{}registry", self.api_url); // Simple formatting

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "securiety")
            .send()
            .map_err(|err| OntologyRegistryError::ProvidingMetadata {
                reason: err.to_string(),
            })?;

        // Handle the text conversion error explicitly
        let text = response
            .text()
            .map_err(|err| OntologyRegistryError::ProvidingMetadata {
                reason: format!("Failed to read text: {}", err),
            })?;

        let bio_registry_metadata: Registry = serde_json::from_str(&text).map_err(|err| {
            OntologyRegistryError::ProvidingMetadata {
                reason: format!("JSON Error at line {}: {}", err.line(), err),
            }
        })?;

        Ok(bio_registry_metadata)
    }
}

impl Default for BioRegistryMetadataProvider {
    fn default() -> Self {
        BioRegistryMetadataProvider::new("https://bioregistry.io/api/")
    }
}

pub const STRICT_KEYWORDS: [&str; 39] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "gen",
];

fn main() -> Result<(), OntologyRegistryError> {
    let reg = BioRegistryMetadataProvider::default().provide_metadata()?;

    let mut entries: Vec<_> = reg.into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by ID (the original HashMap key)

    let mut code = String::with_capacity(100 * 1024);
    code.push_str("define_curie_validators! {\n");

    for (_id, entry) in entries {
        let pattern = match entry.pattern.as_ref() {
            Some(p) => p,
            None => continue,
        };

        let mappings = match entry.mappings.as_ref() {
            Some(m) => m,
            None => continue,
        };

        let candidate_prefix = mappings.ontobee.as_ref().or(mappings.bioportal.as_ref());

        if let Some(prefix) = candidate_prefix {
            let prefix_lower = prefix.to_lowercase();

            if prefix.contains('-') || STRICT_KEYWORDS.contains(&prefix_lower.as_str()) {
                continue;
            }

            if let Some(name) = &entry.name {
                let _ = writeln!(code, "    // {}", name);
            }

            let safe_pattern = pattern
                .replace('\\', "\\\\")
                .replace('^', &format!("^{}:", prefix));

            let _ = writeln!(
                code,
                "    {}, {}_PATTERN => \"{}\",",
                prefix_lower,
                prefix.to_uppercase(),
                safe_pattern
            );
        }
    }

    code.push('}');
    println!("{}", code);

    Ok(())
}
*/