use std::path::PathBuf;

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
        writeln!(file, "{}", serde_json::to_string(record)?)?;
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
}
