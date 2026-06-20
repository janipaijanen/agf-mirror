use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum YamlError {
    NoFrontmatter,
    ParseError(String),
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YamlError::NoFrontmatter => write!(f, "no YAML front matter found"),
            YamlError::ParseError(msg) => write!(f, "YAML parse error: {msg}"),
        }
    }
}

/// Extract the raw YAML string from a `---\n...\n---` frontmatter block.
///
/// Mirrors `Agov.CoordValidator.extract_frontmatter/1`. Returns
/// `Err(YamlError::NoFrontmatter)` when no frontmatter block is present.
pub fn extract_frontmatter(content: &str) -> Result<String, YamlError> {
    todo!()
}

/// Parse a YAML string into a typed value.
pub fn parse_yaml<T: DeserializeOwned>(yaml: &str) -> Result<T, YamlError> {
    todo!()
}

/// Extract frontmatter and immediately deserialize it.
pub fn parse_frontmatter<T: DeserializeOwned>(content: &str) -> Result<T, YamlError> {
    let yaml = extract_frontmatter(content)?;
    parse_yaml(&yaml)
}
