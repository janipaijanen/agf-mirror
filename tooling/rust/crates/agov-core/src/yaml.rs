use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum YamlError {
    Io(String),
    NoFrontmatter,
    ParseError(String),
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YamlError::Io(msg) => write!(f, "YAML read error: {msg}"),
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
    let Some(rest) = content.strip_prefix("---\n") else {
        return Err(YamlError::NoFrontmatter);
    };

    match rest.find("\n---") {
        Some(index) => {
            let after = &rest[index + 4..];
            if after.is_empty() || after.starts_with('\n') {
                Ok(rest[..index].to_string())
            } else {
                Err(YamlError::NoFrontmatter)
            }
        }
        None => Err(YamlError::NoFrontmatter),
    }
}

/// Parse a YAML string into a typed value.
pub fn parse_yaml<T: DeserializeOwned + 'static>(yaml: &str) -> Result<T, YamlError> {
    serde_norway::from_str(yaml).map_err(|err| YamlError::ParseError(err.to_string()))
}

/// Extract frontmatter and immediately deserialize it.
pub fn parse_frontmatter<T: DeserializeOwned + 'static>(content: &str) -> Result<T, YamlError> {
    let yaml = extract_frontmatter(content)?;
    parse_yaml(&yaml)
}

/// Convert YAML frontmatter into JSON values for JSON Schema validation.
pub fn parse_frontmatter_json(content: &str) -> Result<serde_json::Value, YamlError> {
    parse_frontmatter(content)
}

/// Read a file and deserialize the whole YAML document into JSON values.
pub fn read_yaml_file_json(path: &std::path::Path) -> Result<serde_json::Value, YamlError> {
    let content = std::fs::read_to_string(path).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            YamlError::Io("file not found".to_string())
        } else {
            YamlError::Io(err.to_string())
        }
    })?;
    parse_yaml(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_frontmatter() {
        let content = "---\nkind: note\n---\n# Body\n";
        assert_eq!(extract_frontmatter(content).unwrap(), "kind: note");
    }

    #[test]
    fn extracts_frontmatter_at_eof() {
        let content = "---\nkind: note\n---";
        assert_eq!(extract_frontmatter(content).unwrap(), "kind: note");
    }

    #[test]
    fn rejects_missing_frontmatter() {
        assert!(matches!(
            extract_frontmatter("# Body\n"),
            Err(YamlError::NoFrontmatter)
        ));
    }
}
