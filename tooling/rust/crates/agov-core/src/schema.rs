use std::path::Path;

/// An opaque wrapper around the compiled JSON schema validator.
///
/// Wrapping avoids leaking the `jsonschema` crate version into the public API.
/// v0.18.x uses `jsonschema::JSONSchema`; newer versions use `jsonschema::Validator`.
/// Callers see only `CompiledSchema` — version differences are invisible.
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
    todo!()
}

/// Validate a JSON value against a compiled schema.
///
/// Returns an empty `Vec` when valid. Each `SchemaFinding` corresponds to one
/// `(message, json_pointer)` pair from the underlying validator — mirroring
/// `ExJsonSchema.Validator.validate/2` error tuples.
pub fn validate(schema: &CompiledSchema, data: &serde_json::Value) -> Vec<SchemaFinding> {
    todo!()
}
