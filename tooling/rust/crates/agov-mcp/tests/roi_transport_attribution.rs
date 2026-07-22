#![cfg(unix)]

// Acceptance test 8 (v2.4.0 Workstream C): a real `agov-mcp`-mediated
// invocation of the real compiled `agov` binary records
// `transport: "mcp"`, `adapter: "agov-mcp"`, and a tool name from the
// same allowlist `tool_cli_args` uses -- not a mock, the real spawned
// binary and the real ROI writer.

use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

/// Locate the real compiled `agov` binary. `CARGO_BIN_EXE_<name>` is only
/// guaranteed for binaries in the *same* package as the test; it is not
/// set for a cross-crate path dev-dependency's binary. The `agov-mcp`
/// path dev-dependency on `agov` (in `Cargo.toml`) still makes Cargo build
/// `agov` into the same `target/<profile>/` directory this test binary's
/// own executable lives two levels below
/// (`target/<profile>/deps/<this-test-binary>`), so we find it there.
fn agov_bin_path() -> std::path::PathBuf {
    let test_exe = std::env::current_exe().expect("current_exe");
    let profile_dir = test_exe
        .parent() // .../target/<profile>/deps
        .and_then(|p| p.parent()) // .../target/<profile>
        .expect("test binary should be two levels under target/<profile>");
    let candidate = profile_dir.join("agov");
    assert!(
        candidate.is_file(),
        "expected a real agov binary at {candidate:?} (built via agov-mcp's \
         path dev-dependency on the agov crate); found none"
    );
    candidate
}

#[test]
fn mcp_mediated_call_records_mcp_transport_in_real_roi_log() {
    let roi_dir = tempfile::tempdir().unwrap();
    let roi_log = roi_dir.path().join("roi.jsonl");

    let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
        .env("AGOV_MCP_AGOV_BIN", agov_bin_path())
        .env("AGOV_TOKEN_ROI_LOG", &roi_log)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        writeln!(
            stdin,
            "{}",
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/call",
                "params": {
                    "name": "agov_coord_list",
                    "arguments": { "dir": "docs/coordination", "format": "json" }
                }
            })
        )
        .unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        response["result"]["structuredContent"]["ok"], true,
        "real agov invocation should succeed: {response}"
    );

    let content = std::fs::read_to_string(&roi_log)
        .unwrap_or_else(|err| panic!("expected a real ROI log at {roi_log:?}: {err}"));
    let records: Vec<Value> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(
        records.len(),
        1,
        "expected exactly one ROI record from the single tools/call: {content}"
    );

    let record = &records[0];
    assert_eq!(record["transport"], "mcp");
    assert_eq!(record["adapter"], "agov-mcp");
    assert_eq!(record["tool"], "agov_coord_list");

    // Redaction still applies to the real spawned invocation: the closed
    // command-path tokens survive, the free-form --dir value does not.
    let args = record["args"].as_array().unwrap();
    let args_str: Vec<&str> = args.iter().map(|v| v.as_str().unwrap()).collect();
    assert!(args_str.contains(&"coord"));
    assert!(args_str.contains(&"list"));
    assert!(!args_str.contains(&"docs/coordination"));
}

#[test]
fn direct_agov_invocation_records_explicit_cli_transport() {
    let roi_dir = tempfile::tempdir().unwrap();
    let roi_log = roi_dir.path().join("roi.jsonl");

    let output = Command::new(agov_bin_path())
        .arg("coord")
        .arg("list")
        .arg("--dir")
        .arg("docs/coordination")
        .arg("--format")
        .arg("json")
        .env("AGOV_TOKEN_ROI_LOG", &roi_log)
        .env_remove("AGOV_MCP_TRANSPORT")
        .env_remove("AGOV_MCP_ADAPTER")
        .env_remove("AGOV_MCP_TOOL")
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/../../..")
        .output()
        .unwrap();
    assert!(output.status.success());

    let content = std::fs::read_to_string(&roi_log).unwrap();
    let record: Value = serde_json::from_str(content.lines().next().unwrap()).unwrap();
    assert_eq!(record["transport"], "cli");
    assert!(record.get("adapter").is_none());
    assert!(record.get("tool").is_none());
}
