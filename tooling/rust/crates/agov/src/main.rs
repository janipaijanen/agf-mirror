mod artifact;
mod ci_cache_topology;
mod commercial_corpus;
mod dist;
mod handoff_branch;
mod hold_status;
mod plan_collision;
mod provenance;
mod release_namespace;

use agov_coord::coord_validator::{looks_like_coord_filename, normalize_coord_reference};
use agov_coord::readable_id::{is_readable_id, iso_timestamp, readable_id};
use clap::{Parser, Subcommand};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use time::format_description::well_known::Rfc3339;

type ValidationErrors = Vec<(String, String)>;
type FileValidator<T> = fn(&std::path::Path) -> Result<T, ValidationErrors>;
type AdvisoryFn<T> = fn(&T) -> Vec<String>;

// Kept as one contiguous string so release CI can verify provenance in the
// exact cross-compiled bytes without executing a foreign-platform binary.
#[used]
static BUILD_PROVENANCE_MARKER: &str = concat!(
    "AGOV_BUILD_PROVENANCE_V1|",
    env!("CARGO_PKG_VERSION"),
    "|",
    env!("AGOV_GIT_SHA"),
    "|",
    env!("AGOV_GIT_DIRTY"),
    "|",
    env!("AGOV_SOURCE_DIRTY")
);

#[derive(Parser)]
#[command(name = "agov", about = "AGF governance tooling", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generic project-owned artifact validation and indexing.
    Artifact {
        #[command(subcommand)]
        sub: artifact::ArtifactCommand,
    },
    /// Read-only release preflight checks.
    Release {
        #[command(subcommand)]
        sub: release_namespace::ReleaseCommand,
    },
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
    /// Mechanical gate-change checks.
    Gate {
        #[command(subcommand)]
        sub: GateCommand,
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
    /// Restricted commercial-corpus manifest commands.
    CommercialCorpus {
        #[command(subcommand)]
        sub: CommercialCorpusCommand,
    },
    /// Coordination usage reporting commands.
    Usage {
        #[command(subcommand)]
        sub: UsageCommand,
    },
    /// CI pipeline configuration checks.
    Ci {
        #[command(subcommand)]
        sub: CiCommand,
    },
}

#[derive(Subcommand)]
enum CiCommand {
    /// Statically validate the shared Rust CI cache writer topology against
    /// GitLab CI Lint's merged YAML (extends/anchors already resolved).
    CacheTopologyCheck {
        /// Path to a file containing the merged YAML text (the
        /// `merged_yaml` field of a `POST /projects/:id/ci/lint` response
        /// with `include_jobs=true`), or the full JSON response itself.
        #[arg(long)]
        merged_yaml: PathBuf,
        /// The shared cache key that identifies the writer/consumer graph
        /// under inspection.
        #[arg(long, default_value = "rust-v3-main")]
        shared_key: String,
        /// The single job name authorized to hold the writer role.
        #[arg(long, default_value = "rust:tooling:test")]
        writer_job: String,
        /// CI variable name the writer job's cache.policy must reference.
        #[arg(long, default_value = "RUST_CACHE_WRITER_POLICY")]
        writer_policy_var: String,
        /// CI variable name every consumer job's cache.policy must reference.
        #[arg(long, default_value = "RUST_CACHE_POLICY")]
        consumer_policy_var: String,
        /// "text" prints the OK/ERR summary. "dot" prints the resolved
        /// writer/consumer graph as Graphviz DOT to stdout instead --
        /// written regardless of pass/fail, so a second writer, an
        /// unrecognised role, or a wrong policy variable are visible
        /// directly in the graph shape. ERR lines from a failing check
        /// still go to stderr in either format.
        #[arg(long, default_value = "text", value_parser = ["text", "dot"])]
        format: String,
    },
}

#[derive(Subcommand)]
enum GateCommand {
    /// Require declared test accompaniment and preserve active gate paths and gate IDs.
    ChangeCheck {
        /// Git ref used as the comparison base.
        #[arg(long)]
        base_ref: String,
        /// Repository-relative gate registry path.
        #[arg(long, default_value = ".agf/gates.yaml")]
        registry: PathBuf,
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
        /// Annotate handoffs addressed to another session; never filters or reorders.
        #[arg(long)]
        session: Option<String>,
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
    /// Report declarations from explicit pinned local Git sources; never permits delay.
    HoldStatus {
        #[arg(long)]
        target_branch: String,
        #[arg(long, default_value = "docs/coordination")]
        dir: String,
        #[arg(long)]
        candidate_ref: Vec<String>,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// Mechanically verify a coordination-only publication candidate against
    /// its target ref, per coordination-publication-rule.md: every commit in
    /// the range (not only the tip diff) must be a pure addition under the
    /// coordination directory, with no merge/imported history.
    PublicationCheck {
        #[arg(long)]
        base_ref: String,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        /// Required subject prefix every commit in the range must use
        /// (condition 6 of coordination-publication-rule.md).
        #[arg(long, default_value = "coordination:")]
        subject_prefix: String,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// List pending (unresolved) requires-ack:true records of any kind other
    /// than handoff (owned exclusively by `handoff next/take/complete`) and
    /// ack. A record is resolved when any later record's references-to
    /// lists its file -- direct reference, not handoff's transitive
    /// descendant-chain resolution. Records older than the cutoff are
    /// grandfathered and never appear. Issue #80,
    /// docs/plans/v250-rc1-contract-core.md.
    Pending {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        /// RFC3339 timestamp; only records at or after this cutoff are
        /// considered. Defaults to the governed introduction boundary set
        /// by governor-reviewer decision-record
        /// `20260802T220154Z-governor-reviewer-sejb` (amending
        /// docs/plans/v250-rc1-contract-core.md decision (c)): the cutoff
        /// is that decision-record's own timestamp, not a future
        /// merge-commit date (unimplementable pre-merge, per Gate 2 review
        /// `20260802T193425Z-contribution-auditor-rtga.md` finding B2 and
        /// its renewed disposition in
        /// `20260802T204834Z-contribution-auditor-ah7j.md`). This value is
        /// later than every pre-existing non-handoff requires-ack:true
        /// record at decision time, so it grandfathers that full set
        /// rather than manufacturing a surprise pending backlog.
        /// `--cutoff` always overrides this default explicitly.
        #[arg(long, default_value = "2026-08-02T22:01:54Z")]
        cutoff: String,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// Report the deterministic state of one opt-in review-convergence series.
    Convergence {
        /// Coordination id that opened the convergence series.
        #[arg(long)]
        series: String,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
}

#[derive(Subcommand)]
enum PlanCommand {
    /// Report possible plan/ref overlaps; advisory, never a gate decision.
    CollisionCheck {
        #[arg(long, default_value = "main")]
        base_ref: String,
        /// Query GitLab through configured glab authentication (CLI only).
        #[arg(long)]
        include_forge: bool,
    },
    /// Validate one or more Gate 1 plan files.
    Validate {
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
    },
    /// Validate a structured six-surface rule-projection evidence manifest
    /// (docs/reviews/rule-projection/<plan-id>.yaml), per
    /// documentation-rule-projection-rule.md.
    ProjectionCheck {
        /// Comparison base used to verify that every changed Pages/runbook
        /// file is declared by the manifest.
        #[arg(long)]
        base_ref: String,
        /// Allow an absent normative-owner path only when that manifest row
        /// explicitly carries `stage: planned`. Repository escapes and stale
        /// planned markers remain blocking.
        #[arg(long)]
        allow_planned: bool,
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
    },
    /// Validate every rule-projection manifest's paths and lifecycle state,
    /// plus diff-scoped Pages/runbook coverage for changed manifests.
    ProjectionCheckAll {
        #[arg(long)]
        base_ref: String,
        /// Repository whose Git history, manifests, schema, distribution, and
        /// declared paths must be used. Defaults to the current repository.
        #[arg(long)]
        repo_root: Option<PathBuf>,
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
    /// Show the next unresolved handoff; reply evidence does not confirm acknowledgement.
    Next {
        /// Add advisory pending handoffs from local and remote-tracking refs.
        #[arg(long)]
        include_branch_local: bool,
        /// Annotate handoffs addressed to another session; never filters or reorders.
        #[arg(long)]
        session: Option<String>,
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
    /// Report accumulated project-local token ROI telemetry.
    Report {
        /// Log to read; overrides AGOV_TOKEN_ROI_LOG and the detected project default.
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
        /// Output format: human-readable text (default) or a stable
        /// machine-readable JSON array, one object per validated file:
        /// {"file", "assurance_level", "findings"}.
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// Extract a packaged executable from its release archive, run its own
    /// `version --format json` self-report, and cross-check it against a
    /// file-kind provenance record.
    SelfReportCheck {
        /// Path to the .tar.gz or .zip archive containing the executable.
        #[arg(long)]
        archive: PathBuf,
        /// The executable's filename inside the archive (e.g. "agov").
        #[arg(long)]
        binary_name: String,
        /// The provenance record for this archive.
        #[arg(long)]
        record: PathBuf,
    },
    /// Anonymously discover and download every asset linked from a real
    /// published GitLab Release, then validate the downloaded provenance
    /// records against the downloaded bytes -- not the CI workspace copy.
    /// This is the check that catches "provenance says X, published
    /// artifact is actually Y".
    PostPublicationCheck {
        /// GitLab Release API URL, e.g.
        /// https://gitlab.com/api/v4/projects/<id>/releases/<tag>
        #[arg(long)]
        releases_url: String,
        /// Local directory to download assets into (created if missing).
        #[arg(long)]
        download_dir: PathBuf,
    },
    /// Build a single release-provenance JSON record from real artifact
    /// evidence (file bytes/sha256, or OCI image/label identifiers).
    ProvenanceRecord {
        #[command(subcommand)]
        sub: Box<ProvenanceRecordCommand>,
    },
}

#[derive(Subcommand)]
enum CommercialCorpusCommand {
    /// Generate a closed, versioned manifest from a reviewed inventory.
    Generate {
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
        /// Exact commit containing the reviewed inventory CSV.
        #[arg(long)]
        inventory_ref: String,
        #[arg(
            long,
            default_value = "docs/reviews/commercial-licensing/20260817T224900Z-rights-provenance-inventory/inventory.csv"
        )]
        inventory_path: PathBuf,
        /// Exact pre-implementation commit whose content bytes are hashed.
        #[arg(long)]
        corpus_content_revision: String,
        #[arg(long, default_value = "LICENSES/commercial-corpus-manifest-v1.yaml")]
        output: PathBuf,
        #[arg(
            long,
            default_value = "LICENSES/commercial-corpus-manifest-current.yaml"
        )]
        pointer: PathBuf,
        #[arg(
            long,
            default_value = "agf/schemas/commercial-corpus-manifest.schema.json"
        )]
        schema: PathBuf,
    },
    /// Validate the current manifest. Schema defects block; drift is advisory.
    Validate {
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
        #[arg(
            long,
            default_value = "LICENSES/commercial-corpus-manifest-current.yaml"
        )]
        pointer: PathBuf,
        #[arg(
            long,
            default_value = "agf/schemas/commercial-corpus-manifest.schema.json"
        )]
        schema: PathBuf,
        /// Optional comparison base for advisory numbered-manifest mutation checks.
        #[arg(long)]
        base_ref: Option<String>,
    },
    /// Verify one historical manifest against its own recorded Git revision.
    VerifyVersion {
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
        #[arg(long, default_value = "LICENSES/commercial-corpus-manifest-v1.yaml")]
        manifest: PathBuf,
        #[arg(
            long,
            default_value = "agf/schemas/commercial-corpus-manifest.schema.json"
        )]
        schema: PathBuf,
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
    // `#[used]` retains the marker in compiler output, but a cross-linker may
    // still garbage-collect its section when no reachable code references it.
    // Keep the exact bytes reachable without changing version output.
    std::hint::black_box(BUILD_PROVENANCE_MARKER);

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
        Command::Artifact { sub } => artifact::run(sub, &args),
        Command::Release { sub } => release_namespace::run(sub),
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
                    session,
                    base_branch,
                    branch,
                    dir,
                    format,
                },
        } => coord_visibility(base_branch, branch, dir, format, session.as_deref(), &args),
        Command::Coord {
            sub: CoordCommand::Graph { dir, format },
        } => coord_graph(dir, format, &args),
        Command::Coord {
            sub: CoordCommand::GraphIndex { dir, output },
        } => coord_graph_index(dir, output, &args),
        Command::Coord {
            sub:
                CoordCommand::PublicationCheck {
                    base_ref,
                    dir,
                    subject_prefix,
                    format,
                },
        } => coord_publication_check(base_ref, dir, subject_prefix, format, &args),
        Command::Coord {
            sub:
                CoordCommand::HoldStatus {
                    target_branch,
                    dir,
                    candidate_ref,
                    format,
                },
        } => hold_status::run(&target_branch, &dir, &candidate_ref, &format, &args),
        Command::Coord {
            sub:
                CoordCommand::Pending {
                    dir,
                    cutoff,
                    format,
                },
        } => coord_pending(dir, &cutoff, &format, &args),
        Command::Coord {
            sub:
                CoordCommand::Convergence {
                    series,
                    dir,
                    format,
                },
        } => coord_convergence(&series, dir, &format, &args),
        Command::Plan {
            sub: PlanCommand::CollisionCheck { base_ref, include_forge },
        } => plan_collision::run(&base_ref, include_forge, &args),
        Command::Plan {
            sub: PlanCommand::Validate { files },
        } => validate_files(
            files,
            agov_coord::plan_validator::validate_file,
            None,
            &args,
            "plan-validate",
        ),
        Command::Plan {
            sub:
                PlanCommand::ProjectionCheck {
                    base_ref,
                    allow_planned,
                    files,
                },
        } => plan_projection_check(&base_ref, allow_planned, files, &args),
        Command::Plan {
            sub:
                PlanCommand::ProjectionCheckAll {
                    base_ref,
                    repo_root,
                },
        } => plan_projection_check_all(&base_ref, repo_root, &args),
        Command::Gate {
            sub: GateCommand::ChangeCheck { base_ref, registry },
        } => gate_change_check_command(&base_ref, &registry, &args),
        Command::Validate { sub } => run_validate(sub, &args),
        Command::Handoff {
            sub:
                HandoffCommand::Next {
                    agent_role,
                    dir,
                    session,
                    include_branch_local,
                },
        } => handoff_next(&agent_role, dir, session.as_deref(), include_branch_local, &args),
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
        Command::CommercialCorpus { sub } => commercial_corpus_command(sub, &args),
        Command::Usage {
            sub:
                UsageCommand::Report {
                    dir,
                    manifest,
                    last,
                },
        } => usage_report(dir, manifest, last, &args),
        Command::Ci {
            sub:
                CiCommand::CacheTopologyCheck {
                    merged_yaml,
                    shared_key,
                    writer_job,
                    writer_policy_var,
                    consumer_policy_var,
                    format,
                },
        } => ci_cache_topology_check_command(
            &merged_yaml,
            &shared_key,
            &writer_job,
            &writer_policy_var,
            &consumer_policy_var,
            &format,
            &args,
        ),
    }
}

fn gate_change_check_command(base_ref: &str, registry: &Path, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let repo_root = match git_output(&["rev-parse", "--show-toplevel"]) {
        Ok(path) => PathBuf::from(path),
        Err(reason) => {
            output.stderr(format!("ERR gate change-check failed: {reason}"));
            return 2;
        }
    };
    let label = registry.display().to_string();
    let exit_code = match agov_validators::gate_change::change_check(&repo_root, base_ref, registry)
    {
        Ok(()) => {
            output.validate_ok(&format!("gate change-check {label}"));
            0
        }
        Err(findings) => {
            for finding in findings {
                let location = finding.location.as_deref().unwrap_or(&label);
                output.validate_err("gate change-check", location, &finding.message);
            }
            1
        }
    };
    agov_core::roi::record_tool_output("agov", args, "gate-change-check", output.chars);
    exit_code
}

fn ci_cache_topology_check_command(
    merged_yaml_path: &Path,
    shared_key: &str,
    writer_job: &str,
    writer_policy_var: &str,
    consumer_policy_var: &str,
    format: &str,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let content = match std::fs::read_to_string(merged_yaml_path) {
        Ok(content) => content,
        Err(err) => {
            output.stderr(format!(
                "ERR cannot read {}: {err}",
                merged_yaml_path.display()
            ));
            agov_core::roi::record_tool_output(
                "agov",
                args,
                "ci-cache-topology-check",
                output.chars,
            );
            return 1;
        }
    };
    // Accept either raw merged-YAML text or a full CI Lint JSON response
    // (whose "merged_yaml" field holds the same text) -- both are valid
    // inputs a caller might have saved to disk.
    let doc: serde_json::Value = match serde_json::from_str::<serde_json::Value>(&content)
        .ok()
        .and_then(|v| {
            v.get("merged_yaml")
                .and_then(|m| m.as_str())
                .map(str::to_string)
        }) {
        Some(inner) => match serde_norway::from_str::<serde_json::Value>(&inner) {
            Ok(doc) => doc,
            Err(err) => {
                output.stderr(format!("ERR failed to parse merged_yaml field: {err}"));
                agov_core::roi::record_tool_output(
                    "agov",
                    args,
                    "ci-cache-topology-check",
                    output.chars,
                );
                return 1;
            }
        },
        None => match serde_norway::from_str::<serde_json::Value>(&content) {
            Ok(doc) => doc,
            Err(err) => {
                output.stderr(format!(
                    "ERR failed to parse {}: {err}",
                    merged_yaml_path.display()
                ));
                agov_core::roi::record_tool_output(
                    "agov",
                    args,
                    "ci-cache-topology-check",
                    output.chars,
                );
                return 1;
            }
        },
    };

    if format == "dot" {
        let jobs = ci_cache_topology::inventory_shared_cache_jobs(&doc, shared_key);
        for line in ci_cache_topology::render_dot(&jobs, shared_key).lines() {
            output.stdout(line);
        }
    }

    let exit_code = match ci_cache_topology::check_topology(
        &doc,
        shared_key,
        writer_job,
        writer_policy_var,
        consumer_policy_var,
    ) {
        Ok(report) => {
            if format != "dot" {
                let names: Vec<&str> = report
                    .shared_key_jobs
                    .iter()
                    .map(|j| j.name.as_str())
                    .collect();
                output.stdout(format!(
                    "OK cache-topology: key={shared_key} writer={writer_job} consumers={} total_shared_jobs={} jobs={names:?}",
                    report.shared_key_jobs.len().saturating_sub(1),
                    report.shared_key_jobs.len()
                ));
            }
            0
        }
        Err(errors) => {
            for error in errors {
                output.stderr(format!("ERR {error}"));
            }
            1
        }
    };
    agov_core::roi::record_tool_output("agov", args, "ci-cache-topology-check", output.chars);
    exit_code
}

#[derive(Serialize)]
struct VersionInfo {
    version: &'static str,
    commit: &'static str,
    dirty: bool,
    source_dirty: bool,
    source_fingerprint: &'static str,
}

fn build_version_info() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        commit: option_env!("AGOV_GIT_SHA").unwrap_or("unknown"),
        dirty: option_env!("AGOV_GIT_DIRTY").unwrap_or("false") == "true",
        source_dirty: option_env!("AGOV_SOURCE_DIRTY").unwrap_or("false") == "true",
        source_fingerprint: option_env!("AGOV_SOURCE_FINGERPRINT").unwrap_or("unknown"),
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
            format,
        } => validate_provenance_command(
            &files,
            artifact_dir.as_deref(),
            oci_image.as_deref(),
            &format,
            &mut output,
        ),
        DistCommand::SelfReportCheck {
            archive,
            binary_name,
            record,
        } => self_report_check_command(&archive, &binary_name, &record, &mut output),
        DistCommand::PostPublicationCheck {
            releases_url,
            download_dir,
        } => post_publication_check_command(&releases_url, &download_dir, &mut output),
        DistCommand::ProvenanceRecord { sub } => provenance_record_command(*sub, &mut output),
    };

    if exit_code == 0 {
        agov_core::roi::record_tool_output("agov", args, "dist", output.chars);
    }
    exit_code
}

fn commercial_corpus_command(sub: CommercialCorpusCommand, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let result = match sub {
        CommercialCorpusCommand::Generate {
            repo_root,
            inventory_ref,
            inventory_path,
            corpus_content_revision,
            output,
            pointer,
            schema,
        } => commercial_corpus::generate(commercial_corpus::GenerateOptions {
            repo_root,
            inventory_ref,
            inventory_path,
            corpus_content_revision,
            output,
            pointer,
            schema,
        }),
        CommercialCorpusCommand::Validate {
            repo_root,
            pointer,
            schema,
            base_ref,
        } => commercial_corpus::validate_current(commercial_corpus::ValidateOptions {
            repo_root,
            pointer,
            schema,
            base_ref,
        }),
        CommercialCorpusCommand::VerifyVersion {
            repo_root,
            manifest,
            schema,
        } => commercial_corpus::verify_version(commercial_corpus::VerifyOptions {
            repo_root,
            manifest,
            schema,
        }),
    };

    let exit_code = match result {
        Ok(report) => {
            for line in report.lines {
                output.stdout(line);
            }
            0
        }
        Err(errors) => {
            for error in errors {
                output.stderr(format!("ERR {error}"));
            }
            1
        }
    };
    if exit_code == 0 {
        agov_core::roi::record_tool_output("agov", args, "commercial-corpus", output.chars);
    }
    exit_code
}

/// Validate one or more release-provenance JSON records: schema validation
/// (or legacy classification) for each file, plus cross-record duplicate-
/// filename checks and, when `artifact_dir` is given, artifact/sha256/
/// checksum-sidecar agreement and dangling-reference checks. When
/// `oci_image` is given, `podman inspect`s that real image and cross-checks
/// its actual labels/digest against any oci-image records among `files`.
/// `format` is `"text"` (default, human-readable) or `"json"` (a stable
/// machine-readable array: `{"file", "assurance_level", "findings"}` per
/// entry, plus a synthetic `file: null` entry for cross-record findings).
fn validate_provenance_command(
    files: &[PathBuf],
    artifact_dir: Option<&Path>,
    oci_image: Option<&str>,
    format: &str,
    output: &mut OutputCounter,
) -> i32 {
    let json_mode = format == "json";
    let mut parsed = Vec::new();
    let mut json_results = Vec::new();
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
        let assurance_level = outcome.assurance_level;
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

        if !findings.is_empty() {
            exit_code = 1;
        }
        if json_mode {
            json_results.push(serde_json::json!({
                "file": file.display().to_string(),
                "assurance_level": assurance_level.as_str(),
                "findings": findings,
            }));
        } else if findings.is_empty() {
            output.stdout(format!(
                "OK  {} ({})",
                file.display(),
                assurance_level.as_str()
            ));
        } else {
            for finding in &findings {
                output.stderr(format!("ERR  {}: {finding}", file.display()));
            }
        }

        parsed.push(data);
    }

    let mut cross_record_findings = Vec::new();
    cross_record_findings.extend(provenance::check_no_duplicate_filenames(&parsed));
    cross_record_findings.extend(provenance::check_no_duplicate_oci_identity(&parsed));
    if let Some(dir) = artifact_dir {
        cross_record_findings.extend(provenance::check_primary_artifact_coverage(&parsed, dir));
    }
    if !cross_record_findings.is_empty() {
        exit_code = 1;
    }

    if json_mode {
        if !cross_record_findings.is_empty() {
            json_results.push(serde_json::json!({
                "file": null,
                "assurance_level": null,
                "findings": cross_record_findings,
            }));
        }
        match serde_json::to_string_pretty(&json_results) {
            Ok(json) => output.stdout(json),
            Err(err) => {
                output.stderr(format!("ERR  could not encode JSON output: {err}"));
                return 1;
            }
        }
    } else {
        for finding in &cross_record_findings {
            output.stderr(format!("ERR  {finding}"));
        }
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

/// Extract `binary_name` from `archive` (`.tar.gz` or `.zip`, by
/// extension), run its own `version --format json` self-report, and
/// cross-check it against `record`.
fn self_report_check_command(
    archive: &Path,
    binary_name: &str,
    record: &Path,
    output: &mut OutputCounter,
) -> i32 {
    let content = match std::fs::read_to_string(record) {
        Ok(content) => content,
        Err(err) => {
            output.stderr(format!("ERR  could not read {}: {err}", record.display()));
            return 1;
        }
    };
    let data: serde_json::Value = match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(err) => {
            output.stderr(format!("ERR  {}: malformed JSON: {err}", record.display()));
            return 1;
        }
    };

    let staging =
        std::env::temp_dir().join(format!("agov-self-report-check-{}", std::process::id()));
    if let Err(err) = std::fs::create_dir_all(&staging) {
        output.stderr(format!(
            "ERR  could not create staging directory {}: {err}",
            staging.display()
        ));
        return 1;
    }
    let extracted_path = staging.join(binary_name);

    let archive_str = archive.to_string_lossy();
    let extraction = if archive_str.ends_with(".zip") {
        provenance::extract_from_zip(archive, binary_name, &extracted_path)
    } else {
        provenance::extract_from_tar_gz(archive, binary_name, &extracted_path)
    };
    if let Err(err) = extraction {
        let _ = std::fs::remove_dir_all(&staging);
        output.stderr(format!("ERR  {err}"));
        return 1;
    }

    let self_report = match provenance::run_self_report(&extracted_path) {
        Ok(report) => report,
        Err(err) => {
            let _ = std::fs::remove_dir_all(&staging);
            output.stderr(format!("ERR  {err}"));
            return 1;
        }
    };
    let _ = std::fs::remove_dir_all(&staging);

    let findings = provenance::check_self_report_agreement(&data, &self_report);
    if findings.is_empty() {
        output.stdout(format!(
            "OK  {} self-report agrees with {}",
            binary_name,
            record.display()
        ));
        0
    } else {
        for finding in &findings {
            output.stderr(format!("ERR  {}: {finding}", record.display()));
        }
        1
    }
}

/// Fetch a URL's body via `curl`. This project's release tooling already
/// shells out to `curl` for HTTP (see the `release:create` CI job); this
/// follows the same convention rather than embedding an HTTP client
/// library for one read-only discovery step.
fn curl_get(url: &str) -> Result<String, String> {
    let output = std::process::Command::new("curl")
        .args(["--silent", "--fail", "--show-error", "--location", url])
        .output()
        .map_err(|err| format!("could not run curl: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "curl {url} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Maximum redirect hops `curl_get_bytes_validated` will follow before
/// giving up. Bounded so a redirect loop cannot hang the command.
const MAX_REDIRECTS: u8 = 5;

/// Parse the combined stdout of a `curl --output - --write-out
/// "\n%{http_code} %{redirect_url}"` invocation into `(body, http_code,
/// redirect_url)`. Pure and dependency-free so the parsing logic is fully
/// unit-testable without a real network call: the interesting bugs in a
/// hand-rolled redirect loop live in this parsing step, not in the loop
/// itself. Splits on the *last* `\n` in the buffer, since curl writes the
/// body first and the `--write-out` trailer last, and nothing this
/// project fetches legitimately ends its own final line with a bare
/// `\n<digits> ` sequence that would be ambiguous with the trailer.
fn parse_curl_write_out_response(stdout: &[u8]) -> Result<(Vec<u8>, u16, Option<String>), String> {
    let newline_pos = stdout
        .iter()
        .rposition(|&b| b == b'\n')
        .ok_or("curl output missing the expected trailing status line")?;
    let body = stdout[..newline_pos].to_vec();
    let trailer = String::from_utf8_lossy(&stdout[newline_pos + 1..]);
    let mut parts = trailer.trim().splitn(2, ' ');
    let code: u16 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| format!("could not parse HTTP status code from {trailer:?}"))?;
    let redirect_url = parts.next().filter(|s| !s.is_empty()).map(str::to_string);
    Ok((body, code, redirect_url))
}

/// Fetch `url`'s body to memory, validating the host of *every* hop
/// (the initial URL and every redirect target) against `trusted_host`
/// before it is ever requested -- `curl --location` alone follows a
/// redirect to any HTTPS host with no restriction, which would let an
/// initially-trusted URL hand control to an arbitrary host. Curl is
/// invoked once per hop with no automatic redirect-following
/// (`--location` is deliberately absent); this function decides whether
/// to follow each `Location` itself.
fn curl_get_bytes_validated(url: &str, trusted_host: &str) -> Result<Vec<u8>, String> {
    let mut current = url.to_string();
    for _ in 0..MAX_REDIRECTS {
        let host = provenance::parse_https_host(&current)?;
        if host != trusted_host {
            return Err(format!(
                "refusing to fetch from untrusted host {host:?} (trusted host is \
                 {trusted_host:?}): {current}"
            ));
        }

        let output = std::process::Command::new("curl")
            .args([
                "--silent",
                "--show-error",
                "--write-out",
                "\n%{http_code} %{redirect_url}",
                "--output",
                "-",
            ])
            .arg(&current)
            .output()
            .map_err(|err| format!("could not run curl: {err}"))?;
        if !output.status.success() {
            return Err(format!(
                "curl {current} failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let (body, code, redirect_url) = parse_curl_write_out_response(&output.stdout)
            .map_err(|err| format!("curl {current}: {err}"))?;

        if (300..400).contains(&code) {
            let Some(next) = redirect_url else {
                return Err(format!(
                    "curl {current}: HTTP {code} redirect with no Location header"
                ));
            };
            current = next;
            continue;
        }
        if !(200..300).contains(&code) {
            return Err(format!("curl {current}: HTTP {code}"));
        }
        return Ok(body);
    }
    Err(format!(
        "too many redirects fetching {url} (limit {MAX_REDIRECTS})"
    ))
}

/// Write `bytes` to `dest`, refusing to write through a pre-positioned
/// symlink or over an existing file: `dest` is opened with `create_new`,
/// which fails if anything (including a symlink, even a dangling one)
/// already occupies that path.
fn write_bytes_safely(bytes: &[u8], dest: &Path) -> Result<(), String> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dest)
        .map_err(|err| {
            format!(
                "could not create {} (must not already exist -- refusing to follow a \
                 pre-positioned symlink or overwrite an existing file): {err}",
                dest.display()
            )
        })?;
    use std::io::Write as _;
    file.write_all(bytes)
        .map_err(|err| format!("could not write {}: {err}", dest.display()))?;
    Ok(())
}

/// Download `url` to `dest`: every hop's host is validated against
/// `trusted_host` before being fetched (`curl_get_bytes_validated`), and
/// the destination is written via `write_bytes_safely` so curl itself
/// never touches the destination path and a pre-positioned symlink or
/// existing file is refused rather than followed/overwritten.
fn download_asset_safely(url: &str, trusted_host: &str, dest: &Path) -> Result<(), String> {
    let bytes = curl_get_bytes_validated(url, trusted_host)?;
    write_bytes_safely(&bytes, dest)
}

/// Parse a GitLab Release API response's `assets.links[]` into
/// `(name, direct_asset_url)` pairs.
fn discover_release_assets(release_json: &str) -> Result<Vec<(String, String)>, String> {
    let parsed: serde_json::Value = serde_json::from_str(release_json)
        .map_err(|err| format!("malformed release JSON: {err}"))?;
    let links = parsed
        .get("assets")
        .and_then(|assets| assets.get("links"))
        .and_then(serde_json::Value::as_array)
        .ok_or("release JSON has no assets.links array")?;

    let mut assets = Vec::new();
    for (index, link) in links.iter().enumerate() {
        let name = link.get("name").and_then(serde_json::Value::as_str);
        let url = link
            .get("direct_asset_url")
            .and_then(serde_json::Value::as_str);
        match (name, url) {
            (Some(name), Some(url)) => assets.push((name.to_string(), url.to_string())),
            _ => {
                // A malformed link entry fails discovery outright rather
                // than silently shrinking the allegedly complete release
                // set -- a missing name/URL on one link is exactly the
                // kind of gap the cardinality bijection check exists to
                // catch, and it cannot catch what discovery already
                // dropped.
                return Err(format!(
                    "assets.links[{index}] is missing a string \"name\" or \"direct_asset_url\": {link}"
                ));
            }
        }
    }
    Ok(assets)
}

/// The Rust target triple this binary can actually execute a self-report
/// check against. Only the exact triple matching the verifier host's own
/// architecture and OS is executable here -- an `aarch64-unknown-linux-musl`
/// or `x86_64-pc-windows-gnu` archive cannot run its packaged binary on an
/// `x86_64` Linux CI runner. Cross-target archives are not silently
/// skipped: `composed_self_report_check` emits an explicit line naming the
/// gap instead of omitting it.
fn executable_target_platform() -> Option<&'static str> {
    match (std::env::consts::ARCH, std::env::consts::OS) {
        ("x86_64", "linux") => Some("x86_64-unknown-linux-musl"),
        ("aarch64", "linux") => Some("aarch64-unknown-linux-musl"),
        _ => None,
    }
}

/// Pull `record`'s published `image_reference@manifest_digest` from the
/// real registry and inspect it -- the composed public-lifecycle
/// counterpart to `--oci-image` (which only inspects an already-local
/// image by whatever name/tag is passed). Real evidence never comes from
/// a locally-built or locally-tagged image in this path.
fn pull_and_inspect_published_oci_image(
    record: &serde_json::Value,
) -> Result<provenance::OciImageEvidence, String> {
    let image_reference = record
        .get("image_reference")
        .and_then(serde_json::Value::as_str)
        .ok_or("oci-image record missing image_reference")?;
    let manifest_digest = record
        .get("manifest_digest")
        .and_then(serde_json::Value::as_str)
        .ok_or("oci-image record missing manifest_digest")?;
    let pull_ref = provenance::oci_pull_ref(image_reference, manifest_digest);

    let status = std::process::Command::new("podman")
        .args(["pull", &pull_ref])
        .status()
        .map_err(|err| format!("could not run podman pull: {err}"))?;
    if !status.success() {
        return Err(format!("podman pull {pull_ref} failed"));
    }
    inspect_oci_image(&pull_ref)
}

/// Composed public OCI chain check: for every downloaded provenance record
/// that is an `oci-image` record, pull the real published image by its
/// immutable digest and cross-check it with `check_oci_image_evidence`.
/// Distinct from `validate_provenance_command`'s `--oci-image` flag, which
/// only inspects an already-local image the caller names manually.
fn composed_oci_chain_check(provenance_files: &[PathBuf], output: &mut OutputCounter) -> i32 {
    let mut exit_code = 0;
    for file in provenance_files {
        let Ok(content) = std::fs::read_to_string(file) else {
            continue;
        };
        let Ok(record) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };
        if record
            .get("artifact_kind")
            .and_then(serde_json::Value::as_str)
            != Some("oci-image")
        {
            continue;
        }
        match pull_and_inspect_published_oci_image(&record) {
            Ok(evidence) => {
                let findings = provenance::check_oci_image_evidence(&record, &evidence);
                if findings.is_empty() {
                    output.stdout(format!(
                        "OK  {} matches real published OCI image evidence",
                        file.display()
                    ));
                } else {
                    exit_code = 1;
                    for finding in &findings {
                        output.stderr(format!("ERR  {}: {finding}", file.display()));
                    }
                }
            }
            Err(err) => {
                exit_code = 1;
                output.stderr(format!(
                    "ERR  {}: could not pull/inspect published OCI image: {err}",
                    file.display()
                ));
            }
        }
    }
    exit_code
}

/// Composed self-report check: for every downloaded provenance record that
/// is a `product: "agov"` file-kind record packaged as `tar.gz`/`zip`,
/// extract the packaged `agov` binary from its downloaded archive and run
/// `check_self_report_agreement` against it -- but only when the record's
/// `target_platform` matches what this verifier host can actually execute
/// (see `executable_target_platform`). A cross-target archive is not
/// silently skipped: an explicit line names it and states why.
fn composed_self_report_check(
    download_dir: &Path,
    provenance_files: &[PathBuf],
    output: &mut OutputCounter,
) -> i32 {
    let mut exit_code = 0;
    let executable_target = executable_target_platform();
    for file in provenance_files {
        let Ok(content) = std::fs::read_to_string(file) else {
            continue;
        };
        let Ok(record) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };
        if record.get("product").and_then(serde_json::Value::as_str) != Some("agov") {
            continue;
        }
        let Some(packaging) = record.get("packaging").and_then(serde_json::Value::as_str) else {
            continue;
        };
        if packaging != "tar.gz" && packaging != "zip" {
            continue;
        }
        let Some(artifact_filename) = record
            .get("artifact_filename")
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        let target_platform = record
            .get("target_platform")
            .and_then(serde_json::Value::as_str);

        if target_platform != executable_target {
            output.stdout(format!(
                "SKIP {}: target_platform {target_platform:?} is not executable on this \
                 verifier host ({:?}) -- self-report check not evidenced here, only \
                 schema/hash/sidecar agreement was checked for this archive",
                file.display(),
                executable_target
            ));
            continue;
        }

        let archive = download_dir.join(artifact_filename);
        let staging = std::env::temp_dir().join(format!(
            "agov-post-pub-self-report-{}-{}",
            std::process::id(),
            artifact_filename
        ));
        let extraction_result = (|| -> Result<provenance::SelfReport, String> {
            std::fs::create_dir_all(&staging)
                .map_err(|err| format!("could not create staging dir: {err}"))?;
            let extracted_path = staging.join("agov");
            if packaging == "zip" {
                provenance::extract_from_zip(&archive, "agov", &extracted_path)?;
            } else {
                provenance::extract_from_tar_gz(&archive, "agov", &extracted_path)?;
            }
            provenance::run_self_report(&extracted_path)
        })();
        let _ = std::fs::remove_dir_all(&staging);

        match extraction_result {
            Ok(self_report) => {
                let findings = provenance::check_self_report_agreement(&record, &self_report);
                if findings.is_empty() {
                    output.stdout(format!(
                        "OK  {}: packaged agov self-report agrees",
                        file.display()
                    ));
                } else {
                    exit_code = 1;
                    for finding in &findings {
                        output.stderr(format!("ERR  {}: {finding}", file.display()));
                    }
                }
            }
            Err(err) => {
                exit_code = 1;
                output.stderr(format!(
                    "ERR  {}: could not run composed self-report check: {err}",
                    file.display()
                ));
            }
        }
    }
    exit_code
}

/// Anonymously discover every asset linked from a real published GitLab
/// Release, download all of them into `download_dir`, then run the exact
/// same `validate-provenance` logic (schema, cross-checks, and cardinality
/// bijection) against the downloaded provenance records and downloaded
/// artifact bytes -- not a CI workspace copy -- followed by the composed
/// OCI-chain check (real registry pull by digest) and composed self-report
/// check (real extracted-binary execution) for every record that applies.
///
/// `download_dir` must not already exist, or must exist and be empty: this
/// command creates and owns it, so a caller cannot point it at a directory
/// with a pre-positioned symlink and have this command silently write
/// through it.
fn post_publication_check_command(
    releases_url: &str,
    download_dir: &Path,
    output: &mut OutputCounter,
) -> i32 {
    if let Ok(mut entries) = std::fs::read_dir(download_dir) {
        if entries.next().is_some() {
            output.stderr(format!(
                "ERR  {} already exists and is not empty -- this command must own a fresh, \
                 internally-created download directory (refusing a pre-positioned entry, \
                 including a symlink, at that path)",
                download_dir.display()
            ));
            return 1;
        }
    }

    let trusted_host = match provenance::parse_https_host(releases_url) {
        Ok(host) => host,
        Err(err) => {
            output.stderr(format!("ERR  --releases-url must be https: {err}"));
            return 1;
        }
    };

    let release_json = match curl_get(releases_url) {
        Ok(body) => body,
        Err(err) => {
            output.stderr(format!("ERR  could not fetch {releases_url}: {err}"));
            return 1;
        }
    };
    let assets = match discover_release_assets(&release_json) {
        Ok(assets) => assets,
        Err(err) => {
            output.stderr(format!("ERR  {err}"));
            return 1;
        }
    };
    if assets.is_empty() {
        output.stderr(format!("ERR  no assets discovered at {releases_url}"));
        return 1;
    }

    if let Err(err) = provenance::validate_discovered_assets(&assets, &trusted_host) {
        output.stderr(format!("ERR  {err}"));
        return 1;
    }

    if let Err(err) = std::fs::create_dir_all(download_dir) {
        output.stderr(format!(
            "ERR  could not create {}: {err}",
            download_dir.display()
        ));
        return 1;
    }

    let mut provenance_files = Vec::new();
    for (name, url) in &assets {
        let dest = download_dir.join(name);
        if let Err(err) = download_asset_safely(url, &trusted_host, &dest) {
            output.stderr(format!("ERR  {err}"));
            return 1;
        }
        output.stdout(format!("OK  downloaded {name}"));
        if name.ends_with("-provenance.json") {
            provenance_files.push(dest);
        }
    }

    if provenance_files.is_empty() {
        output.stderr(format!(
            "ERR  discovered {} assets at {releases_url} but none were *-provenance.json records",
            assets.len()
        ));
        return 1;
    }

    let base_exit =
        validate_provenance_command(&provenance_files, Some(download_dir), None, "text", output);
    let oci_exit = composed_oci_chain_check(&provenance_files, output);
    let self_report_exit = composed_self_report_check(download_dir, &provenance_files, output);

    if base_exit != 0 || oci_exit != 0 || self_report_exit != 0 {
        1
    } else {
        0
    }
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
            let pipeline_identity = match pipeline_identity_from(&pipeline_id, &job_id) {
                Ok(pipeline_identity) => pipeline_identity,
                Err(err) => {
                    output.stderr(format!("ERR  {err}"));
                    return 1;
                }
            };
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
            let pipeline_identity = match pipeline_identity_from(&pipeline_id, &job_id) {
                Ok(pipeline_identity) => pipeline_identity,
                Err(err) => {
                    output.stderr(format!("ERR  {err}"));
                    return 1;
                }
            };
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

/// `--pipeline-id`/`--job-id` are both-or-neither: supplying only one
/// silently dropped both under the old `match` (a real usage mistake
/// would produce a record with no pipeline_identity at all, no error).
fn pipeline_identity_from<'a>(
    pipeline_id: &'a Option<String>,
    job_id: &'a Option<String>,
) -> Result<Option<provenance::PipelineIdentity<'a>>, String> {
    match (pipeline_id, job_id) {
        (Some(pipeline_id), Some(job_id)) => Ok(Some(provenance::PipelineIdentity {
            pipeline_id,
            job_id,
        })),
        (None, None) => Ok(None),
        (Some(_), None) => Err(
            "--pipeline-id was given without --job-id; both or neither are required".to_string(),
        ),
        (None, Some(_)) => Err(
            "--job-id was given without --pipeline-id; both or neither are required".to_string(),
        ),
    }
}

fn emit_provenance_record(
    record: &serde_json::Value,
    output_path: Option<&Path>,
    output: &mut OutputCounter,
) -> i32 {
    // Validate atomically before writing: a generator refactor that
    // accidentally produces a schema-invalid record must fail here, not
    // silently reach publication.
    let outcome = provenance::validate_record(record);
    if !outcome.is_valid() {
        for finding in &outcome.findings {
            output.stderr(format!(
                "ERR  generated provenance record is invalid: {finding}"
            ));
        }
        return 1;
    }

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
    let resolution = match agov_core::roi::resolve_log_path(log.as_deref()) {
        Ok(resolution) => resolution,
        Err(reason) => {
            eprintln!("ERR  token ROI log unavailable: {reason}");
            return 1;
        }
    };
    agov_core::roi::emit_path_warning(resolution.warning.as_deref());
    let path = resolution.path;
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
        output.stdout("# Review evidence envelope (advisory authoring aid; not a gate decision)");
        output.stdout("Scope: <artifact and exact revision>");
        output.stdout("Coverage: <COMPLETE | INCOMPLETE>");
        output.stdout(
            "Recommendation: <CLEAR | BLOCK; omit when incomplete and no blocker is known>",
        );
        output.stdout("Blocking findings: <open unaccepted count>");
        output.stdout("Advisory findings: <count>");
        output.stdout("Open findings with accepted risk: <count>");
        output.stdout(
            "Authority: recommendation only; the authorized decision-maker decides the gate",
        );
        output.stdout("");
        output.stdout("# Repeat for a tracked fail or needs-review outcome; delete when unused");
        output.stdout("Check: <identifier>");
        output.stdout("Check status: <reviewed | not-reviewed>");
        output.stdout(
            "Outcome: <pass | fail | needs-review | not-applicable; omit when not-reviewed>",
        );
        output.stdout("Requirement strength: <mandatory | recommended | optional | not-stated>");
        output.stdout("Finding: <identifier; only for fail or needs-review>");
        output.stdout("Impact: <critical | material | minor | unknown>");
        output.stdout("Enforcement: <blocking | advisory>");
        output.stdout("Basis: <named rule/requirement, or explicitly advisory judgment>");
        output.stdout("Finding state: <open | resolved | superseded>");
        output.stdout("Risk disposition: <none | accepted>");
        output.stdout("Risk decision: <required when accepted: decision ID, scope/revision, revisit condition>");
        output.stdout("Resolution condition: <observable condition>");
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
        if !agov_coord::channel_reader::is_channel_path(&path, dir) || !path.is_file() {
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
                        ".claude/settings.local.json"
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
    if let Some(timestamp) = parse_compact_timestamp(since) {
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

/// Issue #84: the compact filename-prefix timestamp embedded as the
/// literal first path segment of every coordination filename
/// (`20260729T195431Z`) is exactly RFC3339 with `-` and `:` stripped.
/// Requires an exact 16-character `YYYYMMDDTHHMMSSZ` match before
/// attempting to parse -- a malformed or truncated digit string (e.g. a
/// 13-digit string) must fall through to the id/filename match, not be
/// silently accepted as a valid cursor.
fn parse_compact_timestamp(value: &str) -> Option<time::OffsetDateTime> {
    let bytes = value.as_bytes();
    if bytes.len() != 16 || bytes[8] != b'T' || bytes[15] != b'Z' {
        return None;
    }
    if !value[0..8].bytes().all(|b| b.is_ascii_digit())
        || !value[9..15].bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let rfc3339 = format!(
        "{}-{}-{}T{}:{}:{}Z",
        &value[0..4],
        &value[4..6],
        &value[6..8],
        &value[9..11],
        &value[11..13],
        &value[13..15],
    );
    time::OffsetDateTime::parse(&rfc3339, &Rfc3339).ok()
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ConvergencePhase {
    #[default]
    Open,
    ReviewPending,
    ResetRequired,
    ResetRecorded,
    GovernorReturnRequired,
    Closed,
    Conflict,
}

impl ConvergencePhase {
    fn label(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::ReviewPending => "review-pending",
            Self::ResetRequired => "reset-required",
            Self::ResetRecorded => "reset-recorded",
            Self::GovernorReturnRequired => "governor-return-required",
            Self::Closed => "closed",
            Self::Conflict => "conflict",
        }
    }

    fn required_next_event(self) -> &'static str {
        match self {
            Self::Open | Self::ResetRecorded => "candidate",
            Self::ReviewPending => "targeted-review",
            Self::ResetRequired => "abstraction-reset",
            Self::GovernorReturnRequired => "governor-disposition",
            Self::Closed => "none",
            Self::Conflict => "governor-resolution",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct ConvergenceCandidateReport {
    event_id: String,
    attempt: u64,
    candidate_sha: String,
    scope_paths: Vec<String>,
    scope_digest: String,
}

#[derive(Debug, Clone)]
struct ConvergenceCandidate {
    report: ConvergenceCandidateReport,
    invariant_ids: BTreeSet<String>,
    reviewable_invariants: BTreeSet<String>,
    after_reset_invariants: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ConvergenceReviewIdentity {
    invariant_id: String,
    attempt: u64,
    candidate_event: String,
    candidate_sha: String,
    scope_digest: String,
}

#[derive(Debug, Default)]
struct ConvergenceInvariantState {
    phase: ConvergencePhase,
    consecutive_failures: u64,
    last_attempt: u64,
    last_scope_digest: Option<String>,
    active_candidate: Option<String>,
    canonical_candidate: Option<ConvergenceCandidateReport>,
    reviews: BTreeMap<ConvergenceReviewIdentity, BTreeSet<String>>,
}

#[derive(Debug, Serialize)]
struct ConvergenceInvariantReport {
    invariant_id: String,
    state: String,
    consecutive_semantic_failures: u64,
    required_next_event: String,
    canonical_candidate: Option<ConvergenceCandidateReport>,
    conflict: bool,
}

#[derive(Debug, Serialize)]
struct ConvergenceReport {
    series_id: String,
    status: String,
    invariants: Vec<ConvergenceInvariantReport>,
    errors: Vec<String>,
}

fn coord_convergence(series: &str, dir: PathBuf, format: &str, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);
    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(error) => {
            output.stderr(format!("ERR  coordination directory unavailable: {error}"));
            return 1;
        }
    };

    let mut states: BTreeMap<String, ConvergenceInvariantState> = BTreeMap::new();
    let mut candidates: BTreeMap<String, ConvergenceCandidate> = BTreeMap::new();
    let mut errors = Vec::new();
    let mut matching_events = 0_usize;

    for message in &messages {
        let Some(convergence) = message
            .raw
            .get("convergence")
            .and_then(|value| value.as_object())
        else {
            continue;
        };
        if convergence
            .get("series_id")
            .and_then(|value| value.as_str())
            != Some(series)
        {
            continue;
        }
        matching_events += 1;

        let event_id = string_field(&message.raw, "id")
            .unwrap_or(message.file.as_str())
            .to_string();
        let event = convergence
            .get("event")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        let invariant_ids = convergence_string_array(convergence, "invariant_ids");

        match event {
            "candidate" => process_convergence_candidate(
                &event_id,
                convergence,
                &invariant_ids,
                &mut states,
                &mut candidates,
                &mut errors,
            ),
            "targeted-review" => process_convergence_review(
                &event_id,
                convergence,
                &invariant_ids,
                &mut states,
                &candidates,
                &mut errors,
            ),
            "abstraction-reset" => process_convergence_reset(
                &event_id,
                convergence,
                &invariant_ids,
                &mut states,
                &mut errors,
            ),
            "governor-disposition" => process_convergence_governor_disposition(
                &event_id,
                &invariant_ids,
                &mut states,
                &mut errors,
            ),
            _ => errors.push(format!(
                "{event_id}: unsupported convergence event '{event}'"
            )),
        }
    }

    if matching_events == 0 {
        errors.push(format!("series '{series}' has no convergence events"));
    }

    let has_conflict = states
        .values()
        .any(|state| state.phase == ConvergencePhase::Conflict);
    let status = if !errors.is_empty() {
        "invalid"
    } else if has_conflict {
        "conflict"
    } else {
        "ok"
    };
    let invariants = states
        .into_iter()
        .map(|(invariant_id, state)| ConvergenceInvariantReport {
            invariant_id,
            state: state.phase.label().to_string(),
            consecutive_semantic_failures: state.consecutive_failures,
            required_next_event: state.phase.required_next_event().to_string(),
            canonical_candidate: state.canonical_candidate,
            conflict: state.phase == ConvergencePhase::Conflict,
        })
        .collect();
    let report = ConvergenceReport {
        series_id: series.to_string(),
        status: status.to_string(),
        invariants,
        errors,
    };

    if format == "json" {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => output.stdout(json),
            Err(error) => {
                output.stderr(format!("ERR  failed to encode convergence report: {error}"));
                return 1;
            }
        }
    } else {
        render_convergence_text(&mut output, &report);
    }

    agov_core::roi::record_tool_output("agov", args, "coord-convergence", output.chars);
    if report.status == "ok" {
        0
    } else {
        1
    }
}

fn process_convergence_candidate(
    event_id: &str,
    convergence: &serde_json::Map<String, serde_json::Value>,
    invariant_ids: &[String],
    states: &mut BTreeMap<String, ConvergenceInvariantState>,
    candidates: &mut BTreeMap<String, ConvergenceCandidate>,
    errors: &mut Vec<String>,
) {
    let Some(attempt) = convergence.get("attempt").and_then(|value| value.as_u64()) else {
        errors.push(format!("{event_id}: candidate is missing attempt"));
        return;
    };
    let Some(candidate_sha) = convergence_string(convergence, "candidate_sha") else {
        errors.push(format!("{event_id}: candidate is missing candidate_sha"));
        return;
    };
    let Some(scope_digest) = convergence_string(convergence, "scope_digest") else {
        errors.push(format!("{event_id}: candidate is missing scope_digest"));
        return;
    };
    let scope_paths = convergence_string_array(convergence, "scope_paths");
    if !is_strictly_sorted(&scope_paths) {
        errors.push(format!("{event_id}: scope_paths must be sorted and unique"));
        return;
    }
    match convergence_scope_digest(candidate_sha, &scope_paths) {
        Ok(actual) if actual == scope_digest => {}
        Ok(actual) => {
            errors.push(format!(
                "{event_id}: scope_digest mismatch: declared {scope_digest}, computed {actual}"
            ));
            return;
        }
        Err(error) => {
            errors.push(format!("{event_id}: {error}"));
            return;
        }
    }

    let report = ConvergenceCandidateReport {
        event_id: event_id.to_string(),
        attempt,
        candidate_sha: candidate_sha.to_string(),
        scope_paths,
        scope_digest: scope_digest.to_string(),
    };
    let mut candidate = ConvergenceCandidate {
        report: report.clone(),
        invariant_ids: invariant_ids.iter().cloned().collect(),
        reviewable_invariants: BTreeSet::new(),
        after_reset_invariants: BTreeSet::new(),
    };

    for invariant_id in invariant_ids {
        let state = states.entry(invariant_id.clone()).or_default();
        if matches!(
            state.phase,
            ConvergencePhase::ResetRequired
                | ConvergencePhase::GovernorReturnRequired
                | ConvergencePhase::Closed
                | ConvergencePhase::Conflict
        ) {
            errors.push(format!(
                "{event_id}: invariant '{invariant_id}' does not allow candidate in state {}",
                state.phase.label()
            ));
            continue;
        }

        let unchanged = state.last_scope_digest.as_deref() == Some(scope_digest);
        if unchanged {
            if attempt != state.last_attempt {
                errors.push(format!(
                    "{event_id}: invariant '{invariant_id}' unchanged digest must retain attempt {}, got {attempt}",
                    state.last_attempt
                ));
                continue;
            }
            state.canonical_candidate = Some(report.clone());
            if state.phase == ConvergencePhase::ReviewPending {
                state.active_candidate = Some(event_id.to_string());
                candidate.reviewable_invariants.insert(invariant_id.clone());
            }
            continue;
        }

        if !matches!(
            state.phase,
            ConvergencePhase::Open | ConvergencePhase::ResetRecorded
        ) {
            errors.push(format!(
                "{event_id}: invariant '{invariant_id}' does not allow a new semantic candidate in state {}",
                state.phase.label()
            ));
            continue;
        }
        if attempt <= state.last_attempt {
            errors.push(format!(
                "{event_id}: invariant '{invariant_id}' new digest requires attempt greater than {}, got {attempt}",
                state.last_attempt
            ));
            continue;
        }

        if state.phase == ConvergencePhase::ResetRecorded {
            candidate
                .after_reset_invariants
                .insert(invariant_id.clone());
        }
        state.phase = ConvergencePhase::ReviewPending;
        state.last_attempt = attempt;
        state.last_scope_digest = Some(scope_digest.to_string());
        state.active_candidate = Some(event_id.to_string());
        state.canonical_candidate = Some(report.clone());
        candidate.reviewable_invariants.insert(invariant_id.clone());
    }

    if candidates.insert(event_id.to_string(), candidate).is_some() {
        errors.push(format!("{event_id}: duplicate candidate event id"));
    }
}

fn process_convergence_review(
    event_id: &str,
    convergence: &serde_json::Map<String, serde_json::Value>,
    invariant_ids: &[String],
    states: &mut BTreeMap<String, ConvergenceInvariantState>,
    candidates: &BTreeMap<String, ConvergenceCandidate>,
    errors: &mut Vec<String>,
) {
    let Some(candidate_event) = convergence_string(convergence, "candidate_event") else {
        errors.push(format!(
            "{event_id}: targeted review is missing candidate_event"
        ));
        return;
    };
    let Some(candidate) = candidates.get(candidate_event) else {
        errors.push(format!(
            "{event_id}: candidate_event '{candidate_event}' is not an earlier candidate in this series"
        ));
        return;
    };
    let attempt = convergence
        .get("attempt")
        .and_then(|value| value.as_u64())
        .unwrap_or(0);
    let candidate_sha = convergence_string(convergence, "candidate_sha").unwrap_or("");
    let scope_digest = convergence_string(convergence, "scope_digest").unwrap_or("");
    let scope_paths = convergence_string_array(convergence, "scope_paths");
    if attempt != candidate.report.attempt
        || candidate_sha != candidate.report.candidate_sha
        || scope_digest != candidate.report.scope_digest
        || scope_paths != candidate.report.scope_paths
    {
        errors.push(format!(
            "{event_id}: targeted review fields do not match candidate_event '{candidate_event}'"
        ));
        return;
    }
    let outcome = convergence_string(convergence, "outcome").unwrap_or("");

    for invariant_id in invariant_ids {
        if !candidate.invariant_ids.contains(invariant_id) {
            errors.push(format!(
                "{event_id}: candidate_event '{candidate_event}' does not declare invariant '{invariant_id}'"
            ));
            continue;
        }
        let Some(state) = states.get_mut(invariant_id) else {
            errors.push(format!("{event_id}: unknown invariant '{invariant_id}'"));
            continue;
        };
        let identity = ConvergenceReviewIdentity {
            invariant_id: invariant_id.clone(),
            attempt,
            candidate_event: candidate_event.to_string(),
            candidate_sha: candidate_sha.to_string(),
            scope_digest: scope_digest.to_string(),
        };
        let outcomes = state.reviews.entry(identity).or_default();
        if outcomes.contains(outcome) {
            continue;
        }
        let conflicting = (outcome == "clear" && outcomes.contains("block"))
            || (outcome == "block" && outcomes.contains("clear"));
        outcomes.insert(outcome.to_string());
        if conflicting {
            state.phase = ConvergencePhase::Conflict;
            state.active_candidate = None;
            continue;
        }
        if outcome == "advisory" {
            continue;
        }
        if state.phase != ConvergencePhase::ReviewPending
            || state.active_candidate.as_deref() != Some(candidate_event)
            || !candidate.reviewable_invariants.contains(invariant_id)
        {
            errors.push(format!(
                "{event_id}: invariant '{invariant_id}' review is not for its canonical pending candidate"
            ));
            continue;
        }

        state.active_candidate = None;
        match outcome {
            "clear" => {
                state.phase = ConvergencePhase::Closed;
                state.consecutive_failures = 0;
            }
            "block" if candidate.after_reset_invariants.contains(invariant_id) => {
                state.phase = ConvergencePhase::GovernorReturnRequired;
                state.consecutive_failures += 1;
            }
            "block" => {
                state.consecutive_failures += 1;
                state.phase = if state.consecutive_failures >= 2 {
                    ConvergencePhase::ResetRequired
                } else {
                    ConvergencePhase::Open
                };
            }
            _ => errors.push(format!(
                "{event_id}: unsupported review outcome '{outcome}'"
            )),
        }
    }
}

fn process_convergence_reset(
    event_id: &str,
    convergence: &serde_json::Map<String, serde_json::Value>,
    invariant_ids: &[String],
    states: &mut BTreeMap<String, ConvergenceInvariantState>,
    errors: &mut Vec<String>,
) {
    let disposition = convergence_string(convergence, "disposition").unwrap_or("");
    for invariant_id in invariant_ids {
        let Some(state) = states.get_mut(invariant_id) else {
            errors.push(format!("{event_id}: unknown invariant '{invariant_id}'"));
            continue;
        };
        if state.phase != ConvergencePhase::ResetRequired {
            errors.push(format!(
                "{event_id}: invariant '{invariant_id}' does not allow abstraction-reset in state {}",
                state.phase.label()
            ));
            continue;
        }
        match disposition {
            "redesign" => {
                state.phase = ConvergencePhase::ResetRecorded;
                state.consecutive_failures = 0;
            }
            "accept" | "defer" | "stop" => state.phase = ConvergencePhase::Closed,
            _ => errors.push(format!(
                "{event_id}: unsupported abstraction-reset disposition '{disposition}'"
            )),
        }
    }
}

fn process_convergence_governor_disposition(
    event_id: &str,
    invariant_ids: &[String],
    states: &mut BTreeMap<String, ConvergenceInvariantState>,
    errors: &mut Vec<String>,
) {
    for invariant_id in invariant_ids {
        let Some(state) = states.get_mut(invariant_id) else {
            errors.push(format!("{event_id}: unknown invariant '{invariant_id}'"));
            continue;
        };
        if state.phase != ConvergencePhase::GovernorReturnRequired {
            errors.push(format!(
                "{event_id}: invariant '{invariant_id}' does not allow governor-disposition in state {}",
                state.phase.label()
            ));
            continue;
        }
        state.phase = ConvergencePhase::Closed;
    }
}

fn convergence_scope_digest(sha: &str, paths: &[String]) -> Result<String, String> {
    let mut hasher = Sha256::new();
    for path in paths {
        let object = format!("{sha}:{path}");
        let output = std::process::Command::new("git")
            .args(["show", "--no-textconv", object.as_str()])
            .output()
            .map_err(|error| format!("cannot read candidate scope '{object}': {error}"))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(format!(
                "cannot read candidate scope '{object}': {}",
                if stderr.is_empty() {
                    output.status.to_string()
                } else {
                    stderr
                }
            ));
        }
        hasher.update(path.as_bytes());
        hasher.update([0]);
        hasher.update(&output.stdout);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn convergence_string<'a>(
    convergence: &'a serde_json::Map<String, serde_json::Value>,
    field: &str,
) -> Option<&'a str> {
    convergence.get(field).and_then(|value| value.as_str())
}

fn convergence_string_array(
    convergence: &serde_json::Map<String, serde_json::Value>,
    field: &str,
) -> Vec<String> {
    convergence
        .get(field)
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .map(str::to_string)
        .collect()
}

fn is_strictly_sorted(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn render_convergence_text(output: &mut OutputCounter, report: &ConvergenceReport) {
    output.stdout(format!("Series: {}", report.series_id));
    output.stdout(format!("Status: {}", report.status));
    for invariant in &report.invariants {
        output.stdout(format!("Invariant: {}", invariant.invariant_id));
        output.stdout(format!("  state: {}", invariant.state));
        output.stdout(format!(
            "  consecutive-semantic-failures: {}",
            invariant.consecutive_semantic_failures
        ));
        output.stdout(format!(
            "  required-next-event: {}",
            invariant.required_next_event
        ));
        output.stdout(format!("  conflict: {}", invariant.conflict));
        if let Some(candidate) = &invariant.canonical_candidate {
            output.stdout(format!(
                "  canonical-candidate: {} attempt={} sha={} digest={}",
                candidate.event_id,
                candidate.attempt,
                candidate.candidate_sha,
                candidate.scope_digest
            ));
        } else {
            output.stdout("  canonical-candidate: none");
        }
    }
    for error in &report.errors {
        output.stderr(format!("ERR  {error}"));
    }
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
    candidate_diagnostics: Vec<serde_json::Value>,
    invalid_channel_records: Vec<serde_json::Value>,
    archive_baseline: Vec<serde_json::Value>,
    scanned_refs: BTreeMap<String, String>,
    scanned_scopes: Vec<String>,
    /// The requested base branch name, exactly as given via
    /// `--base-branch` (default `"main"`) -- unchanged meaning; never
    /// repurposed to hold a resolved OID or ref name (issue #89).
    base_branch: String,
    /// Full commit OID actually used as the comparison base after
    /// freshness resolution (issue #89). Always populated -- the local
    /// base branch's own OID when no upstream is configured or upstream
    /// matches local.
    effective_base_oid: String,
    /// Which ref supplied `effective_base_oid`: `"local"` or `"upstream"`.
    effective_base_source: String,
    scanned_branches: Vec<String>,
    unbridged_obligations: Vec<VisibilityObligation>,
    /// Branch-relevant collisions only (issue #85) -- a base-history-only
    /// duplicate seq never appears here; see `legacy_base_collisions`.
    sequence_collisions: Vec<BranchCollision>,
    /// Issue #85: `seq` values of collisions confined to base-branch
    /// history, unrelated to the branch under inspection. Always
    /// present (never conflated with `sequence_collisions`); reported as
    /// a summary count, never drives `exit_code` on its own.
    legacy_base_collisions: Vec<i64>,
    errors: Vec<String>,
}

#[derive(Debug, Clone)]
struct BranchCollision {
    seq: i64,
    branch: String,
    branch_file: String,
    base_file: String,
}

#[derive(Debug, Clone)]
struct VisibilityObligation {
    for_session: Option<String>,
    addressed_elsewhere: bool,
    branch: String,
    file: String,
    seq: Option<i64>,
    kind: String,
    to: String,
    topic: String,
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

fn coord_publication_check(
    base_ref: String,
    dir: PathBuf,
    subject_prefix: String,
    format: String,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let coord_dir = match git_coord_dir(&dir) {
        Ok(path) => path,
        Err(reason) => {
            output.stderr(format!("ERR publication check failed: {reason}"));
            return 2;
        }
    };

    let repo_root = match git_output(&["rev-parse", "--show-toplevel"]) {
        Ok(path) => PathBuf::from(path),
        Err(reason) => {
            output.stderr(format!("ERR publication check failed: {reason}"));
            return 2;
        }
    };

    // Condition 5 validates the introducing Git object, sharing the exact raw
    // bytes used by copy detection. Dirty, deleted or untracked local files
    // cannot supply or replace acceptance evidence.
    let report = agov_coord::publication_check::publication_check_validated(
        &base_ref,
        &coord_dir,
        &subject_prefix,
        &repo_root,
        &|bytes| match std::str::from_utf8(bytes) {
            Ok(content) => agov_coord::coord_validator::validate_content(content)
                .err()
                .unwrap_or_default()
                .into_iter()
                .map(|(field, message)| format!("schema validation failed ({field}: {message})"))
                .collect(),
            Err(error) => vec![format!("schema validation failed (file: invalid UTF-8: {error})")],
        },
    );

    if format == "json" {
        output.stdout(publication_check_json(&report));
    } else {
        render_publication_check_text(&mut output, &report);
    }

    if report.ok {
        agov_core::roi::record_tool_output("agov", args, "coord-publication-check", output.chars);
        0
    } else {
        1
    }
}

fn render_publication_check_text(
    output: &mut OutputCounter,
    report: &agov_coord::publication_check::PublicationCheckReport,
) {
    output.stdout(format!(
        "publication-check base_ref={} target_sha={} candidate_sha={} merge_base={} commits={}",
        report.base_ref,
        report.target_sha.as_deref().unwrap_or("none"),
        report.candidate_sha.as_deref().unwrap_or("none"),
        report.merge_base.as_deref().unwrap_or("none"),
        report.commits_checked.len()
    ));
    for file in &report.added_files {
        output.stdout(format!("  + {file}"));
    }
    for reason in &report.reasons {
        output.stderr(format!("BLOCK {reason}"));
    }
    if report.ok {
        output.stdout("OK  severity: blocking publication-check passed");
    } else {
        output.stdout("FAIL severity: blocking publication-check rejected candidate range");
    }
}

fn publication_check_json(
    report: &agov_coord::publication_check::PublicationCheckReport,
) -> String {
    serde_json::json!({
        "ok": report.ok,
        "base_ref": report.base_ref,
        "merge_base": report.merge_base,
        "candidate_sha": report.candidate_sha,
        "target_sha": report.target_sha,
        "commits_checked": report.commits_checked,
        "added_files": report.added_files,
        "reasons": report.reasons,
    })
    .to_string()
}

fn coord_visibility(
    base_branch: String,
    branches: Vec<String>,
    dir: PathBuf,
    format: String,
    session: Option<&str>,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let coord_dir = match git_coord_dir(&dir) {
        Ok(path) => path,
        Err(reason) => {
            output.stderr(format!(
                "ERR coordination visibility coverage incomplete: {reason}"
            ));
            return 2;
        }
    };

    let mut report = match visibility_scan(&base_branch, branches, &coord_dir) {
        Ok(report) => report,
        Err(reason) => {
            output.stderr(format!(
                "ERR coordination visibility coverage incomplete: {reason}"
            ));
            return 2;
        }
    };

    for item in &mut report.unbridged_obligations {
        item.addressed_elsewhere = session
            .zip(item.for_session.as_deref())
            .is_some_and(|(requested, recipient)| requested != recipient);
    }

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
    let effective_base = resolve_effective_base(base_branch)
        .map_err(|reason| format!("base_branch_unavailable: {base_branch}: {reason}"))?;

    let base_inventory = git_candidate_inventory(&effective_base.oid, coord_dir)?;
    let mut diagnostics = Vec::new();
    let mut invalid_channel = Vec::new();
    let mut archive_baseline = Vec::new();
    let base_messages = load_visibility_snapshot(
        &effective_base.oid,
        coord_dir,
        &base_inventory,
        &base_inventory,
        true,
        &mut diagnostics,
        &mut invalid_channel,
        &mut archive_baseline,
    )?;
    let mut scanned_refs = BTreeMap::from([(base_branch.to_string(), effective_base.oid.clone())]);
    let base_file_set: std::collections::BTreeSet<&str> = base_messages
        .iter()
        .map(|message| message.file.as_str())
        .collect();

    let branches = if requested_branches.is_empty() {
        dedup_and_select_branches(&git_branch_refs()?, &effective_base.oid)
    } else {
        requested_branches
    };

    let mut unbridged_obligations = Vec::new();
    let mut sequence_collisions: Vec<BranchCollision> = Vec::new();
    let legacy_base_collisions = base_sequence_collisions_visibility(&base_messages);

    for branch in &branches {
        let oid = git_output(&["rev-parse", "--verify", &format!("{branch}^{{commit}}")])?
            .trim()
            .to_string();
        let inventory = git_candidate_inventory(&oid, coord_dir)?;
        let branch_messages: Vec<_> = load_visibility_snapshot(
            &oid,
            coord_dir,
            &inventory,
            &base_inventory,
            false,
            &mut diagnostics,
            &mut invalid_channel,
            &mut archive_baseline,
        )?
        .into_iter()
        .filter(|m| !base_file_set.contains(m.file.as_str()))
        .collect();
        scanned_refs.insert(branch.clone(), oid);

        for message in branch_messages
            .iter()
            .filter(|message| visibility_obligation(&message.raw))
            .filter(|message| !visibility_bridged(&base_messages, branch, message))
        {
            unbridged_obligations.push(VisibilityObligation {
                for_session: handoff_for_session(&message.raw).map(str::to_string),
                addressed_elsewhere: false,
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
        candidate_diagnostics: diagnostics,
        invalid_channel_records: invalid_channel,
        archive_baseline,
        scanned_refs,
        scanned_scopes: candidate_scopes(coord_dir),
        base_branch: base_branch.to_string(),
        effective_base_oid: effective_base.oid,
        effective_base_source: effective_base.source.to_string(),
        scanned_branches: branches,
        unbridged_obligations,
        sequence_collisions,
        legacy_base_collisions,
        errors: Vec::new(),
    })
}

/// Issue #89 (B1): resolves the effective comparison base for
/// `--base-branch <base_branch>`. When a configured upstream exists and
/// its OID differs from the local base branch's OID, the upstream OID is
/// preferred (and the upstream ref is what base content is actually read
/// from) -- this makes the documented `git fetch` then `agov coord
/// visibility --base-branch main` workflow correct without requiring the
/// caller to separately fast-forward local `main`. When no upstream is
/// configured, or upstream matches local, behavior is unchanged.
struct EffectiveBase {
    oid: String,
    source: &'static str,
}

fn resolve_effective_base(base_branch: &str) -> Result<EffectiveBase, String> {
    let local_oid = git_output(&[
        "rev-parse",
        "--verify",
        &format!("{base_branch}^{{commit}}"),
    ])?
    .trim()
    .to_string();

    let upstream_short = git_output(&[
        "for-each-ref",
        "--format=%(upstream:short)",
        &format!("refs/heads/{base_branch}"),
    ])?;
    let upstream_short = upstream_short.trim();

    if upstream_short.is_empty() {
        return Ok(EffectiveBase {
            oid: local_oid,
            source: "local",
        });
    }

    let Ok(upstream_oid) = git_output(&[
        "rev-parse",
        "--verify",
        &format!("{upstream_short}^{{commit}}"),
    ]) else {
        return Err(format!("configured upstream unavailable: {upstream_short}; fetch the target before applying archive baseline suppression"));
    };
    let upstream_oid = upstream_oid.trim().to_string();

    if upstream_oid != local_oid {
        Ok(EffectiveBase {
            oid: upstream_oid,
            source: "upstream",
        })
    } else {
        Ok(EffectiveBase {
            oid: local_oid,
            source: "local",
        })
    }
}

/// Issue #89: a local or remote-tracking branch ref, as returned by
/// `git_branch_refs`. `full_name` is the fully-qualified ref (used only
/// to determine class and to sort within a class -- comparing full names
/// and comparing `short_name`s produce the same relative order within a
/// single class, since the prefix stripped by `short_name` is constant
/// across all members of that class). `short_name` is the actual branch
/// identifier used everywhere else (git ref argument, JSON/text output).
struct BranchRef {
    full_name: String,
    short_name: String,
    oid: String,
}

/// Enumerates every `refs/heads/*` and `refs/remotes/<remote>/*` ref that
/// is not a symbolic ref (excludes `refs/remotes/<remote>/HEAD`).
fn git_branch_refs() -> Result<Vec<BranchRef>, String> {
    let output = git_output(&[
        "for-each-ref",
        "--format=%(refname)%09%(refname:short)%09%(objectname)%09%(symref)",
        "refs/heads",
        "refs/remotes",
    ])?;
    let mut refs = Vec::new();
    for line in output.lines() {
        let mut parts = line.splitn(4, '\t');
        let (Some(full_name), Some(short_name), Some(oid), Some(symref)) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        if !symref.is_empty() {
            continue;
        }
        refs.push(BranchRef {
            full_name: full_name.to_string(),
            short_name: short_name.to_string(),
            oid: oid.to_string(),
        });
    }
    Ok(refs)
}

/// Class 1 (local) < Class 2 (`origin/*`) < Class 3 (other remotes) --
/// only reached in that priority order, per issue #89's Slice 3 spec.
fn branch_ref_class(full_name: &str) -> u8 {
    if full_name.starts_with("refs/heads/") {
        0
    } else if full_name.starts_with("refs/remotes/origin/") {
        1
    } else {
        2
    }
}

/// Groups refs by peeled commit OID (tracking configuration is not part
/// of the equivalence test), excludes every ref at the effective base
/// OID, and picks one display name per group: the lexicographically
/// smallest fully-qualified ref name among refs in the first non-empty
/// class. Returns a sorted, deterministic list of branch identifiers.
fn dedup_and_select_branches(refs: &[BranchRef], effective_base_oid: &str) -> Vec<String> {
    let mut by_oid: BTreeMap<&str, Vec<&BranchRef>> = BTreeMap::new();
    for r in refs {
        if r.oid == effective_base_oid {
            continue;
        }
        by_oid.entry(r.oid.as_str()).or_default().push(r);
    }

    let mut names: Vec<String> = by_oid
        .into_values()
        .map(|group| {
            let min_class = group
                .iter()
                .map(|r| branch_ref_class(&r.full_name))
                .min()
                .unwrap_or(2);
            group
                .into_iter()
                .filter(|r| branch_ref_class(&r.full_name) == min_class)
                .min_by(|left, right| left.full_name.cmp(&right.full_name))
                .map(|r| r.short_name.clone())
                .expect("each OID group has at least one member")
        })
        .collect();
    names.sort();
    names
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

fn candidate_scopes(coord_dir: &str) -> Vec<String> {
    let parent = Path::new(coord_dir)
        .parent()
        .unwrap_or(Path::new("docs"))
        .display();
    let name = Path::new(coord_dir)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    vec![
        format!("{coord_dir}/**/*.md"),
        format!("{parent}/{name}*.md (direct siblings only)"),
        format!("{parent}/{name}-archive/**/*.md"),
    ]
}

fn archive_dir(coord_dir: &str) -> String {
    format!("{coord_dir}-archive")
}

/// Two bounded tree queries: recursive channel/archive, and direct parent entries.
/// NUL framing preserves paths; only blob Markdown entries in the declared scope qualify.
fn git_candidate_inventory(oid: &str, coord_dir: &str) -> Result<BTreeMap<String, String>, String> {
    let parent = Path::new(coord_dir).parent().unwrap_or(Path::new("docs"));
    let name = Path::new(coord_dir)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let archive = archive_dir(coord_dir);
    let recursive = git_output(&["ls-tree", "-rz", oid, "--", coord_dir, &archive])?;
    let siblings = git_output(&[
        "ls-tree",
        "-z",
        oid,
        "--",
        &format!("{}/", parent.display()),
    ])?;
    let mut entries = BTreeMap::new();
    for entry in recursive
        .split('\0')
        .chain(siblings.split('\0'))
        .filter(|s| !s.is_empty())
    {
        let (header, path) = entry.split_once('\t').ok_or("incomplete tree entry")?;
        let fields: Vec<_> = header.split_whitespace().collect();
        if fields.len() != 3 {
            return Err("incomplete tree header".into());
        }
        if fields[1] != "blob" || !path.ends_with(".md") {
            continue;
        }
        let p = Path::new(path);
        let sibling = p.parent() == Some(parent)
            && p.file_name()
                .is_some_and(|f| f.to_string_lossy().starts_with(name.as_ref()));
        if p.starts_with(coord_dir) || p.starts_with(&archive) || sibling {
            entries.insert(path.to_string(), fields[2].to_string());
        }
    }
    Ok(entries)
}

#[allow(clippy::too_many_arguments)]
fn load_visibility_snapshot(
    oid: &str,
    coord_dir: &str,
    inventory: &BTreeMap<String, String>,
    base: &BTreeMap<String, String>,
    is_base: bool,
    diagnostics: &mut Vec<serde_json::Value>,
    invalid_channel: &mut Vec<serde_json::Value>,
    archive_baseline: &mut Vec<serde_json::Value>,
) -> Result<Vec<VisibilityMessage>, String> {
    let schema = agov_core::schema::load_and_compile(
        &agov_core::schema_dir().join("coordination-message.schema.json"),
    )
    .map_err(|err| format!("incomplete coverage: channel schema unavailable: {err}"))?;
    // Base channel entries are already validated/reported. Reusing exact
    // path/blob identity avoids repeating baseline invalid-record diagnostics
    // for every branch, while changed bytes and new paths are still inspected.
    let files: Vec<_> = inventory
        .iter()
        .filter(|(path, blob)| {
            is_base
                || !agov_coord::channel_reader::is_channel_path(
                    Path::new(path),
                    Path::new(coord_dir),
                )
                || base.get(*path) != Some(*blob)
        })
        .map(|(path, _)| path.clone())
        .collect();
    let mut messages = Vec::new();
    for (file, content) in git_cat_file_batch(oid, &files)? {
        let content =
            content.ok_or_else(|| format!("incomplete coverage: unreadable blob {oid}:{file}"))?;
        let channel =
            agov_coord::channel_reader::is_channel_path(Path::new(&file), Path::new(coord_dir));
        let validated =
            agov_coord::coord_validator::validate_content_with_schema(&content, &schema);
        if channel {
            match validated {
                Ok(msg) => {
                    let mut raw = msg.raw;
                    raw.as_object_mut()
                        .unwrap()
                        .insert("_body".into(), serde_json::json!(markdown_body(&content)));
                    messages.push(VisibilityMessage { file, raw });
                }
                Err(errors) => invalid_channel
                    .push(serde_json::json!({"ref_sha":oid,"path":file,"errors":errors})),
            }
            continue;
        }
        let blob = &inventory[&file];
        match validated {
            Ok(msg) => {
                let unchanged_archive = Path::new(&file).starts_with(archive_dir(coord_dir))
                    && base.get(&file) == Some(blob);
                let mut evidence = serde_json::json!({"ref_sha":oid,"path":file,"blob":blob,"id":msg.id(),"kind":msg.kind(),"to":msg.to(),"requires_ack":msg.requires_ack(),"handoff":msg.raw.get("handoff"),"lifecycle":"unassessed"});
                if unchanged_archive {
                    if is_base {
                        archive_baseline.push(evidence);
                    }
                } else {
                    evidence["diagnostic"] = serde_json::json!("misplaced_candidate");
                    diagnostics.push(evidence);
                }
            }
            Err(errors) => {
                let front = content
                    .strip_prefix("---")
                    .map(|s| s.split("---").next().unwrap_or(s));
                if front.is_some_and(|s| {
                    s.lines().any(|line| {
                        [
                            "kind:",
                            "from:",
                            "to:",
                            "references-to:",
                            "requires-ack:",
                            "handoff:",
                        ]
                        .iter()
                        .any(|key| line.trim_start().starts_with(key))
                    })
                }) {
                    diagnostics.push(serde_json::json!({"ref_sha":oid,"path":file,"blob":blob,"diagnostic":"invalid_candidate","errors":errors,"lifecycle":"unassessed"}));
                }
            }
        }
    }
    Ok(messages)
}

/// Issue #76: reads all `files` at `ref_name` via a single long-lived
/// `git cat-file --batch` subprocess (stdin/stdout streaming), instead of
/// one `git show <ref>:<file>` process per file -- turns `1 + F`
/// subprocess spawns per branch into `1 + 1` (one `ls-tree`, already
/// done by the caller, plus this one `cat-file --batch`). `git cat-file
/// --batch` accepts `<ref>:<path>` object specifiers directly on stdin,
/// so no separate blob-OID resolution pass is needed.
///
/// Returns one `(file, Some(content))` per readable file and
/// `(file, None)` for a file `cat-file` reports missing, preserving
/// `files`' order and length.
fn git_cat_file_batch(
    ref_name: &str,
    files: &[String],
) -> Result<Vec<(String, Option<String>)>, String> {
    use std::io::{BufRead, Read, Write};

    if files.is_empty() {
        return Ok(Vec::new());
    }

    let mut child = std::process::Command::new("git")
        .args(["cat-file", "--batch"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|err| format!("git cat-file --batch: spawn failed: {err}"))?;

    let mut stdin = child.stdin.take().ok_or("git cat-file --batch: no stdin")?;
    let specifiers: Vec<String> = files
        .iter()
        .map(|file| format!("{ref_name}:{file}"))
        .collect();
    let writer_specifiers = specifiers.clone();
    let writer = std::thread::spawn(move || -> Result<(), String> {
        for specifier in &writer_specifiers {
            writeln!(stdin, "{specifier}").map_err(|err| err.to_string())?;
        }
        // Dropping `stdin` here closes the pipe, signaling EOF so
        // `git cat-file --batch` exits after answering the last request.
        Ok(())
    });

    let stdout = child
        .stdout
        .take()
        .ok_or("git cat-file --batch: no stdout")?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or("git cat-file --batch: no stderr")?;
    let mut reader = std::io::BufReader::new(stdout);
    let mut results = Vec::with_capacity(files.len());

    // B-V26-G2-1: every early-exit path below sets `read_error` and
    // breaks, rather than silently returning a short `results` -- the
    // caller must never receive a shorter-than-requested response list
    // as if it were a normal "some files missing" outcome.
    let mut read_error: Option<String> = None;
    for specifier in &specifiers {
        let mut header = String::new();
        let read_bytes = match reader.read_line(&mut header) {
            Ok(n) => n,
            Err(err) => {
                read_error = Some(format!(
                    "git cat-file --batch: read failed for {specifier}: {err}"
                ));
                break;
            }
        };
        if read_bytes == 0 {
            read_error = Some(format!(
                "git cat-file --batch: unexpected end of output before a response for \
                 {specifier} ({} of {} responses received)",
                results.len(),
                specifiers.len()
            ));
            break;
        }
        let header = header.trim_end();
        if header.ends_with(" missing") {
            results.push(None);
            continue;
        }
        // Header format: "<oid> <type> <size>".
        let mut header_fields = header.rsplitn(3, ' ');
        let size_field = header_fields.next();
        let type_field = header_fields.next();
        let oid_field = header_fields.next();
        // B-V26-G2-R2-1: the object type must be checked, not merely
        // parsed and discarded. A syntactically valid but non-blob
        // header (e.g. `<oid> tree <size>`) must fail closed rather
        // than have its bytes treated as coordination-file content.
        let size: usize = match (
            oid_field,
            type_field,
            size_field.and_then(|v| v.parse().ok()),
        ) {
            (Some(_), Some("blob"), Some(size)) => size,
            (Some(_), Some(other_type), Some(_)) => {
                read_error = Some(format!(
                    "git cat-file --batch: unexpected object type {other_type:?} \
                     (expected blob) for {specifier}"
                ));
                break;
            }
            _ => {
                read_error = Some(format!(
                    "git cat-file --batch: malformed header for {specifier}: {header:?}"
                ));
                break;
            }
        };
        let mut content = vec![0u8; size];
        if let Err(err) = reader.read_exact(&mut content) {
            read_error = Some(format!(
                "git cat-file --batch: truncated content for {specifier} \
                 (expected {size} bytes): {err}"
            ));
            break;
        }
        // `cat-file --batch` appends a trailing newline after each
        // object's content, separate from the content itself.
        let mut trailing = [0u8; 1];
        if reader.read_exact(&mut trailing).is_err() {
            read_error = Some(format!(
                "git cat-file --batch: missing trailing separator for {specifier}"
            ));
            break;
        }
        match String::from_utf8(content) {
            Ok(content) => results.push(Some(content)),
            Err(_) => {
                read_error = Some(format!(
                    "git cat-file --batch: unreadable UTF-8 blob for {specifier}"
                ));
                break;
            }
        }
    }

    let writer_result = writer
        .join()
        .map_err(|_| "git cat-file --batch: writer thread panicked".to_string())?;

    let mut stderr_output = String::new();
    let _ = stderr.read_to_string(&mut stderr_output);
    let exit_status = child
        .wait()
        .map_err(|err| format!("git cat-file --batch: failed to wait on child: {err}"))?;

    if let Some(err) = read_error {
        return Err(err);
    }
    if let Err(err) = writer_result {
        return Err(format!("git cat-file --batch: writer failed: {err}"));
    }
    if !exit_status.success() {
        let stderr_note = if stderr_output.trim().is_empty() {
            String::new()
        } else {
            format!(": {}", stderr_output.trim())
        };
        return Err(format!(
            "git cat-file --batch: exited with {exit_status}{stderr_note}"
        ));
    }
    if results.len() != specifiers.len() {
        return Err(format!(
            "git cat-file --batch: expected {} response(s), got {}",
            specifiers.len(),
            results.len()
        ));
    }

    Ok(files.iter().cloned().zip(results).collect())
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

/// Issue #85: seq values colliding purely within base-branch history,
/// independent of which branch/base-branch pair is being compared.
/// Returned as a sorted list of `seq` values (the `legacy_base_collisions`
/// summary), not as blocking `VisibilityCollision` entries -- see
/// `render_visibility_text`/`visibility_json`/`visibility_exit_code`.
fn base_sequence_collisions_visibility(base_messages: &[VisibilityMessage]) -> Vec<i64> {
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
        .filter(|(_, files)| files.len() > 1)
        .map(|(seq, _)| seq)
        .collect()
}

fn branch_sequence_collisions(
    branch: &str,
    branch_messages: &[VisibilityMessage],
    base_messages: &[VisibilityMessage],
) -> Vec<BranchCollision> {
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
            collisions.push(BranchCollision {
                seq,
                branch: branch.to_string(),
                branch_file: message.file.clone(),
                base_file: base.file.clone(),
            });
        }
    }
    collisions.sort_by(|left, right| {
        (left.seq, left.branch.as_str(), left.branch_file.as_str()).cmp(&(
            right.seq,
            right.branch.as_str(),
            right.branch_file.as_str(),
        ))
    });
    collisions
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
    // Issue #89: always first, every exit path -- states which OID was
    // actually used as the comparison base.
    output.stdout(format!(
        "effective-base: {} ({})",
        report.effective_base_oid, report.effective_base_source
    ));

    output.stdout(format!(
        "coverage: complete within {:?}; refs: {:?}",
        report.scanned_scopes, report.scanned_refs
    ));
    output.stdout(format!(
        "archive baseline: {} record(s), lifecycle/closure unassessed",
        report.archive_baseline.len()
    ));
    for diagnostic in &report.candidate_diagnostics {
        output.stdout(format!("WARN {diagnostic}"));
    }
    for diagnostic in &report.invalid_channel_records {
        output.stdout(format!("WARN invalid_channel_record {diagnostic}"));
    }
    if report.candidate_diagnostics.is_empty()
        && report.invalid_channel_records.is_empty()
        && report.sequence_collisions.is_empty()
        && report.unbridged_obligations.is_empty()
        && report.legacy_base_collisions.is_empty()
    {
        output.stdout(format!(
            "OK coordination visibility clean relative to {}",
            report.base_branch
        ));
        return;
    }

    if !report.sequence_collisions.is_empty() {
        output.stdout("ERR sequence collision");
        for collision in &report.sequence_collisions {
            output.stdout(format!("  seq: {}", collision.seq));
            output.stdout(format!("  base: {}", collision.base_file));
            output.stdout(format!("  branch: {}", collision.branch));
            output.stdout(format!("  branch-file: {}", collision.branch_file));
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
            if obligation.addressed_elsewhere {
                output.stdout(format!(
                    "  addressed elsewhere: {}",
                    obligation.for_session.as_deref().unwrap_or("")
                ));
            }
        }
    }

    // Issue #85: base-history-only legacy collisions are reported as a
    // single summary count, matching `session resume`'s warn-once-and-
    // continue pattern -- never a per-collision ERR block, and always
    // rendered last so a genuine obligation is never buried under noise.
    if !report.legacy_base_collisions.is_empty() {
        output.stdout(format!(
            "WARN {} legacy sequence collision group(s) confined to base branch history; run agov coord validate for detail",
            report.legacy_base_collisions.len()
        ));
    }
}

fn visibility_json(report: &VisibilityReport) -> String {
    let obligations: Vec<serde_json::Value> = report
        .unbridged_obligations
        .iter()
        .map(|entry| {
            let mut value = serde_json::json!({
                "branch": entry.branch,
                "file": entry.file,
                "seq": entry.seq,
                "kind": entry.kind,
                "to": entry.to,
                "topic": entry.topic,
                "bridge": "missing",
            });
            if entry.addressed_elsewhere {
                value["addressed_elsewhere"] = serde_json::json!(entry.for_session);
            }
            value
        })
        .collect();
    let collisions: Vec<serde_json::Value> = report
        .sequence_collisions
        .iter()
        .map(|collision| {
            serde_json::json!({
                "type": "branch_base_seq_collision",
                "seq": collision.seq,
                "branch": collision.branch,
                "branch_file": collision.branch_file,
                "base_file": collision.base_file,
            })
        })
        .collect();

    serde_json::json!({
        "coverage": "complete_within_declared_scopes",
        "scanned_scopes": report.scanned_scopes,
        "scanned_refs": report.scanned_refs,
        "candidate_diagnostics": report.candidate_diagnostics,
        "invalid_channel_records": report.invalid_channel_records,
        "archive_baseline": {"count":report.archive_baseline.len(),"lifecycle":"unassessed","records":report.archive_baseline},
        "base_branch": report.base_branch,
        "effective_base_oid": report.effective_base_oid,
        "effective_base_source": report.effective_base_source,
        "scanned_branches": report.scanned_branches,
        "unbridged_obligations": obligations,
        "sequence_collisions": collisions,
        "legacy_base_collisions": {
            "count": report.legacy_base_collisions.len(),
            "seqs": report.legacy_base_collisions,
        },
        "errors": report.errors,
        "exit_code": visibility_exit_code(report),
    })
    .to_string()
}

fn visibility_exit_code(report: &VisibilityReport) -> i32 {
    if !report.sequence_collisions.is_empty() || !report.errors.is_empty() {
        2
    } else if !report.unbridged_obligations.is_empty()
        || !report.candidate_diagnostics.is_empty()
        || !report.invalid_channel_records.is_empty()
    {
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

        if let Some(message) = version_drift_warning() {
            warnings.push(message);
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
    git_output_at(Path::new("."), args)
}

fn git_output_at(repo_root: &Path, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .current_dir(repo_root)
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
    render_urgent_obligations(output, messages, Some(role));
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
    render_urgent_obligations(output, messages, None);
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
    // Fail-closed by construction, not by extra logic: this function only
    // ever reads `manifest` (the resolved AGOV_MANIFEST_PATH-or-default
    // path). It never falls back to reading MANIFEST.example.yaml as a
    // substitute -- there is no code path here that does so. See issue #79
    // / docs/plans/v250-rc1-contract-core.md.
    let Ok(content) = std::fs::read_to_string(manifest) else {
        output.stdout("Active roles: unavailable");
        if std::env::var_os("AGOV_MANIFEST_PATH").is_some() {
            output.stdout(format!(
                "No adopter manifest found at {} (from AGOV_MANIFEST_PATH).",
                manifest.display()
            ));
        } else {
            output.stdout(format!(
                "No adopter manifest found at {}; copy agf/agents/MANIFEST.example.yaml or set AGOV_MANIFEST_PATH.",
                manifest.display()
            ));
        }
        output.stdout("");
        return;
    };
    let Ok(data) = serde_norway::from_str::<serde_json::Value>(&content) else {
        output.stdout("Active roles: unavailable (manifest is not valid YAML)");
        output.stdout(format!("Fix {} and rerun.", manifest.display()));
        output.stdout("");
        return;
    };
    let Some(agents) = data.get("agents").and_then(|value| value.as_object()) else {
        output.stdout("Active roles: unavailable (manifest has no top-level 'agents' key)");
        output.stdout(format!("Fix {} and rerun.", manifest.display()));
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
            render_reply_evidence(output, message, messages, "  ");
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
                render_reply_evidence(output, message, messages, "      ");
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

// Urgency is advisory ordering only; lifecycle and addressing stay unchanged.
fn is_urgent(message: &HandoffMessage) -> bool {
    string_field(&message.raw, "urgency") == Some("urgent")
}

fn pending_ack_descriptors(
    messages: &[HandoffMessage],
) -> Vec<agov_coord::handoff_tracker::MessageDescriptor> {
    messages
        .iter()
        .map(|m| agov_coord::handoff_tracker::MessageDescriptor {
            file: m.file.clone(),
            sort_key: format!("{}_{}", m.sort_key.0, m.sort_key.1),
            refs: m.refs.clone(),
            msg: agov_coord::coord_validator::CoordMessage { raw: m.raw.clone() },
        })
        .collect()
}

fn render_urgent_obligations(
    output: &mut OutputCounter,
    messages: &[HandoffMessage],
    role: Option<&str>,
) {
    let descriptors = pending_ack_descriptors(messages);
    let pending =
        agov_coord::handoff_tracker::find_pending_all_kinds(&descriptors, "2026-08-02T22:01:54Z");
    let ordinary: std::collections::BTreeSet<_> = pending.iter().map(|p| p.file.as_str()).collect();
    let handoffs = pending_handoffs(messages);
    let handoff_files: std::collections::BTreeSet<_> =
        handoffs.iter().map(|p| p.file.as_str()).collect();
    let urgent: Vec<_> = messages
        .iter()
        .filter(|m| is_urgent(m))
        .filter(|m| {
            role.is_none_or(
                |r| matches!(string_field(&m.raw, "to"), Some(to) if to == r || to == "all"),
            )
        })
        .filter(|m| ordinary.contains(m.file.as_str()) || handoff_files.contains(m.file.as_str()))
        .collect();
    if urgent.is_empty() {
        return;
    }
    output.stdout("Urgent pending obligations (advisory):");
    for m in urgent {
        output.stdout(format!(
            "  - {}  {} -> {}  {}",
            m.file,
            string_field(&m.raw, "from").unwrap_or(""),
            string_field(&m.raw, "to").unwrap_or(""),
            string_field(&m.raw, "topic").unwrap_or("")
        ));
    }
    output.stdout("");
}

fn handoff_next(
    agent_role: &str,
    dir: PathBuf,
    session: Option<&str>,
    include_branch_local: bool,
    args: &[String],
) -> i32 {
    if include_branch_local {
        return handoff_branch::run(agent_role, dir, session, args);
    }
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
        .max_by_key(|message| (is_urgent(message), message.sort_key.clone()));

    let Some(message) = next else {
        output.stdout(format!("No handoff found for role: {agent_role}"));
        return 2;
    };

    render_handoff_summary(&mut output, &message);
    render_reply_evidence(&mut output, &message, &messages, "  ");

    if let Some(recipient) = addressed_elsewhere(&message.raw, session) {
        output.stdout(format!("Addressed elsewhere: {recipient}"));
    }
    agov_core::roi::record_tool_output(
        "agov.handoff.next",
        args,
        &format!("next-handoff:{agent_role}"),
        output.chars,
    );
    0
}

#[derive(Serialize)]
struct PendingAckRow {
    id: String,
    file: String,
    kind: String,
    from: String,
    to: String,
    topic: String,
    pending_since: String,
}

impl From<&agov_coord::handoff_tracker::PendingAck> for PendingAckRow {
    fn from(p: &agov_coord::handoff_tracker::PendingAck) -> Self {
        Self {
            id: p.id.clone(),
            file: p.file.clone(),
            kind: p.kind.clone(),
            from: p.from.clone(),
            to: p.to.clone(),
            topic: p.topic.clone(),
            pending_since: p.pending_since.clone(),
        }
    }
}

fn coord_pending(dir: PathBuf, cutoff: &str, format: &str, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let (messages, invalid_files) = match read_coord_messages_with_invalid(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!(
                "Coordination directory not found: {}",
                dir.display()
            ));
            return 1;
        }
    };

    let descriptors = pending_ack_descriptors(&messages);
    let urgent_files: std::collections::BTreeSet<_> = messages
        .iter()
        .filter(|m| is_urgent(m))
        .map(|m| m.file.as_str())
        .collect();
    let mut pending = agov_coord::handoff_tracker::find_pending_all_kinds(&descriptors, cutoff);
    // Stable sort preserves the existing ascending timestamp/file order inside each tier.
    pending.sort_by_key(|p| !urgent_files.contains(p.file.as_str()));

    let has_challenge_input = messages.iter().any(|m| m.raw.get("challenge").is_some());
    let (challenge_records, challenge_problems) =
        if !has_challenge_input && invalid_files.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            match agov_coord::challenge::read_directory(&dir) {
                Ok(data) => data,
                Err(error) => {
                    output.stderr(format!("Challenge evidence unavailable: {error}"));
                    return 1;
                }
            }
        };
    let challenges =
        agov_coord::challenge::evaluate(&challenge_records, time::OffsetDateTime::now_utc(), true);
    let has_challenge = challenge_records
        .iter()
        .any(|r| r.raw.get("challenge").is_some()) || !challenge_problems.is_empty();
    if format == "json" {
        let rows: Vec<PendingAckRow> = pending.iter().map(PendingAckRow::from).collect();
        // Preserve the legacy array for channels without challenge records.
        let value = if has_challenge {
            serde_json::json!({"pending": rows, "challenges": challenges, "problems": challenge_problems})
        } else {
            serde_json::json!(rows)
        };
        match serde_json::to_string_pretty(&value) {
            Ok(json) => output.stdout(json),
            Err(err) => {
                output.stderr(format!("ERR  could not render JSON: {err}"));
                return 1;
            }
        }
    } else if pending.is_empty() {
        output.stdout(format!("No pending requires-ack records since {cutoff}"));
    } else {
        output.stdout(format!("Pending requires-ack records since {cutoff}:"));
        for p in &pending {
            output.stdout(format!(
                "  {}  {}  {} -> {}  {}  {}",
                p.pending_since, p.kind, p.from, p.to, p.topic, p.file
            ));
        }
    }

    if format != "json" && has_challenge {
        output.stdout("Challenges (advisory; separate from ordinary pending count):");
        for row in &challenges {
            output.stdout(format!(
                "  {}  {}  {}  {}",
                row.challenge_id, row.state, row.file, row.detail
            ));
            for reply in &row.replies {
                output.stdout(format!("    reply evidence: {reply}"));
            }
        }
    }
    if !has_challenge || format != "json" {
        for problem in &challenge_problems {
            output.stderr(format!("Challenge evidence diagnostic: {problem}"));
        }
    }
    agov_core::roi::record_tool_output("agov.coord.pending", args, "coord-pending", output.chars);
    if pending.is_empty() && challenges.is_empty() && challenge_problems.is_empty() {
        2
    } else {
        0
    }
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

/// Advisory evidence only, evaluated after the existing unresolved filter.
/// Structured references are deliberate: prose/indirect graph edges are not replies.
#[derive(Debug, PartialEq, Serialize)]
struct ReplyEvidence {
    id: String,
    from: String,
}

fn reply_evidence(handoff: &HandoffMessage, messages: &[HandoffMessage]) -> Vec<ReplyEvidence> {
    let Some(timestamp) = string_field(&handoff.raw, "timestamp")
        .and_then(|s| time::OffsetDateTime::parse(s, &Rfc3339).ok())
    else {
        return Vec::new();
    };
    let recipient = handoff
        .raw
        .get("handoff")
        .and_then(|h| h.get("next_agent"))
        .and_then(|v| v.as_str());
    messages
        .iter()
        .filter(|m| {
            matches!(string_field(&m.raw, "kind"), Some("note" | "review"))
                && string_field(&m.raw, "from")
                    .is_some_and(|role| Some(role) == recipient || role == "governor-reviewer")
                && string_field(&m.raw, "timestamp")
                    .and_then(|s| time::OffsetDateTime::parse(s, &Rfc3339).ok())
                    .is_some_and(|t| t > timestamp)
                && frontmatter_refs(&m.raw).contains(&handoff.file)
        })
        .map(|m| ReplyEvidence {
            id: string_field(&m.raw, "id").unwrap_or("").to_string(),
            from: string_field(&m.raw, "from").unwrap_or("").to_string(),
        })
        .collect()
}

fn render_reply_evidence(
    output: &mut OutputCounter,
    handoff: &HandoffMessage,
    messages: &[HandoffMessage],
    indent: &str,
) {
    let evidence = reply_evidence(handoff, messages);
    if !evidence.is_empty() {
        output.stdout(format!(
            "{indent}reply present — acknowledgement unconfirmed; reply_evidence: {}",
            serde_json::to_string(&evidence).expect("reply evidence serializes")
        ));
    }
}


fn handoff_for_session(raw: &serde_json::Value) -> Option<&str> {
    if string_field(raw, "kind") != Some("handoff") {
        return None;
    }
    raw.get("handoff")
        .and_then(|h| h.get("for_session"))
        .and_then(|s| s.as_str())
}

fn addressed_elsewhere<'a>(raw: &'a serde_json::Value, session: Option<&str>) -> Option<&'a str> {
    let recipient = handoff_for_session(raw)?;
    session
        .filter(|requested| *requested != recipient)
        .map(|_| recipient)
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

    if valid_messages
        .iter()
        .any(|(_, m)| m.raw.get("challenge").is_some())
    {
        let dirs: BTreeSet<_> = valid_paths.iter().filter_map(|p| p.parent()).collect();
        for dir in dirs {
            match agov_coord::challenge::read_directory(dir) {
                Ok((records, _)) => {
                    for (file, error) in agov_coord::challenge::duplicate_ids(&records) {
                        failed = true;
                        output.coord_err(&dir.join(file).to_string_lossy(), &error);
                    }
                }
                Err(error) => {
                    failed = true;
                    output.coord_err(&dir.to_string_lossy(), &error);
                }
            }
        }
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

fn plan_projection_check(
    base_ref: &str,
    allow_planned: bool,
    files: Vec<std::path::PathBuf>,
    args: &[String],
) -> i32 {
    let mut failed = false;
    let mut output = OutputCounter::default();

    let repo_root = match git_output(&["rev-parse", "--show-toplevel"]) {
        Ok(path) => PathBuf::from(path),
        Err(reason) => {
            output.stderr(format!("ERR projection-check failed: {reason}"));
            return 2;
        }
    };

    let changed_output = match git_output(&[
        "diff",
        "--name-only",
        &format!("{base_ref}...HEAD"),
        "--",
        "agf/docs/site/docs/**/*.md",
    ]) {
        Ok(output) => output,
        Err(reason) => {
            output.stderr(format!(
                "ERR projection-check failed to resolve comparison base {base_ref}: {reason}"
            ));
            return 2;
        }
    };
    let changed_pages_or_runbooks = changed_output
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect::<std::collections::BTreeSet<_>>();
    let schema_path = agov_core::schema_dir().join("rule-projection-manifest.schema.json");
    let distribution = match dist::validate_default() {
        Ok(spec) => spec,
        Err(errors) => {
            output.stderr(format!(
                "ERR projection-check could not validate distribution manifest: {}",
                errors.join("; ")
            ));
            return 2;
        }
    };

    for file in files {
        let label = file.to_string_lossy().to_string();
        match agov_coord::projection_manifest::validate_file_against(
            &file,
            &repo_root,
            &schema_path,
            allow_planned,
            &changed_pages_or_runbooks,
            |path| distribution.contains(path),
        ) {
            Ok(_) => output.coord_ok(&label),
            Err(errors) => {
                failed = true;
                render_coord_errors(&mut output, &label, &errors);
            }
        }
    }

    if failed {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, "plan-projection-check", output.chars);
        0
    }
}

fn plan_projection_check_all(
    base_ref: &str,
    requested_repo_root: Option<PathBuf>,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let explicit_repo_root = requested_repo_root.is_some();
    let repo_root = match requested_repo_root {
        Some(path) => match path.canonicalize() {
            Ok(path) => path,
            Err(error) => {
                output.stderr(format!(
                    "ERR projection-check-all could not resolve repository root {}: {error}",
                    path.display()
                ));
                return 2;
            }
        },
        None => match git_output(&["rev-parse", "--show-toplevel"]) {
            Ok(path) => PathBuf::from(path),
            Err(reason) => {
                output.stderr(format!("ERR projection-check-all failed: {reason}"));
                return 2;
            }
        },
    };

    if let Err(reason) = git_output_at(&repo_root, &["rev-parse", "--show-toplevel"]) {
        output.stderr(format!(
            "ERR projection-check-all repository root {} is not a Git worktree: {reason}",
            repo_root.display()
        ));
        return 2;
    }

    let schema_path = if explicit_repo_root {
        repo_root.join("agf/schemas/rule-projection-manifest.schema.json")
    } else {
        agov_core::schema_dir().join("rule-projection-manifest.schema.json")
    };
    let distribution = match dist::validate(Path::new("agf/dist/release-manifest.yaml"), &repo_root)
    {
        Ok(spec) => spec,
        Err(errors) => {
            output.stderr(format!(
                "ERR projection-check-all could not validate distribution manifest: {}",
                errors.join("; ")
            ));
            return 2;
        }
    };

    let changed_output = match git_output_at(
        &repo_root,
        &[
            "diff",
            "--name-only",
            &format!("{base_ref}...HEAD"),
            "--",
            "agf/docs/site/docs/**/*.md",
            "docs/reviews/rule-projection/*.yaml",
        ],
    ) {
        Ok(output) => output,
        Err(reason) => {
            output.stderr(format!(
                "ERR projection-check-all failed to resolve comparison base {base_ref}: {reason}"
            ));
            return 2;
        }
    };
    let changed_paths = changed_output
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let changed_pages_or_runbooks = changed_paths
        .iter()
        .filter(|path| path.starts_with("agf/docs/site/docs/") && path.ends_with(".md"))
        .cloned()
        .collect::<BTreeSet<_>>();

    let manifest_dir = repo_root.join("docs/reviews/rule-projection");
    let entries = match std::fs::read_dir(&manifest_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            output.stderr(format!(
                "ERR projection-check-all manifest directory not found: {}",
                manifest_dir.display()
            ));
            return 2;
        }
        Err(error) => {
            output.stderr(format!(
                "ERR projection-check-all could not read {}: {error}",
                manifest_dir.display()
            ));
            return 2;
        }
    };
    let mut manifests = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("yaml"))
        .collect::<Vec<_>>();
    manifests.sort();

    let mut failed = false;
    for manifest in manifests {
        let relative = manifest
            .strip_prefix(&repo_root)
            .unwrap_or(&manifest)
            .to_string_lossy()
            .replace('\\', "/");
        let full_tree_result = agov_coord::projection_manifest::validate_file(
            &manifest,
            &repo_root,
            &schema_path,
            true,
        );
        if let Err(errors) = full_tree_result {
            failed = true;
            render_coord_errors(
                &mut output,
                &format!("{relative} [full-tree path/lifecycle]"),
                &errors,
            );
            continue;
        }

        if changed_paths.contains(&relative) {
            if let Err(errors) = agov_coord::projection_manifest::validate_file_against(
                &manifest,
                &repo_root,
                &schema_path,
                true,
                &changed_pages_or_runbooks,
                |path| distribution.contains(path),
            ) {
                failed = true;
                render_coord_errors(&mut output, &format!("{relative} [diff coverage]"), &errors);
                continue;
            }
        }
        output.coord_ok(&relative);
    }

    if failed {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, "plan-projection-check-all", output.chars);
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
        render_version_drift(&mut output),
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

/// Compare the running `agov` binary's own build version against the
/// commissioned `agf/VERSION` of the tree it is invoked against.
/// Returns `Some(warning line)` on mismatch; `None` if `agf/VERSION` is
/// absent or matches -- nothing to report either way.
fn version_drift_warning() -> Option<String> {
    let commissioned = agov_core::commissioned_version()?;
    let running = env!("CARGO_PKG_VERSION");
    if commissioned == running {
        None
    } else {
        Some(format!(
            "agov binary is v{running} but commissioned agf/ is v{commissioned} -- \
             results may be inaccurate if schema, subcommands, or validation rules \
             changed between versions"
        ))
    }
}

fn render_version_drift(output: &mut OutputCounter) -> i32 {
    render_version_drift_result(output, version_drift_warning())
}

/// Pure rendering step, separated from `version_drift_warning()`'s own
/// filesystem read so the advisory/never-blocking contract is directly
/// unit-testable without needing a full validator-passing fixture (issue
/// #114 round-1 review `2ggk`, finding I114-R1: the prior integration
/// test asserted only that the WARN line appeared, never that the
/// overall result stayed non-blocking, since its minimal fixture already
/// failed unrelated validators for unrelated reasons).
fn render_version_drift_result(output: &mut OutputCounter, warning: Option<String>) -> i32 {
    let label = "agov.validate.version_drift";
    match warning {
        Some(message) => {
            output.stdout(format!("WARN {label}: agf/VERSION: {message}"));
            0 // advisory only — never blocking
        }
        None => {
            output.stdout(format!("OK {label}"));
            0
        }
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

    /// Gate 2 review `d8h5` BLOCK 4: walks the live `clap::Command` tree
    /// (not `--help` text, not a hand-maintained list) and cross-checks it
    /// exactly against `agov_core::roi::COMMAND_PATH_TOKENS` and
    /// `KNOWN_FLAG_NAMES`. Fails on either direction of drift: a new
    /// command/flag added to the CLI without an explicit redact_args
    /// classification, or a stale/dead allowlist entry that no longer
    /// corresponds to anything real (both classes of bug `d8h5` found).
    #[test]
    fn roi_redaction_audit_matches_real_clap_surface() {
        use clap::CommandFactory;

        // `--help` is auto-added by clap at every level and excluded
        // unconditionally. `--version` is auto-added by clap only at the
        // *root* command (from `#[command(... version)]` on `Cli`); a
        // `version` flag on any subcommand (e.g. `dist provenance-record
        // file --version`, the artifact's product version) is real and
        // must NOT be excluded, so root and subcommands are walked
        // differently.
        // Collects the primary long flag AND every long alias (visible or
        // hidden) -- `d8h5`'s narrowed BLOCK 4 found `--base_branch`, a
        // hidden-by-default alias for `--base-branch` on `coord
        // visibility`/`session resume`, missing from an earlier revision
        // of this audit that only walked `get_long()`.
        fn walk_flags(command: &clap::Command, flags: &mut BTreeSet<String>) {
            for arg in command.get_arguments() {
                if let Some(long) = arg.get_long() {
                    if long != "help" {
                        flags.insert(format!("--{long}"));
                    }
                }
                if let Some(aliases) = arg.get_all_aliases() {
                    for alias in aliases {
                        flags.insert(format!("--{alias}"));
                    }
                }
            }
        }

        fn walk(
            command: &clap::Command,
            subcommands: &mut BTreeSet<String>,
            flags: &mut BTreeSet<String>,
        ) {
            walk_flags(command, flags);
            for sub in command.get_subcommands() {
                if sub.get_name() == "help" {
                    continue;
                }
                subcommands.insert(sub.get_name().to_string());
                walk(sub, subcommands, flags);
            }
        }

        let mut subcommands = BTreeSet::new();
        let mut flags = BTreeSet::new();
        let root = Cli::command();
        for arg in root.get_arguments() {
            if let Some(long) = arg.get_long() {
                if long != "help" && long != "version" {
                    flags.insert(format!("--{long}"));
                }
            }
        }
        for sub in root.get_subcommands() {
            if sub.get_name() == "help" {
                continue;
            }
            subcommands.insert(sub.get_name().to_string());
            walk(sub, &mut subcommands, &mut flags);
        }

        let expected_subcommands: BTreeSet<String> = agov_core::roi::COMMAND_PATH_TOKENS
            .iter()
            .map(|token| token.to_string())
            .collect();
        let expected_flags: BTreeSet<String> = agov_core::roi::KNOWN_FLAG_NAMES
            .iter()
            .map(|flag| flag.to_string())
            .collect();

        let missing_subcommands: Vec<_> = subcommands.difference(&expected_subcommands).collect();
        let stale_subcommands: Vec<_> = expected_subcommands.difference(&subcommands).collect();
        assert!(
            missing_subcommands.is_empty() && stale_subcommands.is_empty(),
            "COMMAND_PATH_TOKENS drift from the real clap surface — \
             missing (real, not classified): {missing_subcommands:?}; \
             stale (classified, not real): {stale_subcommands:?}"
        );

        let missing_flags: Vec<_> = flags.difference(&expected_flags).collect();
        let stale_flags: Vec<_> = expected_flags.difference(&flags).collect();
        assert!(
            missing_flags.is_empty() && stale_flags.is_empty(),
            "KNOWN_FLAG_NAMES drift from the real clap surface — \
             missing (real, not classified): {missing_flags:?}; \
             stale (classified, not real): {stale_flags:?}"
        );

        // Every CLOSED_VALUE_FLAGS entry must also be a recognized flag
        // name -- a closed-value flag that isn't even in KNOWN_FLAG_NAMES
        // would never reach the value-retention branch in redact_args.
        for closed in agov_core::roi::CLOSED_VALUE_FLAGS {
            assert!(
                expected_flags.contains(*closed),
                "{closed} is in CLOSED_VALUE_FLAGS but not in KNOWN_FLAG_NAMES"
            );
        }
    }

    #[test]
    fn pipeline_identity_from_both_present_succeeds() {
        let pipeline_id = Some("123".to_string());
        let job_id = Some("456".to_string());
        let identity = pipeline_identity_from(&pipeline_id, &job_id).unwrap();
        assert!(identity.is_some());
    }

    #[test]
    fn pipeline_identity_from_neither_present_succeeds_with_none() {
        let identity = pipeline_identity_from(&None, &None).unwrap();
        assert!(identity.is_none());
    }

    #[test]
    fn negative_pipeline_identity_from_pipeline_id_without_job_id_is_rejected() {
        let pipeline_id = Some("123".to_string());
        let result = pipeline_identity_from(&pipeline_id, &None);
        assert!(result.is_err());
    }

    #[test]
    fn negative_pipeline_identity_from_job_id_without_pipeline_id_is_rejected() {
        let job_id = Some("456".to_string());
        let result = pipeline_identity_from(&None, &job_id);
        assert!(result.is_err());
    }

    // ── write_bytes_safely: pure filesystem safety, no network needed ───

    fn temp_test_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "agov-download-safely-test-{label}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn positive_write_bytes_safely_writes_fresh_destination() {
        let dir = temp_test_dir("positive");
        let dest = dir.join("dest.txt");

        let result = write_bytes_safely(b"real content", &dest);
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(std::fs::read(&dest).unwrap(), b"real content");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn negative_write_bytes_safely_refuses_preexisting_symlink_destination() {
        let dir = temp_test_dir("symlink");
        let escape_target = dir.join("outside-the-download-dir.txt");
        let dest = dir.join("dest.txt");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&escape_target, &dest).unwrap();

        let result = write_bytes_safely(b"real content", &dest);
        assert!(
            result.is_err(),
            "must refuse to write through a pre-positioned symlink"
        );
        assert!(
            !escape_target.exists(),
            "the symlink target must never have been written to"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn negative_write_bytes_safely_refuses_preexisting_regular_file_destination() {
        let dir = temp_test_dir("existing-file");
        let dest = dir.join("dest.txt");
        std::fs::write(&dest, b"original content").unwrap();

        let result = write_bytes_safely(b"new content", &dest);
        assert!(result.is_err(), "must refuse to overwrite an existing file");
        assert_eq!(
            std::fs::read(&dest).unwrap(),
            b"original content",
            "the pre-existing file must not have been truncated or overwritten"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ── curl_get_bytes_validated / parse_curl_write_out_response ────────

    #[test]
    fn parse_curl_write_out_response_parses_success() {
        let stdout = b"body bytes here\n200 ".to_vec();
        let (body, code, redirect) = parse_curl_write_out_response(&stdout).unwrap();
        assert_eq!(body, b"body bytes here");
        assert_eq!(code, 200);
        assert_eq!(redirect, None);
    }

    #[test]
    fn parse_curl_write_out_response_parses_redirect_with_location() {
        let stdout = b"\n302 https://gitlab.com/next-hop".to_vec();
        let (body, code, redirect) = parse_curl_write_out_response(&stdout).unwrap();
        assert_eq!(body, b"");
        assert_eq!(code, 302);
        assert_eq!(redirect.as_deref(), Some("https://gitlab.com/next-hop"));
    }

    #[test]
    fn parse_curl_write_out_response_preserves_body_containing_newlines() {
        let stdout = b"line one\nline two\nline three\n200 ".to_vec();
        let (body, code, _) = parse_curl_write_out_response(&stdout).unwrap();
        assert_eq!(body, b"line one\nline two\nline three");
        assert_eq!(code, 200);
    }

    #[test]
    fn negative_parse_curl_write_out_response_rejects_missing_trailer() {
        assert!(parse_curl_write_out_response(b"no trailing newline at all").is_err());
    }

    #[test]
    fn negative_parse_curl_write_out_response_rejects_non_numeric_status() {
        let stdout = b"body\nnot-a-code ".to_vec();
        assert!(parse_curl_write_out_response(&stdout).is_err());
    }

    #[test]
    fn negative_curl_get_bytes_validated_rejects_wrong_initial_host() {
        let result = curl_get_bytes_validated("https://attacker.example/asset", "gitlab.com");
        assert!(result.is_err(), "{result:?}");
        assert!(result.unwrap_err().contains("untrusted host"));
    }

    #[test]
    fn executable_target_platform_returns_a_known_triple_or_none() {
        // Whatever this host is, the result (if any) must be one of the
        // triples the release pipeline actually produces archives for.
        if let Some(target) = executable_target_platform() {
            assert!(
                target == "x86_64-unknown-linux-musl" || target == "aarch64-unknown-linux-musl",
                "unexpected target: {target}"
            );
        }
    }

    #[test]
    fn discover_release_assets_parses_well_formed_links() {
        let json = serde_json::json!({
            "assets": {
                "links": [
                    {"name": "agf-2.4.0.tar.gz", "direct_asset_url": "https://gitlab.com/a"},
                    {"name": "agf-2.4.0.zip", "direct_asset_url": "https://gitlab.com/b"}
                ]
            }
        })
        .to_string();
        let assets = discover_release_assets(&json).unwrap();
        assert_eq!(assets.len(), 2);
    }

    #[test]
    fn negative_discover_release_assets_fails_closed_on_malformed_link() {
        // A link missing "name" must fail discovery outright, not silently
        // shrink the release set to just the well-formed entries.
        let json = serde_json::json!({
            "assets": {
                "links": [
                    {"name": "agf-2.4.0.tar.gz", "direct_asset_url": "https://gitlab.com/a"},
                    {"direct_asset_url": "https://gitlab.com/b"}
                ]
            }
        })
        .to_string();
        let result = discover_release_assets(&json);
        assert!(result.is_err(), "{result:?}");
    }

    // ── Composed post-publication checks: the exact functions
    // post_publication_check_command calls, exercised directly against
    // real registry data / a real packaged binary, not just their
    // standalone CLI-flag counterparts (`--oci-image`,
    // `self-report-check`). `#[ignore]`d for the same reason as
    // `tests/oci_pull.rs`: missing podman/network is an infrastructure
    // failure for these tests, not something they should silently pass
    // through.

    #[test]
    #[ignore = "requires a real podman binary and real network access; run explicitly via \
                `cargo test -p agov --bin agov -- --ignored composed_oci_chain_check`, as the \
                dedicated rust:tooling:oci-pull-test CI job does"]
    fn composed_oci_chain_check_rejects_real_busybox_image_with_wrong_labels() {
        // Exercises the exact function post_publication_check_command
        // calls -- discover the oci-image record on disk, build the
        // digest-pinned pull ref, podman pull it for real, inspect it,
        // and compare -- not the adjacent `--oci-image` CLI flag path.
        const DIGEST: &str =
            "sha256:fd8d9aa63ba2f0982b5304e1ee8d3b90a210bc1ffb5314d980eb6962f1a9715d";
        const REF: &str = "docker.io/library/busybox";

        let dir = std::env::temp_dir().join(format!(
            "agov-composed-oci-check-test-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let record_path = dir.join("busybox-provenance.json");
        let record = serde_json::json!({
            "schema_version": "2.4",
            "product": "agov",
            "artifact_kind": "oci-image",
            "version": "2.4.0",
            "source_commit": "abc123def456",
            "tag": "v2.4.0",
            "build_timestamp": "2026-07-22T00:00:00Z",
            "implementation": "rust",
            "packaging": "oci-image",
            "image_reference": REF,
            "manifest_digest": DIGEST,
            "labels": {
                "version": "2.4.0",
                "revision": "abc123def456"
            }
        });
        std::fs::write(&record_path, serde_json::to_string_pretty(&record).unwrap()).unwrap();

        let mut output = OutputCounter::default();
        let exit_code = composed_oci_chain_check(&[record_path], &mut output);
        let _ = std::fs::remove_dir_all(&dir);

        assert_ne!(
            exit_code, 0,
            "expected rejection: busybox carries no matching org.opencontainers.image labels"
        );
    }

    #[test]
    #[ignore = "requires this host to be able to execute the real compiled agov binary under \
                its own reported target_platform; run explicitly via \
                `cargo test -p agov --bin agov -- --ignored composed_self_report_check`"]
    fn composed_self_report_check_runs_real_binary_end_to_end() {
        // Exercises the exact function post_publication_check_command
        // calls, against a real archive containing the *actual compiled
        // agov binary* (not a fake fixture script) -- download-dir layout,
        // extraction, execution, and comparison against real self-report
        // ground truth all in one composed call.
        let Some(target) = executable_target_platform() else {
            panic!("this host has no known executable target_platform to test against");
        };

        // CARGO_BIN_EXE_agov is only set for integration test binaries
        // under tests/, not for #[cfg(test)] unit tests compiled as part
        // of this same bin crate -- locate the real built binary
        // ourselves, relative to the workspace target directory `cargo
        // test` already populated (debug preferred; release as fallback
        // for a release-profile-only CI run).
        let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target")
            .canonicalize()
            .expect("workspace target directory must exist after `cargo build`/`cargo test`");
        let real_binary = [
            target_dir.join("debug/agov"),
            target_dir.join("release/agov"),
        ]
        .into_iter()
        .find(|path| path.is_file())
        .expect(
            "no built agov binary found under target/debug or target/release -- run \
                 `cargo build -p agov` first",
        );
        let self_report_output = std::process::Command::new(&real_binary)
            .args(["version", "--format", "json"])
            .output()
            .expect("run the real agov binary for ground truth");
        let ground_truth: serde_json::Value =
            serde_json::from_slice(&self_report_output.stdout).expect("parse real version JSON");
        let real_version = ground_truth["version"].as_str().unwrap().to_string();
        let real_commit = ground_truth["commit"].as_str().unwrap().to_string();
        // This binary may legitimately have been built from a dirty tree
        // (e.g. mid-development locally) -- the composed check must agree
        // with whatever ground truth actually is, not assume a clean
        // build. A dirty build is a real, correct rejection, not a test
        // failure.
        let real_dirty = ground_truth["dirty"].as_bool().unwrap();

        let dir = std::env::temp_dir().join(format!(
            "agov-composed-self-report-test-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();

        let artifact_filename = "agov-test-2.4.0-composed.tar.gz";
        let archive_path = dir.join(artifact_filename);
        {
            let file = std::fs::File::create(&archive_path).unwrap();
            let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut builder = tar::Builder::new(encoder);
            builder.append_path_with_name(&real_binary, "agov").unwrap();
            builder.finish().unwrap();
        }

        let record_path = dir.join(format!("{artifact_filename}-provenance.json"));
        let record = serde_json::json!({
            "schema_version": "2.4",
            "product": "agov",
            "artifact_kind": "file",
            "version": real_version,
            "source_commit": real_commit,
            "tag": "v2.4.0",
            "build_timestamp": "2026-07-22T00:00:00Z",
            "implementation": "rust",
            "packaging": "tar.gz",
            "target_platform": target,
            "artifact_filename": artifact_filename,
            "artifact_bytes": std::fs::metadata(&archive_path).unwrap().len(),
            "sha256": "0".repeat(64)
        });
        std::fs::write(&record_path, serde_json::to_string_pretty(&record).unwrap()).unwrap();

        let mut output = OutputCounter::default();
        let exit_code = composed_self_report_check(&dir, &[record_path], &mut output);
        let _ = std::fs::remove_dir_all(&dir);

        if real_dirty {
            assert_ne!(
                exit_code, 0,
                "the real binary reports a dirty build -- the composed check must reject it, \
                 not silently accept it"
            );
        } else {
            assert_eq!(
                exit_code, 0,
                "composed self-report check must pass a record matching the real binary's own \
                 clean version/commit (sha256 field is irrelevant to this check)"
            );
        }
    }

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
    fn reply_evidence_is_direct_later_recipient_or_governor_only() {
        let file = "20260621T000000Z-planner-architect-aaaa.md";
        let mut h = message(file, "handoff", "2026-06-21T00:00:00Z", &[]);
        h.raw["handoff"]["next_agent"] = serde_json::json!("coder-tester");
        let mut reply = message(
            "20260621T000100Z-coder-tester-bbbb.md",
            "note",
            "2026-06-21T00:01:00Z",
            &[file],
        );
        reply.raw["references-to"] = serde_json::json!([file.trim_end_matches(".md")]);
        for kind in ["note", "review"] {
            for role in ["coder-tester", "governor-reviewer"] {
                reply.raw["kind"] = serde_json::json!(kind);
                reply.raw["from"] = serde_json::json!(role);
                let ev = reply_evidence(&h, &[h.clone(), reply.clone()]);
                assert_eq!(
                    ev,
                    vec![ReplyEvidence {
                        id: "20260621T000100Z-coder-tester-bbbb".into(),
                        from: role.into()
                    }]
                );
                assert_eq!(pending_handoffs(&[h.clone(), reply.clone()]).len(), 1);
            }
        }
        for (field, value) in [
            ("from", serde_json::json!("planner-architect")),
            ("timestamp", serde_json::json!("2026-06-21T00:00:00Z")),
            ("timestamp", serde_json::json!("2026-06-21T01:00:00+01:00")),
            ("timestamp", serde_json::json!("bad timestamp")),
            ("references-to", serde_json::json!([])), // graph/prose edge alone is insufficient
            (
                "references-to",
                serde_json::json!(["20260621T000050Z-coder-tester-cccc"]),
            ),
            ("references-to", serde_json::json!("malformed")),
        ] {
            let mut invalid = reply.clone();
            invalid.raw[field] = value;
            assert!(reply_evidence(&h, &[invalid]).is_empty(), "{field}");
        }
        for kind in ["ack", "completion", "decision-record"] {
            let mut resolver = message(
                "20260621T000200Z-governor-reviewer-cccc.md",
                kind,
                "2026-06-21T00:02:00Z",
                &[&reply.file],
            );
            resolver.raw["from"] = serde_json::json!("governor-reviewer");
            assert!(pending_handoffs(&[h.clone(), reply.clone(), resolver]).is_empty());
        }
        let other = message(
            "20260621T000300Z-planner-architect-dddd.md",
            "handoff",
            "2026-06-21T00:03:00Z",
            &[],
        );
        let base = pending_handoffs(&[h.clone(), other.clone()]);
        let annotated = pending_handoffs(&[h, reply, other]);
        assert_eq!(
            base.iter().map(|m| (&m.file, &m.raw)).collect::<Vec<_>>(),
            annotated
                .iter()
                .map(|m| (&m.file, &m.raw))
                .collect::<Vec<_>>()
        );
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

    /// Issue #114 round-1 review `2ggk`, finding I114-R1: the version-drift
    /// check must remain advisory regardless of whether a mismatch is
    /// present. Unit-tests `render_version_drift_result` directly (rather
    /// than through a full-repo fixture that fails unrelated validators
    /// for unrelated reasons) so a future edit that accidentally turned
    /// this check blocking would fail this test specifically.
    #[test]
    fn version_drift_result_is_never_blocking() {
        let mut output = OutputCounter::default();
        let exit_on_mismatch = render_version_drift_result(
            &mut output,
            Some("agov binary is v0.0.1 but commissioned agf/ is v9.9.9".to_string()),
        );
        assert_eq!(
            exit_on_mismatch, 0,
            "mismatch must stay advisory, not blocking"
        );

        let mut output = OutputCounter::default();
        let exit_on_match = render_version_drift_result(&mut output, None);
        assert_eq!(exit_on_match, 0, "no-mismatch case must stay 0");
    }
}
