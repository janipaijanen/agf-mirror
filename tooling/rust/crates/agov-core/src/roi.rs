use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// A saved-token ROI record appended to `tmp/agov-token-roi.jsonl`.
///
/// Fields mirror the Elixir `Agov.TokenRoi` JSONL format (§8.2 of the plan).
/// Unknown workflows may omit the `saved_*` fields.
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

/// Append `record` to the ROI log as a JSONL line.
///
/// Failures are silently swallowed — logging must not affect stdout, stderr,
/// output ordering, or exit codes (plan §8.4).
///
/// Writes the JSON content and its trailing newline as a **single**
/// `write_all` call, not `writeln!`. `writeln!` on a `File` performs two
/// separate `write()` syscalls (the formatted content, then the literal
/// `"\n"` from the macro), and `O_APPEND` only guarantees each individual
/// `write()` is atomic against other appenders -- the pair is not. Two
/// processes racing between those two writes can interleave, dropping the
/// separating newline and concatenating two records onto one physical
/// line. A single `write_all` of the pre-joined `"{json}\n"` string is one
/// `write()` call for a line this size, restoring `O_APPEND` atomicity.
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
        use std::io::Write as _;
        let line = format!("{}\n", serde_json::to_string(record)?);
        file.write_all(line.as_bytes())?;
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
    let prompt_overhead_tokens = estimate_tokens(command_prompt_chars(command, args));
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
        args: args.to_vec(),
        workflow: workflow.to_string(),
        chars,
        approx_tokens,
        prompt_overhead_tokens,
        baseline_source,
        approx_saved_min_tokens,
        approx_saved_max_tokens,
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
    /// corruption before the `write_all`-of-one-joined-string fix.
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
