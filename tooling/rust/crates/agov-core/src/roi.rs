use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// A saved-token ROI record appended to `tmp/agov-token-roi.jsonl`.
///
/// Fields mirror the Elixir `Agov.TokenRoi` JSONL format (§8.2 of the plan).
/// Unknown workflows may omit the `saved_*` fields. The optional
/// `transport`/`adapter`/`tool` attribution fields (v2.4.0 Workstream C)
/// are self-reported workflow measurement, never authenticated identity —
/// see `docs/reference/tooling-token-roi.md`.
#[derive(Debug, serde::Serialize)]
pub struct RoiRecord {
    pub event: &'static str,
    pub timestamp: String,
    pub implementation: &'static str,
    pub command: String,
    pub args: Vec<String>,
    pub workflow: String,
    pub chars: u64,
    pub approx_tokens: u64,
    pub prompt_overhead_tokens: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approx_saved_min_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approx_saved_max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<&'static str>,
}

/// The closed vocabulary of `agov-mcp` tool names that may appear in the
/// `tool` attribution field. One source of truth for both sides of the
/// boundary: `agov-mcp` sets the tool marker only for names it has already
/// validated in `tool_cli_args`, and `resolve_transport` below refuses any
/// marker value not in this list. A parity test in `agov-mcp` asserts its
/// published tool definitions match this list exactly.
pub const MCP_TOOL_ALLOWLIST: &[&str] = &[
    "agov_validate",
    "agov_coord_validate",
    "agov_plan_validate",
    "agov_session_resume",
    "agov_handoff_next",
    "agov_handoff_take",
    "agov_handoff_complete",
    "agov_coord_post",
    "agov_coord_list",
];

/// Environment marker names the trusted local `agov-mcp` adapter sets on
/// the child `agov` process. Documented in
/// `docs/reference/tooling-token-roi.md`; self-reported, not verified.
pub const MCP_TRANSPORT_MARKER: &str = "AGOV_MCP_TRANSPORT";
pub const MCP_ADAPTER_MARKER: &str = "AGOV_MCP_ADAPTER";
pub const MCP_TOOL_MARKER: &str = "AGOV_MCP_TOOL";

/// Resolve transport attribution as one coherent tuple decision — never
/// three independently-parsed fields. Exactly three outcomes:
///
/// 1. **No markers present** → direct CLI: `(Some("cli"), None, None)`.
///    `transport: "cli"` is explicit so a new direct-CLI record is
///    distinguishable from a historical record predating these fields.
/// 2. **Exact allowlisted MCP tuple** (`AGOV_MCP_TRANSPORT=mcp`,
///    `AGOV_MCP_ADAPTER=agov-mcp`, `AGOV_MCP_TOOL` in
///    [`MCP_TOOL_ALLOWLIST`]) → `(Some("mcp"), Some("agov-mcp"),
///    Some(<allowlisted name>))`. Emitted values are these fixed
///    constants/allowlist entries — never the raw environment strings.
/// 3. **Any partial, malformed, or contradictory tuple** → `(None, None,
///    None)`: all three fields absent from the serialized record. This is
///    a third, distinct shape — a rejected self-report is not silently
///    reclassified as a clean `"cli"` record — and the source values that
///    produced the rejection are never persisted.
///
/// A local caller can still spoof a *valid* tuple; this is self-reported
/// workflow measurement by design, with no pseudo-authentication.
fn resolve_transport() -> (
    Option<&'static str>,
    Option<&'static str>,
    Option<&'static str>,
) {
    let transport = std::env::var(MCP_TRANSPORT_MARKER).ok();
    let adapter = std::env::var(MCP_ADAPTER_MARKER).ok();
    let tool = std::env::var(MCP_TOOL_MARKER).ok();

    match (transport.as_deref(), adapter.as_deref(), tool.as_deref()) {
        (None, None, None) => (Some("cli"), None, None),
        (Some("mcp"), Some("agov-mcp"), Some(tool_name)) => {
            match MCP_TOOL_ALLOWLIST
                .iter()
                .find(|allowed| **allowed == tool_name)
            {
                Some(allowed) => (Some("mcp"), Some("agov-mcp"), Some(allowed)),
                None => (None, None, None),
            }
        }
        _ => (None, None, None),
    }
}

/// Fixed replacement for redacted argument values.
pub const REDACTED: &str = "[redacted]";

/// Command-path tokens (subcommand / sub-subcommand names) retained by
/// `redact_args`. Closed vocabulary: the CLI's own compiled surface.
const COMMAND_PATH_TOKENS: &[&str] = &[
    "agov",
    "coord",
    "plan",
    "dist",
    "session",
    "handoff",
    "validate",
    "post",
    "list",
    "brief",
    "status",
    "stale",
    "graph",
    "graph-index",
    "visibility",
    "resume",
    "next",
    "take",
    "complete",
    "version",
    "usage-report",
    "build",
    "provenance-record",
    "validate-provenance",
    "self-report-check",
    "post-publication-check",
    "file",
    "oci",
];

/// Flags whose *values* are closed vocabulary and may be retained.
/// Everything else's value is redacted. Starter set per the approved plan;
/// extending it is an additive follow-up once real usage shows a flag is
/// both safe and useful.
const CLOSED_VALUE_FLAGS: &[&str] = &["--kind", "--id-format", "--format", "--implementation"];

/// Recognized flag names (long form). A flag name in this list is retained
/// (names are the CLI's compiled surface); its value is retained only if
/// the flag is also in [`CLOSED_VALUE_FLAGS`]. An argument token starting
/// with `-` that is NOT in this list is caller-controlled and redacted in
/// its entirety — name and value — under default-deny.
const KNOWN_FLAG_NAMES: &[&str] = &[
    "--kind",
    "--id-format",
    "--format",
    "--implementation",
    "--from",
    "--to",
    "--topic",
    "--ref",
    "--artifact",
    "--task",
    "--model",
    "--dir",
    "--role",
    "--base-branch",
    "--check",
    "--paths",
    "--files",
    "--since",
    "--id",
    "--product",
    "--version",
    "--source-commit",
    "--tag",
    "--build-timestamp",
    "--pipeline-id",
    "--job-id",
    "--packaging",
    "--target-platform",
    "--image-reference",
    "--manifest-digest",
    "--label-version",
    "--label-revision",
    "--output",
    "--archive",
    "--binary-name",
    "--record",
    "--artifact-dir",
    "--oci-image",
    "--releases-url",
    "--download-dir",
];

/// Default-deny redaction over the exact token stream received (v2.4.0
/// Workstream C, boundary 2). The caller passes the subcommand-and-flags
/// slice only — `argv[0]` never reaches this function.
///
/// Binding edge-case rules (from the approved plan):
/// - `--flag=value` is split; the name is matched as if space-separated,
///   the value follows the same retain/redact rule. No separate
///   equals-form allowlist.
/// - Everything after a bare `--` is positional values: redacted, never
///   matched against the flag-name allowlist.
/// - Positional non-flag tokens are redacted unless they are exact
///   allowlisted command-path tokens.
/// - Repeated flags: each occurrence evaluated independently.
/// - Short aliases: none exist in this CLI today, so any short-form token
///   has no allowlist mapping and is treated as unrecognized (redacted
///   whole).
/// - A flag with a missing value: the name is retained per the ordinary
///   rule; nothing panics, no token shift.
/// - Unknown/unrecognized flags: redacted **in their entirety** — an
///   unknown token is caller-controlled and can smuggle identifying
///   content in the name itself (`--customer-acme`), so neither name nor
///   value survives.
///
/// Values are replaced with [`REDACTED`] rather than dropped, so record
/// shape and argument-count diagnostics stay stable.
fn redact_args(args: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(args.len());
    let mut after_double_dash = false;
    // Whether the previous token was a retained flag name whose value is
    // NOT closed-vocabulary (so this token, if positional, is its value
    // and must be redacted) or a closed-value flag (value retained).
    let mut pending_value: Option<bool> = None; // Some(retain_value)

    for token in args {
        if after_double_dash {
            out.push(REDACTED.to_string());
            continue;
        }
        if token == "--" {
            after_double_dash = true;
            out.push(token.clone());
            pending_value = None;
            continue;
        }

        if let Some(retain) = pending_value.take() {
            if !token.starts_with('-') {
                out.push(if retain {
                    token.clone()
                } else {
                    REDACTED.to_string()
                });
                continue;
            }
            // Previous flag had a missing value; fall through and treat
            // this token on its own merits.
        }

        if let Some(eq_pos) = token.find('=').filter(|_| token.starts_with("--")) {
            let (name, value) = token.split_at(eq_pos);
            let value = &value[1..];
            if KNOWN_FLAG_NAMES.contains(&name) {
                if CLOSED_VALUE_FLAGS.contains(&name) {
                    out.push(format!("{name}={value}"));
                } else {
                    out.push(format!("{name}={REDACTED}"));
                }
            } else {
                out.push(REDACTED.to_string());
            }
            continue;
        }

        if token.starts_with('-') {
            if KNOWN_FLAG_NAMES.contains(&token.as_str()) {
                out.push(token.clone());
                pending_value = Some(CLOSED_VALUE_FLAGS.contains(&token.as_str()));
            } else {
                // Unknown flag: redact the token itself; its value (next
                // positional token) is redacted by the pending rule.
                out.push(REDACTED.to_string());
                pending_value = Some(false);
            }
            continue;
        }

        if COMMAND_PATH_TOKENS.contains(&token.as_str()) {
            out.push(token.clone());
        } else {
            out.push(REDACTED.to_string());
        }
    }

    out
}

/// Resolve the ROI log path from environment or default.
///
/// Returns `None` when logging is disabled via `AGOV_TOKEN_ROI=0/false/off`.
pub fn log_path() -> Option<PathBuf> {
    if std::env::var("AGOV_TOKEN_ROI")
        .is_ok_and(|value| matches!(value.as_str(), "0" | "false" | "off"))
    {
        return None;
    }

    Some(
        std::env::var_os("AGOV_TOKEN_ROI_LOG")
            .map(PathBuf::from)
            .unwrap_or_else(|| crate::repo_root().join("tmp").join("agov-token-roi.jsonl")),
    )
}

/// Write one pre-joined record line with exactly one `Write::write` call.
///
/// Deliberately NOT `write_all`: `write_all` retries in a loop until the
/// buffer is exhausted, so under a short write one logical record could
/// span multiple append operations and interleave with another appender.
/// One `write` call with the full buffer is one attempted append. A short
/// write (`n < line.len()`) is reported as an error, never retried — the
/// already-written partial bytes cannot be recalled, but the condition is
/// detected and surfaced instead of silently split across appends.
///
/// Scope of the guarantee: one attempted OS-level write per record. POSIX
/// `O_APPEND` makes each individual `write` atomic with respect to the
/// file offset, and local filesystems in practice accept a buffer of this
/// size (a few hundred bytes) in full; this is a documented
/// platform/filesystem expectation, not a universal atomicity claim.
fn write_line_once(writer: &mut impl std::io::Write, line: &[u8]) -> std::io::Result<()> {
    let written = writer.write(line)?;
    if written != line.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::WriteZero,
            format!(
                "short append: wrote {written} of {} bytes; record may be truncated on disk",
                line.len()
            ),
        ));
    }
    Ok(())
}

/// Append `record` to the ROI log as a JSONL line.
///
/// Failures are silently swallowed — logging must not affect stdout, stderr,
/// output ordering, or exit codes (plan §8.4).
///
/// Writes the JSON content and its trailing newline via [`write_line_once`]:
/// one attempted `write()` per record, never `writeln!` (two calls that can
/// interleave between content and newline) and never `write_all` (a retry
/// loop that can split one record across multiple appends on a short
/// write). See [`write_line_once`] for the exact guarantee and its
/// platform-expectation boundary.
pub fn try_append(record: &RoiRecord) {
    let Some(path) = log_path() else {
        return;
    };

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        let line = format!("{}\n", serde_json::to_string(record)?);
        write_line_once(&mut file, line.as_bytes())?;
        Ok(())
    })();

    if cfg!(any(test, debug_assertions)) {
        if let Err(err) = result {
            eprintln!("WARN agov token ROI logging failed: {err}");
        }
    }
}

pub fn record_tool_output(command: &str, args: &[String], workflow: &str, chars: u64) {
    let approx_tokens = estimate_tokens(chars);
    // Prompt-overhead estimation may use the in-memory raw invocation
    // locally (n8s6: allowed); only the redacted form is persisted below.
    let prompt_overhead_tokens = estimate_tokens(command_prompt_chars(command, args));
    let (transport, adapter, tool) = resolve_transport();
    let baseline = workflow_baseline(workflow);
    let (baseline_source, approx_saved_min_tokens, approx_saved_max_tokens) =
        if let Some(baseline) = baseline {
            (
                Some(baseline.source.to_string()),
                Some(saved_tokens(
                    baseline.min_tokens,
                    approx_tokens,
                    prompt_overhead_tokens,
                )),
                Some(saved_tokens(
                    baseline.max_tokens,
                    approx_tokens,
                    prompt_overhead_tokens,
                )),
            )
        } else {
            (None, None, None)
        };

    let record = RoiRecord {
        event: "tool_output",
        timestamp: timestamp(),
        implementation: "rust-agov",
        command: command.to_string(),
        args: redact_args(args),
        workflow: workflow.to_string(),
        chars,
        approx_tokens,
        prompt_overhead_tokens,
        baseline_source,
        approx_saved_min_tokens,
        approx_saved_max_tokens,
        transport,
        adapter,
        tool,
    };

    try_append(&record);
}

pub fn estimate_tokens(chars: u64) -> u64 {
    chars.div_ceil(4)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoiSummary {
    pub total_records: usize,
    pub total_chars: u64,
    pub total_approx_tokens: u64,
    pub manual_chars: u64,
    pub manual_approx_tokens: u64,
    pub estimated_saved_min_tokens: u64,
    pub estimated_saved_max_tokens: u64,
    pub commands: Vec<RoiCommandRow>,
    pub workflows: Vec<RoiWorkflowRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoiCommandRow {
    pub command: String,
    pub count: usize,
    pub chars: u64,
    pub approx_tokens: u64,
    pub approx_saved_min_tokens: u64,
    pub approx_saved_max_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoiWorkflowRow {
    pub workflow: String,
    pub count: usize,
    pub chars: u64,
    pub approx_tokens: u64,
    pub approx_saved_min_tokens: u64,
    pub approx_saved_max_tokens: u64,
}

pub fn default_report_path() -> PathBuf {
    std::env::var_os("AGOV_TOKEN_ROI_LOG")
        .map(PathBuf::from)
        .unwrap_or_else(|| crate::repo_root().join("tmp").join("agov-token-roi.jsonl"))
}

pub fn load_records(path: &Path) -> Result<Vec<serde_json::Value>, std::io::Error> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(err),
    };

    Ok(content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .collect())
}

pub fn summarize(records: &[serde_json::Value]) -> RoiSummary {
    let mut command_groups: BTreeMap<String, Vec<&serde_json::Value>> = BTreeMap::new();
    let mut workflow_groups: BTreeMap<String, Vec<&serde_json::Value>> = BTreeMap::new();

    for record in records {
        if let Some(command) = record.get("command").and_then(|value| value.as_str()) {
            command_groups
                .entry(command.to_string())
                .or_default()
                .push(record);
        }
        if let Some(workflow) = record.get("workflow").and_then(|value| value.as_str()) {
            workflow_groups
                .entry(workflow.to_string())
                .or_default()
                .push(record);
        }
    }

    let mut commands: Vec<RoiCommandRow> = command_groups
        .into_iter()
        .map(|(command, group)| RoiCommandRow {
            command,
            count: group.len(),
            chars: sum_field(&group, "chars"),
            approx_tokens: sum_field(&group, "approx_tokens"),
            approx_saved_min_tokens: sum_field(&group, "approx_saved_min_tokens"),
            approx_saved_max_tokens: sum_field(&group, "approx_saved_max_tokens"),
        })
        .collect();
    commands.sort_by(|left, right| {
        right
            .approx_tokens
            .cmp(&left.approx_tokens)
            .then_with(|| left.command.cmp(&right.command))
    });

    let mut workflows: Vec<RoiWorkflowRow> = workflow_groups
        .into_iter()
        .map(|(workflow, group)| RoiWorkflowRow {
            workflow,
            count: group.len(),
            chars: sum_field(&group, "chars"),
            approx_tokens: sum_field(&group, "approx_tokens"),
            approx_saved_min_tokens: sum_field(&group, "approx_saved_min_tokens"),
            approx_saved_max_tokens: sum_field(&group, "approx_saved_max_tokens"),
        })
        .collect();
    workflows.sort_by(|left, right| {
        right
            .approx_tokens
            .cmp(&left.approx_tokens)
            .then_with(|| left.workflow.cmp(&right.workflow))
    });

    let manual_chars = sum_field_values(records, "manual_chars");
    let total_chars = sum_field_values(records, "chars");

    RoiSummary {
        total_records: records.len(),
        total_chars,
        total_approx_tokens: estimate_tokens(total_chars),
        manual_chars,
        manual_approx_tokens: estimate_tokens(manual_chars),
        estimated_saved_min_tokens: sum_field_values(records, "approx_saved_min_tokens"),
        estimated_saved_max_tokens: sum_field_values(records, "approx_saved_max_tokens"),
        commands,
        workflows,
    }
}

pub fn format_report(summary: &RoiSummary, log_path: &Path) -> String {
    [
        "Token ROI telemetry".to_string(),
        format!("Log: {}", log_path.display()),
        format!("Records: {}", summary.total_records),
        format!(
            "Measured tool output: {} chars / {} approx tokens",
            summary.total_chars, summary.total_approx_tokens
        ),
        savings_line(summary),
        String::new(),
        format_command_rows(&summary.commands),
        String::new(),
        format_workflow_rows(&summary.workflows),
    ]
    .join("\n")
}

fn savings_line(summary: &RoiSummary) -> String {
    if summary.estimated_saved_min_tokens == 0
        && summary.estimated_saved_max_tokens == 0
        && summary.manual_chars == 0
    {
        return "Potential savings: none recorded".to_string();
    }

    let manual = if summary.manual_chars == 0 {
        "Manual baseline: automatic workflow baselines only".to_string()
    } else {
        format!(
            "Manual baseline: {} chars / {} approx tokens",
            summary.manual_chars, summary.manual_approx_tokens
        )
    };

    format!(
        "{manual}; potential saved: {}",
        savings_range(
            summary.estimated_saved_min_tokens,
            summary.estimated_saved_max_tokens
        )
    )
}

fn format_command_rows(rows: &[RoiCommandRow]) -> String {
    if rows.is_empty() {
        return "Commands: none".to_string();
    }

    std::iter::once("Commands:".to_string())
        .chain(rows.iter().map(|row| {
            format!(
                "  {:<24} {:>3} runs  {:>7} chars  {:>6} tokens  {:>18} saved",
                row.command,
                row.count,
                row.chars,
                row.approx_tokens,
                savings_range(row.approx_saved_min_tokens, row.approx_saved_max_tokens)
            )
        }))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_workflow_rows(rows: &[RoiWorkflowRow]) -> String {
    if rows.is_empty() {
        return "Workflows: none".to_string();
    }

    std::iter::once("Workflows:".to_string())
        .chain(rows.iter().map(|row| {
            format!(
                "  {:<28} {:>3} records  {:>7} chars  {:>6} tokens  {:>18} saved",
                row.workflow,
                row.count,
                row.chars,
                row.approx_tokens,
                savings_range(row.approx_saved_min_tokens, row.approx_saved_max_tokens)
            )
        }))
        .collect::<Vec<_>>()
        .join("\n")
}

fn savings_range(min: u64, max: u64) -> String {
    if min == 0 && max == 0 {
        "0".to_string()
    } else if min == max {
        min.to_string()
    } else {
        format!("{min}-{max}")
    }
}

fn timestamp() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

fn command_prompt_chars(command: &str, args: &[String]) -> u64 {
    std::iter::once(command)
        .chain(args.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .count() as u64
}

struct Baseline {
    min_tokens: u64,
    max_tokens: u64,
    source: &'static str,
}

fn workflow_baseline(workflow: &str) -> Option<Baseline> {
    let source = "docs/reference/tooling-token-roi.md#validation-baseline";
    match workflow {
        "coord-validate" => Some(Baseline {
            min_tokens: 500,
            max_tokens: 1_500,
            source,
        }),
        "plan-validate" => Some(Baseline {
            min_tokens: 800,
            max_tokens: 2_000,
            source,
        }),
        "validate:umbrella" => Some(Baseline {
            min_tokens: 1_500,
            max_tokens: 4_000,
            source,
        }),
        workflow if workflow.starts_with("validate:") => Some(Baseline {
            min_tokens: 500,
            max_tokens: 2_000,
            source,
        }),
        _ => None,
    }
}

fn saved_tokens(manual_tokens: u64, tool_tokens: u64, prompt_overhead_tokens: u64) -> u64 {
    manual_tokens.saturating_sub(tool_tokens + prompt_overhead_tokens)
}

fn sum_field(records: &[&serde_json::Value], field: &str) -> u64 {
    records
        .iter()
        .filter_map(|record| record.get(field).and_then(|value| value.as_u64()))
        .sum()
}

fn sum_field_values(records: &[serde_json::Value], field: &str) -> u64 {
    records
        .iter()
        .filter_map(|record| record.get(field).and_then(|value| value.as_u64()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn estimates_tokens_with_ceil_div_four() {
        assert_eq!(estimate_tokens(0), 0);
        assert_eq!(estimate_tokens(1), 1);
        assert_eq!(estimate_tokens(4), 1);
        assert_eq!(estimate_tokens(5), 2);
    }

    #[test]
    fn disable_switch_turns_off_log_path() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("AGOV_TOKEN_ROI", "off");
        std::env::remove_var("AGOV_TOKEN_ROI_LOG");
        assert_eq!(log_path(), None);
        std::env::remove_var("AGOV_TOKEN_ROI");
    }

    #[test]
    fn env_override_controls_log_path() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var("AGOV_TOKEN_ROI");
        std::env::set_var("AGOV_TOKEN_ROI_LOG", "/tmp/custom-agov-token-roi.jsonl");
        assert_eq!(
            log_path(),
            Some(PathBuf::from("/tmp/custom-agov-token-roi.jsonl"))
        );
        std::env::remove_var("AGOV_TOKEN_ROI_LOG");
    }

    #[test]
    fn default_log_path_is_repo_tmp() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var("AGOV_TOKEN_ROI");
        std::env::remove_var("AGOV_TOKEN_ROI_LOG");
        assert_eq!(
            log_path(),
            Some(crate::repo_root().join("tmp").join("agov-token-roi.jsonl"))
        );
    }

    /// Regression test for issue #50: concurrent writers used to corrupt
    /// the log by interleaving between `writeln!`'s two separate `write()`
    /// calls (content, then newline), producing lines with two or more
    /// JSON objects concatenated with no separator. Spawns real OS
    /// threads (not just async tasks) hammering `try_append` on the same
    /// file concurrently, then asserts every physical line parses as
    /// exactly one JSON object -- this reliably reproduced the
    /// corruption before the single-`write`-of-one-joined-line fix.
    ///
    /// This is observed-behaviour evidence on this host/filesystem, not
    /// proof of a syscall count -- the write-call contract itself is
    /// pinned by `write_line_once_makes_exactly_one_write_call` and
    /// `write_line_once_short_write_is_an_error_not_a_retry` below.
    #[test]
    fn concurrent_appends_never_corrupt_a_line() {
        let path = std::env::temp_dir().join(format!(
            "agov-roi-concurrency-test-{}-{}.jsonl",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var("AGOV_TOKEN_ROI");
        std::env::set_var("AGOV_TOKEN_ROI_LOG", &path);

        const THREADS: usize = 16;
        const RECORDS_PER_THREAD: usize = 50;
        let handles: Vec<_> = (0..THREADS)
            .map(|thread_id| {
                std::thread::spawn(move || {
                    for i in 0..RECORDS_PER_THREAD {
                        let record = RoiRecord {
                            event: "tool_output",
                            timestamp: "2026-07-22T00:00:00Z".to_string(),
                            implementation: "rust-agov",
                            command: "agov".to_string(),
                            args: vec![format!("thread-{thread_id}"), format!("record-{i}")],
                            workflow: "coord-validate".to_string(),
                            chars: 40,
                            approx_tokens: 10,
                            prompt_overhead_tokens: 5,
                            baseline_source: None,
                            approx_saved_min_tokens: None,
                            approx_saved_max_tokens: None,
                            transport: Some("cli"),
                            adapter: None,
                            tool: None,
                        };
                        try_append(&record);
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }

        std::env::remove_var("AGOV_TOKEN_ROI_LOG");

        let content = std::fs::read_to_string(&path).unwrap();
        let _ = std::fs::remove_file(&path);

        let lines: Vec<&str> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        assert_eq!(
            lines.len(),
            THREADS * RECORDS_PER_THREAD,
            "expected exactly one physical line per record -- a corrupted \
             (concatenated) line would reduce this count"
        );
        for line in &lines {
            assert!(
                serde_json::from_str::<serde_json::Value>(line).is_ok(),
                "line does not parse as exactly one JSON object (corruption): {line:?}"
            );
        }
    }

    /// Test writer that records every `write` call it receives and can
    /// simulate a short write on the first call.
    struct CountingWriter {
        calls: Vec<usize>,
        short_first_write: Option<usize>,
    }

    impl std::io::Write for CountingWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let n = match (self.calls.is_empty(), self.short_first_write) {
                (true, Some(short)) => short,
                _ => buf.len(),
            };
            self.calls.push(n);
            Ok(n)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    /// Pins the write-call contract: one logical record line is exactly one
    /// `write` call, never a `write_all`-style retry loop.
    #[test]
    fn write_line_once_makes_exactly_one_write_call() {
        let mut writer = CountingWriter {
            calls: Vec::new(),
            short_first_write: None,
        };
        let line = b"{\"event\":\"tool_output\"}\n";
        write_line_once(&mut writer, line).unwrap();
        assert_eq!(
            writer.calls,
            vec![line.len()],
            "expected exactly one write call covering the full line"
        );
    }

    /// Pins the short-write behaviour: a partial `write` is surfaced as an
    /// error and is NOT retried with the remaining bytes -- retrying is
    /// exactly the `write_all` interleaving hazard issue #50's fix exists
    /// to avoid.
    #[test]
    fn write_line_once_short_write_is_an_error_not_a_retry() {
        let mut writer = CountingWriter {
            calls: Vec::new(),
            short_first_write: Some(5),
        };
        let line = b"{\"event\":\"tool_output\"}\n";
        let err = write_line_once(&mut writer, line).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::WriteZero);
        assert!(
            err.to_string().contains("short append"),
            "error should name the short-append condition: {err}"
        );
        assert_eq!(
            writer.calls.len(),
            1,
            "short write must not be retried with the remaining bytes"
        );
    }

    // ── Workstream C: transport attribution (one coherent tuple) ──────────

    fn clear_mcp_markers() {
        std::env::remove_var(MCP_TRANSPORT_MARKER);
        std::env::remove_var(MCP_ADAPTER_MARKER);
        std::env::remove_var(MCP_TOOL_MARKER);
    }

    /// Acceptance test 7: no markers → explicit `transport: "cli"`.
    #[test]
    fn no_markers_resolves_to_explicit_cli() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_mcp_markers();
        assert_eq!(resolve_transport(), (Some("cli"), None, None));
    }

    /// Exact allowlisted tuple → all three MCP fields, from constants.
    #[test]
    fn exact_mcp_tuple_resolves_all_three_fields() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(MCP_TRANSPORT_MARKER, "mcp");
        std::env::set_var(MCP_ADAPTER_MARKER, "agov-mcp");
        std::env::set_var(MCP_TOOL_MARKER, "agov_coord_validate");
        let resolved = resolve_transport();
        clear_mcp_markers();
        assert_eq!(
            resolved,
            (Some("mcp"), Some("agov-mcp"), Some("agov_coord_validate"))
        );
    }

    /// Acceptance test 3: partial, malformed, or contradictory tuples emit
    /// none of the three fields — a third shape, distinct from the
    /// explicit-"cli" no-markers case, never the raw string.
    #[test]
    fn invalid_tuples_emit_no_attribution_fields() {
        let _guard = ENV_LOCK.lock().unwrap();

        // Partial: transport says mcp, tool marker missing.
        clear_mcp_markers();
        std::env::set_var(MCP_TRANSPORT_MARKER, "mcp");
        std::env::set_var(MCP_ADAPTER_MARKER, "agov-mcp");
        assert_eq!(resolve_transport(), (None, None, None));

        // Malformed: unrecognized transport value.
        clear_mcp_markers();
        std::env::set_var(MCP_TRANSPORT_MARKER, "MCP-ish");
        std::env::set_var(MCP_ADAPTER_MARKER, "agov-mcp");
        std::env::set_var(MCP_TOOL_MARKER, "agov_coord_validate");
        assert_eq!(resolve_transport(), (None, None, None));

        // Contradictory: tool name not in the compiled allowlist.
        clear_mcp_markers();
        std::env::set_var(MCP_TRANSPORT_MARKER, "mcp");
        std::env::set_var(MCP_ADAPTER_MARKER, "agov-mcp");
        std::env::set_var(MCP_TOOL_MARKER, "agov_totally_new_tool");
        assert_eq!(resolve_transport(), (None, None, None));

        // Stray marker alone (adapter set, nothing else).
        clear_mcp_markers();
        std::env::set_var(MCP_ADAPTER_MARKER, "agov-mcp");
        assert_eq!(resolve_transport(), (None, None, None));

        clear_mcp_markers();
    }

    /// Acceptance test 4: identifying / envelope / prompt / path / secret
    /// shaped marker values never reach the serialized record — the closed
    /// parse rejects the tuple and the raw strings are not persisted.
    #[test]
    fn hostile_marker_values_never_reach_serialized_record() {
        let _guard = ENV_LOCK.lock().unwrap();
        let hostile = [
            "session-0193fd2a-request-77",
            r#"{"jsonrpc":"2.0","method":"tools/call","id":9}"#,
            "user prompt: please summarize the private notes",
            "/home/someone/.ssh/id_ed25519",
            "sk-live-abcdef0123456789",
        ];
        for value in hostile {
            clear_mcp_markers();
            std::env::set_var(MCP_TRANSPORT_MARKER, "mcp");
            std::env::set_var(MCP_ADAPTER_MARKER, "agov-mcp");
            std::env::set_var(MCP_TOOL_MARKER, value);
            let (transport, adapter, tool) = resolve_transport();
            let record = RoiRecord {
                event: "tool_output",
                timestamp: "2026-07-22T00:00:00Z".to_string(),
                implementation: "rust-agov",
                command: "agov".to_string(),
                args: redact_args(&["coord".to_string(), "validate".to_string()]),
                workflow: "coord-validate".to_string(),
                chars: 10,
                approx_tokens: 3,
                prompt_overhead_tokens: 2,
                baseline_source: None,
                approx_saved_min_tokens: None,
                approx_saved_max_tokens: None,
                transport,
                adapter,
                tool,
            };
            let json = serde_json::to_string(&record).unwrap();
            assert!(
                !json.contains(value),
                "hostile marker value leaked into serialized record: {value}"
            );
            assert!(
                !json.contains("\"transport\"") && !json.contains("\"tool\""),
                "invalid tuple must serialize with no attribution fields: {json}"
            );
        }
        clear_mcp_markers();
    }

    // ── Workstream C: default-deny argument redaction ─────────────────────

    fn strings(tokens: &[&str]) -> Vec<String> {
        tokens.iter().map(|token| token.to_string()).collect()
    }

    /// Acceptance test 6 (positive): closed-vocabulary command path and
    /// closed-value flags survive unredacted.
    #[test]
    fn redact_args_retains_closed_vocabulary() {
        assert_eq!(
            redact_args(&strings(&["coord", "validate", "--kind", "review"])),
            strings(&["coord", "validate", "--kind", "review"])
        );
    }

    /// Acceptance test 5 (negative): free-form flag values are redacted.
    #[test]
    fn redact_args_redacts_free_form_flag_values() {
        assert_eq!(
            redact_args(&strings(&[
                "coord",
                "post",
                "--topic",
                "some sensitive-looking text",
                "--from",
                "coder-tester",
            ])),
            strings(&["coord", "post", "--topic", REDACTED, "--from", REDACTED])
        );
    }

    /// kheg BLOCK 1 negative tests: an unknown flag token is caller-
    /// controlled and is redacted in its entirety — both space-separated
    /// and equals forms — along with its value.
    #[test]
    fn redact_args_redacts_unknown_flags_entirely() {
        assert_eq!(
            redact_args(&strings(&["coord", "--customer-acme", "secret"])),
            strings(&["coord", REDACTED, REDACTED])
        );
        assert_eq!(
            redact_args(&strings(&["coord", "--unknown=secret"])),
            strings(&["coord", REDACTED])
        );
    }

    /// Equals-form known flags: name matched as if space-separated; value
    /// follows the same retain/redact rule.
    #[test]
    fn redact_args_handles_equals_form() {
        assert_eq!(
            redact_args(&strings(&["coord", "post", "--kind=review"])),
            strings(&["coord", "post", "--kind=review"])
        );
        assert_eq!(
            redact_args(&strings(&["coord", "post", "--topic=secret stuff"])),
            strings(&["coord", "post", &format!("--topic={REDACTED}")])
        );
    }

    /// After a bare `--`, everything is a positional value: redacted, never
    /// flag-matched — even if it looks like an allowlisted flag.
    #[test]
    fn redact_args_redacts_everything_after_double_dash() {
        assert_eq!(
            redact_args(&strings(&["coord", "--", "--kind", "review", "validate"])),
            strings(&["coord", "--", REDACTED, REDACTED, REDACTED])
        );
    }

    /// Positional free-text (paths, ids) is redacted; repeated flags are
    /// evaluated independently; a trailing flag with a missing value does
    /// not panic or shift tokens.
    #[test]
    fn redact_args_positionals_repeats_and_missing_values() {
        assert_eq!(
            redact_args(&strings(&[
                "coord",
                "validate",
                "docs/coordination/20260722T000000Z-role-abcd.md",
            ])),
            strings(&["coord", "validate", REDACTED])
        );
        assert_eq!(
            redact_args(&strings(&["post", "--ref", "a.md", "--ref", "b.md"])),
            strings(&["post", "--ref", REDACTED, "--ref", REDACTED])
        );
        assert_eq!(
            redact_args(&strings(&["coord", "post", "--kind"])),
            strings(&["coord", "post", "--kind"])
        );
    }

    /// Short-form tokens have no allowlist mapping in this CLI and are
    /// treated as unrecognized: redacted whole.
    #[test]
    fn redact_args_redacts_short_form_tokens() {
        assert_eq!(
            redact_args(&strings(&["coord", "-k", "review"])),
            strings(&["coord", REDACTED, REDACTED])
        );
    }

    /// Acceptance test 2: a historical JSONL line without the new fields
    /// still loads and summarizes correctly (feature-regression-guard).
    #[test]
    fn summarize_accepts_records_without_attribution_fields() {
        let records = vec![serde_json::json!({
            "event": "tool_output",
            "implementation": "rust-agov",
            "command": "agov",
            "args": ["coord", "validate"],
            "workflow": "coord-validate",
            "chars": 40,
            "approx_tokens": 10,
            "prompt_overhead_tokens": 5
        })];
        let summary = summarize(&records);
        assert_eq!(summary.total_records, 1);
        assert_eq!(summary.total_approx_tokens, 10);
        assert_eq!(summary.commands.len(), 1);
        assert_eq!(summary.commands[0].command, "agov");
    }

    /// Acceptance test 1: a record with all three attribution fields `None`
    /// serializes with no `transport`/`adapter`/`tool` keys at all —
    /// byte-shape compatible with the pre-Workstream-C record.
    #[test]
    fn none_attribution_fields_are_absent_from_json() {
        let record = RoiRecord {
            event: "tool_output",
            timestamp: "2026-07-22T00:00:00Z".to_string(),
            implementation: "rust-agov",
            command: "agov".to_string(),
            args: vec!["coord".to_string(), "validate".to_string()],
            workflow: "coord-validate".to_string(),
            chars: 40,
            approx_tokens: 10,
            prompt_overhead_tokens: 5,
            baseline_source: None,
            approx_saved_min_tokens: None,
            approx_saved_max_tokens: None,
            transport: None,
            adapter: None,
            tool: None,
        };
        let json = serde_json::to_string(&record).unwrap();
        assert!(!json.contains("\"transport\":"));
        assert!(!json.contains("\"adapter\":"));
        assert!(!json.contains("\"tool\":"));
    }

    #[test]
    fn serializes_rust_tool_output_record_shape() {
        let record = RoiRecord {
            event: "tool_output",
            timestamp: "2026-06-20T00:00:00Z".to_string(),
            implementation: "rust-agov",
            command: "agov".to_string(),
            args: vec!["coord".to_string(), "validate".to_string()],
            workflow: "coord-validate".to_string(),
            chars: 40,
            approx_tokens: 10,
            prompt_overhead_tokens: 5,
            baseline_source: Some(
                "docs/reference/tooling-token-roi.md#validation-baseline".to_string(),
            ),
            approx_saved_min_tokens: Some(485),
            approx_saved_max_tokens: Some(1_485),
            transport: None,
            adapter: None,
            tool: None,
        };

        let json = serde_json::to_value(record).unwrap();
        assert_eq!(json["event"], "tool_output");
        assert_eq!(json["implementation"], "rust-agov");
        assert_eq!(json["approx_tokens"], 10);
        assert_eq!(
            json["baseline_source"],
            "docs/reference/tooling-token-roi.md#validation-baseline"
        );
    }

    #[test]
    fn summarize_groups_mixed_rust_and_elixir_records() {
        let records = vec![
            serde_json::json!({
                "event": "tool_output",
                "implementation": "rust-agov",
                "command": "agov",
                "workflow": "coord-validate",
                "chars": 40,
                "approx_tokens": 10,
                "approx_saved_min_tokens": 20,
                "approx_saved_max_tokens": 30
            }),
            serde_json::json!({
                "event": "tool_output",
                "command": "agov",
                "workflow": "coord-validate",
                "chars": 80,
                "approx_tokens": 20,
                "approx_saved_min_tokens": 40,
                "approx_saved_max_tokens": 50
            }),
            serde_json::json!({
                "event": "measurement",
                "command": "manual",
                "workflow": "cold-start",
                "chars": 100,
                "manual_chars": 500,
                "approx_tokens": 25,
                "approx_saved_min_tokens": 100,
                "approx_saved_max_tokens": 100
            }),
        ];

        let summary = summarize(&records);

        assert_eq!(summary.total_records, 3);
        assert_eq!(summary.total_chars, 220);
        assert_eq!(summary.total_approx_tokens, 55);
        assert_eq!(summary.manual_chars, 500);
        assert_eq!(summary.manual_approx_tokens, 125);
        assert_eq!(summary.estimated_saved_min_tokens, 160);
        assert_eq!(summary.estimated_saved_max_tokens, 180);
        assert!(summary.commands.contains(&RoiCommandRow {
            command: "agov".to_string(),
            count: 2,
            chars: 120,
            approx_tokens: 30,
            approx_saved_min_tokens: 60,
            approx_saved_max_tokens: 80,
        }));
        assert!(summary.workflows.contains(&RoiWorkflowRow {
            workflow: "coord-validate".to_string(),
            count: 2,
            chars: 120,
            approx_tokens: 30,
            approx_saved_min_tokens: 60,
            approx_saved_max_tokens: 80,
        }));
    }

    #[test]
    fn format_report_prints_empty_shape() {
        let summary = summarize(&[]);
        let report = format_report(&summary, PathBuf::from("tmp/roi.jsonl").as_path());

        assert!(report.contains("Token ROI telemetry"));
        assert!(report.contains("Log: tmp/roi.jsonl"));
        assert!(report.contains("Records: 0"));
        assert!(report.contains("Commands: none"));
        assert!(report.contains("Workflows: none"));
    }
}
