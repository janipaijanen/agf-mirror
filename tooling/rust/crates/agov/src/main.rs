mod dist;
mod provenance;

use agov_coord::readable_id::{is_readable_id, iso_timestamp, readable_id};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use time::format_description::well_known::Rfc3339;

type ValidationErrors = Vec<(String, String)>;
type FileValidator<T> = fn(&std::path::Path) -> Result<T, ValidationErrors>;
type AdvisoryFn<T> = fn(&T) -> Vec<String>;

#[derive(Parser)]
#[command(name = "agov", about = "AGF governance tooling", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print agov version and build provenance.
    Version {
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// Coordination channel commands.
    Coord {
        #[command(subcommand)]
        sub: CoordCommand,
    },
    /// Gate 1 plan commands.
    Plan {
        #[command(subcommand)]
        sub: PlanCommand,
    },
    /// Run validators.
    Validate {
        #[command(subcommand)]
        sub: Option<ValidateCommand>,
    },
    /// Handoff queue commands.
    Handoff {
        #[command(subcommand)]
        sub: HandoffCommand,
    },
    /// Session orientation commands.
    Session {
        #[command(subcommand)]
        sub: SessionCommand,
    },
    /// Token ROI telemetry commands.
    Token {
        #[command(subcommand)]
        sub: TokenCommand,
    },
    /// Framework source distribution commands.
    Dist {
        #[command(subcommand)]
        sub: DistCommand,
    },
    /// Coordination usage reporting commands.
    Usage {
        #[command(subcommand)]
        sub: UsageCommand,
    },
}

#[derive(Subcommand)]
enum CoordCommand {
    /// Validate one or more coordination message files.
    Validate {
        /// Files to validate.
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Generate a coordination message skeleton with a new coordination id.
    Brief {
        #[arg(long, default_value = "readable", value_parser = ["readable", "ulid"])]
        id_format: String,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
        #[arg(long, default_value = "note")]
        kind: String,
        #[arg(long)]
        topic: Option<String>,
        /// LLM/runtime provenance for the coordination record. Defaults to AGOV_MODEL when set.
        #[arg(long)]
        model: Option<String>,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
    /// Write and validate a coordination message.
    Post {
        #[arg(long, default_value = "readable", value_parser = ["readable", "ulid"])]
        id_format: String,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long, default_value = "note")]
        kind: String,
        #[arg(long)]
        topic: String,
        /// Existing coordination message ids or filenames this message references.
        #[arg(long = "ref")]
        refs: Vec<String>,
        /// Artifact path claimed by a claim message.
        #[arg(long = "artifact")]
        artifacts: Vec<String>,
        /// Task line for a handoff message.
        #[arg(long = "task")]
        tasks: Vec<String>,
        /// LLM/runtime provenance for the coordination record. Defaults to AGOV_MODEL when set.
        #[arg(long)]
        model: Option<String>,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
    /// List valid coordination messages after an id, filename, or RFC3339 timestamp.
    List {
        #[arg(long)]
        since: Option<String>,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// Print coordination channel overview.
    Status {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value_t = 5)]
        recent: usize,
        /// Include per-role activity derived from manifest + coordination messages.
        #[arg(long)]
        roles: bool,
    },
    /// Report stale artifact claims in the coordination channel.
    Stale {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
    /// Scan local branches for hidden coordination obligations.
    Visibility {
        #[arg(long, default_value = "main", alias = "base_branch")]
        base_branch: String,
        #[arg(long)]
        branch: Vec<String>,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// Emit a Graphviz dot communication map of the coordination channel.
    Graph {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value = "dot", value_parser = ["dot", "text"])]
        format: String,
    },
    /// Emit a compact JSON index of the coordination graph for LLM cold-start.
    GraphIndex {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        /// Write output to a file instead of stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum PlanCommand {
    /// Validate one or more Gate 1 plan files.
    Validate {
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
enum ValidateCommand {
    /// Validate the adoption structure.
    Adoption,
    /// Validate adopter-facing documentation coverage for release-relevant changes.
    AdopterDocs,
    /// Validate all agent YAML files.
    Agents,
    /// Validate learning records.
    Learnings {
        /// Optional learning YAML files or directories to validate. Defaults to AGOV_LEARNINGS_DIR or agf/learnings.
        paths: Vec<PathBuf>,
    },
    /// Scan learning records for InfoSec findings.
    Infosec,
    /// Validate contribution-auditor findings.
    Contributions,
    /// Validate framework-config.
    FrameworkConfig,
    /// Check README.md stat claims against actual file counts.
    ReadmeStats,
}

#[derive(Subcommand)]
enum HandoffCommand {
    /// Show the next ready handoff for an agent role.
    Next {
        agent_role: String,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
    /// Acknowledge and take a pending handoff.
    Take {
        id: String,
        #[arg(long)]
        from: String,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
    /// Mark a pending handoff complete.
    Complete {
        id: String,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: Option<String>,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
}

#[derive(Subcommand)]
enum SessionCommand {
    /// Summarize session state for a returning agent.
    Resume {
        #[arg(long)]
        role: Option<String>,
        #[arg(long, default_value = "main", alias = "base_branch")]
        base_branch: String,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value = "agf/agents/MANIFEST.yaml")]
        manifest: PathBuf,
    },
}

#[derive(Subcommand)]
enum TokenCommand {
    /// Token ROI telemetry.
    Roi {
        #[command(subcommand)]
        sub: TokenRoiCommand,
    },
}

#[derive(Subcommand)]
enum TokenRoiCommand {
    /// Report accumulated token ROI telemetry.
    Report {
        #[arg(long)]
        log: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum DistCommand {
    /// Validate the framework source release manifest.
    Validate,
    /// Build framework source archives from the release manifest.
    Build,
    /// Check CI_COMMIT_TAG against agf/VERSION.
    CheckTag,
    /// Validate one or more release-provenance JSON records against the
    /// unified v2.4 schema (embedded at compile time; no source checkout
    /// required), classifying pre-v2.4 records as legacy rather than
    /// scoring them against the v2.4 contract.
    ValidateProvenance {
        /// Provenance JSON record files to validate.
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Directory containing the published artifacts the records
        /// describe: a local dist/ during build-time validation, or a
        /// directory of freshly-downloaded files during post-publication
        /// validation. When given, artifact_bytes/sha256/checksum-sidecar
        /// agreement and dangling-reference checks run in addition to
        /// schema validation.
        #[arg(long)]
        artifact_dir: Option<PathBuf>,
        /// An OCI image reference or ID to inspect (via `podman inspect`)
        /// for real label/digest evidence, cross-checked against any
        /// oci-image records among `files`. Without this, oci-image
        /// records are only schema-validated -- no image evidence check
        /// runs, since there is nothing to compare against.
        #[arg(long)]
        oci_image: Option<String>,
    },
    /// Build a single release-provenance JSON record from real artifact
    /// evidence (file bytes/sha256, or OCI image/label identifiers).
    ProvenanceRecord {
        #[command(subcommand)]
        sub: Box<ProvenanceRecordCommand>,
    },
}

#[derive(Subcommand)]
enum ProvenanceRecordCommand {
    /// Build an `artifact_kind: file` record for an archive already on disk
    /// (framework source archives and Rust tooling archives share this
    /// shape).
    File {
        #[arg(long)]
        product: String,
        #[arg(long)]
        version: String,
        #[arg(long)]
        source_commit: String,
        #[arg(long)]
        tag: String,
        #[arg(long)]
        build_timestamp: String,
        #[arg(long)]
        pipeline_id: Option<String>,
        #[arg(long)]
        job_id: Option<String>,
        #[arg(long)]
        manifest_path: Option<String>,
        #[arg(long)]
        implementation: String,
        #[arg(long)]
        packaging: String,
        /// Path to the already-built artifact file; its filename, byte
        /// count, and sha256 are read directly rather than passed by hand.
        #[arg(long)]
        artifact: PathBuf,
        #[arg(long)]
        target_platform: Option<String>,
        /// Write the record here instead of printing it to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Build an `artifact_kind: oci-image` record.
    Oci {
        #[arg(long)]
        product: String,
        #[arg(long)]
        version: String,
        #[arg(long)]
        source_commit: String,
        #[arg(long)]
        tag: String,
        #[arg(long)]
        build_timestamp: String,
        #[arg(long)]
        pipeline_id: Option<String>,
        #[arg(long)]
        job_id: Option<String>,
        #[arg(long)]
        implementation: String,
        #[arg(long)]
        packaging: String,
        #[arg(long)]
        image_reference: String,
        #[arg(long)]
        manifest_digest: String,
        #[arg(long)]
        label_version: String,
        #[arg(long)]
        label_revision: String,
        /// Write the record here instead of printing it to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum UsageCommand {
    /// Print coordination channel usage report.
    Report {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value = "agf/agents/MANIFEST.yaml")]
        manifest: PathBuf,
        #[arg(long)]
        last: Option<usize>,
    },
}

fn main() {
    let args = normalized_args();
    if matches!(args.get(1).map(String::as_str), Some("--version" | "-V")) {
        println!("{}", version_text());
        std::process::exit(0);
    }
    let cli = Cli::parse_from(args.clone());
    let exit_code = run(cli, args.into_iter().skip(1).collect());
    std::process::exit(exit_code);
}

fn normalized_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();
    if let Some(command) = args.get_mut(1) {
        match command.as_str() {
            "agov.coord.validate" => {
                *command = "coord".to_string();
                args.insert(2, "validate".to_string());
            }
            "agov.plan.validate" => {
                *command = "plan".to_string();
                args.insert(2, "validate".to_string());
            }
            "agov.session.resume" => {
                *command = "session".to_string();
                args.insert(2, "resume".to_string());
            }
            "agov.token.roi" => {
                *command = "token".to_string();
                args.insert(2, "roi".to_string());
            }
            "agov.usage.report" => {
                *command = "usage".to_string();
                args.insert(2, "report".to_string());
            }
            _ => {}
        }
    }
    args
}

fn run(cli: Cli, args: Vec<String>) -> i32 {
    match cli.command {
        Command::Version { format } => version_command(&format, &args),
        Command::Coord {
            sub: CoordCommand::Validate { files },
        } => validate_coord_files(files, &args),
        Command::Coord {
            sub:
                CoordCommand::Brief {
                    id_format,
                    from,
                    to,
                    kind,
                    topic,
                    model,
                    dir,
                },
        } => coord_brief(
            CoordBriefOptions {
                id_format,
                from,
                to,
                kind,
                topic,
                model,
                dir,
            },
            &args,
        ),
        Command::Coord {
            sub:
                CoordCommand::Post {
                    id_format,
                    from,
                    to,
                    kind,
                    topic,
                    refs,
                    artifacts,
                    tasks,
                    model,
                    dir,
                },
        } => coord_post(
            CoordPostOptions {
                id_format,
                from,
                to,
                kind,
                topic,
                refs,
                artifacts,
                tasks,
                model,
                dir,
            },
            &args,
        ),
        Command::Coord {
            sub: CoordCommand::List { since, dir, format },
        } => coord_list(since, dir, format, &args),
        Command::Coord {
            sub: CoordCommand::Status { dir, recent, roles },
        } => coord_status(dir, recent, roles, &args),
        Command::Coord {
            sub: CoordCommand::Stale { dir },
        } => coord_stale(dir, &args),
        Command::Coord {
            sub:
                CoordCommand::Visibility {
                    base_branch,
                    branch,
                    dir,
                    format,
                },
        } => coord_visibility(base_branch, branch, dir, format, &args),
        Command::Coord {
            sub: CoordCommand::Graph { dir, format },
        } => coord_graph(dir, format, &args),
        Command::Coord {
            sub: CoordCommand::GraphIndex { dir, output },
        } => coord_graph_index(dir, output, &args),
        Command::Plan {
            sub: PlanCommand::Validate { files },
        } => validate_files(
            files,
            agov_coord::plan_validator::validate_file,
            None,
            &args,
            "plan-validate",
        ),
        Command::Validate { sub } => run_validate(sub, &args),
        Command::Handoff {
            sub: HandoffCommand::Next { agent_role, dir },
        } => handoff_next(&agent_role, dir, &args),
        Command::Handoff {
            sub: HandoffCommand::Take { id, from, dir },
        } => handoff_take(&id, &from, dir, &args),
        Command::Handoff {
            sub: HandoffCommand::Complete { id, from, to, dir },
        } => handoff_complete(&id, &from, to, dir, &args),
        Command::Session {
            sub:
                SessionCommand::Resume {
                    role,
                    base_branch,
                    dir,
                    manifest,
                },
        } => session_resume(role, &base_branch, dir, manifest, &args),
        Command::Token {
            sub:
                TokenCommand::Roi {
                    sub: TokenRoiCommand::Report { log },
                },
        } => token_roi_report(log),
        Command::Dist { sub } => dist_command(sub, &args),
        Command::Usage {
            sub:
                UsageCommand::Report {
                    dir,
                    manifest,
                    last,
                },
        } => usage_report(dir, manifest, last, &args),
    }
}

#[derive(Serialize)]
struct VersionInfo {
    version: &'static str,
    commit: &'static str,
    dirty: bool,
}

fn build_version_info() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        commit: option_env!("AGOV_GIT_SHA").unwrap_or("unknown"),
        dirty: option_env!("AGOV_GIT_DIRTY").unwrap_or("false") == "true",
    }
}

fn version_text() -> String {
    let info = build_version_info();
    let state = if info.dirty { "dirty" } else { "clean" };
    format!("agov {} ({} {state})", info.version, info.commit)
}

fn version_command(format: &str, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    match format {
        "json" => match serde_json::to_string(&build_version_info()) {
            Ok(json) => output.stdout(json),
            Err(err) => {
                output.stderr(format!("ERR failed to serialise version info: {err}"));
                return 1;
            }
        },
        _ => output.stdout(version_text()),
    }
    agov_core::roi::record_tool_output("agov", args, "version", output.chars);
    0
}

fn dist_command(sub: DistCommand, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let exit_code = match sub {
        DistCommand::Validate => match dist::validate_default() {
            Ok(spec) => {
                output.stdout(dist::format_validate_summary(&spec));
                0
            }
            Err(errors) => {
                for error in errors {
                    output.stderr(format!("ERR {error}"));
                }
                1
            }
        },
        DistCommand::Build => match dist::build_default() {
            Ok(result) => {
                output.stdout(dist::format_build_summary(&result));
                0
            }
            Err(errors) => {
                for error in errors {
                    output.stderr(format!("ERR {error}"));
                }
                1
            }
        },
        DistCommand::CheckTag => match dist::check_tag_from_env() {
            Ok(dist::TagCheck::Matched) => {
                output.stdout("OK tag matches agf/VERSION");
                0
            }
            Ok(dist::TagCheck::NoTag) => {
                output.stdout("OK no CI_COMMIT_TAG set; tag guard skipped");
                0
            }
            Err(error) => {
                output.stderr(format!("ERR {error}"));
                1
            }
        },
        DistCommand::ValidateProvenance {
            files,
            artifact_dir,
            oci_image,
        } => validate_provenance_command(
            &files,
            artifact_dir.as_deref(),
            oci_image.as_deref(),
            &mut output,
        ),
        DistCommand::ProvenanceRecord { sub } => provenance_record_command(*sub, &mut output),
    };

    if exit_code == 0 {
        agov_core::roi::record_tool_output("agov", args, "dist", output.chars);
    }
    exit_code
}

/// Validate one or more release-provenance JSON records: schema validation
/// (or legacy classification) for each file, plus cross-record duplicate-
/// filename checks and, when `artifact_dir` is given, artifact/sha256/
/// checksum-sidecar agreement and dangling-reference checks. When
/// `oci_image` is given, `podman inspect`s that real image and cross-checks
/// its actual labels/digest against any oci-image records among `files`.
fn validate_provenance_command(
    files: &[PathBuf],
    artifact_dir: Option<&Path>,
    oci_image: Option<&str>,
    output: &mut OutputCounter,
) -> i32 {
    let mut parsed = Vec::new();
    let mut exit_code = 0;

    let oci_evidence = match oci_image {
        Some(image) => match inspect_oci_image(image) {
            Ok(evidence) => Some(evidence),
            Err(err) => {
                output.stderr(format!("ERR  could not inspect OCI image {image}: {err}"));
                return 1;
            }
        },
        None => None,
    };

    for file in files {
        let content = match std::fs::read_to_string(file) {
            Ok(content) => content,
            Err(err) => {
                output.stderr(format!("ERR  could not read {}: {err}", file.display()));
                exit_code = 1;
                continue;
            }
        };
        let outcome = match provenance::validate_json_text(&content) {
            Ok(outcome) => outcome,
            Err(err) => {
                output.stderr(format!("ERR  {}: {err}", file.display()));
                exit_code = 1;
                continue;
            }
        };
        let data: serde_json::Value =
            serde_json::from_str(&content).expect("validate_json_text already parsed this JSON");

        let schema_valid = outcome.is_valid();
        let mut findings = outcome.findings;

        if schema_valid && outcome.assurance_level == provenance::AssuranceLevel::V24 {
            if data
                .get("artifact_kind")
                .and_then(serde_json::Value::as_str)
                == Some("oci-image")
            {
                if let Some(evidence) = &oci_evidence {
                    findings.extend(provenance::check_oci_image_evidence(&data, evidence));
                }
            }
            if let Some(dir) = artifact_dir {
                findings.extend(provenance::check_artifact_agreement(&data, dir));
                if let Some(filename) = data
                    .get("artifact_filename")
                    .and_then(serde_json::Value::as_str)
                {
                    // Mandatory: check_sha256_sidecar_agreement itself reports
                    // a missing sidecar as a finding rather than being skipped.
                    let sidecar = dir.join(format!("{filename}.sha256"));
                    findings.extend(provenance::check_sha256_sidecar_agreement(&data, &sidecar));
                }
            }
        }

        if findings.is_empty() {
            output.stdout(format!(
                "OK  {} ({})",
                file.display(),
                outcome.assurance_level.as_str()
            ));
        } else {
            exit_code = 1;
            for finding in &findings {
                output.stderr(format!("ERR  {}: {finding}", file.display()));
            }
        }

        parsed.push(data);
    }

    for finding in provenance::check_no_duplicate_filenames(&parsed) {
        exit_code = 1;
        output.stderr(format!("ERR  {finding}"));
    }

    exit_code
}

/// Shell out to `podman inspect <image>` and parse real label/digest
/// evidence from its output. This project's OCI tooling (`agov-oci-
/// image.sh`) already shells out to `podman` for every image operation
/// rather than embedding an OCI client library in Rust; this follows the
/// same convention.
fn inspect_oci_image(image: &str) -> Result<provenance::OciImageEvidence, String> {
    let output = std::process::Command::new("podman")
        .args(["inspect", image])
        .output()
        .map_err(|err| format!("could not run podman: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "podman inspect {image} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    provenance::parse_podman_inspect(&stdout)
}

fn provenance_record_command(sub: ProvenanceRecordCommand, output: &mut OutputCounter) -> i32 {
    match sub {
        ProvenanceRecordCommand::File {
            product,
            version,
            source_commit,
            tag,
            build_timestamp,
            pipeline_id,
            job_id,
            manifest_path,
            implementation,
            packaging,
            artifact,
            target_platform,
            output: output_path,
        } => {
            let Some(artifact_filename) = artifact.file_name().and_then(|name| name.to_str())
            else {
                output.stderr(format!(
                    "ERR  artifact path has no filename: {}",
                    artifact.display()
                ));
                return 1;
            };
            let artifact_bytes = match std::fs::metadata(&artifact) {
                Ok(metadata) => metadata.len(),
                Err(err) => {
                    output.stderr(format!("ERR  could not stat {}: {err}", artifact.display()));
                    return 1;
                }
            };
            let sha256 = match provenance::sha256_file(&artifact) {
                Ok(hash) => hash,
                Err(err) => {
                    output.stderr(format!("ERR  {err}"));
                    return 1;
                }
            };
            let pipeline_identity = pipeline_identity_from(&pipeline_id, &job_id);
            let record = provenance::build_file_record(&provenance::FileRecordInput {
                product: &product,
                version: &version,
                source_commit: &source_commit,
                tag: &tag,
                build_timestamp: &build_timestamp,
                pipeline_identity,
                manifest_path: manifest_path.as_deref(),
                implementation: &implementation,
                packaging: &packaging,
                artifact_filename,
                artifact_bytes,
                sha256: &sha256,
                target_platform: target_platform.as_deref(),
            });
            emit_provenance_record(&record, output_path.as_deref(), output)
        }
        ProvenanceRecordCommand::Oci {
            product,
            version,
            source_commit,
            tag,
            build_timestamp,
            pipeline_id,
            job_id,
            implementation,
            packaging,
            image_reference,
            manifest_digest,
            label_version,
            label_revision,
            output: output_path,
        } => {
            let pipeline_identity = pipeline_identity_from(&pipeline_id, &job_id);
            let record = provenance::build_oci_record(&provenance::OciRecordInput {
                product: &product,
                version: &version,
                source_commit: &source_commit,
                tag: &tag,
                build_timestamp: &build_timestamp,
                pipeline_identity,
                implementation: &implementation,
                packaging: &packaging,
                image_reference: &image_reference,
                manifest_digest: &manifest_digest,
                label_version: &label_version,
                label_revision: &label_revision,
            });
            emit_provenance_record(&record, output_path.as_deref(), output)
        }
    }
}

fn pipeline_identity_from<'a>(
    pipeline_id: &'a Option<String>,
    job_id: &'a Option<String>,
) -> Option<provenance::PipelineIdentity<'a>> {
    match (pipeline_id, job_id) {
        (Some(pipeline_id), Some(job_id)) => Some(provenance::PipelineIdentity {
            pipeline_id,
            job_id,
        }),
        _ => None,
    }
}

fn emit_provenance_record(
    record: &serde_json::Value,
    output_path: Option<&Path>,
    output: &mut OutputCounter,
) -> i32 {
    let json = match serde_json::to_string_pretty(record) {
        Ok(json) => json,
        Err(err) => {
            output.stderr(format!("ERR  could not encode provenance JSON: {err}"));
            return 1;
        }
    };
    match output_path {
        Some(path) => match std::fs::write(path, format!("{json}\n")) {
            Ok(()) => {
                output.stdout(format!("OK  wrote {}", path.display()));
                0
            }
            Err(err) => {
                output.stderr(format!("ERR  could not write {}: {err}", path.display()));
                1
            }
        },
        None => {
            output.stdout(json);
            0
        }
    }
}

fn token_roi_report(log: Option<PathBuf>) -> i32 {
    let path = log.unwrap_or_else(agov_core::roi::default_report_path);
    match agov_core::roi::load_records(&path) {
        Ok(records) => {
            let summary = agov_core::roi::summarize(&records);
            println!("{}", agov_core::roi::format_report(&summary, &path));
            0
        }
        Err(reason) => {
            eprintln!(
                "ERR  could not read token ROI log {}: {reason}",
                path.display()
            );
            1
        }
    }
}

fn usage_report(dir: PathBuf, manifest: PathBuf, last: Option<usize>, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);
    let manifest = repo_relative_path(manifest);

    let (messages, invalid_files) = match read_coord_messages_with_invalid(&dir) {
        Ok(result) => result,
        Err(_) => {
            output.stderr(format!("ERR  directory not found: {}", dir.display()));
            return 1;
        }
    };

    let agents = match load_agent_statuses(&manifest) {
        Ok(agents) => agents,
        Err(_) => {
            output.stderr(format!(
                "WARN manifest not found: {}  (agent cross-reference unavailable)",
                manifest.display()
            ));
            BTreeMap::new()
        }
    };
    let aliases = load_from_aliases(&dir);
    let report = build_usage_report(&messages, &agents, &aliases, last, invalid_files);

    render_usage_report(&mut output, &report, &dir, messages.len());
    agov_core::roi::record_tool_output("agov", args, "usage-report", output.chars);
    0
}

#[derive(Debug)]
struct UsageReport {
    window: UsageWindow,
    range: Option<(String, String)>,
    kind_counts: BTreeMap<String, usize>,
    sender_counts: BTreeMap<String, usize>,
    inactive_agents: Vec<String>,
    unknown_senders: Vec<String>,
    invalid_files: Vec<String>,
}

#[derive(Debug)]
enum UsageWindow {
    All,
    Last(usize),
}

fn build_usage_report(
    messages: &[HandoffMessage],
    agents: &BTreeMap<String, String>,
    aliases: &BTreeMap<String, String>,
    last: Option<usize>,
    invalid_files: Vec<String>,
) -> UsageReport {
    let window_start = last
        .map(|last| messages.len().saturating_sub(last))
        .unwrap_or(0);
    let window_messages = &messages[window_start..];
    let mut kind_counts = BTreeMap::new();
    let mut sender_counts = BTreeMap::new();

    for message in window_messages {
        let kind = string_field(&message.raw, "kind").unwrap_or("unknown");
        *kind_counts.entry(kind.to_string()).or_insert(0) += 1;

        let sender = resolve_alias(
            string_field(&message.raw, "from").unwrap_or("unknown"),
            aliases,
        );
        *sender_counts.entry(sender).or_insert(0) += 1;
    }

    let inactive_agents = agents
        .iter()
        .filter(|(_agent, status)| status.as_str() == "active")
        .filter(|(agent, _status)| !sender_counts.contains_key(*agent))
        .map(|(agent, _status)| agent.clone())
        .collect();

    let unknown_senders = sender_counts
        .keys()
        .filter(|sender| !agents.contains_key(*sender))
        .cloned()
        .collect();

    UsageReport {
        window: last.map(UsageWindow::Last).unwrap_or(UsageWindow::All),
        range: usage_timestamp_range(messages),
        kind_counts,
        sender_counts,
        inactive_agents,
        unknown_senders,
        invalid_files,
    }
}

fn render_usage_report(output: &mut OutputCounter, report: &UsageReport, dir: &Path, total: usize) {
    let range = report
        .range
        .as_ref()
        .map(|(first, last)| format!("{first} to {last}"))
        .unwrap_or_else(|| "(empty)".to_string());
    output.stdout(format!(
        "Channel: {}  ({} messages, {})",
        display_coord_dir(dir),
        total,
        range
    ));

    let window = match report.window {
        UsageWindow::All => "all messages".to_string(),
        UsageWindow::Last(last) => format!("last {last} messages"),
    };
    output.stdout(format!("Window:  {window}"));
    output.stdout("");

    render_usage_counts(output, "Kind distribution:", &report.kind_counts, 18, None);
    render_usage_counts(
        output,
        "Sender activity (aliases applied):",
        &report.sender_counts,
        24,
        Some(&report.inactive_agents),
    );
    render_usage_list(
        output,
        "Inactive active-status agents:",
        "none",
        &report.inactive_agents,
    );
    render_usage_list(
        output,
        "Unknown senders (after alias resolution):",
        "none",
        &report.unknown_senders,
    );

    if report.invalid_files.is_empty() {
        output.stdout("Invalid coordination files: none");
    } else {
        output.stderr(format!(
            "WARN  {} invalid coordination file(s) skipped:",
            report.invalid_files.len()
        ));
        for file in &report.invalid_files {
            output.stderr(format!("  {file}"));
        }
    }
}

fn render_usage_counts(
    output: &mut OutputCounter,
    title: &str,
    counts: &BTreeMap<String, usize>,
    pad: usize,
    inactive_agents: Option<&[String]>,
) {
    if counts.is_empty() {
        output.stdout(format!("{title} (none)"));
        output.stdout("");
        return;
    }

    let mut rows: Vec<_> = counts.iter().collect();
    rows.sort_by(|(left_name, left_count), (right_name, right_count)| {
        right_count
            .cmp(left_count)
            .then_with(|| left_name.cmp(right_name))
    });

    output.stdout(title);
    for (name, count) in rows {
        let flag = if inactive_agents
            .map(|agents| agents.iter().any(|agent| agent == name))
            .unwrap_or(false)
        {
            "  <- inactive"
        } else {
            ""
        };
        output.stdout(format!("  {name:<pad$}  {count}{flag}"));
    }
    output.stdout("");
}

fn render_usage_list(output: &mut OutputCounter, title: &str, empty: &str, items: &[String]) {
    if items.is_empty() {
        output.stdout(format!("{title} {empty}"));
        output.stdout("");
    } else {
        output.stdout(title);
        for item in items {
            output.stdout(format!("  {item}"));
        }
        output.stdout("");
    }
}

fn usage_timestamp_range(messages: &[HandoffMessage]) -> Option<(String, String)> {
    let first = messages
        .iter()
        .filter_map(|message| string_field(&message.raw, "timestamp"))
        .min()?;
    let last = messages
        .iter()
        .filter_map(|message| string_field(&message.raw, "timestamp"))
        .max()?;
    Some((first.to_string(), last.to_string()))
}

fn load_agent_statuses(path: &Path) -> Result<BTreeMap<String, String>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let data = serde_norway::from_str::<serde_json::Value>(&content)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let agents = data
        .get("agents")
        .and_then(|value| value.as_object())
        .into_iter()
        .flatten()
        .filter_map(|(agent, attrs)| {
            let status = attrs.get("status").and_then(|value| value.as_str())?;
            Some((agent.clone(), status.to_string()))
        })
        .collect();
    Ok(agents)
}

struct CoordBriefOptions {
    id_format: String,
    from: Option<String>,
    to: Option<String>,
    kind: String,
    topic: Option<String>,
    model: Option<String>,
    dir: PathBuf,
}

struct CoordPostOptions {
    id_format: String,
    from: String,
    to: String,
    kind: String,
    topic: String,
    refs: Vec<String>,
    artifacts: Vec<String>,
    tasks: Vec<String>,
    model: Option<String>,
    dir: PathBuf,
}

struct CoordDraft {
    id_format: String,
    from: String,
    to: String,
    kind: String,
    topic: String,
    refs: Vec<String>,
    artifacts: Vec<String>,
    tasks: Vec<String>,
    model: Option<String>,
    dir: PathBuf,
    body: String,
}

fn coord_brief(options: CoordBriefOptions, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let CoordBriefOptions {
        id_format,
        from,
        to,
        kind,
        topic,
        model,
        dir,
    } = options;

    if !valid_coord_kind(&kind) {
        output.stderr(format!(
            "ERR  unknown kind: {kind}  (supported: claim completion review ack handoff decision-record fast-track-waiver lesson-proposal note)"
        ));
        return 1;
    }

    if id_format == "readable" && from.is_none() {
        output.stderr("ERR  --id-format readable requires --from <role>");
        return 1;
    }

    let now = time::OffsetDateTime::now_utc();
    let next_id = if id_format == "readable" {
        readable_id(from.as_deref().unwrap_or("unknown"), now)
    } else {
        agov_core::ulid_util::new()
    };
    let suggested_filename = suggested_filename(&id_format, from.as_deref(), &next_id);
    let timestamp = if id_format == "readable" {
        iso_timestamp(now)
    } else {
        "<TIMESTAMP>".to_string()
    };
    let from = from.unwrap_or_else(|| "<from>".to_string());
    let to = to.unwrap_or_else(|| "<to>".to_string());
    let topic = topic.unwrap_or_else(|| "<topic>".to_string());
    let model = model
        .or_else(|| std::env::var("AGOV_MODEL").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let coord_dir = repo_relative_path(dir);
    let channel = channel_summary(&coord_dir);

    output.stdout(format!(
        "# Channel: {} messages, {}",
        channel.total, channel.range
    ));
    output.stdout(format!(
        "# Ownership: {} (run agov coord status --roles for details)",
        channel.claims
    ));
    output.stdout(format!("# Next id: {next_id}"));
    output.stdout(format!("# Suggested file: {suggested_filename}"));
    output.stdout("---");
    output.stdout(format!("kind: {kind}"));
    output.stdout(format!("from: {from}"));
    output.stdout(format!("to: {to}"));
    output.stdout(format!("id: {next_id}"));
    output.stdout(format!("timestamp: {timestamp}"));
    if let Some(model) = model {
        output.stdout(format!("model: {model}"));
    }
    output.stdout("immutable: true");
    render_kind_front_matter(&mut output, &kind, &to, &topic);
    output.stdout("---");

    if kind == "review" {
        output.stdout("");
        output.stdout("# <findings — severity: BLOCK | WARN | INFO>");
    } else {
        output.stdout("");
        output.stdout("# <message body>");
    }

    agov_core::roi::record_tool_output("agov", args, "coord-brief", output.chars);
    0
}

fn coord_post(options: CoordPostOptions, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();

    if !valid_coord_kind(&options.kind) {
        output.stderr(format!(
            "ERR  unknown kind: {}  (supported: claim completion review ack handoff decision-record fast-track-waiver lesson-proposal note)",
            options.kind
        ));
        return 1;
    }

    if options.from.trim().is_empty()
        || options.to.trim().is_empty()
        || options.topic.trim().is_empty()
    {
        output.stderr("ERR  --from, --to, and --topic must be non-empty");
        return 1;
    }

    if options.kind == "ack" && options.refs.is_empty() {
        output.stderr("ERR  ack messages require at least one --ref");
        return 1;
    }

    if options.kind == "claim" && options.artifacts.is_empty() {
        output.stderr("ERR  claim messages require at least one --artifact");
        return 1;
    }

    let refs = match normalize_refs(&options.refs) {
        Ok(refs) => refs,
        Err(reference) => {
            output.stderr(format!("ERR  invalid coordination reference: {reference}"));
            return 1;
        }
    };

    let body = if options.kind == "review" {
        "# Findings\n\n- INFO: Generated coordination record.\n".to_string()
    } else {
        "# Message\n\nGenerated by agov coord post.\n".to_string()
    };

    let draft = CoordDraft {
        id_format: options.id_format,
        from: options.from,
        to: options.to,
        kind: options.kind,
        topic: options.topic,
        refs,
        artifacts: options.artifacts,
        tasks: options.tasks,
        model: options
            .model
            .or_else(|| std::env::var("AGOV_MODEL").ok())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        dir: repo_relative_path(options.dir),
        body,
    };

    match write_coord_draft(draft) {
        Ok(path) => {
            output.stdout(format!("Wrote {}", path.display()));
            output.stdout(format!("Validate: agov coord validate {}", path.display()));
            agov_core::roi::record_tool_output("agov", args, "coord-post", output.chars);
            0
        }
        Err(message) => {
            output.stderr(format!("ERR  {message}"));
            1
        }
    }
}

fn coord_list(since: Option<String>, dir: PathBuf, format: String, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);
    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!("ERR  directory not found: {}", dir.display()));
            return 1;
        }
    };

    let cursor = match since {
        Some(value) => match coord_list_cursor(&messages, &value) {
            Ok(cursor) => Some(cursor),
            Err(message) => {
                output.stderr(format!("ERR  {message}"));
                return 1;
            }
        },
        None => None,
    };

    let items: Vec<HandoffMessage> = messages
        .into_iter()
        .filter(|message| match &cursor {
            Some(cursor) => message.sort_key > *cursor,
            None => true,
        })
        .collect();

    if items.is_empty() {
        output.stdout("No coordination messages found");
        return 2;
    }

    if format == "json" {
        let rows: Vec<CoordListRow> = items.iter().map(CoordListRow::from).collect();
        match serde_json::to_string_pretty(&rows) {
            Ok(json) => output.stdout(json),
            Err(err) => {
                output.stderr(format!("ERR  could not render JSON: {err}"));
                return 1;
            }
        }
    } else {
        for message in &items {
            output.stdout(format!(
                "{}  {}  {}  {} -> {}  {}  {}",
                string_field(&message.raw, "timestamp").unwrap_or(""),
                string_field(&message.raw, "id").unwrap_or(""),
                string_field(&message.raw, "kind").unwrap_or(""),
                string_field(&message.raw, "from").unwrap_or(""),
                string_field(&message.raw, "to").unwrap_or(""),
                string_field(&message.raw, "topic").unwrap_or(""),
                message.file
            ));
        }
    }

    agov_core::roi::record_tool_output("agov", args, "coord-list", output.chars);
    0
}

struct ChannelSummary {
    total: usize,
    range: String,
    claims: String,
}

fn channel_summary(dir: &Path) -> ChannelSummary {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            return ChannelSummary {
                total: 0,
                range: "(empty)".to_string(),
                claims: "no active claims".to_string(),
            }
        }
    };

    let mut timestamps = Vec::new();
    let mut total = 0usize;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        if let Ok(message) = agov_coord::coord_validator::validate_file(&path) {
            total += 1;
            if let Some(timestamp) = message
                .raw
                .get("timestamp")
                .and_then(|value| value.as_str())
                .map(str::to_string)
            {
                timestamps.push(timestamp);
            }
        }
    }

    timestamps.sort();
    let range = match (timestamps.first(), timestamps.last()) {
        (Some(first), Some(last)) => format!("{first} to {last}"),
        _ => "(empty)".to_string(),
    };

    ChannelSummary {
        total,
        range,
        claims: "no active claims".to_string(),
    }
}

fn repo_relative_path(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        agov_core::repo_root().join(path)
    }
}

fn render_kind_front_matter(output: &mut OutputCounter, kind: &str, to: &str, topic: &str) {
    match kind {
        "handoff" => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned: []");
            output.stdout("handoff:");
            output.stdout(format!("  next_agent: {to}"));
            output.stdout("  status: ready");
            output.stdout("  task: []");
            output.stdout("  must_read: []");
            output.stdout("  do_not_touch:");
            output.stdout("    - .claude/settings.local.json");
            output.stdout("    - tooling/elixir/mix.lock");
        }
        "claim" => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned:");
            output.stdout("  # list paths you are claiming ownership of");
            output.stdout("  - <path>");
        }
        "completion" => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned:");
            output.stdout("  # list paths being released; [] releases all by this sender");
            output.stdout("  []");
        }
        "ack" => {
            output.stdout("references-to:");
            output.stdout("  # filename(s) this ACK acknowledges");
            output.stdout("  - <filename>");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned: []");
        }
        _ => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned: []");
        }
    }
}

fn valid_coord_kind(kind: &str) -> bool {
    matches!(
        kind,
        "claim"
            | "completion"
            | "review"
            | "ack"
            | "handoff"
            | "decision-record"
            | "fast-track-waiver"
            | "lesson-proposal"
            | "note"
    )
}

fn suggested_filename(id_format: &str, from: Option<&str>, id: &str) -> String {
    if id_format == "readable" {
        format!("{id}.md")
    } else if let Some(from) = from {
        format!("{id}-{from}.md")
    } else {
        format!("{id}-<from>.md")
    }
}

fn normalize_refs(refs: &[String]) -> Result<Vec<String>, String> {
    let mut normalized = Vec::new();
    for reference in refs {
        let Some(filename) = normalize_coord_reference(reference) else {
            return Err(reference.clone());
        };
        if !normalized.contains(&filename) {
            normalized.push(filename);
        }
    }
    Ok(normalized)
}

fn write_coord_draft(draft: CoordDraft) -> Result<PathBuf, String> {
    if draft.id_format == "readable" && draft.from.trim().is_empty() {
        return Err("--id-format readable requires --from <role>".to_string());
    }

    let now = time::OffsetDateTime::now_utc();
    let id = if draft.id_format == "readable" {
        readable_id(&draft.from, now)
    } else {
        agov_core::ulid_util::new()
    };
    let filename = suggested_filename(&draft.id_format, Some(&draft.from), &id);
    let path = draft.dir.join(filename);

    std::fs::create_dir_all(&draft.dir)
        .map_err(|err| format!("could not create {}: {err}", draft.dir.display()))?;
    if path.exists() {
        return Err(format!("refusing to overwrite {}", path.display()));
    }

    let content = render_coord_draft_markdown(&draft, &id, &iso_timestamp(now))?;
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .and_then(|mut file| std::io::Write::write_all(&mut file, content.as_bytes()))
        .map_err(|err| format!("could not write {}: {err}", path.display()))?;

    if let Err(errors) = agov_coord::coord_validator::validate_file(&path) {
        let _ = std::fs::remove_file(&path);
        let rendered = errors
            .into_iter()
            .map(|(message, pointer)| format!("{pointer}: {message}"))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(format!(
            "generated coordination message failed validation; removed {}: {rendered}",
            path.display()
        ));
    }

    let duplicates = duplicate_id_errors_in_dirs(std::slice::from_ref(&path));
    if !duplicates.is_empty() {
        let _ = std::fs::remove_file(&path);
        let rendered = duplicates
            .into_iter()
            .map(|(path, message)| format!("{path}: {message}"))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(format!(
            "generated coordination message duplicated an existing id; removed {}: {rendered}",
            path.display()
        ));
    }

    Ok(path)
}

fn render_coord_draft_markdown(
    draft: &CoordDraft,
    id: &str,
    timestamp: &str,
) -> Result<String, String> {
    let mut front = serde_json::Map::new();
    front.insert(
        "kind".to_string(),
        serde_json::Value::String(draft.kind.clone()),
    );
    front.insert(
        "from".to_string(),
        serde_json::Value::String(draft.from.clone()),
    );
    front.insert(
        "to".to_string(),
        serde_json::Value::String(draft.to.clone()),
    );
    front.insert("id".to_string(), serde_json::Value::String(id.to_string()));
    front.insert(
        "timestamp".to_string(),
        serde_json::Value::String(timestamp.to_string()),
    );
    if let Some(model) = &draft.model {
        front.insert(
            "model".to_string(),
            serde_json::Value::String(model.clone()),
        );
    }
    front.insert("immutable".to_string(), serde_json::Value::Bool(true));
    front.insert(
        "references-to".to_string(),
        serde_json::Value::Array(
            draft
                .refs
                .iter()
                .map(|value| serde_json::Value::String(value.clone()))
                .collect(),
        ),
    );
    front.insert(
        "topic".to_string(),
        serde_json::Value::String(draft.topic.clone()),
    );
    front.insert("requires-ack".to_string(), serde_json::Value::Bool(false));

    match draft.kind.as_str() {
        "claim" => {
            front.insert(
                "artifacts_owned".to_string(),
                serde_json::Value::Array(
                    draft
                        .artifacts
                        .iter()
                        .map(|value| serde_json::Value::String(value.clone()))
                        .collect(),
                ),
            );
        }
        "handoff" => {
            front.insert(
                "artifacts_owned".to_string(),
                serde_json::Value::Array(vec![]),
            );
            let tasks = if draft.tasks.is_empty() {
                vec!["Continue the referenced work.".to_string()]
            } else {
                draft.tasks.clone()
            };
            front.insert(
                "handoff".to_string(),
                serde_json::json!({
                    "next_agent": draft.to,
                    "status": "ready",
                    "task": tasks,
                    "must_read": [],
                    "do_not_touch": [
                        ".claude/settings.local.json",
                        "tooling/elixir/mix.lock"
                    ]
                }),
            );
        }
        _ => {
            front.insert(
                "artifacts_owned".to_string(),
                serde_json::Value::Array(vec![]),
            );
        }
    }

    let yaml = serde_norway::to_string(&serde_json::Value::Object(front))
        .map_err(|err| format!("could not render front matter: {err}"))?;
    Ok(format!("---\n{yaml}---\n\n{}", draft.body))
}

#[derive(Serialize)]
struct CoordListRow {
    file: String,
    id: String,
    timestamp: String,
    kind: String,
    from: String,
    to: String,
    topic: String,
}

impl From<&HandoffMessage> for CoordListRow {
    fn from(message: &HandoffMessage) -> Self {
        Self {
            file: message.file.clone(),
            id: string_field(&message.raw, "id").unwrap_or("").to_string(),
            timestamp: string_field(&message.raw, "timestamp")
                .unwrap_or("")
                .to_string(),
            kind: string_field(&message.raw, "kind").unwrap_or("").to_string(),
            from: string_field(&message.raw, "from").unwrap_or("").to_string(),
            to: string_field(&message.raw, "to").unwrap_or("").to_string(),
            topic: string_field(&message.raw, "topic")
                .unwrap_or("")
                .to_string(),
        }
    }
}

fn coord_list_cursor(messages: &[HandoffMessage], since: &str) -> Result<(String, String), String> {
    if let Ok(timestamp) = time::OffsetDateTime::parse(since, &Rfc3339) {
        return Ok((iso_timestamp(timestamp), String::new()));
    }

    let normalized = normalize_coord_reference(since).unwrap_or_else(|| since.to_string());
    let stem = normalized.strip_suffix(".md").unwrap_or(&normalized);
    messages
        .iter()
        .find(|message| {
            message.file == normalized
                || string_field(&message.raw, "id") == Some(since)
                || string_field(&message.raw, "id") == Some(stem)
        })
        .map(|message| message.sort_key.clone())
        .ok_or_else(|| format!("--since did not match an RFC3339 timestamp or message id: {since}"))
}

#[derive(Clone)]
struct HandoffMessage {
    file: String,
    sort_key: (String, String),
    refs: Vec<String>,
    raw: serde_json::Value,
}

fn coord_status(dir: PathBuf, recent: usize, roles: bool, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!("ERR  directory not found: {}", dir.display()));
            return 1;
        }
    };

    let display_dir = display_coord_dir(&dir);
    if messages.is_empty() {
        output.stdout(format!("Channel: {display_dir}  (empty)"));
        agov_core::roi::record_tool_output("agov", args, "coord-status", output.chars);
        return 0;
    }

    output.stdout(format!(
        "Channel: {display_dir}  ({} messages, {})",
        messages.len(),
        message_range(&messages)
    ));
    output.stdout("");
    output.stdout("Ownership:    none");
    output.stdout("");
    output.stdout("Stale claims: none");
    output.stdout("");

    let pending = pending_handoffs(&messages);
    if pending.is_empty() {
        output.stdout("Pending handoffs: none");
    } else {
        output.stdout("Pending handoffs:");
        for message in pending.iter().rev() {
            output.stdout(format!(
                "  → {}  {}  {}  [{}]",
                string_field(&message.raw, "to").unwrap_or("unknown"),
                display_id(&message.raw),
                string_field(&message.raw, "topic").unwrap_or(""),
                message
                    .raw
                    .get("handoff")
                    .and_then(|value| value.get("status"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("ready")
            ));
        }
    }
    output.stdout("");

    if recent > 0 {
        output.stdout(format!("Recent messages (last {recent}):"));
        for message in messages.iter().rev().take(recent) {
            output.stdout(format!(
                "  {}  {} → {}  {:<14}  {}",
                display_id(&message.raw),
                string_field(&message.raw, "from").unwrap_or("unknown"),
                string_field(&message.raw, "to").unwrap_or("unknown"),
                string_field(&message.raw, "kind").unwrap_or("unknown"),
                string_field(&message.raw, "topic").unwrap_or("")
            ));
        }
    }

    if roles {
        if recent > 0 {
            output.stdout("");
        }
        render_role_activity(&mut output, &messages);
    }

    agov_core::roi::record_tool_output("agov", args, "coord-status", output.chars);
    0
}

fn display_coord_dir(dir: &Path) -> String {
    let root = agov_core::repo_root();
    dir.strip_prefix(&root)
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| dir.display().to_string())
}

fn message_range(messages: &[HandoffMessage]) -> String {
    let first = messages
        .iter()
        .filter_map(|message| string_field(&message.raw, "timestamp"))
        .min();
    let last = messages
        .iter()
        .filter_map(|message| string_field(&message.raw, "timestamp"))
        .max();

    match (first, last) {
        (Some(first), Some(last)) => format!("{first} to {last}"),
        _ => "(empty)".to_string(),
    }
}

fn display_id(raw: &serde_json::Value) -> String {
    if let Some(id) = string_field(raw, "id") {
        format!("id {}", id.chars().take(8).collect::<String>())
    } else if let Some(seq) = raw.get("seq").and_then(|value| value.as_i64()) {
        format!("seq {seq}")
    } else {
        "id <missing>".to_string()
    }
}

#[derive(Default)]
struct RoleActivity {
    status: String,
    from_count: usize,
    to_count: usize,
    last_seen: Option<(String, String)>,
}

fn render_role_activity(output: &mut OutputCounter, messages: &[HandoffMessage]) {
    let mut roles = manifest_roles();

    for message in messages {
        for field in ["from", "to"] {
            let Some(role) = string_field(&message.raw, field) else {
                continue;
            };
            roles
                .entry(role.to_string())
                .or_insert_with(|| RoleActivity {
                    status: "virtual".to_string(),
                    ..RoleActivity::default()
                });
            let entry = roles.get_mut(role).expect("role entry was just inserted");
            if field == "from" {
                entry.from_count += 1;
            } else {
                entry.to_count += 1;
            }
            if let Some(timestamp) = string_field(&message.raw, "timestamp") {
                let candidate = (timestamp.to_string(), message.file.clone());
                if entry
                    .last_seen
                    .as_ref()
                    .map(|current| candidate > *current)
                    .unwrap_or(true)
                {
                    entry.last_seen = Some(candidate);
                }
            }
        }
    }

    output.stdout("Role activity:");
    for (role, activity) in roles {
        let last_seen = activity
            .last_seen
            .as_ref()
            .map(|(timestamp, file)| format!("{timestamp} ({file})"))
            .unwrap_or_else(|| "never".to_string());
        output.stdout(format!(
            "  {role}  status: {}  from: {}  to: {}  last-seen: {last_seen}",
            activity.status, activity.from_count, activity.to_count
        ));
    }
}

fn manifest_roles() -> BTreeMap<String, RoleActivity> {
    let Ok(content) = std::fs::read_to_string(agov_core::manifest_path()) else {
        return BTreeMap::new();
    };
    let Ok(data) = serde_norway::from_str::<serde_json::Value>(&content) else {
        return BTreeMap::new();
    };
    let Some(agents) = data.get("agents").and_then(|value| value.as_object()) else {
        return BTreeMap::new();
    };

    agents
        .iter()
        .map(|(role, attrs)| {
            let status = attrs
                .get("status")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown")
                .to_string();
            (
                role.clone(),
                RoleActivity {
                    status,
                    ..RoleActivity::default()
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct ClaimEntry {
    path: String,
    from: String,
    seq: Option<i64>,
    id: Option<String>,
    claimed_at: String,
    expires_on: Option<time::OffsetDateTime>,
    expires_raw: Option<String>,
}

#[derive(Debug, Clone)]
struct InvalidExpiresOn {
    file: String,
    value: String,
}

fn coord_stale(dir: PathBuf, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!("ERR  directory not found: {}", dir.display()));
            return 2;
        }
    };

    let aliases = load_from_aliases(&dir);
    let now = time::OffsetDateTime::now_utc();
    let stale = match stale_claims(&messages, &aliases, now) {
        Ok(stale) => stale,
        Err(errors) => {
            for error in errors {
                output.stderr(format!(
                    "ERR  {}  invalid expires_on: {}",
                    error.file, error.value
                ));
            }
            return 2;
        }
    };

    if stale.is_empty() {
        output.stdout("OK  no stale claims");
        agov_core::roi::record_tool_output("agov", args, "coord-stale", output.chars);
        return 0;
    }

    for entry in stale {
        output.stdout(format!(
            "STALE  {}  claimed-by: {}  seq: {}  claimed-at: {}  expired-at: {}",
            entry.path,
            entry.from,
            claim_seq_label(&entry),
            entry.claimed_at,
            entry.expires_raw.unwrap_or_default()
        ));
    }
    1
}

fn stale_claims(
    messages: &[HandoffMessage],
    aliases: &BTreeMap<String, String>,
    now: time::OffsetDateTime,
) -> Result<Vec<ClaimEntry>, Vec<InvalidExpiresOn>> {
    Ok(claim_states(messages, aliases)?
        .into_iter()
        .filter(|entry| {
            entry
                .expires_on
                .map(|expires| expires < now)
                .unwrap_or(false)
        })
        .collect())
}

fn claim_states(
    messages: &[HandoffMessage],
    aliases: &BTreeMap<String, String>,
) -> Result<Vec<ClaimEntry>, Vec<InvalidExpiresOn>> {
    let mut states: BTreeMap<String, ClaimEntry> = BTreeMap::new();
    let mut errors = Vec::new();

    for message in messages {
        match string_field(&message.raw, "kind") {
            Some("claim") => {
                let expires_raw = string_field(&message.raw, "expires_on").map(str::to_string);
                let expires_on = match expires_raw.as_deref() {
                    Some(value) => match time::OffsetDateTime::parse(value, &Rfc3339) {
                        Ok(parsed) => Some(parsed),
                        Err(_) => {
                            errors.push(InvalidExpiresOn {
                                file: message.file.clone(),
                                value: value.to_string(),
                            });
                            continue;
                        }
                    },
                    None => None,
                };
                let from = string_field(&message.raw, "from")
                    .unwrap_or("unknown")
                    .to_string();
                let seq = message.raw.get("seq").and_then(|value| value.as_i64());
                let id = string_field(&message.raw, "id").map(str::to_string);
                let claimed_at = string_field(&message.raw, "timestamp")
                    .unwrap_or("")
                    .to_string();

                for path in string_array_field(&message.raw, "artifacts_owned") {
                    states.insert(
                        path.clone(),
                        ClaimEntry {
                            path,
                            from: from.clone(),
                            seq,
                            id: id.clone(),
                            claimed_at: claimed_at.clone(),
                            expires_on,
                            expires_raw: expires_raw.clone(),
                        },
                    );
                }
            }
            Some("completion") => release_claims(&mut states, &message.raw, aliases),
            _ => {}
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(states.into_values().collect())
}

fn release_claims(
    states: &mut BTreeMap<String, ClaimEntry>,
    raw: &serde_json::Value,
    aliases: &BTreeMap<String, String>,
) {
    let from = resolve_alias(string_field(raw, "from").unwrap_or("unknown"), aliases);
    let paths = string_array_field(raw, "artifacts_owned");

    if paths.is_empty() {
        states.retain(|_path, entry| resolve_alias(&entry.from, aliases) != from);
    } else {
        for path in paths {
            let should_release = states
                .get(&path)
                .map(|entry| resolve_alias(&entry.from, aliases) == from)
                .unwrap_or(false);
            if should_release {
                states.remove(&path);
            }
        }
    }
}

fn claim_seq_label(entry: &ClaimEntry) -> String {
    entry
        .seq
        .map(|seq| seq.to_string())
        .or_else(|| entry.id.as_ref().map(|_id| String::new()))
        .unwrap_or_default()
}

fn load_from_aliases(dir: &Path) -> BTreeMap<String, String> {
    let path = dir.join("agov-config.yaml");
    let Ok(content) = std::fs::read_to_string(path) else {
        return BTreeMap::new();
    };
    let Ok(data) = serde_norway::from_str::<serde_json::Value>(&content) else {
        return BTreeMap::new();
    };
    let Some(aliases) = data.get("from_aliases").and_then(|value| value.as_object()) else {
        return BTreeMap::new();
    };

    aliases
        .iter()
        .filter_map(|(from, to)| Some((from.clone(), to.as_str()?.to_string())))
        .collect()
}

fn resolve_alias(value: &str, aliases: &BTreeMap<String, String>) -> String {
    aliases
        .get(value)
        .cloned()
        .unwrap_or_else(|| value.to_string())
}

fn string_array_field(raw: &serde_json::Value, field: &str) -> Vec<String> {
    raw.get(field)
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .map(str::to_string)
        .collect()
}

#[derive(Debug)]
struct VisibilityMessage {
    file: String,
    raw: serde_json::Value,
}

#[derive(Debug)]
struct VisibilityReport {
    base_branch: String,
    scanned_branches: Vec<String>,
    unbridged_obligations: Vec<VisibilityObligation>,
    sequence_collisions: Vec<VisibilityCollision>,
    errors: Vec<String>,
}

#[derive(Debug, Clone)]
struct VisibilityObligation {
    branch: String,
    file: String,
    seq: Option<i64>,
    kind: String,
    to: String,
    topic: String,
}

#[derive(Debug, Clone)]
enum VisibilityCollision {
    BaseDuplicateSeq {
        seq: i64,
        files: Vec<String>,
    },
    BranchBaseSeq {
        seq: i64,
        branch: String,
        branch_file: String,
        base_file: String,
    },
}

fn coord_graph(dir: PathBuf, format: String, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!("ERR  directory not found: {}", dir.display()));
            agov_core::roi::record_tool_output("agov", args, "coord-graph", output.chars);
            return 1;
        }
    };

    // Collect (from, to, kind) counts
    let mut edges: std::collections::BTreeMap<(String, String, String), usize> =
        std::collections::BTreeMap::new();
    let mut roles: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    for message in &messages {
        let from = match string_field(&message.raw, "from") {
            Some(v) => v.to_string(),
            None => continue,
        };
        let to = match string_field(&message.raw, "to") {
            Some(v) => v.to_string(),
            None => continue,
        };
        let kind = string_field(&message.raw, "kind")
            .unwrap_or("note")
            .to_string();
        roles.insert(from.clone());
        roles.insert(to.clone());
        *edges.entry((from, to, kind)).or_insert(0) += 1;
    }

    if format == "dot" {
        output.stdout("digraph agf_coordination {");
        output.stdout("  rankdir=LR;");
        output.stdout("  node [fontname=\"Helvetica\", fontsize=11];");
        output.stdout("  edge [fontname=\"Helvetica\", fontsize=9];");
        for role in &roles {
            let attrs = if role == "governor-reviewer" {
                "shape=box, style=filled, fillcolor=\"#e8f0fe\""
            } else if role == "all" {
                "shape=ellipse"
            } else {
                "shape=box"
            };
            output.stdout(format!("  \"{role}\" [{attrs}];"));
        }
        for ((from, to, kind), count) in &edges {
            output.stdout(format!(
                "  \"{from}\" -> \"{to}\" [label=\"{kind} ({count})\"];"
            ));
        }
        output.stdout("}");
    } else {
        for ((from, to, kind), count) in &edges {
            output.stdout(format!("{from} -> {to}  {kind}  ({count})"));
        }
    }

    agov_core::roi::record_tool_output("agov", args, "coord-graph", output.chars);
    0
}

fn coord_graph_index(dir: PathBuf, output_path: Option<PathBuf>, args: &[String]) -> i32 {
    use agov_coord::channel_reader::{all_refs, read_channel};
    use agov_coord::handoff_tracker::{find_pending, MessageDescriptor};

    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let entries = read_channel(&dir);
    if entries.is_empty() && !dir.is_dir() {
        output.stderr(format!("ERR  directory not found: {}", dir.display()));
        return 1;
    }

    let generated_at = iso_timestamp(time::OffsetDateTime::now_utc());

    // Separate valid from invalid.
    let mut descriptors: Vec<MessageDescriptor> = Vec::new();
    let mut invalid_files: Vec<String> = Vec::new();

    for (file, result) in &entries {
        match result {
            Ok(msg) => {
                let path = dir.join(file);
                let refs = all_refs(&path, msg);
                let sort_key = format!("{}_{}", msg.timestamp().unwrap_or(""), file);
                descriptors.push(MessageDescriptor {
                    file: file.clone(),
                    sort_key,
                    refs,
                    msg: msg.clone(),
                });
            }
            Err(_) => invalid_files.push(file.clone()),
        }
    }

    // Determine resolved state for handoffs.
    let pending = find_pending(&descriptors);
    let pending_files: std::collections::BTreeSet<&str> =
        pending.iter().map(|p| p.file.as_str()).collect();

    // Build nodes.
    let nodes: Vec<serde_json::Value> = descriptors
        .iter()
        .map(|d| {
            let resolved = if d.msg.kind() == Some("handoff") {
                serde_json::Value::Bool(!pending_files.contains(d.file.as_str()))
            } else {
                serde_json::Value::Null
            };
            let summary = graph_index_summary(&dir.join(&d.file), d.msg.topic());
            serde_json::json!({
                "id": d.msg.id(),
                "file": d.file,
                "kind": d.msg.kind(),
                "from": d.msg.from(),
                "to": d.msg.to(),
                "topic": d.msg.topic(),
                "timestamp": d.msg.timestamp(),
                "requires_ack": d.msg.requires_ack(),
                "resolved": resolved,
                "summary": summary
            })
        })
        .collect();

    // Build edges. Frontmatter refs use type "references-to"; body-text refs
    // that were not in the frontmatter use "body-reference". Both sets are used
    // by find_pending() for resolution, so the graph exposes the same reference
    // set the resolver sees.
    let mut edges: Vec<serde_json::Value> = Vec::new();
    for d in &descriptors {
        let frontmatter_refs = d.msg.references_to();
        let frontmatter_set: std::collections::BTreeSet<&str> =
            frontmatter_refs.iter().map(|s| s.as_str()).collect();
        for referenced in &frontmatter_refs {
            edges.push(serde_json::json!({
                "from": d.file,
                "to": referenced,
                "type": "references-to"
            }));
        }
        for referenced in &d.refs {
            if !frontmatter_set.contains(referenced.as_str()) {
                edges.push(serde_json::json!({
                    "from": d.file,
                    "to": referenced,
                    "type": "body-reference"
                }));
            }
        }
    }

    // Build pending list.
    let pending_json: Vec<serde_json::Value> = pending
        .iter()
        .map(|p| {
            serde_json::json!({
                "id": p.id,
                "file": p.file,
                "from": p.from,
                "to": p.to,
                "topic": p.topic,
                "pending_since": p.pending_since
            })
        })
        .collect();

    let index = serde_json::json!({
        "generated_at": generated_at,
        "nodes": nodes,
        "edges": edges,
        "pending": pending_json,
        "invalid_files": invalid_files
    });

    let json_str = match serde_json::to_string_pretty(&index) {
        Ok(s) => s,
        Err(err) => {
            output.stderr(format!("ERR  failed to serialise graph index: {err}"));
            return 1;
        }
    };

    if let Some(path) = output_path {
        if let Err(err) = std::fs::write(&path, &json_str) {
            output.stderr(format!("ERR  could not write {}: {err}", path.display()));
            return 1;
        }
        output.stdout(format!("OK  wrote graph index to {}", path.display()));
    } else {
        println!("{json_str}");
    }

    agov_core::roi::record_tool_output("agov", args, "coord-graph-index", output.chars);
    0
}

/// Deterministic one-line summary: first H1/H2 heading from body, else topic.
fn graph_index_summary(path: &std::path::Path, topic: Option<&str>) -> String {
    if let Ok(content) = std::fs::read_to_string(path) {
        let body = content.splitn(3, "---").nth(2).unwrap_or("");
        for line in body.lines() {
            let trimmed = line.trim_start_matches('#').trim();
            if line.starts_with('#') && !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    topic.unwrap_or("").to_string()
}

fn coord_visibility(
    base_branch: String,
    branches: Vec<String>,
    dir: PathBuf,
    format: String,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let coord_dir = match git_coord_dir(&dir) {
        Ok(path) => path,
        Err(reason) => {
            output.stderr(format!("ERR coordination visibility scan failed: {reason}"));
            return 2;
        }
    };

    let report = match visibility_scan(&base_branch, branches, &coord_dir) {
        Ok(report) => report,
        Err(reason) => {
            output.stderr(format!("ERR coordination visibility scan failed: {reason}"));
            return 2;
        }
    };

    if format == "json" {
        output.stdout(visibility_json(&report));
    } else {
        render_visibility_text(&mut output, &report);
    }

    let exit_code = visibility_exit_code(&report);
    if exit_code == 0 {
        agov_core::roi::record_tool_output("agov", args, "coord-visibility", output.chars);
    }
    exit_code
}

fn visibility_scan(
    base_branch: &str,
    requested_branches: Vec<String>,
    coord_dir: &str,
) -> Result<VisibilityReport, String> {
    let base_files = git_coord_files(base_branch, coord_dir)
        .map_err(|_| format!("base_branch_unavailable: {base_branch}"))?;
    let base_messages = load_git_messages(base_branch, &base_files)?;
    let base_file_set: std::collections::BTreeSet<&str> = base_messages
        .iter()
        .map(|message| message.file.as_str())
        .collect();

    let branches = if requested_branches.is_empty() {
        git_local_branches()?
            .into_iter()
            .filter(|branch| branch != base_branch)
            .collect()
    } else {
        requested_branches
    };

    let mut unbridged_obligations = Vec::new();
    let mut sequence_collisions = base_sequence_collisions_visibility(&base_messages);

    for branch in &branches {
        let branch_files = git_coord_files(branch, coord_dir)?;
        let branch_local_files: Vec<String> = branch_files
            .into_iter()
            .filter(|file| !base_file_set.contains(file.as_str()))
            .collect();
        let branch_messages = load_git_messages(branch, &branch_local_files)?;

        for message in branch_messages
            .iter()
            .filter(|message| visibility_obligation(&message.raw))
            .filter(|message| !visibility_bridged(&base_messages, branch, message))
        {
            unbridged_obligations.push(VisibilityObligation {
                branch: branch.clone(),
                file: message.file.clone(),
                seq: message.raw.get("seq").and_then(|value| value.as_i64()),
                kind: string_field(&message.raw, "kind").unwrap_or("").to_string(),
                to: string_field(&message.raw, "to").unwrap_or("").to_string(),
                topic: string_field(&message.raw, "topic")
                    .unwrap_or("")
                    .to_string(),
            });
        }

        sequence_collisions.extend(branch_sequence_collisions(
            branch,
            &branch_messages,
            &base_messages,
        ));
    }

    unbridged_obligations.sort_by(|left, right| {
        (
            left.branch.as_str(),
            left.seq.unwrap_or_default(),
            left.file.as_str(),
        )
            .cmp(&(
                right.branch.as_str(),
                right.seq.unwrap_or_default(),
                right.file.as_str(),
            ))
    });

    Ok(VisibilityReport {
        base_branch: base_branch.to_string(),
        scanned_branches: branches,
        unbridged_obligations,
        sequence_collisions,
        errors: Vec::new(),
    })
}

fn git_coord_dir(dir: &Path) -> Result<String, String> {
    if !dir.is_absolute() {
        return Ok(dir.display().to_string());
    }

    let root = git_output(&["rev-parse", "--show-toplevel"])?;
    let root = PathBuf::from(root);
    dir.strip_prefix(&root)
        .map(|path| path.display().to_string())
        .map_err(|_| {
            format!(
                "coordination directory is outside git root: {}",
                dir.display()
            )
        })
}

fn git_local_branches() -> Result<Vec<String>, String> {
    let output = git_output(&["for-each-ref", "--format=%(refname:short)", "refs/heads"])?;
    let mut branches: Vec<String> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect();
    branches.sort();
    Ok(branches)
}

fn git_coord_files(ref_name: &str, coord_dir: &str) -> Result<Vec<String>, String> {
    let output = git_output(&["ls-tree", "-r", "--name-only", ref_name, "--", coord_dir])?;
    let mut files: Vec<String> = output
        .lines()
        .filter(|line| line.ends_with(".md"))
        .map(str::to_string)
        .collect();
    files.sort();
    Ok(files)
}

fn load_git_messages(ref_name: &str, files: &[String]) -> Result<Vec<VisibilityMessage>, String> {
    let mut messages = Vec::new();
    for file in files {
        let content = match git_output(&["show", &format!("{ref_name}:{file}")]) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let Ok(mut raw) = agov_core::yaml::parse_frontmatter_json(&content) else {
            continue;
        };
        if let Some(object) = raw.as_object_mut() {
            object.insert(
                "_body".to_string(),
                serde_json::Value::String(markdown_body(&content)),
            );
        }
        messages.push(VisibilityMessage {
            file: file.clone(),
            raw,
        });
    }
    Ok(messages)
}

fn markdown_body(content: &str) -> String {
    content
        .splitn(3, "---")
        .nth(2)
        .unwrap_or_default()
        .to_string()
}

fn visibility_obligation(raw: &serde_json::Value) -> bool {
    if string_field(raw, "kind") == Some("handoff") {
        return true;
    }
    if raw
        .get("requires-ack")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return true;
    }
    if string_field(raw, "to") == Some("all") {
        return true;
    }
    if matches!(
        string_field(raw, "kind"),
        Some("review" | "decision-record" | "note")
    ) {
        return string_field(raw, "from") != string_field(raw, "to");
    }
    false
}

fn visibility_bridged(
    base_messages: &[VisibilityMessage],
    branch: &str,
    branch_message: &VisibilityMessage,
) -> bool {
    base_messages.iter().any(|base| {
        visibility_later(base, branch_message)
            && (string_array_field(&base.raw, "references-to").contains(&branch_message.file)
                || string_field(&base.raw, "_body")
                    .map(|body| body.contains(&branch_message.file))
                    .unwrap_or(false)
                || string_field(&base.raw, "_body")
                    .map(|body| {
                        body.contains(branch)
                            && Path::new(&branch_message.file)
                                .file_name()
                                .and_then(|name| name.to_str())
                                .map(|filename| body.contains(filename))
                                .unwrap_or(false)
                    })
                    .unwrap_or(false))
    })
}

fn visibility_later(left: &VisibilityMessage, right: &VisibilityMessage) -> bool {
    visibility_sort_key(left) > visibility_sort_key(right)
}

fn visibility_sort_key(message: &VisibilityMessage) -> (String, String) {
    (
        string_field(&message.raw, "timestamp")
            .unwrap_or("")
            .to_string(),
        Path::new(&message.file)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&message.file)
            .to_string(),
    )
}

fn base_sequence_collisions_visibility(
    base_messages: &[VisibilityMessage],
) -> Vec<VisibilityCollision> {
    let mut by_seq: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for message in base_messages {
        if visibility_modern_filename(&message.file) {
            continue;
        }
        let Some(seq) = message.raw.get("seq").and_then(|value| value.as_i64()) else {
            continue;
        };
        by_seq.entry(seq).or_default().push(message.file.clone());
    }

    by_seq
        .into_iter()
        .filter_map(|(seq, mut files)| {
            if files.len() <= 1 {
                return None;
            }
            files.sort();
            Some(VisibilityCollision::BaseDuplicateSeq { seq, files })
        })
        .collect()
}

fn branch_sequence_collisions(
    branch: &str,
    branch_messages: &[VisibilityMessage],
    base_messages: &[VisibilityMessage],
) -> Vec<VisibilityCollision> {
    let mut base_by_seq: BTreeMap<i64, Vec<&VisibilityMessage>> = BTreeMap::new();
    for message in base_messages {
        if visibility_modern_filename(&message.file) {
            continue;
        }
        let Some(seq) = message.raw.get("seq").and_then(|value| value.as_i64()) else {
            continue;
        };
        base_by_seq.entry(seq).or_default().push(message);
    }

    let mut collisions = Vec::new();
    for message in branch_messages {
        if visibility_modern_filename(&message.file) {
            continue;
        }
        let Some(seq) = message.raw.get("seq").and_then(|value| value.as_i64()) else {
            continue;
        };
        for base in base_by_seq.get(&seq).into_iter().flatten() {
            if base.file == message.file {
                continue;
            }
            collisions.push(VisibilityCollision::BranchBaseSeq {
                seq,
                branch: branch.to_string(),
                branch_file: message.file.clone(),
                base_file: base.file.clone(),
            });
        }
    }
    collisions.sort_by(|left, right| {
        visibility_collision_key(left).cmp(&visibility_collision_key(right))
    });
    collisions
}

fn visibility_collision_key(collision: &VisibilityCollision) -> (i64, String, String) {
    match collision {
        VisibilityCollision::BaseDuplicateSeq { seq, files } => (
            *seq,
            String::new(),
            files.first().cloned().unwrap_or_default(),
        ),
        VisibilityCollision::BranchBaseSeq {
            seq,
            branch,
            branch_file,
            ..
        } => (*seq, branch.clone(), branch_file.clone()),
    }
}

fn visibility_modern_filename(file: &str) -> bool {
    let basename = Path::new(file)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(file);
    let stem = basename.strip_suffix(".md").unwrap_or(basename);
    is_ulid_stem(stem)
        || is_readable_id(stem)
        || stem
            .rsplit_once('-')
            .map(|(possible, _sender)| is_readable_id(possible))
            .unwrap_or(false)
}

fn is_ulid_stem(value: &str) -> bool {
    value.len() == 26
        && value
            .chars()
            .all(|ch| matches!(ch, '0'..='9' | 'A'..='H' | 'J'..='K' | 'M'..='N' | 'P'..='T' | 'V'..='Z'))
}

fn render_visibility_text(output: &mut OutputCounter, report: &VisibilityReport) {
    if report.sequence_collisions.is_empty() && report.unbridged_obligations.is_empty() {
        output.stdout(format!(
            "OK coordination visibility clean relative to {}",
            report.base_branch
        ));
        return;
    }

    if !report.sequence_collisions.is_empty() {
        output.stdout("ERR sequence collision");
        for collision in &report.sequence_collisions {
            match collision {
                VisibilityCollision::BaseDuplicateSeq { seq, files } => {
                    output.stdout(format!("  seq: {seq}"));
                    for file in files {
                        output.stdout(format!("  base-file: {file}"));
                    }
                }
                VisibilityCollision::BranchBaseSeq {
                    seq,
                    branch,
                    branch_file,
                    base_file,
                } => {
                    output.stdout(format!("  seq: {seq}"));
                    output.stdout(format!("  base: {base_file}"));
                    output.stdout(format!("  branch: {branch}"));
                    output.stdout(format!("  branch-file: {branch_file}"));
                }
            }
        }
    }

    if !report.unbridged_obligations.is_empty() {
        output.stdout("WARN branch-local coordination obligations not visible on base branch");
        for obligation in &report.unbridged_obligations {
            output.stdout(format!("  branch: {}", obligation.branch));
            output.stdout(format!("  file: {}", obligation.file));
            output.stdout(format!(
                "  seq: {}",
                obligation
                    .seq
                    .map(|seq| seq.to_string())
                    .unwrap_or_default()
            ));
            output.stdout(format!("  kind: {}", obligation.kind));
            output.stdout(format!("  to: {}", obligation.to));
            output.stdout(format!("  topic: {}", obligation.topic));
            output.stdout("  bridge: missing");
        }
    }
}

fn visibility_json(report: &VisibilityReport) -> String {
    let obligations: Vec<serde_json::Value> = report
        .unbridged_obligations
        .iter()
        .map(|entry| {
            serde_json::json!({
                "branch": entry.branch,
                "file": entry.file,
                "seq": entry.seq,
                "kind": entry.kind,
                "to": entry.to,
                "topic": entry.topic,
                "bridge": "missing",
            })
        })
        .collect();
    let collisions: Vec<serde_json::Value> = report
        .sequence_collisions
        .iter()
        .map(|collision| match collision {
            VisibilityCollision::BaseDuplicateSeq { seq, files } => {
                serde_json::json!({
                    "type": "base_duplicate_seq",
                    "seq": seq,
                    "files": files,
                })
            }
            VisibilityCollision::BranchBaseSeq {
                seq,
                branch,
                branch_file,
                base_file,
            } => serde_json::json!({
                "type": "branch_base_seq_collision",
                "seq": seq,
                "branch": branch,
                "branch_file": branch_file,
                "base_file": base_file,
            }),
        })
        .collect();

    serde_json::json!({
        "base_branch": report.base_branch,
        "scanned_branches": report.scanned_branches,
        "unbridged_obligations": obligations,
        "sequence_collisions": collisions,
        "errors": report.errors,
        "exit_code": visibility_exit_code(report),
    })
    .to_string()
}

fn visibility_exit_code(report: &VisibilityReport) -> i32 {
    if !report.sequence_collisions.is_empty() || !report.errors.is_empty() {
        2
    } else if !report.unbridged_obligations.is_empty() {
        1
    } else {
        0
    }
}

fn session_resume(
    role: Option<String>,
    base_branch: &str,
    dir: PathBuf,
    manifest: PathBuf,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);
    let manifest = repo_relative_path(manifest);

    let (messages, invalid_files) = match read_coord_messages_with_invalid(&dir) {
        Ok(result) => result,
        Err(_) => {
            output.stderr(format!(
                "Coordination directory not found: {}",
                dir.display()
            ));
            return 1;
        }
    };

    let selected_role = role.or_else(|| load_current_role(&dir));
    let git = SessionGitStatus::read(base_branch);
    let aliases = load_from_aliases(&dir);

    if let Some(role) = selected_role.filter(|role| !role.is_empty()) {
        render_session_resume(
            &mut output,
            &role,
            base_branch,
            &git,
            &messages,
            &invalid_files,
            &aliases,
        );
        agov_core::roi::record_tool_output(
            "agov.session.resume",
            args,
            &format!("session-resume:{role}"),
            output.chars,
        );
    } else {
        render_session_bootstrap(
            &mut output,
            &git,
            &messages,
            &invalid_files,
            &manifest,
            base_branch,
        );
        agov_core::roi::record_tool_output("agov.session.resume", args, "cold-start", output.chars);
    }

    0
}

struct SessionGitStatus {
    branch: String,
    dirty_files: Vec<String>,
    branch_local_result: BranchLocalResult,
}

enum BranchLocalResult {
    Files(Vec<String>),
    Skipped(String),
}

impl SessionGitStatus {
    fn read(base_branch: &str) -> Self {
        Self {
            branch: current_git_branch(),
            dirty_files: git_dirty_files()
                .unwrap_or_else(|reason| vec![format!("unable to read git status: {reason}")]),
            branch_local_result: branch_local_coord_files(base_branch),
        }
    }

    fn warnings(&self, base_branch: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        if !self.dirty_files.is_empty() {
            warnings.push(format!(
                "working tree has uncommitted changes: {}",
                self.dirty_files.join(", ")
            ));
        }

        match &self.branch_local_result {
            BranchLocalResult::Files(files) if files.is_empty() => {}
            BranchLocalResult::Files(files) => warnings.push(format!(
                "coordination files exist on this branch but not on {base_branch}: {}",
                files.join(", ")
            )),
            BranchLocalResult::Skipped(reason) => {
                warnings.push(format!("branch-local coordination check skipped: {reason}"))
            }
        }

        warnings
    }
}

fn current_git_branch() -> String {
    match git_output(&["rev-parse", "--abbrev-ref", "HEAD"]) {
        Ok(value) if !value.is_empty() => value,
        Ok(_) => "unknown".to_string(),
        Err(reason) => format!("unknown ({reason})"),
    }
}

fn git_dirty_files() -> Result<Vec<String>, String> {
    let output = git_output(&["status", "--short", "--untracked-files=all"])?;
    Ok(output
        .lines()
        .filter_map(|line| line.get(3..).or(Some(line)))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect())
}

fn branch_local_coord_files(base_branch: &str) -> BranchLocalResult {
    let candidates = [
        format!("origin/{base_branch}"),
        format!("refs/remotes/origin/{base_branch}"),
        base_branch.to_string(),
    ];

    for candidate in candidates {
        let range = format!("{candidate}...HEAD");
        if let Ok(output) = git_output(&["diff", "--name-only", &range, "--", "docs/coordination"])
        {
            let files = output
                .lines()
                .filter(|line| line.ends_with(".md"))
                .map(str::to_string)
                .collect();
            return BranchLocalResult::Files(files);
        }
    }

    BranchLocalResult::Skipped("base_branch_unavailable".to_string())
}

fn git_output(args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .map_err(|err| err.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(['\r', '\n'])
            .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(if stderr.is_empty() {
            format!("git exited with {}", output.status)
        } else {
            stderr
        })
    }
}

fn load_current_role(dir: &Path) -> Option<String> {
    let content = std::fs::read_to_string(dir.join("agov-config.yaml")).ok()?;
    let data = serde_norway::from_str::<serde_json::Value>(&content).ok()?;
    data.get("current_role")
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

fn render_session_resume(
    output: &mut OutputCounter,
    role: &str,
    base_branch: &str,
    git: &SessionGitStatus,
    messages: &[HandoffMessage],
    invalid_files: &[String],
    aliases: &BTreeMap<String, String>,
) {
    output.stdout("Session resume");
    output.stdout(format!("Role: {role}"));
    output.stdout(format!("Branch: {}", git.branch));
    output.stdout("");
    render_warnings(output, &git.warnings(base_branch));
    render_invalid_files(output, invalid_files);
    render_pending_handoffs_for_role(output, role, messages);
    render_claims_for_role(output, role, messages, aliases);
    render_recent_decisions(output, messages);
    render_must_read(output, role, messages);
}

fn render_session_bootstrap(
    output: &mut OutputCounter,
    git: &SessionGitStatus,
    messages: &[HandoffMessage],
    invalid_files: &[String],
    manifest: &Path,
    base_branch: &str,
) {
    output.stdout("Session bootstrap");
    output.stdout("Role: not selected");
    output.stdout(format!("Branch: {}", git.branch));
    output.stdout("");
    render_warnings(output, &git.warnings(base_branch));
    render_invalid_files(output, invalid_files);
    render_active_roles(output, manifest, base_branch);
    render_pending_handoffs_by_role(output, messages);
    render_recent_decisions(output, messages);
    output.stdout("Next step: rerun with the role you are playing, for example:");
    output.stdout(format!(
        "  agov session resume --role coder-tester --base-branch {base_branch}"
    ));
}

fn render_warnings(output: &mut OutputCounter, warnings: &[String]) {
    if warnings.is_empty() {
        output.stdout("Warnings: none");
    } else {
        output.stdout("Warnings:");
        for warning in warnings {
            output.stdout(format!("  - {warning}"));
        }
    }
    output.stdout("");
}

fn render_invalid_files(output: &mut OutputCounter, invalid_files: &[String]) {
    if invalid_files.is_empty() {
        return;
    }

    const INVALID_FILE_SAMPLE_LIMIT: usize = 10;

    output.stdout(format!(
        "Invalid coordination files ignored: {}",
        invalid_files.len()
    ));
    for file in invalid_files.iter().take(INVALID_FILE_SAMPLE_LIMIT) {
        output.stdout(format!("  - {file}"));
    }
    if invalid_files.len() > INVALID_FILE_SAMPLE_LIMIT {
        output.stdout(format!(
            "  ... {} more; run `agov coord validate docs/coordination/<file>.md` for details",
            invalid_files.len() - INVALID_FILE_SAMPLE_LIMIT
        ));
    }
    output.stdout("");
}

fn render_active_roles(output: &mut OutputCounter, manifest: &Path, base_branch: &str) {
    let Ok(content) = std::fs::read_to_string(manifest) else {
        output.stdout("Active roles: unavailable (manifest not found)");
        output.stdout("Choose a role from AGENTS.md or agf/agents/MANIFEST.yaml.");
        output.stdout("");
        return;
    };
    let Ok(data) = serde_norway::from_str::<serde_json::Value>(&content) else {
        output.stdout("Active roles: unavailable (manifest not found)");
        output.stdout("Choose a role from AGENTS.md or agf/agents/MANIFEST.yaml.");
        output.stdout("");
        return;
    };
    let Some(agents) = data.get("agents").and_then(|value| value.as_object()) else {
        output.stdout("Active roles: unavailable (manifest not found)");
        output.stdout("Choose a role from AGENTS.md or agf/agents/MANIFEST.yaml.");
        output.stdout("");
        return;
    };

    let mut roles: Vec<&str> = agents
        .iter()
        .filter(|(_role, attrs)| {
            attrs
                .get("status")
                .and_then(|value| value.as_str())
                .is_some_and(|status| status == "active")
        })
        .map(|(role, _attrs)| role.as_str())
        .collect();
    roles.sort();

    if roles.is_empty() {
        output.stdout("Active roles: none");
    } else {
        output.stdout("Active roles:");
        for role in roles {
            output.stdout(format!(
                "  - {role}  -> agov session resume --role {role} --base-branch {base_branch}"
            ));
        }
    }
    output.stdout("");
}

fn render_pending_handoffs_for_role(
    output: &mut OutputCounter,
    role: &str,
    messages: &[HandoffMessage],
) {
    let mut pending: Vec<HandoffMessage> = pending_handoffs(messages)
        .into_iter()
        .filter(|message| string_field(&message.raw, "to") == Some(role))
        .collect();
    pending.sort_by_key(|message| message.sort_key.clone());

    if pending.is_empty() {
        output.stdout(format!("Pending handoffs for {role}: none"));
    } else {
        output.stdout(format!("Pending handoffs for {role}:"));
        for message in pending.iter().rev() {
            output.stdout(format_pending_handoff(message));
        }
    }
    output.stdout("");
}

fn render_pending_handoffs_by_role(output: &mut OutputCounter, messages: &[HandoffMessage]) {
    let mut grouped: BTreeMap<String, Vec<HandoffMessage>> = BTreeMap::new();
    for message in pending_handoffs(messages) {
        let role = string_field(&message.raw, "to").unwrap_or("unknown");
        grouped.entry(role.to_string()).or_default().push(message);
    }

    if grouped.is_empty() {
        output.stdout("Pending handoffs by role: none");
    } else {
        output.stdout("Pending handoffs by role:");
        for (role, mut items) in grouped {
            output.stdout(format!("  {role}:"));
            items.sort_by_key(|message| message.sort_key.clone());
            for message in items.iter().rev() {
                output.stdout(format!("    - {}", pending_handoff_label(message)));
            }
        }
    }
    output.stdout("");
}

fn format_pending_handoff(message: &HandoffMessage) -> String {
    format!("  - {}", pending_handoff_label(message))
}

fn pending_handoff_label(message: &HandoffMessage) -> String {
    format!(
        "{}  {}  [{}]",
        display_id(&message.raw),
        string_field(&message.raw, "topic").unwrap_or(""),
        message
            .raw
            .get("handoff")
            .and_then(|value| value.get("status"))
            .and_then(|value| value.as_str())
            .unwrap_or("ready")
    )
}

fn render_claims_for_role(
    output: &mut OutputCounter,
    role: &str,
    messages: &[HandoffMessage],
    aliases: &BTreeMap<String, String>,
) {
    let resolved_role = resolve_alias(role, aliases);
    let now = time::OffsetDateTime::now_utc();
    let claims = match claim_states(messages, aliases) {
        Ok(claims) => claims,
        Err(errors) => {
            output.stdout(format!("Open claims for {role}: none"));
            output.stdout("");
            output.stdout(format!("Stale claims for {role}: none"));
            for error in errors {
                output.stdout(format!(
                    "  - invalid expires_on in {}: {}",
                    error.file, error.value
                ));
            }
            output.stdout("");
            return;
        }
    };

    let mut active = Vec::new();
    let mut stale = Vec::new();
    for claim in claims {
        if resolve_alias(&claim.from, aliases) != resolved_role {
            continue;
        }
        if claim
            .expires_on
            .map(|expires| expires < now)
            .unwrap_or(false)
        {
            stale.push(claim);
        } else {
            active.push(claim);
        }
    }

    active.sort_by_key(claim_sort_key);
    stale.sort_by_key(claim_sort_key);
    render_claim_list(output, &format!("Open claims for {role}:"), &active);
    render_claim_list(output, &format!("Stale claims for {role}:"), &stale);
}

fn claim_sort_key(claim: &ClaimEntry) -> (String, String) {
    (
        claim.claimed_at.clone(),
        claim
            .id
            .clone()
            .unwrap_or_else(|| claim.seq.map(|seq| seq.to_string()).unwrap_or_default()),
    )
}

fn render_claim_list(output: &mut OutputCounter, title: &str, claims: &[ClaimEntry]) {
    if claims.is_empty() {
        output.stdout(format!("{title} none"));
    } else {
        output.stdout(title);
        for claim in claims.iter().rev() {
            let expires = claim
                .expires_on
                .and_then(|expires| expires.format(&Rfc3339).ok())
                .map(|expires| format!("  expires {expires}"))
                .unwrap_or_default();
            output.stdout(format!(
                "  - {}  {}{}",
                claim.path,
                claim_display_label(claim),
                expires
            ));
        }
    }
    output.stdout("");
}

fn claim_display_label(claim: &ClaimEntry) -> String {
    if let Some(id) = &claim.id {
        format!("id {}", id.chars().take(8).collect::<String>())
    } else if let Some(seq) = claim.seq {
        format!("seq {seq}")
    } else {
        "id <unknown>".to_string()
    }
}

fn render_recent_decisions(output: &mut OutputCounter, messages: &[HandoffMessage]) {
    let mut decisions: Vec<&HandoffMessage> = messages
        .iter()
        .filter(|message| string_field(&message.raw, "kind") == Some("decision-record"))
        .collect();
    decisions.sort_by_key(|message| message.sort_key.clone());

    if decisions.is_empty() {
        output.stdout("Recent decisions: none");
    } else {
        output.stdout("Recent decisions:");
        for decision in decisions.iter().rev().take(5) {
            output.stdout(format!(
                "  - {}  {}",
                display_id(&decision.raw),
                string_field(&decision.raw, "topic").unwrap_or("")
            ));
        }
    }
    output.stdout("");
}

fn render_must_read(output: &mut OutputCounter, role: &str, messages: &[HandoffMessage]) {
    let mut items = Vec::new();
    for handoff in pending_handoffs(messages)
        .iter()
        .filter(|message| string_field(&message.raw, "to") == Some(role))
    {
        if let Some(handoff_data) = handoff
            .raw
            .get("handoff")
            .and_then(|value| value.as_object())
        {
            for item in handoff_data
                .get("must_read")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
                .filter_map(|value| value.as_str())
            {
                if !items.contains(&item.to_string()) {
                    items.push(item.to_string());
                }
            }
        }
    }

    if items.is_empty() {
        output.stdout("Must read: none");
    } else {
        output.stdout("Must read:");
        for item in items {
            output.stdout(format!("  - {item}"));
        }
    }
}

fn handoff_next(agent_role: &str, dir: PathBuf, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!(
                "Coordination directory not found: {}",
                dir.display()
            ));
            return 1;
        }
    };

    let pending = pending_handoffs(&messages);
    let next = pending
        .into_iter()
        .filter(|message| string_field(&message.raw, "to") == Some(agent_role))
        .max_by_key(|message| message.sort_key.clone());

    let Some(message) = next else {
        output.stdout(format!("No handoff found for role: {agent_role}"));
        return 2;
    };

    render_handoff_summary(&mut output, &message);
    agov_core::roi::record_tool_output(
        "agov.handoff.next",
        args,
        &format!("next-handoff:{agent_role}"),
        output.chars,
    );
    0
}

fn handoff_take(id: &str, from: &str, dir: PathBuf, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);
    let Some(handoff) = find_pending_handoff(&dir, id, &mut output) else {
        return 1;
    };

    let expected = handoff
        .raw
        .get("handoff")
        .and_then(|value| value.get("next_agent"))
        .and_then(|value| value.as_str())
        .or_else(|| string_field(&handoff.raw, "to"))
        .unwrap_or("");
    if expected != from {
        output.stderr(format!(
            "WARN from role '{from}' differs from handoff next_agent '{expected}'"
        ));
    }

    let draft = CoordDraft {
        id_format: "readable".to_string(),
        from: from.to_string(),
        to: string_field(&handoff.raw, "from")
            .unwrap_or("all")
            .to_string(),
        kind: "ack".to_string(),
        topic: string_field(&handoff.raw, "topic")
            .unwrap_or("handoff-take")
            .to_string(),
        refs: vec![handoff.file.clone()],
        artifacts: Vec::new(),
        tasks: Vec::new(),
        model: std::env::var("AGOV_MODEL")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        dir,
        body: format!("# ACK\n\nTaking handoff `{}`.\n", handoff.file),
    };

    match write_coord_draft(draft) {
        Ok(path) => {
            output.stdout(format!("Wrote {}", path.display()));
            agov_core::roi::record_tool_output(
                "agov.handoff.take",
                args,
                "handoff-take",
                output.chars,
            );
            0
        }
        Err(message) => {
            output.stderr(format!("ERR  {message}"));
            1
        }
    }
}

fn handoff_complete(
    id: &str,
    from: &str,
    to: Option<String>,
    dir: PathBuf,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);
    let Some(handoff) = find_pending_handoff(&dir, id, &mut output) else {
        return 1;
    };

    let draft = CoordDraft {
        id_format: "readable".to_string(),
        from: from.to_string(),
        to: to.unwrap_or_else(|| "all".to_string()),
        kind: "completion".to_string(),
        topic: string_field(&handoff.raw, "topic")
            .unwrap_or("handoff-complete")
            .to_string(),
        refs: vec![handoff.file.clone()],
        artifacts: Vec::new(),
        tasks: Vec::new(),
        model: std::env::var("AGOV_MODEL")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        dir,
        body: format!("# Completion\n\nCompleted handoff `{}`.\n", handoff.file),
    };

    match write_coord_draft(draft) {
        Ok(path) => {
            output.stdout(format!("Wrote {}", path.display()));
            agov_core::roi::record_tool_output(
                "agov.handoff.complete",
                args,
                "handoff-complete",
                output.chars,
            );
            0
        }
        Err(message) => {
            output.stderr(format!("ERR  {message}"));
            1
        }
    }
}

fn find_pending_handoff(
    dir: &Path,
    id: &str,
    output: &mut OutputCounter,
) -> Option<HandoffMessage> {
    let messages = match read_valid_coord_messages(dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!(
                "ERR  coordination directory not found: {}",
                dir.display()
            ));
            return None;
        }
    };

    let normalized = normalize_coord_reference(id).unwrap_or_else(|| id.to_string());
    let stem = normalized.strip_suffix(".md").unwrap_or(&normalized);
    let handoff = pending_handoffs(&messages).into_iter().find(|message| {
        message.file == normalized
            || string_field(&message.raw, "id") == Some(id)
            || string_field(&message.raw, "id") == Some(stem)
    });

    if handoff.is_none() {
        output.stderr(format!("ERR  pending handoff not found: {id}"));
    }
    handoff
}

fn read_valid_coord_messages(dir: &Path) -> Result<Vec<HandoffMessage>, std::io::Error> {
    read_coord_messages_with_invalid(dir).map(|(messages, _invalid)| messages)
}

fn read_coord_messages_with_invalid(
    dir: &Path,
) -> Result<(Vec<HandoffMessage>, Vec<String>), std::io::Error> {
    let mut messages = Vec::new();
    let mut invalid_files = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        let Ok(message) = agov_coord::coord_validator::validate_file(&path) else {
            invalid_files.push(file_name.to_string());
            continue;
        };

        let timestamp = string_field(&message.raw, "timestamp")
            .unwrap_or("")
            .to_string();
        let file = file_name.to_string();
        let refs = message_refs(&path, &message.raw);
        messages.push(HandoffMessage {
            sort_key: (timestamp, file.clone()),
            file,
            refs,
            raw: message.raw,
        });
    }

    messages.sort_by_key(|message| message.sort_key.clone());
    invalid_files.sort();
    Ok((messages, invalid_files))
}

fn pending_handoffs(messages: &[HandoffMessage]) -> Vec<HandoffMessage> {
    let descendants_by_file = descendants_by_file(messages);
    let messages_by_file: BTreeMap<&str, &HandoffMessage> = messages
        .iter()
        .map(|message| (message.file.as_str(), message))
        .collect();

    messages
        .iter()
        .filter(|message| string_field(&message.raw, "kind") == Some("handoff"))
        .filter(|message| !handoff_resolved(message, &descendants_by_file, &messages_by_file))
        .cloned()
        .collect()
}

fn handoff_resolved(
    handoff: &HandoffMessage,
    descendants_by_file: &BTreeMap<String, Vec<String>>,
    messages_by_file: &BTreeMap<&str, &HandoffMessage>,
) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut stack = vec![handoff.file.clone()];

    while let Some(file) = stack.pop() {
        if !seen.insert(file.clone()) {
            continue;
        }

        for descendant in descendants_by_file.get(&file).into_iter().flatten() {
            if let Some(message) = messages_by_file.get(descendant.as_str()) {
                if message.sort_key > handoff.sort_key
                    && matches!(
                        string_field(&message.raw, "kind"),
                        Some("ack" | "completion" | "decision-record")
                    )
                {
                    return true;
                }
                stack.push(descendant.clone());
            }
        }
    }

    false
}

fn descendants_by_file(messages: &[HandoffMessage]) -> BTreeMap<String, Vec<String>> {
    let messages_by_file: BTreeMap<&str, &HandoffMessage> = messages
        .iter()
        .map(|message| (message.file.as_str(), message))
        .collect();
    let mut descendants: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for message in messages {
        for referenced in &message.refs {
            let Some(referenced_message) = messages_by_file.get(referenced.as_str()) else {
                continue;
            };
            if message.sort_key <= referenced_message.sort_key {
                continue;
            }

            descendants
                .entry(referenced.clone())
                .or_default()
                .push(message.file.clone());
        }
    }

    descendants
}

fn message_refs(path: &Path, raw: &serde_json::Value) -> Vec<String> {
    let mut refs = frontmatter_refs(raw);
    refs.extend(body_refs(path));
    refs.sort();
    refs.dedup();
    refs
}

fn frontmatter_refs(raw: &serde_json::Value) -> Vec<String> {
    raw.get("references-to")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .filter_map(normalize_coord_reference)
        .collect()
}

fn normalize_coord_reference(reference: &str) -> Option<String> {
    let filename = Path::new(reference).file_name()?.to_str()?;

    if filename.ends_with(".md") {
        if looks_like_coord_filename(filename) {
            Some(filename.to_string())
        } else {
            None
        }
    } else {
        let candidate = format!("{filename}.md");
        if looks_like_coord_filename(&candidate) {
            Some(candidate)
        } else {
            None
        }
    }
}

fn body_refs(path: &Path) -> Vec<String> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let body = content.splitn(3, "---").nth(2).unwrap_or(content.as_str());

    body.split(|ch: char| ch.is_whitespace() || matches!(ch, '`' | '"' | '\'' | '(' | ')' | ','))
        .filter_map(clean_coord_filename)
        .collect()
}

fn clean_coord_filename(token: &str) -> Option<String> {
    let filename = token
        .trim_matches(|ch: char| matches!(ch, '.' | ':' | ';' | ']' | '[' | ')' | '('))
        .rsplit('/')
        .next()?;

    if filename.ends_with(".md") && looks_like_coord_filename(filename) {
        Some(filename.to_string())
    } else {
        None
    }
}

fn looks_like_coord_filename(filename: &str) -> bool {
    let stem = filename.strip_suffix(".md").unwrap_or(filename);
    let first = stem.split('-').next().unwrap_or("");

    first.len() == 5 && first.chars().all(|ch| ch.is_ascii_digit())
        || first.len() == 26
            && first
                .chars()
                .all(|ch| matches!(ch, '0'..='9' | 'A'..='H' | 'J'..='K' | 'M'..='N' | 'P'..='T' | 'V'..='Z'))
        || stem.len() >= 17
            && stem.as_bytes().get(8) == Some(&b'T')
            && stem.as_bytes().get(15) == Some(&b'Z')
}

fn render_handoff_summary(output: &mut OutputCounter, message: &HandoffMessage) {
    let handoff = message
        .raw
        .get("handoff")
        .and_then(|value| value.as_object());
    let next_agent = handoff
        .and_then(|value| value.get("next_agent"))
        .and_then(|value| value.as_str())
        .unwrap_or_else(|| string_field(&message.raw, "to").unwrap_or("unknown"));
    let status = handoff
        .and_then(|value| value.get("status"))
        .and_then(|value| value.as_str())
        .unwrap_or("ready");

    output.stdout(format!("Handoff for: {next_agent}"));
    output.stdout(format!("Status:      {status}"));
    output.stdout("Task:");
    render_string_list(output, handoff, "task");
    render_titled_string_list(output, handoff, "Must read:", "must_read");
    render_titled_string_list(output, handoff, "Do not touch:", "do_not_touch");
    output.stdout(format!("Source: {}", message.file));
}

fn render_titled_string_list(
    output: &mut OutputCounter,
    handoff: Option<&serde_json::Map<String, serde_json::Value>>,
    title: &str,
    field: &str,
) {
    let Some(items) = handoff
        .and_then(|value| value.get(field))
        .and_then(|value| value.as_array())
        .filter(|items| !items.is_empty())
    else {
        return;
    };

    output.stdout(title);
    for item in items.iter().filter_map(|item| item.as_str()) {
        output.stdout(format!("  - {item}"));
    }
}

fn render_string_list(
    output: &mut OutputCounter,
    handoff: Option<&serde_json::Map<String, serde_json::Value>>,
    field: &str,
) {
    if let Some(items) = handoff
        .and_then(|value| value.get(field))
        .and_then(|value| value.as_array())
    {
        for item in items.iter().filter_map(|item| item.as_str()) {
            output.stdout(format!("  - {item}"));
        }
    }
}

fn string_field<'a>(raw: &'a serde_json::Value, field: &str) -> Option<&'a str> {
    raw.get(field).and_then(|value| value.as_str())
}

fn validate_coord_files(files: Vec<std::path::PathBuf>, args: &[String]) -> i32 {
    let mut failed = false;
    let mut output = OutputCounter::default();
    let mut valid_messages = Vec::new();
    let mut valid_paths = Vec::new();

    for file in files {
        let label = file.to_string_lossy().to_string();
        match agov_coord::coord_validator::validate_file(&file) {
            Ok(data) => {
                for message in
                    agov_coord::coord_validator::advisory_findings_for_path(&data, Some(&file))
                {
                    output.coord_warn(&label, &message);
                }
                output.coord_ok(&label);
                valid_messages.push((label, data));
                valid_paths.push(file);
            }
            Err(errors) => {
                failed = true;
                render_coord_errors(&mut output, &label, &errors);
            }
        }
    }

    for (path, message) in duplicate_id_errors(&valid_messages) {
        failed = true;
        output.coord_err(&path, &message);
    }

    for (path, message) in duplicate_id_errors_in_dirs(&valid_paths) {
        failed = true;
        output.coord_err(&path, &message);
    }

    if failed {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, "coord-validate", output.chars);
        0
    }
}

fn duplicate_id_errors(
    messages: &[(String, agov_coord::coord_validator::CoordMessage)],
) -> Vec<(String, String)> {
    let mut by_id: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (path, message) in messages {
        if let Some(id) = message.id() {
            by_id.entry(id.to_string()).or_default().push(path.clone());
        }
    }

    let mut errors = Vec::new();
    for (id, mut paths) in by_id {
        paths.sort();
        if let Some(first) = paths.first().cloned() {
            for path in paths.into_iter().skip(1) {
                errors.push((
                    path,
                    format!("duplicate coordination id '{id}' also used by {first}"),
                ));
            }
        }
    }

    errors
}

fn duplicate_id_errors_in_dirs(files: &[PathBuf]) -> Vec<(String, String)> {
    let target_paths: BTreeSet<PathBuf> = files
        .iter()
        .map(|path| normalize_path_for_compare(path))
        .collect();
    let mut by_id: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let mut dirs: BTreeSet<PathBuf> = BTreeSet::new();
    for file in files {
        if let Some(parent) = file.parent() {
            dirs.insert(parent.to_path_buf());
        }
    }

    for dir in dirs {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }
            let Ok(message) = agov_coord::coord_validator::validate_file(&path) else {
                continue;
            };
            let Some(id) = message.id() else {
                continue;
            };
            by_id
                .entry(id.to_string())
                .or_default()
                .push(path.to_string_lossy().to_string());
        }
    }

    let mut errors = Vec::new();
    for (id, mut paths) in by_id {
        paths.sort();
        paths.dedup();
        if paths.len() < 2 {
            continue;
        }
        for path in &paths {
            if target_paths.contains(&normalize_path_for_compare(Path::new(&path))) {
                let other = paths
                    .iter()
                    .find(|candidate| *candidate != path)
                    .cloned()
                    .unwrap_or_else(|| path.clone());
                errors.push((
                    path.clone(),
                    format!("duplicate coordination id '{id}' also used by {other}"),
                ));
            }
        }
    }

    errors
}

fn normalize_path_for_compare(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn validate_files<T>(
    files: Vec<std::path::PathBuf>,
    validator: FileValidator<T>,
    advisory_fun: Option<AdvisoryFn<T>>,
    args: &[String],
    workflow: &str,
) -> i32 {
    let mut failed = false;
    let mut output = OutputCounter::default();

    for file in files {
        let label = file.to_string_lossy().to_string();
        match validator(&file) {
            Ok(data) => {
                if let Some(advisory_fun) = advisory_fun {
                    for message in advisory_fun(&data) {
                        output.coord_warn(&label, &message);
                    }
                }
                output.coord_ok(&label);
            }
            Err(errors) => {
                failed = true;
                render_coord_errors(&mut output, &label, &errors);
            }
        }
    }

    if failed {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, workflow, output.chars);
        0
    }
}

fn render_coord_errors(output: &mut OutputCounter, path: &str, errors: &[(String, String)]) {
    if let [(message, location)] = errors {
        if location.is_empty() {
            output.coord_err(path, message);
            return;
        }
    }

    output.coord_err_schema(path, errors);
}

fn run_validate(sub: Option<ValidateCommand>, args: &[String]) -> i32 {
    let Some(sub) = sub else {
        return run_all_validators(args);
    };

    let mut output = OutputCounter::default();
    let (exit_code, workflow) = match sub {
        ValidateCommand::Adoption => (
            render_single_collection(
                &mut output,
                "agov.validate.adoption",
                agov_validators::adoption::validate_all(&agov_core::repo_root().join("agf")),
            ),
            "validate:adoption",
        ),
        ValidateCommand::AdopterDocs => (
            render_collection(
                &mut output,
                "agov.validate.adopter_docs",
                agov_validators::adopter_docs::validate_all(&agov_core::repo_root()),
            ),
            "validate:adopter-docs",
        ),
        ValidateCommand::Agents => (
            render_collection(
                &mut output,
                "agov.validate.agents",
                agov_validators::agents::validate_all(
                    &agov_core::repo_root().join("agf").join("agents"),
                ),
            ),
            "validate:agents",
        ),
        ValidateCommand::Learnings { paths } => {
            let result = if paths.is_empty() {
                agov_validators::learnings::validate_all(&agov_core::learnings_dir())
            } else {
                agov_validators::learnings::validate_paths(&paths)
            };
            (
                render_collection(&mut output, "agov.validate.learnings", result),
                "validate:learnings",
            )
        }
        ValidateCommand::Infosec => (
            render_collection(
                &mut output,
                "agov.validate.infosec",
                agov_validators::infosec::validate_all(&agov_core::learnings_dir()),
            ),
            "validate:infosec",
        ),
        ValidateCommand::Contributions => (
            render_warning_collection(
                &mut output,
                "agov.validate.contributions",
                agov_validators::contributions::validate_all(&agov_core::learnings_dir()),
            ),
            "validate:contributions",
        ),
        ValidateCommand::FrameworkConfig => {
            let root = agov_core::repo_root();
            if !root.join("agf").join("framework.yaml").exists() {
                output.stdout("OK agov.validate.framework_config: skipped");
                (0, "validate:framework-config")
            } else {
                (
                    render_single_collection(
                        &mut output,
                        "agov.validate.framework_config",
                        agov_validators::framework_config::validate_all(&root.join("agf")),
                    ),
                    "validate:framework-config",
                )
            }
        }
        ValidateCommand::ReadmeStats => (
            render_readme_stats(&mut output, &agov_core::repo_root()),
            "validate:readme-stats",
        ),
    };

    if exit_code == 0 {
        agov_core::roi::record_tool_output("agov", args, workflow, output.chars);
    }

    exit_code
}

fn run_all_validators(args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let results = [
        render_collection(
            &mut output,
            "agov.validate.learnings",
            agov_validators::learnings::validate_all(&agov_core::learnings_dir()),
        ),
        render_collection(
            &mut output,
            "agov.validate.agents",
            agov_validators::agents::validate_all(
                &agov_core::repo_root().join("agf").join("agents"),
            ),
        ),
        render_single_collection(
            &mut output,
            "agov.validate.adoption",
            agov_validators::adoption::validate_all(&agov_core::repo_root().join("agf")),
        ),
        render_collection(
            &mut output,
            "agov.validate.adopter_docs",
            agov_validators::adopter_docs::validate_all(&agov_core::repo_root()),
        ),
        render_collection(
            &mut output,
            "agov.validate.infosec",
            agov_validators::infosec::validate_all(&agov_core::learnings_dir()),
        ),
        render_warning_collection(
            &mut output,
            "agov.validate.contributions",
            agov_validators::contributions::validate_all(&agov_core::learnings_dir()),
        ),
        render_warning_collection(
            &mut output,
            "agov.validate.assistant_drift",
            agov_validators::assistant_drift::validate_all(&agov_core::repo_root()),
        ),
        render_warning_collection(
            &mut output,
            "agov.validate.assistant_loader",
            agov_validators::assistant_loader::validate_all(&agov_core::repo_root()),
        ),
        render_framework_config(&mut output),
        render_readme_stats(&mut output, &agov_core::repo_root()),
    ];

    output.stdout("OK agov.validate.coverage: framework checks complete; for complete governance coverage, also run `agov plan validate` and `agov coord validate`; the umbrella command does not validate those records");

    if results.contains(&1) {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, "validate:umbrella", output.chars);
        0
    }
}

fn render_framework_config(output: &mut OutputCounter) -> i32 {
    let root = agov_core::repo_root();
    if !root.join("agf").join("framework.yaml").exists() {
        output.stdout("OK agov.validate.framework_config: skipped");
        0
    } else {
        render_single_collection(
            output,
            "agov.validate.framework_config",
            agov_validators::framework_config::validate_all(&root.join("agf")),
        )
    }
}

fn render_readme_stats(output: &mut OutputCounter, root: &std::path::Path) -> i32 {
    let result = agov_validators::readme_stats::validate(root);
    let label = "agov.validate.readme_stats";
    if result.findings.is_empty() {
        output.stdout(format!("OK {label}"));
        0
    } else {
        for f in &result.findings {
            let loc = f.location.as_deref().unwrap_or("README.md");
            output.stdout(format!("WARN {label}: {} ({loc})", f.message));
        }
        0 // advisory only — never blocking
    }
}

fn render_collection(
    output: &mut OutputCounter,
    label: &str,
    result: agov_validators::CollectionResult,
) -> i32 {
    let has_blocking = result
        .file_errors
        .iter()
        .flat_map(|file_error| file_error.findings.iter())
        .any(|finding| finding.severity == agov_core::Severity::Blocking);

    if !has_blocking {
        for file in result.ok_files {
            output.validate_ok(&file);
        }
        for file_error in result.file_errors {
            for finding in file_error.findings {
                let location = finding.location.as_deref().unwrap_or(&file_error.file);
                output.validate_warn(&file_error.file, location, &finding.message);
            }
        }
        output.validate_ok(label);
        return 0;
    }

    for file_error in result.file_errors {
        if file_error.findings.is_empty() {
            output.validate_err_label(&file_error.file);
        }

        for finding in file_error.findings {
            let location = finding.location.as_deref().unwrap_or(&file_error.file);
            match finding.severity {
                agov_core::Severity::Blocking => {
                    output.validate_err(label, location, &finding.message);
                }
                agov_core::Severity::Advisory => {
                    output.validate_warn(&file_error.file, location, &finding.message);
                }
            }
        }
    }

    1
}

fn render_single_collection(
    output: &mut OutputCounter,
    label: &str,
    result: agov_validators::CollectionResult,
) -> i32 {
    if result.is_ok() {
        output.validate_ok(label);
        return 0;
    }

    for file_error in result.file_errors {
        for finding in file_error.findings {
            let location = finding.location.as_deref().unwrap_or(&file_error.file);
            output.validate_err(label, location, &finding.message);
        }
    }

    1
}

fn render_warning_collection(
    output: &mut OutputCounter,
    label: &str,
    result: agov_validators::CollectionResult,
) -> i32 {
    for file_error in result.file_errors {
        for finding in file_error.findings {
            let location = finding.location.as_deref().unwrap_or(&file_error.file);
            output.validate_warn(&file_error.file, location, &finding.message);
        }
    }

    output.validate_ok(label);
    0
}

#[derive(Default)]
struct OutputCounter {
    chars: u64,
}

impl OutputCounter {
    fn stdout(&mut self, line: impl AsRef<str>) {
        let line = line.as_ref();
        println!("{line}");
        self.count_line(line);
    }

    fn stderr(&mut self, line: impl AsRef<str>) {
        let line = line.as_ref();
        eprintln!("{line}");
        self.count_line(line);
    }

    fn coord_ok(&mut self, path: &str) {
        self.stdout(format!("OK  severity: blocking {path}"));
    }

    fn coord_warn(&mut self, path: &str, msg: &str) {
        self.stdout(format!("WARN severity: advisory {path}: {msg}"));
    }

    fn coord_err(&mut self, path: &str, msg: &str) {
        self.stderr(format!("ERR severity: blocking {path}: {msg}"));
    }

    fn coord_err_schema(&mut self, path: &str, errors: &[(String, String)]) {
        self.stderr(format!("ERR severity: blocking {path}:"));
        for (msg, ptr) in errors {
            self.stderr(format!("    {ptr}: {msg}"));
        }
    }

    fn validate_ok(&mut self, label: &str) {
        self.stdout(format!("OK {label}"));
    }

    fn validate_warn(&mut self, label: &str, location: &str, message: &str) {
        self.stdout(format!("WARN {label}: {location}: {message}"));
    }

    fn validate_err(&mut self, label: &str, location: &str, message: &str) {
        self.stderr(format!("ERR {label}: {location}: {message}"));
    }

    fn validate_err_label(&mut self, label: &str) {
        self.stderr(format!("ERR {label}"));
    }

    fn count_line(&mut self, line: &str) {
        self.chars += line.chars().count() as u64 + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(file: &str, kind: &str, timestamp: &str, refs: &[&str]) -> HandoffMessage {
        HandoffMessage {
            file: file.to_string(),
            sort_key: (timestamp.to_string(), file.to_string()),
            refs: refs.iter().map(|value| value.to_string()).collect(),
            raw: serde_json::json!({
                "kind": kind,
                "from": "coder-tester",
                "to": "governor-reviewer",
                "topic": "test",
                "timestamp": timestamp,
                "id": file.trim_end_matches(".md"),
                "handoff": {
                    "status": "ready"
                }
            }),
        }
    }

    #[test]
    fn pending_handoffs_follow_transitive_resolution_refs() {
        let handoff = message(
            "00001-coder-tester.md",
            "handoff",
            "2026-06-21T00:00:00Z",
            &[],
        );
        let note = message(
            "00002-planner-architect.md",
            "note",
            "2026-06-21T00:01:00Z",
            &["00001-coder-tester.md"],
        );
        let completion = message(
            "00003-coder-tester.md",
            "completion",
            "2026-06-21T00:02:00Z",
            &["00002-planner-architect.md"],
        );

        let pending = pending_handoffs(&[handoff, note, completion]);

        assert!(pending.is_empty());
    }

    #[test]
    fn frontmatter_refs_normalize_bare_readable_ids_to_filenames() {
        let raw = serde_json::json!({
            "references-to": [
                "20260622T202034Z-governor-reviewer-8ma3",
                "docs/coordination/20260623T051041Z-coder-tester-p4ph.md",
                "not-a-coordination-id"
            ]
        });

        assert_eq!(
            frontmatter_refs(&raw),
            vec![
                "20260622T202034Z-governor-reviewer-8ma3.md".to_string(),
                "20260623T051041Z-coder-tester-p4ph.md".to_string()
            ]
        );
    }

    #[test]
    fn pending_handoffs_resolve_with_bare_frontmatter_refs() {
        let handoff = message(
            "20260622T202119Z-governor-reviewer-idra.md",
            "handoff",
            "2026-06-22T20:21:19Z",
            &[],
        );
        let raw = serde_json::json!({
            "references-to": ["20260622T202119Z-governor-reviewer-idra"]
        });
        let resolution = message(
            "20260623T051032Z-governor-reviewer-g1ap.md",
            "decision-record",
            "2026-06-23T05:10:32Z",
            &frontmatter_refs(&raw)
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        );

        let pending = pending_handoffs(&[handoff, resolution]);

        assert!(pending.is_empty());
    }

    #[test]
    fn note_descendants_do_not_resolve_handoffs() {
        let handoff = message(
            "00001-coder-tester.md",
            "handoff",
            "2026-06-21T00:00:00Z",
            &[],
        );
        let note = message(
            "00002-planner-architect.md",
            "note",
            "2026-06-21T00:01:00Z",
            &["00001-coder-tester.md"],
        );

        let pending = pending_handoffs(&[handoff.clone(), note]);

        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].file, handoff.file);
    }

    fn claim_message(
        file: &str,
        from: &str,
        seq: i64,
        path: &str,
        expires_on: Option<&str>,
    ) -> HandoffMessage {
        let mut raw = serde_json::json!({
            "kind": "claim",
            "from": from,
            "to": "all",
            "topic": "test",
            "timestamp": "2026-01-01T00:00:00Z",
            "seq": seq,
            "id": file.trim_end_matches(".md"),
            "artifacts_owned": [path],
        });
        if let Some(expires_on) = expires_on {
            raw["expires_on"] = serde_json::Value::String(expires_on.to_string());
        }

        HandoffMessage {
            file: file.to_string(),
            sort_key: ("2026-01-01T00:00:00Z".to_string(), file.to_string()),
            refs: Vec::new(),
            raw,
        }
    }

    fn completion_message(file: &str, from: &str, paths: Vec<&str>) -> HandoffMessage {
        HandoffMessage {
            file: file.to_string(),
            sort_key: ("2026-01-01T00:01:00Z".to_string(), file.to_string()),
            refs: Vec::new(),
            raw: serde_json::json!({
                "kind": "completion",
                "from": from,
                "to": "all",
                "topic": "test",
                "timestamp": "2026-01-01T00:01:00Z",
                "id": file.trim_end_matches(".md"),
                "artifacts_owned": paths,
            }),
        }
    }

    #[test]
    fn stale_claims_reports_expired_unreleased_claims() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let claim = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/stale.md",
            Some("2025-01-01T00:00:00Z"),
        );

        let stale = stale_claims(&[claim], &BTreeMap::new(), now).unwrap();

        assert_eq!(stale.len(), 1);
        assert_eq!(stale[0].path, "docs/stale.md");
        assert_eq!(stale[0].seq, Some(10));
    }

    #[test]
    fn stale_claims_ignores_active_and_non_expiring_claims() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let active = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/active.md",
            Some("2099-01-01T00:00:00Z"),
        );
        let no_expiry = claim_message(
            "00011-coder-tester.md",
            "coder-tester",
            11,
            "docs/no-expiry.md",
            None,
        );

        let stale = stale_claims(&[active, no_expiry], &BTreeMap::new(), now).unwrap();

        assert!(stale.is_empty());
    }

    #[test]
    fn stale_claims_excludes_released_claims() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let claim = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/stale.md",
            Some("2025-01-01T00:00:00Z"),
        );
        let completion = completion_message("00011-coder-tester.md", "coder-tester", vec![]);

        let stale = stale_claims(&[claim, completion], &BTreeMap::new(), now).unwrap();

        assert!(stale.is_empty());
    }

    #[test]
    fn stale_claims_rejects_invalid_expires_on() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let claim = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/stale.md",
            Some("not-a-date"),
        );

        let errors = stale_claims(&[claim], &BTreeMap::new(), now).unwrap_err();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].file, "00010-coder-tester.md");
        assert_eq!(errors[0].value, "not-a-date");
    }
}
