//! Unified release-provenance contract (v2.4.0 Workstream B).
//!
//! One JSON Schema governs every published release artifact: framework
//! source archives, Rust tooling archives, and OCI images. The schema is a
//! closed discriminated union keyed by `artifact_kind`; this module builds
//! and validates records against it, and carries the cross-record checks
//! (artifact/sha256 agreement, duplicate filenames, checksum-sidecar
//! agreement, OCI label agreement) that a JSON Schema alone cannot express.
//!
//! Pre-v2.4 records (no `schema_version` field) are never upgraded
//! implicitly: `validate_record` classifies them against a *separate*
//! schema of recognized historical shapes
//! (`release-provenance-legacy.schema.json`, verified against the actual
//! published v2.3.1 records) and reports `AssuranceLevel::Legacy` only when
//! one matches. Arbitrary JSON, an explicit but unsupported
//! `schema_version`, or a near-miss malformed legacy shape are rejected,
//! not silently accepted as legacy.
//!
//! ## Two copies, one canonical direction
//!
//! Both schemas are embedded from copies inside this crate
//! (`schemas/*.schema.json`), not from `agf/schemas/` directly. `cross
//! build` copies only the `tooling/rust` workspace into an isolated build
//! context for cross-compilation; a path reaching outside `tooling/rust`
//! fails there even though it compiles from a full repository checkout.
//! The adopter-facing `agf/schemas/` copies are canonical; the crate-local
//! copies are plain file copies of them, kept in sync by hand today and
//! verified as parsed-JSON-equal (not byte-identical -- whitespace is not
//! significant) by `crate_schema_matches_agf_schema` in this module's test
//! suite. Any future schema edit must update both files in the same
//! commit -- the test fails otherwise.

use agov_core::schema::{compile_str, validate as schema_validate, CompiledSchema};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::OnceLock;

const SCHEMA_JSON: &str = include_str!("../schemas/release-provenance.schema.json");
const LEGACY_SCHEMA_JSON: &str = include_str!("../schemas/release-provenance-legacy.schema.json");

pub const SCHEMA_VERSION: &str = "2.4";

fn compiled_schema() -> &'static CompiledSchema {
    static SCHEMA: OnceLock<CompiledSchema> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        compile_str(SCHEMA_JSON).expect("embedded release-provenance schema must compile")
    })
}

fn compiled_legacy_schema() -> &'static CompiledSchema {
    static SCHEMA: OnceLock<CompiledSchema> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        compile_str(LEGACY_SCHEMA_JSON)
            .expect("embedded legacy release-provenance schema must compile")
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssuranceLevel {
    V24,
    Legacy,
    /// Neither a valid v2.4 record nor a recognized legacy shape: arbitrary
    /// JSON, an explicit but unsupported `schema_version`, or a near-miss
    /// malformed legacy shape. Always has non-empty `findings`.
    Invalid,
}

impl AssuranceLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            AssuranceLevel::V24 => "v2.4",
            AssuranceLevel::Legacy => "legacy",
            AssuranceLevel::Invalid => "invalid",
        }
    }
}

#[derive(Debug)]
pub struct ValidationOutcome {
    pub assurance_level: AssuranceLevel,
    pub findings: Vec<String>,
}

impl ValidationOutcome {
    pub fn is_valid(&self) -> bool {
        self.findings.is_empty()
    }
}

/// Parse and schema-validate a single provenance record from its JSON text.
///
/// Returns `Err` for malformed JSON (negative case: malformed JSON) rather
/// than a `ValidationOutcome` with findings, since there is no record to
/// classify as v2.4 or legacy without valid JSON first.
pub fn validate_json_text(content: &str) -> Result<ValidationOutcome, String> {
    let data: Value =
        serde_json::from_str(content).map_err(|err| format!("malformed JSON: {err}"))?;
    Ok(validate_record(&data))
}

/// Classify and schema-validate a parsed record.
///
/// - `schema_version: "2.4"` present → scored against the v2.4 schema
///   (`AssuranceLevel::V24`).
/// - `schema_version` present but not `"2.4"` → `AssuranceLevel::Invalid`
///   with a finding naming the unsupported version. Never silently treated
///   as legacy: an explicit version claim that isn't 2.4 is a contract
///   violation, not an old record.
/// - `schema_version` absent → scored against the legacy schema
///   (recognized pre-v2.4 shapes only). A match is
///   `AssuranceLevel::Legacy`; anything else (empty object, array, scalar,
///   unrelated JSON, malformed near-miss legacy shape) is
///   `AssuranceLevel::Invalid`. A legacy-shaped record is NEVER also
///   reported as satisfying the v2.4 contract, even if every field it
///   happens to have would individually pass.
pub fn validate_record(data: &Value) -> ValidationOutcome {
    match data.get("schema_version") {
        Some(Value::String(version)) if version == SCHEMA_VERSION => {
            let findings = schema_validate(compiled_schema(), data)
                .into_iter()
                .map(|finding| format!("{}: {}", finding.pointer, finding.message))
                .collect();
            ValidationOutcome {
                assurance_level: AssuranceLevel::V24,
                findings,
            }
        }
        Some(other) => ValidationOutcome {
            assurance_level: AssuranceLevel::Invalid,
            findings: vec![format!(
                "unsupported schema_version (expected \"{SCHEMA_VERSION}\"): {other}"
            )],
        },
        None => {
            let findings: Vec<String> = schema_validate(compiled_legacy_schema(), data)
                .into_iter()
                .map(|finding| format!("{}: {}", finding.pointer, finding.message))
                .collect();
            if findings.is_empty() {
                ValidationOutcome {
                    assurance_level: AssuranceLevel::Legacy,
                    findings,
                }
            } else {
                let mut all_findings =
                    vec!["not a recognized legacy or v2.4 provenance shape".to_string()];
                all_findings.extend(findings);
                ValidationOutcome {
                    assurance_level: AssuranceLevel::Invalid,
                    findings: all_findings,
                }
            }
        }
    }
}

// ── Builders ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct PipelineIdentity<'a> {
    pub pipeline_id: &'a str,
    pub job_id: &'a str,
}

#[derive(Debug, Clone)]
pub struct FileRecordInput<'a> {
    pub product: &'a str,
    pub version: &'a str,
    pub source_commit: &'a str,
    pub tag: &'a str,
    pub build_timestamp: &'a str,
    pub pipeline_identity: Option<PipelineIdentity<'a>>,
    pub manifest_path: Option<&'a str>,
    pub implementation: &'a str,
    pub packaging: &'a str,
    pub artifact_filename: &'a str,
    pub artifact_bytes: u64,
    pub sha256: &'a str,
    pub target_platform: Option<&'a str>,
}

/// Build an `artifact_kind: file` record (framework source archives and
/// Rust tooling archives share this shape).
pub fn build_file_record(input: &FileRecordInput) -> Value {
    let mut record = serde_json::json!({
        "schema_version": SCHEMA_VERSION,
        "product": input.product,
        "artifact_kind": "file",
        "version": input.version,
        "source_commit": input.source_commit,
        "tag": input.tag,
        "build_timestamp": input.build_timestamp,
        "implementation": input.implementation,
        "packaging": input.packaging,
        "artifact_filename": input.artifact_filename,
        "artifact_bytes": input.artifact_bytes,
        "sha256": input.sha256,
    });
    set_pipeline_identity(&mut record, input.pipeline_identity);
    if let Some(manifest_path) = input.manifest_path {
        record["manifest_path"] = Value::String(manifest_path.to_string());
    }
    if let Some(target_platform) = input.target_platform {
        record["target_platform"] = Value::String(target_platform.to_string());
    }
    record
}

#[derive(Debug, Clone)]
pub struct OciRecordInput<'a> {
    pub product: &'a str,
    pub version: &'a str,
    pub source_commit: &'a str,
    pub tag: &'a str,
    pub build_timestamp: &'a str,
    pub pipeline_identity: Option<PipelineIdentity<'a>>,
    pub implementation: &'a str,
    pub packaging: &'a str,
    pub image_reference: &'a str,
    pub manifest_digest: &'a str,
    pub label_version: &'a str,
    pub label_revision: &'a str,
}

/// Build an `artifact_kind: oci-image` record.
pub fn build_oci_record(input: &OciRecordInput) -> Value {
    let mut record = serde_json::json!({
        "schema_version": SCHEMA_VERSION,
        "product": input.product,
        "artifact_kind": "oci-image",
        "version": input.version,
        "source_commit": input.source_commit,
        "tag": input.tag,
        "build_timestamp": input.build_timestamp,
        "implementation": input.implementation,
        "packaging": input.packaging,
        "image_reference": input.image_reference,
        "manifest_digest": input.manifest_digest,
        "labels": {
            "version": input.label_version,
            "revision": input.label_revision,
        },
    });
    set_pipeline_identity(&mut record, input.pipeline_identity);
    record
}

fn set_pipeline_identity(record: &mut Value, pipeline_identity: Option<PipelineIdentity<'_>>) {
    if let Some(pipeline) = pipeline_identity {
        record["pipeline_identity"] = serde_json::json!({
            "pipeline_id": pipeline.pipeline_id,
            "job_id": pipeline.job_id,
        });
    }
}

// ── Cross-checks (require evidence beyond the record's own JSON shape) ────

/// SHA-256 of a file's contents, as a lowercase hex string.
pub fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file =
        File::open(path).map_err(|err| format!("could not open {}: {err}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// A safe `artifact_filename` is a plain basename: no path separators (`/`
/// or `\`, so this is enforced the same way on Windows and Unix), and not
/// exactly `.` or `..`. The schema's `pattern` on `artifact_filename`
/// already requires an alphanumeric first character (which excludes `.`
/// and `..` structurally), but this check runs independently of schema
/// validation so callers cannot bypass it by skipping `validate_record`.
fn is_safe_artifact_filename(filename: &str) -> bool {
    !filename.is_empty()
        && !filename.contains('/')
        && !filename.contains('\\')
        && filename != "."
        && filename != ".."
}

/// Cross-check a `file`-kind record's `artifact_filename`/`artifact_bytes`/
/// `sha256` against the actual artifact on disk.
///
/// `artifact_dir` is the directory the record's `artifact_filename` is
/// resolved relative to (the published release surface: a local `dist/`
/// during build-time validation, or a directory of freshly-downloaded
/// files during post-publication validation).
pub fn check_artifact_agreement(record: &Value, artifact_dir: &Path) -> Vec<String> {
    let mut findings = Vec::new();
    let Some(filename) = record.get("artifact_filename").and_then(Value::as_str) else {
        return findings;
    };

    if !is_safe_artifact_filename(filename) {
        findings.push(format!(
            "unsafe artifact_filename (must be a plain basename with no path \
             separators and not '.' or '..'): {filename:?}"
        ));
        return findings;
    }

    let artifact_path = artifact_dir.join(filename);

    if !artifact_path.is_file() {
        findings.push(format!(
            "dangling reference: provenance names {filename}, which does not exist in {}",
            artifact_dir.display()
        ));
        return findings;
    }

    if let Some(expected_bytes) = record.get("artifact_bytes").and_then(Value::as_u64) {
        match std::fs::metadata(&artifact_path) {
            Ok(metadata) if metadata.len() != expected_bytes => findings.push(format!(
                "artifact_bytes mismatch for {filename}: record says {expected_bytes}, actual file is {} bytes",
                metadata.len()
            )),
            Ok(_) => {}
            Err(err) => findings.push(format!("could not stat {filename}: {err}")),
        }
    }

    if let Some(expected_sha256) = record.get("sha256").and_then(Value::as_str) {
        match sha256_file(&artifact_path) {
            Ok(actual) if actual != expected_sha256 => findings.push(format!(
                "sha256 mismatch for {filename}: record says {expected_sha256}, actual is {actual}"
            )),
            Ok(_) => {}
            Err(err) => findings.push(err),
        }
    }

    findings
}

/// Cross-check a record's `sha256`/`artifact_filename` against a
/// GNU-coreutils-style `.sha256` sidecar file: exactly one line shaped
/// `<64-char-lowercase-hex><space><mode><filename>`, where `mode` is `' '`
/// (text) or `'*'` (binary) — the shape written by `sha256sum` and by this
/// crate's own `dist::write_checksums`. The sidecar is mandatory: a
/// missing or unreadable file is a finding, not a silent pass. Both the
/// digest and the filename inside the sidecar must agree with the record —
/// a correct hash paired with the wrong filename is rejected.
pub fn check_sha256_sidecar_agreement(record: &Value, sidecar_path: &Path) -> Vec<String> {
    let Some(expected_sha256) = record.get("sha256").and_then(Value::as_str) else {
        return Vec::new();
    };
    let expected_filename = record.get("artifact_filename").and_then(Value::as_str);

    let content = match std::fs::read_to_string(sidecar_path) {
        Ok(content) => content,
        Err(err) => {
            return vec![format!(
                "missing or unreadable checksum sidecar {}: {err}",
                sidecar_path.display()
            )]
        }
    };

    let lines: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    if lines.len() != 1 {
        return vec![format!(
            "checksum sidecar {} must contain exactly one entry, found {}",
            sidecar_path.display(),
            lines.len()
        )];
    }
    let line = lines[0];

    if line.len() < 66 {
        return vec![format!(
            "checksum sidecar {} entry is too short to be \"<sha256>  <filename>\"",
            sidecar_path.display()
        )];
    }
    let hex = &line[0..64];
    let separator = &line[64..66];
    let sidecar_filename = &line[66..];

    if separator != "  " && separator != " *" {
        return vec![format!(
            "checksum sidecar {} entry has a malformed delimiter {separator:?} (expected \"  \" or \" *\")",
            sidecar_path.display()
        )];
    }

    let mut findings = Vec::new();
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        findings.push(format!(
            "checksum sidecar {} entry does not start with a valid hex digest",
            sidecar_path.display()
        ));
    }
    if hex != expected_sha256 {
        findings.push(format!(
            "checksum format inconsistency: sidecar {} says {hex}, record says {expected_sha256}",
            sidecar_path.display()
        ));
    }
    if let Some(expected_filename) = expected_filename {
        if sidecar_filename != expected_filename {
            findings.push(format!(
                "checksum sidecar {} names {sidecar_filename:?}, record artifact_filename is {expected_filename:?}",
                sidecar_path.display()
            ));
        }
    }
    findings
}

/// Cross-check that no two `file`-kind records in a release publish the
/// same `artifact_filename`.
pub fn check_no_duplicate_filenames(records: &[Value]) -> Vec<String> {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for record in records {
        if let Some(filename) = record.get("artifact_filename").and_then(Value::as_str) {
            *seen.entry(filename.to_string()).or_insert(0) += 1;
        }
    }
    seen.into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(filename, count)| {
            format!("duplicate artifact_filename across records: {filename} ({count} records)")
        })
        .collect()
}

/// Cross-check that an `oci-image` record's `labels.version`/`labels.revision`
/// agree with its own `version`/`source_commit` fields — i.e. the provenance
/// JSON agrees with the OCI image labels it describes.
pub fn check_label_agreement(record: &Value) -> Vec<String> {
    let mut findings = Vec::new();
    let version = record.get("version").and_then(Value::as_str);
    let source_commit = record.get("source_commit").and_then(Value::as_str);
    let label_version = record
        .get("labels")
        .and_then(|labels| labels.get("version"))
        .and_then(Value::as_str);
    let label_revision = record
        .get("labels")
        .and_then(|labels| labels.get("revision"))
        .and_then(Value::as_str);

    if version.is_some() && version != label_version {
        findings.push(format!(
            "OCI label disagreement: record version {:?} != labels.version {:?}",
            version, label_version
        ));
    }
    if source_commit.is_some() && source_commit != label_revision {
        findings.push(format!(
            "OCI label disagreement: record source_commit {:?} != labels.revision {:?}",
            source_commit, label_revision
        ));
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn valid_file_record() -> Value {
        build_file_record(&FileRecordInput {
            product: "agf",
            version: "2.4.0",
            source_commit: "abc123",
            tag: "v2.4.0",
            build_timestamp: "2026-07-21T00:00:00Z",
            pipeline_identity: Some(PipelineIdentity {
                pipeline_id: "1",
                job_id: "2",
            }),
            manifest_path: Some("agf/dist/release-manifest.yaml"),
            implementation: "rust",
            packaging: "manifest-source-archive",
            artifact_filename: "agf-2.4.0.tar.gz",
            artifact_bytes: 4,
            sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            target_platform: None,
        })
    }

    fn valid_oci_record() -> Value {
        build_oci_record(&OciRecordInput {
            product: "agov",
            version: "2.4.0",
            source_commit: "abc123",
            tag: "v2.4.0",
            build_timestamp: "2026-07-21T00:00:00Z",
            pipeline_identity: None,
            implementation: "rust",
            packaging: "oci-image",
            image_reference: "registry.example.com/agov:2.4.0",
            manifest_digest:
                "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            label_version: "2.4.0",
            label_revision: "abc123",
        })
    }

    // ── Positive cases ──────────────────────────────────────────────────

    #[test]
    fn valid_file_record_passes_v24_validation() {
        let outcome = validate_record(&valid_file_record());
        assert_eq!(outcome.assurance_level, AssuranceLevel::V24);
        assert!(outcome.is_valid(), "findings: {:?}", outcome.findings);
    }

    #[test]
    fn valid_oci_record_passes_v24_validation() {
        let outcome = validate_record(&valid_oci_record());
        assert_eq!(outcome.assurance_level, AssuranceLevel::V24);
        assert!(outcome.is_valid(), "findings: {:?}", outcome.findings);
    }

    /// The exact JSON published at
    /// https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.3.1/agf-2.3.1-provenance.json
    /// (fetched 2026-07-21) -- the real historical shape, not a synthetic
    /// approximation of it.
    fn real_v231_framework_record() -> Value {
        serde_json::json!({
            "archive_filenames": ["agf-2.3.1.tar.gz", "agf-2.3.1.zip"],
            "build_timestamp": "2026-07-21T15:56:48.252871887Z",
            "checksums": {
                "agf-2.3.1.tar.gz": "cf79ad8bb5a5e7378ea704c56675eabc32c6d3268910a701abf8231893eb0fda",
                "agf-2.3.1.zip": "c079a64c9cff468b075f44849323d9b4ec4c1e240b0b9bc63c10c70ed8c1993f"
            },
            "implementation": "rust",
            "manifest_path": "agf/dist/release-manifest.yaml",
            "packaging": "manifest-source-archive",
            "pipeline_identity": {"job_id": "15457824866", "pipeline_id": "2694360644"},
            "source_commit": "1f7359896e81edcc7f092f55d156c6be9b2eecdc",
            "tag": "v2.3.1",
            "version": "2.3.1"
        })
    }

    /// The exact JSON published at
    /// https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.3.1/agov-2.3.1-x86_64-unknown-linux-musl-provenance.json
    /// (fetched 2026-07-21).
    fn real_v231_tooling_record() -> Value {
        serde_json::json!({
            "version": "2.3.1",
            "source_commit": "1f7359896e81edcc7f092f55d156c6be9b2eecdc",
            "tag": "v2.3.1",
            "build_timestamp": "2026-07-21T15:39:25Z",
            "pipeline_identity": {"pipeline_id": "2694360644", "job_id": "15457824857"},
            "target_platform": "x86_64-unknown-linux-musl",
            "implementation": "rust",
            "packaging": "tar.gz"
        })
    }

    #[test]
    fn real_v231_framework_record_is_recognized_as_legacy() {
        let outcome = validate_record(&real_v231_framework_record());
        assert_eq!(outcome.assurance_level, AssuranceLevel::Legacy);
        assert!(outcome.is_valid(), "findings: {:?}", outcome.findings);
    }

    #[test]
    fn real_v231_tooling_record_is_recognized_as_legacy() {
        let outcome = validate_record(&real_v231_tooling_record());
        assert_eq!(outcome.assurance_level, AssuranceLevel::Legacy);
        assert!(outcome.is_valid(), "findings: {:?}", outcome.findings);
    }

    #[test]
    fn legacy_record_is_never_scored_as_satisfying_v24() {
        // Every field individually would pass a lenient reading, but this
        // shape (no schema_version, no artifact_kind) must never be
        // reported as V24 -- it is Legacy or nothing.
        let outcome = validate_record(&real_v231_framework_record());
        assert_ne!(outcome.assurance_level, AssuranceLevel::V24);
    }

    #[test]
    fn oci_label_agreement_passes_when_labels_match() {
        assert!(check_label_agreement(&valid_oci_record()).is_empty());
    }

    // ── Negative cases for legacy/version classification (finding #1) ───

    #[test]
    fn negative_empty_object_is_invalid_not_legacy() {
        let outcome = validate_record(&serde_json::json!({}));
        assert_eq!(outcome.assurance_level, AssuranceLevel::Invalid);
        assert!(!outcome.is_valid());
    }

    #[test]
    fn negative_json_array_is_invalid() {
        let outcome = validate_record(&serde_json::json!([1, 2, 3]));
        assert_eq!(outcome.assurance_level, AssuranceLevel::Invalid);
    }

    #[test]
    fn negative_json_scalar_is_invalid() {
        let outcome = validate_record(&serde_json::json!("just a string"));
        assert_eq!(outcome.assurance_level, AssuranceLevel::Invalid);
    }

    #[test]
    fn negative_unsupported_schema_version_is_invalid_not_legacy_or_v24() {
        let mut record = real_v231_framework_record();
        record["schema_version"] = serde_json::json!("2.5");
        let outcome = validate_record(&record);
        assert_eq!(outcome.assurance_level, AssuranceLevel::Invalid);
        assert!(outcome
            .findings
            .iter()
            .any(|f| f.contains("unsupported schema_version")));
    }

    #[test]
    fn negative_misspelled_v24_version_is_invalid() {
        let mut record = valid_file_record();
        record["schema_version"] = serde_json::json!("2.4.0");
        let outcome = validate_record(&record);
        assert_eq!(outcome.assurance_level, AssuranceLevel::Invalid);
    }

    #[test]
    fn negative_near_legacy_shape_missing_field_is_invalid() {
        let mut record = real_v231_framework_record();
        record.as_object_mut().unwrap().remove("checksums");
        let outcome = validate_record(&record);
        assert_eq!(outcome.assurance_level, AssuranceLevel::Invalid);
    }

    #[test]
    fn negative_unrelated_json_with_no_recognized_fields_is_invalid() {
        let outcome = validate_record(&serde_json::json!({"hello": "world", "n": 1}));
        assert_eq!(outcome.assurance_level, AssuranceLevel::Invalid);
    }

    // ── Negative case 1: missing required core field ───────────────────

    #[test]
    fn negative_missing_required_core_field_is_rejected() {
        let mut record = valid_file_record();
        record.as_object_mut().unwrap().remove("version");
        let outcome = validate_record(&record);
        assert_eq!(outcome.assurance_level, AssuranceLevel::V24);
        assert!(!outcome.is_valid());
        assert!(outcome.findings.iter().any(|f| f.contains("version")));
    }

    // ── Negative cases for target_platform / product-kind closed subtype
    // (finding #7) ────────────────────────────────────────────────────

    #[test]
    fn positive_framework_record_without_target_platform_passes() {
        // valid_file_record() is product agf / packaging manifest-source-archive
        // / no target_platform -- exactly the required framework shape.
        let outcome = validate_record(&valid_file_record());
        assert!(outcome.is_valid(), "findings: {:?}", outcome.findings);
    }

    #[test]
    fn negative_tooling_record_without_target_platform_is_rejected() {
        let record = build_file_record(&FileRecordInput {
            product: "agov",
            version: "2.4.0",
            source_commit: "abc123",
            tag: "v2.4.0",
            build_timestamp: "2026-07-21T00:00:00Z",
            pipeline_identity: None,
            manifest_path: None,
            implementation: "rust",
            packaging: "tar.gz",
            artifact_filename: "agov-2.4.0-x86_64-unknown-linux-musl.tar.gz",
            artifact_bytes: 4,
            sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            target_platform: None,
        });
        let outcome = validate_record(&record);
        assert!(!outcome.is_valid());
    }

    #[test]
    fn negative_framework_record_with_target_platform_is_rejected() {
        let mut record = valid_file_record();
        record["target_platform"] = serde_json::json!("x86_64-unknown-linux-musl");
        let outcome = validate_record(&record);
        assert!(!outcome.is_valid());
    }

    #[test]
    fn negative_wrong_product_for_packaging_is_rejected() {
        // packaging manifest-source-archive requires product "agf", not "agov".
        let mut record = valid_file_record();
        record["product"] = serde_json::json!("agov");
        let outcome = validate_record(&record);
        assert!(!outcome.is_valid());
    }

    // ── Negative cases for artifact_filename path traversal (finding #8) ─

    #[test]
    fn negative_absolute_path_artifact_filename_is_rejected() {
        let dir = TempDir::new().unwrap();
        let mut record = valid_file_record();
        record["artifact_filename"] = serde_json::json!("/etc/passwd");
        let findings = check_artifact_agreement(&record, dir.path());
        assert!(findings
            .iter()
            .any(|f| f.contains("unsafe artifact_filename")));
    }

    #[test]
    fn negative_parent_traversal_artifact_filename_is_rejected() {
        let dir = TempDir::new().unwrap();
        let mut record = valid_file_record();
        record["artifact_filename"] = serde_json::json!("../../etc/passwd");
        let findings = check_artifact_agreement(&record, dir.path());
        assert!(findings
            .iter()
            .any(|f| f.contains("unsafe artifact_filename")));
    }

    #[test]
    fn negative_nested_subdir_artifact_filename_is_rejected() {
        let dir = TempDir::new().unwrap();
        let mut record = valid_file_record();
        record["artifact_filename"] = serde_json::json!("sub/dir/agf-2.4.0.tar.gz");
        let findings = check_artifact_agreement(&record, dir.path());
        assert!(findings
            .iter()
            .any(|f| f.contains("unsafe artifact_filename")));
    }

    #[test]
    fn negative_windows_backslash_artifact_filename_is_rejected() {
        let dir = TempDir::new().unwrap();
        let mut record = valid_file_record();
        record["artifact_filename"] = serde_json::json!("..\\..\\evil.tar.gz");
        let findings = check_artifact_agreement(&record, dir.path());
        assert!(findings
            .iter()
            .any(|f| f.contains("unsafe artifact_filename")));
    }

    #[test]
    fn negative_artifact_filename_with_unsafe_chars_is_rejected_by_schema() {
        let mut record = valid_file_record();
        record["artifact_filename"] = serde_json::json!("../evil");
        let outcome = validate_record(&record);
        assert!(!outcome.is_valid());
    }

    // ── Negative case 2: mismatched sha256 vs real artifact ────────────

    #[test]
    fn negative_mismatched_sha256_vs_real_artifact_is_rejected() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz"), b"real-bytes").unwrap();
        let mut record = valid_file_record();
        record["artifact_bytes"] = serde_json::json!(10);
        record["sha256"] =
            serde_json::json!("0000000000000000000000000000000000000000000000000000000000000000");
        let findings = check_artifact_agreement(&record, dir.path());
        assert!(findings.iter().any(|f| f.contains("sha256 mismatch")));
    }

    // ── Negative case 3: dangling reference ─────────────────────────────

    #[test]
    fn negative_dangling_reference_is_rejected() {
        let dir = TempDir::new().unwrap();
        let record = valid_file_record();
        let findings = check_artifact_agreement(&record, dir.path());
        assert!(findings.iter().any(|f| f.contains("dangling reference")));
    }

    // ── Negative case 4: duplicated artifact_filename across records ───

    #[test]
    fn negative_duplicated_artifact_filename_across_records_is_rejected() {
        let records = vec![valid_file_record(), valid_file_record()];
        let findings = check_no_duplicate_filenames(&records);
        assert!(findings
            .iter()
            .any(|f| f.contains("duplicate artifact_filename")));
    }

    #[test]
    fn positive_distinct_artifact_filenames_produce_no_duplicate_finding() {
        let mut second = valid_file_record();
        second["artifact_filename"] = serde_json::json!("agf-2.4.0.zip");
        let records = vec![valid_file_record(), second];
        assert!(check_no_duplicate_filenames(&records).is_empty());
    }

    // ── Negative case 5: malformed JSON ─────────────────────────────────

    #[test]
    fn negative_malformed_json_is_rejected() {
        let error = validate_json_text("{not json").unwrap_err();
        assert!(error.contains("malformed JSON"));
    }

    // ── Negative case 6: unknown/extra key and wrong-kind field ─────────

    #[test]
    fn negative_unknown_top_level_key_is_rejected() {
        let mut record = valid_file_record();
        record["unexpected_field"] = serde_json::json!("nope");
        let outcome = validate_record(&record);
        assert!(!outcome.is_valid());
    }

    #[test]
    fn negative_wrong_kind_field_on_file_record_is_rejected() {
        let mut record = valid_file_record();
        // image_reference belongs only to artifact_kind: oci-image.
        record["image_reference"] = serde_json::json!("registry.example.com/agov:2.4.0");
        let outcome = validate_record(&record);
        assert!(!outcome.is_valid());
    }

    // ── Negative case 7: checksum format inconsistency ──────────────────

    #[test]
    fn negative_checksum_sidecar_disagreement_is_rejected() {
        let dir = TempDir::new().unwrap();
        let sidecar = dir.path().join("agf-2.4.0.tar.gz.sha256");
        std::fs::write(
            &sidecar,
            "1111111111111111111111111111111111111111111111111111111111111111  agf-2.4.0.tar.gz\n",
        )
        .unwrap();
        let record = valid_file_record();
        let findings = check_sha256_sidecar_agreement(&record, &sidecar);
        assert!(findings
            .iter()
            .any(|f| f.contains("checksum format inconsistency")));
    }

    #[test]
    fn positive_checksum_sidecar_agreement_passes() {
        let dir = TempDir::new().unwrap();
        let sidecar = dir.path().join("agf-2.4.0.tar.gz.sha256");
        std::fs::write(
            &sidecar,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  agf-2.4.0.tar.gz\n",
        )
        .unwrap();
        let record = valid_file_record();
        assert!(check_sha256_sidecar_agreement(&record, &sidecar).is_empty());
    }

    #[test]
    fn negative_missing_sidecar_is_mandatory_not_silently_skipped() {
        let dir = TempDir::new().unwrap();
        let sidecar = dir.path().join("does-not-exist.sha256");
        let record = valid_file_record();
        let findings = check_sha256_sidecar_agreement(&record, &sidecar);
        assert!(findings
            .iter()
            .any(|f| f.contains("missing or unreadable checksum sidecar")));
    }

    #[test]
    fn negative_sidecar_correct_hash_wrong_filename_is_rejected() {
        let dir = TempDir::new().unwrap();
        let sidecar = dir.path().join("agf-2.4.0.tar.gz.sha256");
        std::fs::write(
            &sidecar,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  wrong-name.tar.gz\n",
        )
        .unwrap();
        let record = valid_file_record();
        let findings = check_sha256_sidecar_agreement(&record, &sidecar);
        assert!(findings.iter().any(|f| f.contains("names")));
    }

    #[test]
    fn negative_sidecar_malformed_delimiter_is_rejected() {
        let dir = TempDir::new().unwrap();
        let sidecar = dir.path().join("agf-2.4.0.tar.gz.sha256");
        // Single space instead of the required two-char separator.
        std::fs::write(
            &sidecar,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 agf-2.4.0.tar.gz\n",
        )
        .unwrap();
        let record = valid_file_record();
        let findings = check_sha256_sidecar_agreement(&record, &sidecar);
        assert!(findings.iter().any(|f| f.contains("malformed delimiter")));
    }

    #[test]
    fn negative_sidecar_multiple_entries_is_rejected() {
        let dir = TempDir::new().unwrap();
        let sidecar = dir.path().join("agf-2.4.0.tar.gz.sha256");
        std::fs::write(
            &sidecar,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  agf-2.4.0.tar.gz\n\
             e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  agf-2.4.0.zip\n",
        )
        .unwrap();
        let record = valid_file_record();
        let findings = check_sha256_sidecar_agreement(&record, &sidecar);
        assert!(findings.iter().any(|f| f.contains("exactly one entry")));
    }

    /// The crate-local embedded schema and the adopter-facing
    /// `agf/schemas/` copy must never drift. This test needs a full
    /// repository checkout (via `agov_core::repo_root()`), unlike the
    /// embedded `SCHEMA_JSON` constant itself, which compiles fine inside
    /// the isolated `cross build` context that only has `tooling/rust`.
    #[test]
    fn crate_schema_matches_agf_schema() {
        let agf_schema_path = agov_core::repo_root()
            .join("agf")
            .join("schemas")
            .join("release-provenance.schema.json");
        let agf_schema_text = std::fs::read_to_string(&agf_schema_path)
            .unwrap_or_else(|err| panic!("could not read {}: {err}", agf_schema_path.display()));

        let embedded: Value =
            serde_json::from_str(SCHEMA_JSON).expect("embedded schema is valid JSON");
        let canonical: Value =
            serde_json::from_str(&agf_schema_text).expect("agf/schemas copy is valid JSON");

        assert_eq!(
            embedded, canonical,
            "tooling/rust/crates/agov/schemas/release-provenance.schema.json has drifted from \
             agf/schemas/release-provenance.schema.json -- update both files in the same commit"
        );
    }
}
