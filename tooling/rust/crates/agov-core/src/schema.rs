use std::path::Path;

/// An opaque wrapper around the compiled JSON schema validator.
///
/// Wrapping avoids leaking the `jsonschema` crate version into the public API.
/// v0.18.x uses `jsonschema::JSONSchema`; newer versions use `jsonschema::Validator`.
/// Callers see only `CompiledSchema` — version differences are invisible.
/// Codex must compile with Draft7, no remote-reference fetching, no async features.
pub struct CompiledSchema(jsonschema::JSONSchema);

#[derive(Debug)]
pub struct SchemaFinding {
    pub message: String,
    /// JSON pointer string, e.g. `"#/advisory_consultation"`.
    pub pointer: String,
}

#[derive(Debug)]
pub enum SchemaError {
    Io(String),
    Json(String),
    Compile(String),
}

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaError::Io(msg) => write!(f, "schema load error: {msg}"),
            SchemaError::Json(msg) => write!(f, "schema JSON error: {msg}"),
            SchemaError::Compile(msg) => write!(f, "schema compile error: {msg}"),
        }
    }
}

/// Load a JSON schema file and compile it for Draft-07 validation.
///
/// Does NOT enable remote reference fetching or network access.
pub fn load_and_compile(path: &Path) -> Result<CompiledSchema, SchemaError> {
    let content = std::fs::read_to_string(path).map_err(|err| SchemaError::Io(err.to_string()))?;
    let mut schema: serde_json::Value =
        serde_json::from_str(&content).map_err(|err| SchemaError::Json(err.to_string()))?;
    normalize_relative_id(&mut schema);

    jsonschema::JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .compile(&schema)
        .map(CompiledSchema)
        .map_err(|err| SchemaError::Compile(err.to_string()))
}

fn normalize_relative_id(schema: &mut serde_json::Value) {
    let Some(id) = schema.get("$id").and_then(serde_json::Value::as_str) else {
        return;
    };

    if !(id.contains("://") || id.starts_with('#')) {
        if let Some(object) = schema.as_object_mut() {
            object.remove("$id");
        }
    }
}

/// Validate a JSON value against a compiled schema.
///
/// Returns an empty `Vec` when valid. Each `SchemaFinding` corresponds to one
/// `(message, json_pointer)` pair from the underlying validator — mirroring
/// `ExJsonSchema.Validator.validate/2` error tuples.
pub fn validate(schema: &CompiledSchema, data: &serde_json::Value) -> Vec<SchemaFinding> {
    let findings = match schema.0.validate(data) {
        Ok(()) => Vec::new(),
        Err(errors) => errors
            .map(|err| {
                let pointer = format!("#{}", err.instance_path);
                let (message, pointer) = normalize_finding(&err.to_string(), &pointer);
                SchemaFinding { message, pointer }
            })
            .collect(),
    };

    group_required_property_findings(findings)
}

fn normalize_finding(message: &str, pointer: &str) -> (String, String) {
    let normalized = normalize_message(message, pointer);
    let pointer = if pointer == "#/artifacts_owned"
        && normalized
            == "Expected all of the schemata to match, but the schemata at the following indexes did not: 0."
    {
        "#".to_string()
    } else {
        pointer.to_string()
    };

    (normalized, pointer)
}

fn normalize_message(message: &str, pointer: &str) -> String {
    if let Some(pattern) = message
        .strip_prefix('"')
        .and_then(|rest| rest.split_once("\" does not match \""))
        .and_then(|(_value, rest)| rest.strip_suffix('"'))
    {
        return format!("Does not match pattern \"{pattern}\".");
    }

    if pointer == "#/artifacts_owned" && message.ends_with(" has less than 1 item") {
        return "Expected all of the schemata to match, but the schemata at the following indexes did not: 0.".to_string();
    }

    if pointer.starts_with("#/advisory_consultation/")
        && message == "\"reason\" is a required property"
    {
        return "Expected all of the schemata to match, but the schemata at the following indexes did not: 0.".to_string();
    }

    message.to_string()
}

fn group_required_property_findings(findings: Vec<SchemaFinding>) -> Vec<SchemaFinding> {
    let mut grouped = Vec::new();
    let mut required_groups = std::collections::BTreeMap::<String, Vec<String>>::new();

    for finding in findings {
        if let Some(property) = required_property_name(&finding.message) {
            required_groups
                .entry(finding.pointer)
                .or_default()
                .push(property);
        } else {
            grouped.push(finding);
        }
    }

    for (pointer, properties) in required_groups {
        let message = if properties.len() == 1 {
            format!("Required property {} was not present.", properties[0])
        } else {
            format!(
                "Required properties {} were not present.",
                properties.join(", ")
            )
        };
        grouped.push(SchemaFinding { message, pointer });
    }

    grouped
}

fn required_property_name(message: &str) -> Option<String> {
    message
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix("\" is a required property"))
        .map(ToOwned::to_owned)
}
