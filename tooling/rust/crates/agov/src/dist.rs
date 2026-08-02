use crate::provenance::{self, FileRecordInput, PipelineIdentity};
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Builder;
use time::format_description::well_known::Rfc3339;
use zip::write::SimpleFileOptions;

const DEFAULT_MANIFEST: &str = "agf/dist/release-manifest.yaml";
const DEFAULT_VERSION_FILE: &str = "agf/VERSION";

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u8,
    package: Package,
    #[serde(default)]
    include: Vec<IncludeEntry>,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    forbidden: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version_file: String,
    output_dir: String,
    basename: String,
    formats: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum IncludeEntry {
    PathString(String),
    Map {
        path: Option<String>,
        glob: Option<String>,
        #[serde(default)]
        optional: bool,
    },
}

#[derive(Debug)]
pub struct DistSpec {
    manifest_path: PathBuf,
    root: PathBuf,
    package_name: String,
    version: String,
    files: Vec<String>,
    outputs: BTreeMap<String, PathBuf>,
}

impl DistSpec {
    pub fn contains(&self, path: &str) -> bool {
        self.files
            .binary_search_by(|item| item.as_str().cmp(path))
            .is_ok()
    }
}

#[derive(Debug)]
pub struct BuildResult {
    spec: DistSpec,
    checksums: BTreeMap<String, String>,
    /// One provenance record path per output format — the unified
    /// release-provenance contract requires exactly one record per
    /// published artifact, not one aggregate record covering every format.
    provenance_paths: BTreeMap<String, PathBuf>,
}

pub fn validate_default() -> Result<DistSpec, Vec<String>> {
    validate(&PathBuf::from(DEFAULT_MANIFEST), &agov_core::repo_root())
}

pub fn build_default() -> Result<BuildResult, Vec<String>> {
    build(&PathBuf::from(DEFAULT_MANIFEST), &agov_core::repo_root())
}

pub fn check_tag_from_env() -> Result<TagCheck, String> {
    let root = agov_core::repo_root();
    let version_path = root.join(DEFAULT_VERSION_FILE);
    let version = fs::read_to_string(&version_path).map_err(|err| {
        format!(
            "could not read VERSION file {}: {err}",
            version_path.display()
        )
    })?;
    check_tag(std::env::var("CI_COMMIT_TAG").ok().as_deref(), &version)
}

pub fn check_tag(tag: Option<&str>, version: &str) -> Result<TagCheck, String> {
    let Some(tag) = tag.filter(|tag| !tag.is_empty()) else {
        return Ok(TagCheck::NoTag);
    };
    let normalized_tag = tag.trim_start_matches('v');
    let normalized_version = version.trim();

    if normalized_tag == normalized_version {
        Ok(TagCheck::Matched)
    } else {
        Err(format!(
            "CI_COMMIT_TAG {tag} does not match agf/VERSION {normalized_version}: tag would produce a release labelled {tag} from a tree versioned {normalized_version}"
        ))
    }
}

#[derive(Debug)]
pub enum TagCheck {
    Matched,
    NoTag,
}

pub fn validate(manifest_path: &Path, root: &Path) -> Result<DistSpec, Vec<String>> {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let manifest_path = if manifest_path.is_absolute() {
        manifest_path.to_path_buf()
    } else {
        root.join(manifest_path)
    };
    let content = fs::read_to_string(&manifest_path).map_err(|err| {
        vec![format!(
            "could not read manifest {}: {err}",
            manifest_path.display()
        )]
    })?;
    let manifest: Manifest = serde_norway::from_str(&content)
        .map_err(|err| vec![format!("could not parse manifest YAML: {err}")])?;

    if manifest.schema_version != 1 {
        return Err(vec!["schema_version must be 1".to_string()]);
    }

    let mut errors = validate_package(&manifest.package);
    if !errors.is_empty() {
        return Err(errors);
    }

    let version_path = safe_join(&root, &manifest.package.version_file).ok_or_else(|| {
        vec!["package.version_file must be a relative path inside the repository".to_string()]
    })?;
    let version = fs::read_to_string(&version_path)
        .map_err(|err| vec![format!("could not read package.version_file: {err}")])?
        .trim()
        .to_string();

    let mut files = expand_includes(&root, &manifest.include)?;
    files.retain(|path| !matches_any(&root, &manifest.exclude, path));
    files.sort();
    files.dedup();

    errors.extend(forbidden_errors(&root, &manifest.forbidden, &files));
    errors.extend(duplicate_archive_errors(&files));
    if !errors.is_empty() {
        return Err(errors);
    }

    let outputs = output_paths(&root, &manifest.package, &version);
    Ok(DistSpec {
        manifest_path,
        root,
        package_name: manifest.package.name,
        version,
        files,
        outputs,
    })
}

pub fn build(manifest_path: &Path, root: &Path) -> Result<BuildResult, Vec<String>> {
    let spec = validate(manifest_path, root)?;
    let output_dir = spec
        .outputs
        .values()
        .next()
        .and_then(|path| path.parent())
        .ok_or_else(|| vec!["package.formats must contain at least one output".to_string()])?;
    fs::create_dir_all(output_dir).map_err(|err| {
        vec![format!(
            "could not create output directory {}: {err}",
            output_dir.display()
        )]
    })?;

    for (format, output) in &spec.outputs {
        match format.as_str() {
            "tar.gz" => write_tar(output, &spec.root, &spec.files)?,
            "zip" => write_zip(output, &spec.root, &spec.files)?,
            _ => return Err(vec![format!("unsupported output format: {format}")]),
        }
    }

    let checksums = write_checksums(&spec)?;
    let provenance_paths = write_provenance(&spec, &checksums)?;
    Ok(BuildResult {
        spec,
        checksums,
        provenance_paths,
    })
}

pub fn format_validate_summary(spec: &DistSpec) -> String {
    let outputs = spec
        .outputs
        .values()
        .map(|path| format!("  - {}", relative_to(&spec.root, path)))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "OK  release manifest valid\nManifest: {}\nPackage:  {} {}\nFiles:    {}\nOutputs:\n{}",
        relative_to(&spec.root, &spec.manifest_path),
        spec.package_name,
        spec.version,
        spec.files.len(),
        outputs
    )
}

pub fn format_build_summary(result: &BuildResult) -> String {
    let outputs = result
        .spec
        .outputs
        .iter()
        .map(|(format, path)| {
            let checksum = result.checksums.get(format).cloned().unwrap_or_default();
            format!(
                "  - {} (sha256: {})",
                relative_to(&result.spec.root, path),
                checksum
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let provenance = result
        .provenance_paths
        .values()
        .map(|path| format!("  - {}", relative_to(&result.spec.root, path)))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "OK  release manifest valid\nManifest: {}\nPackage:  {} {}\nFiles:    {}\nOutputs:\n{}\nProvenance:\n{}",
        relative_to(&result.spec.root, &result.spec.manifest_path),
        result.spec.package_name,
        result.spec.version,
        result.spec.files.len(),
        outputs,
        provenance
    )
}

fn validate_package(package: &Package) -> Vec<String> {
    let mut errors = Vec::new();
    if package.name.is_empty() {
        errors.push("package.name is required".to_string());
    }
    if package.version_file.is_empty() {
        errors.push("package.version_file is required".to_string());
    }
    if package.output_dir.is_empty() {
        errors.push("package.output_dir is required".to_string());
    }
    if package.basename.is_empty() {
        errors.push("package.basename is required".to_string());
    }
    if package.formats.is_empty() {
        errors.push("package.formats must be a non-empty list".to_string());
    }
    for format in &package.formats {
        if format != "tar.gz" && format != "zip" {
            errors.push("package.formats may only contain tar.gz and zip".to_string());
        }
    }
    errors
}

fn expand_includes(root: &Path, includes: &[IncludeEntry]) -> Result<Vec<String>, Vec<String>> {
    let mut files = Vec::new();
    let mut errors = Vec::new();

    for entry in includes {
        match expand_include(root, entry) {
            Ok(mut more) => files.append(&mut more),
            Err(error) => errors.push(error),
        }
    }

    if errors.is_empty() {
        Ok(files)
    } else {
        Err(errors)
    }
}

fn expand_include(root: &Path, entry: &IncludeEntry) -> Result<Vec<String>, String> {
    match entry {
        IncludeEntry::PathString(path) => expand_path(root, path, false),
        IncludeEntry::Map {
            path: Some(path),
            optional,
            ..
        } => expand_path(root, path, *optional),
        IncludeEntry::Map {
            glob: Some(pattern),
            optional,
            ..
        } => expand_glob(root, pattern, *optional),
        IncludeEntry::Map { .. } => Err("invalid include entry".to_string()),
    }
}

fn expand_path(root: &Path, path: &str, optional: bool) -> Result<Vec<String>, String> {
    let Some(full_path) = safe_join(root, path) else {
        return Err(format!("include path escapes repository: {path}"));
    };

    if full_path.is_file() {
        Ok(vec![path.to_string()])
    } else if optional {
        Ok(vec![])
    } else {
        Err(format!("include path not found or not a file: {path}"))
    }
}

fn expand_glob(root: &Path, pattern: &str, optional: bool) -> Result<Vec<String>, String> {
    if let Some((prefix, _)) = pattern.split_once("/**") {
        return expand_recursive_prefix(root, prefix, pattern, optional);
    }

    let Some(full_pattern) = safe_join(root, pattern) else {
        return Err(format!("include glob escapes repository: {pattern}"));
    };
    let full_pattern = full_pattern.to_string_lossy().into_owned();
    let mut files = glob::glob(&full_pattern)
        .map_err(|err| format!("invalid include glob {pattern}: {err}"))?
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .filter_map(|path| relative_to_root(root, &path))
        .collect::<Vec<_>>();
    files.sort();

    if files.is_empty() && !optional {
        Err(format!("include glob matched no files: {pattern}"))
    } else {
        Ok(files)
    }
}

fn expand_recursive_prefix(
    root: &Path,
    prefix: &str,
    pattern: &str,
    optional: bool,
) -> Result<Vec<String>, String> {
    let Some(full_prefix) = safe_join(root, prefix) else {
        return Err(format!("include glob escapes repository: {pattern}"));
    };

    let mut files = Vec::new();
    if full_prefix.exists() {
        collect_regular_files(root, &full_prefix, &mut files)
            .map_err(|err| format!("could not expand include glob {pattern}: {err}"))?;
    }
    files.sort();

    if files.is_empty() && !optional {
        Err(format!("include glob matched no files: {pattern}"))
    } else {
        Ok(files)
    }
}

fn collect_regular_files(
    root: &Path,
    dir: &Path,
    files: &mut Vec<String>,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_regular_files(root, &path, files)?;
        } else if path.is_file() {
            if let Some(relative) = relative_to_root(root, &path) {
                files.push(relative);
            }
        }
    }
    Ok(())
}

fn matches_any(root: &Path, patterns: &[String], path: &str) -> bool {
    patterns.iter().any(|pattern| {
        pattern_prefix_match(pattern, path) || glob_matches(root, pattern, path).unwrap_or(false)
    })
}

fn pattern_prefix_match(pattern: &str, path: &str) -> bool {
    if let Some((prefix, _)) = pattern.split_once("/**") {
        path == prefix || path.starts_with(&format!("{prefix}/"))
    } else {
        false
    }
}

fn glob_matches(root: &Path, pattern: &str, path: &str) -> Result<bool, String> {
    let Some(full_pattern) = safe_join(root, pattern) else {
        return Ok(false);
    };
    let full_pattern = full_pattern.to_string_lossy().into_owned();
    for candidate in glob::glob(&full_pattern).map_err(|err| err.to_string())? {
        let Ok(candidate) = candidate else {
            continue;
        };
        if relative_to_root(root, &candidate).as_deref() == Some(path) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn forbidden_errors(root: &Path, forbidden: &[String], files: &[String]) -> Vec<String> {
    files
        .iter()
        .filter(|path| matches_any(root, forbidden, path))
        .map(|path| format!("forbidden file selected for release: {path}"))
        .collect()
}

fn duplicate_archive_errors(files: &[String]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    for file in files {
        if !seen.insert(file) {
            duplicates.insert(file.clone());
        }
    }
    duplicates
        .into_iter()
        .map(|path| format!("duplicate archive path: {path}"))
        .collect()
}

fn output_paths(root: &Path, package: &Package, version: &str) -> BTreeMap<String, PathBuf> {
    let output_dir = root.join(&package.output_dir);
    let basename = package.basename.replace("{{version}}", version);
    package
        .formats
        .iter()
        .map(|format| {
            let filename = match format.as_str() {
                "tar.gz" => format!("{basename}.tar.gz"),
                "zip" => format!("{basename}.zip"),
                _ => format!("{basename}.{format}"),
            };
            (format.clone(), output_dir.join(filename))
        })
        .collect()
}

fn write_tar(output: &Path, root: &Path, files: &[String]) -> Result<(), Vec<String>> {
    let file = File::create(output).map_err(|err| {
        vec![format!(
            "could not write tar.gz archive {}: {err}",
            output.display()
        )]
    })?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(encoder);

    for archive_path in files {
        let full_path = root.join(archive_path);
        tar.append_path_with_name(&full_path, archive_path)
            .map_err(|err| {
                vec![format!(
                    "could not add {archive_path} to tar.gz archive: {err}"
                )]
            })?;
    }

    tar.finish()
        .map_err(|err| vec![format!("could not finish tar.gz archive: {err}")])
}

fn write_zip(output: &Path, root: &Path, files: &[String]) -> Result<(), Vec<String>> {
    let file = File::create(output).map_err(|err| {
        vec![format!(
            "could not write zip archive {}: {err}",
            output.display()
        )]
    })?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for archive_path in files {
        let full_path = root.join(archive_path);
        let mut input = File::open(&full_path).map_err(|err| {
            vec![format!(
                "could not read {archive_path} for zip archive: {err}"
            )]
        })?;
        zip.start_file(archive_path, options).map_err(|err| {
            vec![format!(
                "could not add {archive_path} to zip archive: {err}"
            )]
        })?;
        std::io::copy(&mut input, &mut zip).map_err(|err| {
            vec![format!(
                "could not write {archive_path} to zip archive: {err}"
            )]
        })?;
    }

    zip.finish()
        .map(|_| ())
        .map_err(|err| vec![format!("could not finish zip archive: {err}")])
}

fn write_checksums(spec: &DistSpec) -> Result<BTreeMap<String, String>, Vec<String>> {
    let mut checksums = BTreeMap::new();
    for (format, archive_path) in &spec.outputs {
        let hex = sha256_file(archive_path).map_err(|err| {
            vec![format!(
                "could not checksum {}: {err}",
                archive_path.display()
            )]
        })?;
        let basename = archive_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("archive");
        let checksum_path = archive_path.with_file_name(format!("{basename}.sha256"));
        fs::write(&checksum_path, format!("{hex}  {basename}\n")).map_err(|err| {
            vec![format!(
                "could not write checksum file {}: {err}",
                checksum_path.display()
            )]
        })?;
        checksums.insert(format.clone(), hex);
    }
    Ok(checksums)
}

fn sha256_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Write one `artifact_kind: file` provenance record per published output
/// (tar.gz, zip, ...), per the unified release-provenance contract's
/// cardinality rule: exactly one record per published artifact, never an
/// aggregate record covering multiple files.
fn write_provenance(
    spec: &DistSpec,
    checksums: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, PathBuf>, Vec<String>> {
    let source_commit = std::env::var("CI_COMMIT_SHA").unwrap_or_else(|_| "unknown".to_string());
    let tag = std::env::var("CI_COMMIT_TAG").unwrap_or_else(|_| "untagged".to_string());
    let build_timestamp = time::OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "unknown".to_string());
    let pipeline_id = std::env::var("CI_PIPELINE_ID").unwrap_or_else(|_| "unknown".to_string());
    let job_id = std::env::var("CI_JOB_ID").unwrap_or_else(|_| "unknown".to_string());
    let manifest_path = relative_to(&spec.root, &spec.manifest_path);

    let mut provenance_paths = BTreeMap::new();
    for (format, output) in &spec.outputs {
        let artifact_filename = output
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| vec![format!("output path has no filename: {}", output.display())])?;
        let artifact_bytes = fs::metadata(output)
            .map_err(|err| vec![format!("could not stat {}: {err}", output.display())])?
            .len();
        let sha256 = checksums
            .get(format)
            .ok_or_else(|| vec![format!("no checksum recorded for format {format}")])?;

        let record = provenance::build_file_record(&FileRecordInput {
            product: &spec.package_name,
            version: &spec.version,
            source_commit: &source_commit,
            tag: &tag,
            build_timestamp: &build_timestamp,
            pipeline_identity: Some(PipelineIdentity {
                pipeline_id: &pipeline_id,
                job_id: &job_id,
            }),
            manifest_path: Some(&manifest_path),
            implementation: "rust",
            packaging: "manifest-source-archive",
            artifact_filename,
            artifact_bytes,
            sha256,
            target_platform: None,
        });

        let outcome = provenance::validate_record(&record);
        if !outcome.is_valid() {
            return Err(outcome
                .findings
                .into_iter()
                .map(|finding| {
                    format!(
                        "generated provenance record for {artifact_filename} is invalid: {finding}"
                    )
                })
                .collect());
        }

        let provenance_path = output.with_file_name(format!("{artifact_filename}-provenance.json"));
        let json = serde_json::to_string_pretty(&record)
            .map_err(|err| vec![format!("could not encode provenance JSON: {err}")])?;
        fs::write(&provenance_path, format!("{json}\n"))
            .map_err(|err| vec![format!("could not write provenance: {err}")])?;
        provenance_paths.insert(format.clone(), provenance_path);
    }

    Ok(provenance_paths)
}

fn safe_join(root: &Path, path: &str) -> Option<PathBuf> {
    let path = Path::new(path);
    if path.is_absolute() || path.components().any(|part| part.as_os_str() == "..") {
        return None;
    }
    let full = root.join(path);
    if full == root || full.starts_with(root) {
        Some(full)
    } else {
        None
    }
}

fn relative_to_root(root: &Path, path: &Path) -> Option<String> {
    path.strip_prefix(root)
        .ok()
        .and_then(|path| path.to_str())
        .map(|path| path.replace('\\', "/"))
}

fn relative_to(root: &Path, path: &Path) -> String {
    relative_to_root(root, path).unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn rejects_mismatched_tag() {
        let error = check_tag(Some("v9.99.99"), "2.0.0").unwrap_err();
        assert!(error.contains("v9.99.99"));
        assert!(error.contains("2.0.0"));
    }

    #[test]
    fn validates_forbidden_selected_files() {
        let temp = fixture_root();
        write(temp.path(), "agf/VERSION", "2.0.0\n");
        write(temp.path(), "tmp/leak.txt", "nope\n");
        let manifest = write(
            temp.path(),
            "manifest.yaml",
            r#"
schema_version: 1
package:
  name: agf
  version_file: agf/VERSION
  output_dir: dist
  basename: agf-{{version}}
  formats: [tar.gz]
include:
  - glob: tmp/**
forbidden:
  - tmp/**
"#,
        );

        let errors = validate(&manifest, temp.path()).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| error == "forbidden file selected for release: tmp/leak.txt"));
    }

    #[test]
    fn rejects_selected_mix_and_elixir_sources() {
        let temp = fixture_root();
        write(temp.path(), "agf/VERSION", "2.0.0\n");
        write(temp.path(), "agf/legacy/mix.lock", "retired\n");
        write(temp.path(), "agf/legacy/lib/sample.ex", "retired\n");
        let manifest = write(
            temp.path(),
            "manifest.yaml",
            r#"
schema_version: 1
package:
  name: agf
  version_file: agf/VERSION
  output_dir: dist
  basename: agf-{{version}}
  formats: [tar.gz]
include:
  - glob: agf/legacy/**
forbidden:
  - "**/mix.lock"
  - "**/*.ex"
"#,
        );

        let errors = validate(&manifest, temp.path()).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| { error == "forbidden file selected for release: agf/legacy/mix.lock" }));
        assert!(errors.iter().any(|error| {
            error == "forbidden file selected for release: agf/legacy/lib/sample.ex"
        }));
    }

    #[test]
    fn build_writes_archives_checksums_and_provenance() {
        let temp = fixture_root();
        write(temp.path(), "agf/VERSION", "2.0.0\n");
        write(temp.path(), "agf/LICENSE", "license\n");
        let manifest = write(
            temp.path(),
            "agf/dist/release-manifest.yaml",
            r#"
schema_version: 1
package:
  name: agf
  version_file: agf/VERSION
  output_dir: dist
  basename: agf-{{version}}
  formats:
    - tar.gz
    - zip
include:
  - path: agf/VERSION
  - path: agf/LICENSE
forbidden:
  - tmp/**
"#,
        );

        let result = build(&manifest, temp.path()).unwrap();
        let tar_path = result.spec.outputs.get("tar.gz").unwrap();
        let zip_path = result.spec.outputs.get("zip").unwrap();

        assert!(tar_path.is_file());
        assert!(zip_path.is_file());
        assert!(tar_path.with_file_name("agf-2.0.0.tar.gz.sha256").is_file());
        assert!(zip_path.with_file_name("agf-2.0.0.zip.sha256").is_file());

        // Cardinality: exactly one provenance record per published
        // artifact, not one aggregate record covering both formats.
        assert_eq!(result.provenance_paths.len(), 2);
        let tar_provenance_path = result.provenance_paths.get("tar.gz").unwrap();
        let zip_provenance_path = result.provenance_paths.get("zip").unwrap();
        assert!(tar_provenance_path.is_file());
        assert!(zip_provenance_path.is_file());
        assert_ne!(tar_provenance_path, zip_provenance_path);

        let tar_provenance: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(tar_provenance_path).unwrap()).unwrap();
        assert_eq!(tar_provenance["schema_version"], "2.4");
        assert_eq!(tar_provenance["artifact_kind"], "file");
        assert_eq!(tar_provenance["version"], "2.0.0");
        assert_eq!(tar_provenance["implementation"], "rust");
        assert_eq!(tar_provenance["artifact_filename"], "agf-2.0.0.tar.gz");
        assert_eq!(
            tar_provenance["sha256"],
            *result.checksums.get("tar.gz").unwrap()
        );

        let outcome = crate::provenance::validate_record(&tar_provenance);
        assert!(outcome.is_valid(), "findings: {:?}", outcome.findings);
    }

    /// Issue #79: a shipped archive must never contain a file at
    /// agf/agents/MANIFEST.yaml -- that is what let a plain archive
    /// re-extraction overwrite an adopter's own divergent roster (the
    /// exact defect issue #78 reproduced). The template ships as
    /// agf/agents/MANIFEST.example.yaml instead, matched by the same
    /// include glob, and is not excluded.
    #[test]
    fn manifest_yaml_is_excluded_from_packaging_manifest_example_is_not() {
        let temp = fixture_root();
        write(temp.path(), "agf/VERSION", "2.0.0\n");
        // An adopter's own divergent roster -- packaging must leave this
        // exact content alone, which this test proves by never selecting
        // the file for the archive at all.
        write(
            temp.path(),
            "agf/agents/MANIFEST.yaml",
            "agents:\n  a-divergent-adopter-roster:\n    status: active\n",
        );
        write(
            temp.path(),
            "agf/agents/MANIFEST.example.yaml",
            "agents:\n  coder-tester:\n    status: skipped\n",
        );
        let manifest = write(
            temp.path(),
            "manifest.yaml",
            r#"
schema_version: 1
package:
  name: agf
  version_file: agf/VERSION
  output_dir: dist
  basename: agf-{{version}}
  formats: [tar.gz]
include:
  - glob: agf/agents/**
exclude:
  - agf/agents/MANIFEST.yaml
"#,
        );

        let spec = validate(&manifest, temp.path()).expect("validate should succeed");

        assert!(
            !spec.contains("agf/agents/MANIFEST.yaml"),
            "the adopter-owned roster must never be selected for packaging"
        );
        assert!(
            spec.contains("agf/agents/MANIFEST.example.yaml"),
            "the neutral template must still ship"
        );
    }

    /// Gate 2 review `rtga` finding B1: `manifest_yaml_is_excluded_
    /// from_packaging_manifest_example_is_not` above proves only that
    /// `DistSpec::validate()` never *selects* `agf/agents/MANIFEST.yaml`
    /// for packaging. The approved plan's Scope and Acceptance Criterion 1
    /// require the stronger, real-world claim: build the actual governed
    /// archive, extract it over an adopter project that already has its
    /// own divergent `agf/agents/MANIFEST.yaml`, and prove that exact file
    /// is byte-for-byte unchanged afterward -- the literal reproduction of
    /// issue #78's defect, run end to end through real `build()` and real
    /// tar extraction, not inferred from selection logic alone.
    #[test]
    fn archive_extraction_over_divergent_adopter_manifest_leaves_it_byte_for_byte_unchanged() {
        // The framework source tree `build()` packages from.
        let source = fixture_root();
        write(source.path(), "agf/VERSION", "2.0.0\n");
        write(
            source.path(),
            "agf/agents/MANIFEST.example.yaml",
            "agents:\n  coder-tester:\n    status: skipped\n",
        );
        let manifest = write(
            source.path(),
            "agf/dist/release-manifest.yaml",
            r#"
schema_version: 1
package:
  name: agf
  version_file: agf/VERSION
  output_dir: dist
  basename: agf-{{version}}
  formats: [tar.gz]
include:
  - glob: agf/agents/**
  - path: agf/VERSION
exclude:
  - agf/agents/MANIFEST.yaml
"#,
        );

        let result = build(&manifest, source.path()).expect("build should succeed");
        let tar_path = result.spec.outputs.get("tar.gz").unwrap().clone();
        assert!(tar_path.is_file(), "archive was not written");

        // A separate "adopter project" directory, simulating an existing
        // installation with its own divergent roster -- not the source
        // tree build() just packaged from.
        let adopter = fixture_root();
        let adopter_manifest_path = write(
            adopter.path(),
            "agf/agents/MANIFEST.yaml",
            "agents:\n  a-divergent-adopter-roster:\n    status: active\n",
        );
        let before = fs::read(&adopter_manifest_path).unwrap();

        // Real tar.gz extraction over the adopter directory, exactly what
        // an adopter running `tar xzf` over their project root does.
        let tar_gz = File::open(&tar_path).expect("open built archive");
        let decoder = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(decoder);
        archive
            .unpack(adopter.path())
            .expect("extract archive over adopter directory");

        let after = fs::read(&adopter_manifest_path)
            .expect("adopter's agf/agents/MANIFEST.yaml must still exist after extraction");
        assert_eq!(
            before, after,
            "adopter's divergent agf/agents/MANIFEST.yaml must be byte-for-byte \
             unchanged by archive extraction -- the shipped archive must contain \
             no file at that path"
        );

        // Positive control: the archive still shipped the neutral
        // template alongside the adopter's untouched roster.
        assert!(
            adopter.path().join("agf/agents/MANIFEST.example.yaml").is_file(),
            "the neutral template must still be extracted"
        );
    }

    fn fixture_root() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    fn write(root: &Path, path: &str, content: &str) -> PathBuf {
        let full = root.join(path);
        fs::create_dir_all(full.parent().unwrap()).unwrap();
        fs::write(&full, content).unwrap();
        full
    }
}
