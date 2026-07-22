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

/// A safe basename: no path separators (`/` or `\`, so this is enforced
/// the same way on Windows and Unix), and not exactly `.` or `..`. Used
/// both for a record's own `artifact_filename` (the schema's `pattern`
/// already requires an alphanumeric first character, which excludes `.`
/// and `..` structurally, but this check runs independently of schema
/// validation so callers cannot bypass it by skipping `validate_record`)
/// and for externally-discovered release-asset names (GitLab Release API
/// `assets.links[].name`), which carry no schema guarantee at all.
pub fn is_safe_basename(filename: &str) -> bool {
    !filename.is_empty()
        && !filename.contains('/')
        && !filename.contains('\\')
        && filename != "."
        && filename != ".."
}

/// Extract the scheme and host from a URL without a full RFC 3986 parser.
/// Deliberately narrow: this project only needs to (a) require `https`
/// and (b) compare the host against a trusted host derived from the
/// GitLab Releases API URL that was already fetched. Handles the
/// userinfo trick (`https://trusted.example@evil.example/path`, where a
/// naive parser stops at the first `/` or `@` and reports the wrong
/// host) by taking the host as whatever follows the *last* `@` in the
/// authority component, per URL authority syntax (`user:pass@host:port`).
pub fn parse_https_host(url: &str) -> Result<String, String> {
    let rest = url
        .strip_prefix("https://")
        .ok_or_else(|| format!("URL is not https: {url}"))?;
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    let host_and_port = match authority.rfind('@') {
        Some(at) => &authority[at + 1..],
        None => authority,
    };
    let host = match host_and_port.rfind(':') {
        Some(colon) => &host_and_port[..colon],
        None => host_and_port,
    };
    if host.is_empty() {
        return Err(format!("URL has no host: {url}"));
    }
    Ok(host.to_string())
}

/// Validate a batch of externally-discovered release assets
/// (`(name, direct_asset_url)` pairs from the GitLab Release API) before
/// anything is downloaded. Fails closed: any single unsafe name,
/// duplicate name, non-https URL, or wrong-host URL rejects the whole
/// batch rather than silently dropping the bad entry and downloading the
/// rest.
pub fn validate_discovered_assets(
    assets: &[(String, String)],
    trusted_host: &str,
) -> Result<(), String> {
    let mut seen = std::collections::BTreeSet::new();
    for (name, url) in assets {
        if !is_safe_basename(name) {
            return Err(format!("unsafe asset name in release metadata: {name:?}"));
        }
        if !seen.insert(name.as_str()) {
            return Err(format!(
                "duplicate asset name in release metadata: {name:?}"
            ));
        }
        let host = parse_https_host(url)
            .map_err(|err| format!("asset {name:?} has an untrusted URL: {err}"))?;
        if host != trusted_host {
            return Err(format!(
                "asset {name:?} URL host {host:?} does not match trusted host {trusted_host:?}: {url}"
            ));
        }
    }
    Ok(())
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

    if !is_safe_basename(filename) {
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

/// Cross-check that no two `oci-image` records share the same
/// `image_reference` or the same `manifest_digest`.
pub fn check_no_duplicate_oci_identity(records: &[Value]) -> Vec<String> {
    let mut findings = Vec::new();
    let mut seen_refs: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen_digests: BTreeMap<String, usize> = BTreeMap::new();

    for record in records {
        if record.get("artifact_kind").and_then(Value::as_str) != Some("oci-image") {
            continue;
        }
        if let Some(reference) = record.get("image_reference").and_then(Value::as_str) {
            *seen_refs.entry(reference.to_string()).or_insert(0) += 1;
        }
        if let Some(digest) = record.get("manifest_digest").and_then(Value::as_str) {
            *seen_digests.entry(digest.to_string()).or_insert(0) += 1;
        }
    }

    findings.extend(seen_refs.into_iter().filter(|(_, count)| *count > 1).map(
        |(reference, count)| {
            format!("duplicate OCI image_reference across records: {reference} ({count} records)")
        },
    ));
    findings.extend(
        seen_digests
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(digest, count)| {
                format!("duplicate OCI manifest_digest across records: {digest} ({count} records)")
            }),
    );
    findings
}

/// List the primary release artifacts actually present in `artifact_dir`:
/// real files, excluding `.sha256` checksum sidecars and
/// `*-provenance.json` records themselves (which describe artifacts, and
/// are not artifacts).
pub fn list_primary_artifacts(artifact_dir: &Path) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(artifact_dir).map_err(|err| {
        format!(
            "could not read artifact directory {}: {err}",
            artifact_dir.display()
        )
    })?;
    let mut artifacts = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|err| format!("could not read a directory entry: {err}"))?;
        if !entry.path().is_file() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        if name.ends_with(".sha256") || name.ends_with("-provenance.json") {
            continue;
        }
        artifacts.push(name);
    }
    artifacts.sort();
    Ok(artifacts)
}

/// Cross-check that `records` (the full set passed to the validator) form
/// an exact bijection with the primary artifacts actually present in
/// `artifact_dir`: every primary artifact has exactly one `file`-kind
/// record, and every `file`-kind record's `artifact_filename` names a
/// primary artifact that actually exists. Catches a primary artifact with
/// no record, a record omitted from validation, and stale/extra
/// provenance records left over from a previous artifact set (e.g. the
/// old aggregate shape, or a record for a since-removed file).
///
/// Bijection is a v2.4 contract guarantee, not retroactively imposed on
/// pre-v2.4 releases: legacy shapes (framework aggregate, tooling) have no
/// queryable per-artifact `artifact_filename` field to check against, so a
/// batch containing no `schema_version: "2.4"` record at all (e.g.
/// validating a real historical release like v2.3.1 in backward-compat
/// mode) returns no findings here rather than false-flagging every
/// legacy-described artifact as "no record".
pub fn check_primary_artifact_coverage(records: &[Value], artifact_dir: &Path) -> Vec<String> {
    let has_v24_record = records
        .iter()
        .any(|record| record.get("schema_version").and_then(Value::as_str) == Some(SCHEMA_VERSION));
    if !has_v24_record {
        return Vec::new();
    }

    let primary_artifacts = match list_primary_artifacts(artifact_dir) {
        Ok(artifacts) => artifacts,
        Err(err) => return vec![err],
    };

    let record_filenames: std::collections::BTreeSet<&str> = records
        .iter()
        .filter(|record| record.get("artifact_kind").and_then(Value::as_str) == Some("file"))
        .filter_map(|record| record.get("artifact_filename").and_then(Value::as_str))
        .collect();

    let mut findings = Vec::new();
    for artifact in &primary_artifacts {
        if !record_filenames.contains(artifact.as_str()) {
            findings.push(format!(
                "primary artifact {artifact} in {} has no provenance record among the validated files",
                artifact_dir.display()
            ));
        }
    }
    for filename in &record_filenames {
        if !primary_artifacts
            .iter()
            .any(|artifact| artifact == filename)
        {
            findings.push(format!(
                "provenance record names {filename:?}, which is not a primary artifact present in {} \
                 (stale, extra, or renamed artifact)",
                artifact_dir.display()
            ));
        }
    }
    findings
}

/// Real evidence read from an actual OCI image — via `podman inspect` at
/// the CLI layer, never fabricated from the record's own fields. Passing
/// evidence gathered this way is what distinguishes `check_oci_image_evidence`
/// from a self-comparison: the record's `labels` object is self-reported by
/// the same generator call that wrote `version`/`source_commit`, so
/// comparing a record to itself can never fail and proves nothing about
/// the actual published image.
#[derive(Debug, Clone, Default)]
pub struct OciImageEvidence {
    /// The image's real `Config.Labels`, as reported by `podman inspect`
    /// (or `docker inspect` / registry manifest introspection).
    pub labels: BTreeMap<String, String>,
    /// The image's real manifest digest (`sha256:...`), if known.
    pub digest: Option<String>,
}

/// Cross-check an `oci-image` record against REAL evidence read from the
/// actual image, not against the record's own self-reported `labels`
/// field. Compares `record.version`/`record.source_commit` *and*
/// `record.labels.version`/`record.labels.revision` (a record can
/// disagree with itself between the top-level fields and its own
/// `labels` sub-object; both must independently agree with reality) to
/// the image's actual `org.opencontainers.image.{version,revision}`
/// labels, and requires `evidence.digest` to be present and equal to
/// `record.manifest_digest` — a missing digest is a finding, not a
/// silently-skipped check, because "no digest evidence" is exactly the
/// gap that lets a record claim an immutable pin with nothing behind it.
pub fn check_oci_image_evidence(record: &Value, evidence: &OciImageEvidence) -> Vec<String> {
    let mut findings = Vec::new();
    let version = record.get("version").and_then(Value::as_str);
    let source_commit = record.get("source_commit").and_then(Value::as_str);
    let labels_version = record
        .get("labels")
        .and_then(|labels| labels.get("version"))
        .and_then(Value::as_str);
    let labels_revision = record
        .get("labels")
        .and_then(|labels| labels.get("revision"))
        .and_then(Value::as_str);
    let real_version = evidence
        .labels
        .get("org.opencontainers.image.version")
        .map(String::as_str);
    let real_revision = evidence
        .labels
        .get("org.opencontainers.image.revision")
        .map(String::as_str);

    if version != real_version {
        findings.push(format!(
            "OCI image evidence disagreement: record version {version:?} != actual image label org.opencontainers.image.version {real_version:?}"
        ));
    }
    if source_commit != real_revision {
        findings.push(format!(
            "OCI image evidence disagreement: record source_commit {source_commit:?} != actual image label org.opencontainers.image.revision {real_revision:?}"
        ));
    }
    if labels_version != real_version {
        findings.push(format!(
            "OCI image evidence disagreement: record labels.version {labels_version:?} != actual image label org.opencontainers.image.version {real_version:?}"
        ));
    }
    if labels_revision != real_revision {
        findings.push(format!(
            "OCI image evidence disagreement: record labels.revision {labels_revision:?} != actual image label org.opencontainers.image.revision {real_revision:?}"
        ));
    }

    let record_digest = record.get("manifest_digest").and_then(Value::as_str);
    match &evidence.digest {
        Some(actual_digest) => {
            if record_digest != Some(actual_digest.as_str()) {
                findings.push(format!(
                    "OCI image evidence disagreement: record manifest_digest {record_digest:?} != actual image digest {actual_digest:?}"
                ));
            }
        }
        None => {
            findings.push(
                "OCI image evidence missing: no real registry digest available to compare \
                 against record manifest_digest (a record cannot be accepted on label \
                 agreement alone without digest evidence)"
                    .to_string(),
            );
        }
    }

    findings
}

/// Build the immutable pull reference for an `oci-image` record: strip any
/// `:tag` suffix from `image_reference` (a reference cannot carry both a
/// tag and a digest in the pinned form this project uses) and pin to
/// `manifest_digest` instead, so the composed post-publication check
/// always pulls the exact published image, never "whatever the mutable
/// tag currently points to."
pub fn oci_pull_ref(image_reference: &str, manifest_digest: &str) -> String {
    let last_slash = image_reference.rfind('/').map(|i| i + 1).unwrap_or(0);
    let (host_and_path, name) = image_reference.split_at(last_slash);
    let base = match name.rfind(':') {
        Some(colon) => &name[..colon],
        None => name,
    };
    format!("{host_and_path}{base}@{manifest_digest}")
}

/// Parse `podman inspect <image>` (or `docker inspect`) JSON output — a
/// one-element array, `Config.Labels` an object of string→string — into
/// `OciImageEvidence`. The digest is read from `Digest` if present
/// (registry-side digest), else left `None` (a purely local image build
/// has no registry digest yet).
pub fn parse_podman_inspect(inspect_json: &str) -> Result<OciImageEvidence, String> {
    let parsed: Value = serde_json::from_str(inspect_json)
        .map_err(|err| format!("malformed podman inspect JSON: {err}"))?;
    let entry = parsed
        .as_array()
        .and_then(|array| array.first())
        .ok_or_else(|| "podman inspect JSON is not a non-empty array".to_string())?;

    let labels = entry
        .get("Config")
        .and_then(|config| config.get("Labels"))
        .and_then(Value::as_object)
        .map(|labels| {
            labels
                .iter()
                .filter_map(|(key, value)| {
                    value.as_str().map(|value| (key.clone(), value.to_string()))
                })
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();

    let digest = entry
        .get("Digest")
        .and_then(Value::as_str)
        .map(str::to_string);

    Ok(OciImageEvidence { labels, digest })
}

// ── Executable self-report agreement ───────────────────────────────────

/// The packaged binary's own `agov version --format json` output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfReport {
    pub version: String,
    pub commit: String,
    pub dirty: bool,
}

/// Parse `agov version --format json` output (`{"version":...,
/// "commit":...,"dirty":...}`).
pub fn parse_self_report(json: &str) -> Result<SelfReport, String> {
    let parsed: Value =
        serde_json::from_str(json).map_err(|err| format!("malformed self-report JSON: {err}"))?;
    let version = parsed
        .get("version")
        .and_then(Value::as_str)
        .ok_or("self-report JSON missing string field \"version\"")?
        .to_string();
    let commit = parsed
        .get("commit")
        .and_then(Value::as_str)
        .ok_or("self-report JSON missing string field \"commit\"")?
        .to_string();
    let dirty = parsed
        .get("dirty")
        .and_then(Value::as_bool)
        .ok_or("self-report JSON missing boolean field \"dirty\"")?;
    Ok(SelfReport {
        version,
        commit,
        dirty,
    })
}

/// Run `<binary_path> version --format json` and parse its self-report.
pub fn run_self_report(binary_path: &Path) -> Result<SelfReport, String> {
    let output = std::process::Command::new(binary_path)
        .args(["version", "--format", "json"])
        .output()
        .map_err(|err| format!("could not run {}: {err}", binary_path.display()))?;
    if !output.status.success() {
        return Err(format!(
            "{} version --format json failed: {}",
            binary_path.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    parse_self_report(&String::from_utf8_lossy(&output.stdout))
}

/// Minimum length (in hex characters) either commit string must have
/// before a prefix relationship is trusted as agreement. Chosen to match
/// `git`'s own default abbreviation floor (`git rev-parse --short`
/// guarantees at least 7; this project's local dev fallback uses 12) —
/// short enough to accept both a full 40-char CI SHA and a local
/// 12-char short SHA, long enough that an accidental collision is not a
/// realistic concern. An exact match is always accepted regardless of
/// length (covers short==short comparisons of any length).
const MIN_COMMIT_PREFIX_LEN: usize = 7;

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Cross-check a `file`-kind record against the packaged executable's own
/// self-report: `record.version` must equal `self_report.version` exactly;
/// `record.source_commit` must agree with `self_report.commit` — either
/// exactly equal, or one a true prefix of the other with both strings
/// hex and the shorter one at least `MIN_COMMIT_PREFIX_LEN` characters
/// (a full CI-set `AGOV_BUILD_COMMIT` SHA and a local dev build's
/// `git rev-parse --short=12` fallback are both legitimate, differently
/// lengthed forms of the same commit; an arbitrary short/ambiguous
/// prefix is not); and the packaged binary must never report a dirty
/// build (`self_report.dirty` must be `false`) — a real release artifact
/// is never built from an unclean working tree.
pub fn check_self_report_agreement(record: &Value, self_report: &SelfReport) -> Vec<String> {
    let mut findings = Vec::new();
    let record_version = record.get("version").and_then(Value::as_str);
    if record_version != Some(self_report.version.as_str()) {
        findings.push(format!(
            "self-report disagreement: record version {record_version:?} != packaged binary's own version {:?}",
            self_report.version
        ));
    }

    let record_commit = record.get("source_commit").and_then(Value::as_str);
    let commits_agree = match record_commit {
        Some(record_commit) => {
            record_commit == self_report.commit
                || (is_hex(record_commit)
                    && is_hex(&self_report.commit)
                    && record_commit.len().min(self_report.commit.len()) >= MIN_COMMIT_PREFIX_LEN
                    && (record_commit.starts_with(&self_report.commit)
                        || self_report.commit.starts_with(record_commit)))
        }
        None => false,
    };
    if !commits_agree {
        findings.push(format!(
            "self-report disagreement: record source_commit {record_commit:?} != packaged binary's own commit {:?} \
             (exact match, or hex prefix of at least {MIN_COMMIT_PREFIX_LEN} characters, required)",
            self_report.commit
        ));
    }

    if self_report.dirty {
        findings.push(
            "self-report disagreement: packaged binary reports a dirty build \
             (a published release artifact must never be built from an unclean working tree)"
                .to_string(),
        );
    }

    findings
}

/// Extract a single named entry from a `.tar.gz` archive to `dest_path`,
/// setting the executable bit on Unix. Used to run a packaged binary's
/// own self-report without needing the whole archive extracted.
pub fn extract_from_tar_gz(
    archive_path: &Path,
    entry_name: &str,
    dest_path: &Path,
) -> Result<(), String> {
    let file = File::open(archive_path)
        .map_err(|err| format!("could not open {}: {err}", archive_path.display()))?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    let entries = archive.entries().map_err(|err| {
        format!(
            "could not read tar entries in {}: {err}",
            archive_path.display()
        )
    })?;
    for entry in entries {
        let mut entry = entry.map_err(|err| format!("could not read a tar entry: {err}"))?;
        let path = entry
            .path()
            .map_err(|err| format!("could not read a tar entry path: {err}"))?
            .into_owned();
        if path.file_name().and_then(|name| name.to_str()) == Some(entry_name) {
            entry
                .unpack(dest_path)
                .map_err(|err| format!("could not extract {entry_name}: {err}"))?;
            set_executable(dest_path)?;
            return Ok(());
        }
    }
    Err(format!(
        "{entry_name} not found in {}",
        archive_path.display()
    ))
}

/// Extract a single named entry from a `.zip` archive to `dest_path`,
/// setting the executable bit on Unix.
pub fn extract_from_zip(
    archive_path: &Path,
    entry_name: &str,
    dest_path: &Path,
) -> Result<(), String> {
    let file = File::open(archive_path)
        .map_err(|err| format!("could not open {}: {err}", archive_path.display()))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|err| format!("could not read zip {}: {err}", archive_path.display()))?;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|err| format!("could not read a zip entry: {err}"))?;
        let matches = Path::new(entry.name())
            .file_name()
            .and_then(|name| name.to_str())
            == Some(entry_name);
        if matches {
            let mut out = File::create(dest_path)
                .map_err(|err| format!("could not create {}: {err}", dest_path.display()))?;
            std::io::copy(&mut entry, &mut out)
                .map_err(|err| format!("could not extract {entry_name}: {err}"))?;
            set_executable(dest_path)?;
            return Ok(());
        }
    }
    Err(format!(
        "{entry_name} not found in {}",
        archive_path.display()
    ))
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .map_err(|err| format!("could not stat {}: {err}", path.display()))?
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)
        .map_err(|err| format!("could not chmod {}: {err}", path.display()))
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) -> Result<(), String> {
    Ok(())
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

    // ── OCI image evidence (finding #4: real evidence, not self-compare) ─

    fn matching_evidence() -> OciImageEvidence {
        let mut labels = BTreeMap::new();
        labels.insert(
            "org.opencontainers.image.version".to_string(),
            "2.4.0".to_string(),
        );
        labels.insert(
            "org.opencontainers.image.revision".to_string(),
            "abc123".to_string(),
        );
        OciImageEvidence {
            labels,
            digest: Some(
                "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
            ),
        }
    }

    #[test]
    fn oci_image_evidence_passes_when_real_labels_and_digest_match() {
        let findings = check_oci_image_evidence(&valid_oci_record(), &matching_evidence());
        assert!(findings.is_empty(), "findings: {findings:?}");
    }

    #[test]
    fn negative_oci_image_evidence_wrong_real_version_label_is_rejected() {
        let mut evidence = matching_evidence();
        evidence.labels.insert(
            "org.opencontainers.image.version".to_string(),
            "9.9.9".to_string(),
        );
        let findings = check_oci_image_evidence(&valid_oci_record(), &evidence);
        assert!(findings.iter().any(|f| f.contains("record version")));
    }

    #[test]
    fn negative_oci_image_evidence_wrong_real_revision_label_is_rejected() {
        let mut evidence = matching_evidence();
        evidence.labels.insert(
            "org.opencontainers.image.revision".to_string(),
            "deadbeef".to_string(),
        );
        let findings = check_oci_image_evidence(&valid_oci_record(), &evidence);
        assert!(findings.iter().any(|f| f.contains("source_commit")));
    }

    #[test]
    fn negative_oci_image_evidence_wrong_real_digest_is_rejected() {
        let mut evidence = matching_evidence();
        evidence.digest = Some(
            "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string(),
        );
        let findings = check_oci_image_evidence(&valid_oci_record(), &evidence);
        assert!(findings.iter().any(|f| f.contains("manifest_digest")));
    }

    #[test]
    fn negative_oci_image_evidence_missing_real_labels_is_rejected() {
        // A record claiming labels the actual image does not carry: version
        // and source_commit disagree, labels.version and labels.revision
        // disagree (checked independently of the top-level fields), and no
        // digest evidence is present at all -- five findings.
        let findings = check_oci_image_evidence(&valid_oci_record(), &OciImageEvidence::default());
        assert_eq!(findings.len(), 5, "findings: {findings:?}");
    }

    #[test]
    fn negative_oci_image_evidence_missing_digest_is_rejected() {
        // A record with correct labels but no independently-obtained
        // digest evidence must not pass on label agreement alone.
        let mut evidence = matching_evidence();
        evidence.digest = None;
        let findings = check_oci_image_evidence(&valid_oci_record(), &evidence);
        assert!(
            findings.iter().any(|f| f.contains("digest evidence")),
            "findings: {findings:?}"
        );
    }

    #[test]
    fn negative_oci_image_evidence_labels_disagree_with_top_level_fields() {
        // A record whose own labels.version disagrees with its own
        // top-level version (internally inconsistent) must be caught even
        // when the top-level fields happen to match real evidence.
        let mut record = valid_oci_record();
        record["labels"]["version"] = serde_json::json!("9.9.9");
        let findings = check_oci_image_evidence(&record, &matching_evidence());
        assert!(
            findings.iter().any(|f| f.contains("labels.version")),
            "findings: {findings:?}"
        );
    }

    #[test]
    fn oci_pull_ref_strips_tag_and_pins_digest() {
        assert_eq!(
            oci_pull_ref(
                "registry.example.com/agov:2.4.0",
                "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            ),
            "registry.example.com/agov@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn oci_pull_ref_without_tag_still_pins_digest() {
        assert_eq!(
            oci_pull_ref("registry.example.com/agov", "sha256:abc"),
            "registry.example.com/agov@sha256:abc"
        );
    }

    #[test]
    fn self_reported_labels_alone_can_never_fail_which_is_why_evidence_is_required() {
        // Documents the exact tautology finding #4 flagged: comparing a
        // record's own `labels` object to its own `version`/`source_commit`
        // (both written by the same generator call) can never disagree.
        // check_oci_image_evidence requires independently-obtained evidence
        // instead, which real tampering or a real build defect CAN fail.
        let record = valid_oci_record();
        assert_eq!(record["labels"]["version"], record["version"]);
        assert_eq!(record["labels"]["revision"], record["source_commit"]);
    }

    #[test]
    fn parse_podman_inspect_extracts_labels_and_digest() {
        let inspect_json = r#"[{
            "Digest": "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "Config": {
                "Labels": {
                    "org.opencontainers.image.version": "2.4.0",
                    "org.opencontainers.image.revision": "abc123"
                }
            }
        }]"#;
        let evidence = parse_podman_inspect(inspect_json).unwrap();
        assert_eq!(
            evidence.labels.get("org.opencontainers.image.version"),
            Some(&"2.4.0".to_string())
        );
        assert_eq!(
            evidence.digest.as_deref(),
            Some("sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
    }

    #[test]
    fn parse_podman_inspect_rejects_malformed_json() {
        assert!(parse_podman_inspect("not json").is_err());
    }

    #[test]
    fn parse_podman_inspect_rejects_non_array_json() {
        assert!(parse_podman_inspect(r#"{"Config": {}}"#).is_err());
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

    // ── Executable self-report agreement (finding #3) ───────────────────

    fn write_fake_agov_script(path: &Path, version: &str, commit: &str, dirty: bool) {
        std::fs::write(
            path,
            format!(
                "#!/bin/sh\necho '{{\"version\":\"{version}\",\"commit\":\"{commit}\",\"dirty\":{dirty}}}'\n"
            ),
        )
        .unwrap();
        set_executable(path).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn self_report_round_trip_through_tar_gz_extraction() {
        let dir = TempDir::new().unwrap();
        let script_path = dir.path().join("fake-agov");
        write_fake_agov_script(&script_path, "2.4.0", "abc123", false);

        let archive_path = dir.path().join("fixture.tar.gz");
        {
            let file = File::create(&archive_path).unwrap();
            let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut builder = tar::Builder::new(encoder);
            builder
                .append_path_with_name(&script_path, "fake-agov")
                .unwrap();
            builder.finish().unwrap();
        }

        let extracted_path = dir.path().join("extracted-agov");
        extract_from_tar_gz(&archive_path, "fake-agov", &extracted_path).unwrap();

        let self_report = run_self_report(&extracted_path).unwrap();
        assert_eq!(self_report.version, "2.4.0");
        assert_eq!(self_report.commit, "abc123");
        assert!(!self_report.dirty);
    }

    #[cfg(unix)]
    #[test]
    fn self_report_round_trip_through_zip_extraction() {
        let dir = TempDir::new().unwrap();
        let script_path = dir.path().join("fake-agov");
        write_fake_agov_script(&script_path, "2.4.0", "abc123", false);

        let archive_path = dir.path().join("fixture.zip");
        {
            let file = File::create(&archive_path).unwrap();
            let mut zip = zip::ZipWriter::new(file);
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);
            zip.start_file("fake-agov", options).unwrap();
            let script_bytes = std::fs::read(&script_path).unwrap();
            std::io::Write::write_all(&mut zip, &script_bytes).unwrap();
            zip.finish().unwrap();
        }

        let extracted_path = dir.path().join("extracted-agov");
        extract_from_zip(&archive_path, "fake-agov", &extracted_path).unwrap();

        let self_report = run_self_report(&extracted_path).unwrap();
        assert_eq!(self_report.version, "2.4.0");
    }

    #[test]
    fn extract_from_tar_gz_reports_missing_entry() {
        let dir = TempDir::new().unwrap();
        let archive_path = dir.path().join("empty.tar.gz");
        {
            let file = File::create(&archive_path).unwrap();
            let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut builder = tar::Builder::new(encoder);
            builder.finish().unwrap();
        }
        let dest = dir.path().join("out");
        assert!(extract_from_tar_gz(&archive_path, "nonexistent", &dest).is_err());
    }

    #[test]
    fn positive_self_report_matches_record() {
        let mut record = valid_file_record();
        record["version"] = serde_json::json!("2.4.0");
        record["source_commit"] = serde_json::json!("abc123def456");
        let self_report = SelfReport {
            version: "2.4.0".to_string(),
            commit: "abc123def456".to_string(),
            dirty: false,
        };
        assert!(check_self_report_agreement(&record, &self_report).is_empty());
    }

    #[test]
    fn positive_self_report_short_commit_prefix_of_full_record_commit_matches() {
        let mut record = valid_file_record();
        record["version"] = serde_json::json!("2.4.0");
        record["source_commit"] = serde_json::json!("abc123def456abc123def456abc123def456abcd");
        let self_report = SelfReport {
            version: "2.4.0".to_string(),
            // 12-char local dev short hash, prefix of the full record commit above.
            commit: "abc123def456".to_string(),
            dirty: false,
        };
        assert!(check_self_report_agreement(&record, &self_report).is_empty());
    }

    #[test]
    fn negative_self_report_version_mismatch_is_rejected() {
        let mut record = valid_file_record();
        record["version"] = serde_json::json!("2.4.0");
        record["source_commit"] = serde_json::json!("abc123");
        let self_report = SelfReport {
            version: "9.9.9".to_string(),
            commit: "abc123".to_string(),
            dirty: false,
        };
        let findings = check_self_report_agreement(&record, &self_report);
        assert!(findings.iter().any(|f| f.contains("record version")));
    }

    #[test]
    fn negative_self_report_commit_mismatch_is_rejected() {
        let mut record = valid_file_record();
        record["version"] = serde_json::json!("2.4.0");
        record["source_commit"] = serde_json::json!("abc123");
        let self_report = SelfReport {
            version: "2.4.0".to_string(),
            commit: "deadbeef".to_string(),
            dirty: false,
        };
        let findings = check_self_report_agreement(&record, &self_report);
        assert!(findings.iter().any(|f| f.contains("source_commit")));
    }

    #[test]
    fn negative_self_report_dirty_build_is_rejected() {
        let mut record = valid_file_record();
        record["version"] = serde_json::json!("2.4.0");
        record["source_commit"] = serde_json::json!("abc123");
        let self_report = SelfReport {
            version: "2.4.0".to_string(),
            commit: "abc123".to_string(),
            dirty: true,
        };
        let findings = check_self_report_agreement(&record, &self_report);
        assert!(findings.iter().any(|f| f.contains("dirty build")));
    }

    #[test]
    fn negative_self_report_commit_prefix_shorter_than_minimum_is_rejected() {
        // "abc12" is a true prefix of the record commit and both are hex,
        // but at 5 characters it is below MIN_COMMIT_PREFIX_LEN (7) and
        // must not be trusted as agreement -- an arbitrary short prefix is
        // exactly what hmpr's acceptance matrix (row C) forbids.
        let mut record = valid_file_record();
        record["version"] = serde_json::json!("2.4.0");
        record["source_commit"] = serde_json::json!("abc123def456abc123def456");
        let self_report = SelfReport {
            version: "2.4.0".to_string(),
            commit: "abc12".to_string(),
            dirty: false,
        };
        let findings = check_self_report_agreement(&record, &self_report);
        assert!(
            findings.iter().any(|f| f.contains("source_commit")),
            "findings: {findings:?}"
        );
    }

    #[test]
    fn negative_self_report_commit_non_hex_prefix_is_rejected() {
        // A prefix relationship on non-hex strings is not a commit-identity
        // guarantee (git SHAs are always hex); reject even if long enough.
        let mut record = valid_file_record();
        record["version"] = serde_json::json!("2.4.0");
        record["source_commit"] = serde_json::json!("not-a-real-sha-but-long-enough");
        let self_report = SelfReport {
            version: "2.4.0".to_string(),
            commit: "not-a-r".to_string(),
            dirty: false,
        };
        let findings = check_self_report_agreement(&record, &self_report);
        assert!(
            findings.iter().any(|f| f.contains("source_commit")),
            "findings: {findings:?}"
        );
    }

    // ── External release-asset boundary (post-publication discovery) ────

    #[test]
    fn parse_https_host_extracts_plain_host() {
        assert_eq!(
            parse_https_host("https://gitlab.com/api/v4/projects/1/releases/v1").unwrap(),
            "gitlab.com"
        );
    }

    #[test]
    fn parse_https_host_extracts_host_with_port() {
        assert_eq!(
            parse_https_host("https://gitlab.com:8443/path").unwrap(),
            "gitlab.com"
        );
    }

    #[test]
    fn negative_parse_https_host_rejects_non_https() {
        assert!(parse_https_host("http://gitlab.com/path").is_err());
    }

    #[test]
    fn negative_parse_https_host_userinfo_trick_reports_real_host() {
        // A URL of the form https://trusted@evil.example/path must report
        // "evil.example" as the host, not "trusted" -- the part before the
        // last '@' in the authority is userinfo, not the host.
        assert_eq!(
            parse_https_host("https://gitlab.com@evil.example/path").unwrap(),
            "evil.example"
        );
    }

    #[test]
    fn validate_discovered_assets_accepts_safe_matching_batch() {
        let assets = vec![
            (
                "agf-2.4.0.tar.gz".to_string(),
                "https://gitlab.com/asset1".to_string(),
            ),
            (
                "agf-2.4.0.tar.gz-provenance.json".to_string(),
                "https://gitlab.com/asset2".to_string(),
            ),
        ];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_ok());
    }

    #[test]
    fn negative_validate_discovered_assets_rejects_absolute_name() {
        let assets = vec![(
            "/etc/passwd".to_string(),
            "https://gitlab.com/asset".to_string(),
        )];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_err());
    }

    #[test]
    fn negative_validate_discovered_assets_rejects_parent_traversal_name() {
        let assets = vec![(
            "../../etc/passwd".to_string(),
            "https://gitlab.com/asset".to_string(),
        )];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_err());
    }

    #[test]
    fn negative_validate_discovered_assets_rejects_separator_name() {
        let assets = vec![(
            "sub/dir/file.tar.gz".to_string(),
            "https://gitlab.com/asset".to_string(),
        )];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_err());
    }

    #[test]
    fn negative_validate_discovered_assets_rejects_duplicate_names() {
        let assets = vec![
            (
                "agf-2.4.0.tar.gz".to_string(),
                "https://gitlab.com/a".to_string(),
            ),
            (
                "agf-2.4.0.tar.gz".to_string(),
                "https://gitlab.com/b".to_string(),
            ),
        ];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_err());
    }

    #[test]
    fn negative_validate_discovered_assets_rejects_non_https_url() {
        let assets = vec![(
            "agf-2.4.0.tar.gz".to_string(),
            "http://gitlab.com/asset".to_string(),
        )];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_err());
    }

    #[test]
    fn negative_validate_discovered_assets_rejects_wrong_host_url() {
        let assets = vec![(
            "agf-2.4.0.tar.gz".to_string(),
            "https://attacker.example/asset".to_string(),
        )];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_err());
    }

    #[test]
    fn negative_validate_discovered_assets_rejects_userinfo_trick_url() {
        let assets = vec![(
            "agf-2.4.0.tar.gz".to_string(),
            "https://gitlab.com@attacker.example/asset".to_string(),
        )];
        assert!(validate_discovered_assets(&assets, "gitlab.com").is_err());
    }

    #[test]
    fn parse_self_report_rejects_missing_fields() {
        assert!(parse_self_report(r#"{"version":"1.0.0"}"#).is_err());
    }

    #[test]
    fn parse_self_report_rejects_malformed_json() {
        assert!(parse_self_report("not json").is_err());
    }

    // ── Cardinality bijection (finding #6) ───────────────────────────────

    #[test]
    fn positive_legacy_only_batch_skips_coverage_bijection() {
        // The real published v2.3.1 release: framework and tooling archives
        // described only by legacy records, which have no queryable
        // artifact_filename field. Must NOT false-flag every artifact as
        // "no provenance record" -- bijection is a v2.4 contract guarantee,
        // not retroactive.
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("agf-2.3.1.tar.gz"), b"x").unwrap();
        std::fs::write(dir.path().join("agf-2.3.1.zip"), b"x").unwrap();
        std::fs::write(
            dir.path()
                .join("agov-2.3.1-x86_64-unknown-linux-musl.tar.gz"),
            b"x",
        )
        .unwrap();

        let findings = check_primary_artifact_coverage(&[real_v231_framework_record()], dir.path());
        assert!(findings.is_empty(), "findings: {findings:?}");
    }

    #[test]
    fn positive_coverage_passes_when_every_artifact_has_exactly_one_record() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz"), b"x").unwrap();
        std::fs::write(
            dir.path().join("agf-2.4.0.tar.gz.sha256"),
            b"x  agf-2.4.0.tar.gz\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz-provenance.json"), b"{}").unwrap();

        let mut record = valid_file_record();
        record["artifact_filename"] = serde_json::json!("agf-2.4.0.tar.gz");
        let findings = check_primary_artifact_coverage(&[record], dir.path());
        assert!(findings.is_empty(), "findings: {findings:?}");
    }

    #[test]
    fn list_primary_artifacts_excludes_sidecars_and_provenance_files() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz"), b"x").unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz.sha256"), b"x").unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz-provenance.json"), b"x").unwrap();
        let artifacts = list_primary_artifacts(dir.path()).unwrap();
        assert_eq!(artifacts, vec!["agf-2.4.0.tar.gz".to_string()]);
    }

    #[test]
    fn negative_primary_artifact_with_no_record_is_rejected() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz"), b"x").unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.zip"), b"x").unwrap();

        let mut record = valid_file_record();
        record["artifact_filename"] = serde_json::json!("agf-2.4.0.tar.gz");
        // agf-2.4.0.zip is on disk but has no record among the validated set.
        let findings = check_primary_artifact_coverage(&[record], dir.path());
        assert!(findings.iter().any(|f| f.contains("agf-2.4.0.zip")));
        assert!(findings
            .iter()
            .any(|f| f.contains("has no provenance record")));
    }

    #[test]
    fn negative_stale_or_extra_record_with_no_matching_artifact_is_rejected() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("agf-2.4.0.tar.gz"), b"x").unwrap();

        let mut real_record = valid_file_record();
        real_record["artifact_filename"] = serde_json::json!("agf-2.4.0.tar.gz");
        let mut stale_record = valid_file_record();
        stale_record["artifact_filename"] = serde_json::json!("agf-2.3.9-removed.tar.gz");

        let findings = check_primary_artifact_coverage(&[real_record, stale_record], dir.path());
        assert!(findings
            .iter()
            .any(|f| f.contains("agf-2.3.9-removed.tar.gz") && f.contains("stale, extra")));
    }

    #[test]
    fn negative_duplicate_oci_image_reference_is_rejected() {
        let mut second = valid_oci_record();
        second["manifest_digest"] = serde_json::json!(
            "sha256:1111111111111111111111111111111111111111111111111111111111111111"
        );
        let findings = check_no_duplicate_oci_identity(&[valid_oci_record(), second]);
        assert!(findings.iter().any(|f| f.contains("image_reference")));
    }

    #[test]
    fn negative_duplicate_oci_manifest_digest_is_rejected() {
        let mut second = valid_oci_record();
        second["image_reference"] = serde_json::json!("registry.example.com/agov:other-tag");
        let findings = check_no_duplicate_oci_identity(&[valid_oci_record(), second]);
        assert!(findings.iter().any(|f| f.contains("manifest_digest")));
    }

    #[test]
    fn positive_distinct_oci_records_produce_no_duplicate_finding() {
        let mut second = valid_oci_record();
        second["image_reference"] = serde_json::json!("registry.example.com/agov:other-tag");
        second["manifest_digest"] = serde_json::json!(
            "sha256:1111111111111111111111111111111111111111111111111111111111111111"
        );
        assert!(check_no_duplicate_oci_identity(&[valid_oci_record(), second]).is_empty());
    }
}
