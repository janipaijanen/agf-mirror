#![cfg(unix)]

use serde_json::{json, Value};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

#[test]
fn version_json_reports_structured_provenance() {
    let output = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
        .args(["version", "--format", "json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["version"], env!("CARGO_PKG_VERSION"));
    assert!(value["dirty"].is_boolean());
    assert!(value["source_dirty"].is_boolean());
    let commit = value["commit"].as_str().unwrap();
    assert!(commit == "unknown" || commit.len() == 40);
}

#[test]
fn stdio_server_lists_tools_and_calls_agov() {
    let fixture = tempfile::tempdir().unwrap();
    let mock = fixture.path().join("agov-mock");
    std::fs::write(
        &mock,
        r#"#!/usr/bin/env sh
printf 'mock agov:'
for arg in "$@"; do
  printf ' [%s]' "$arg"
done
printf '\n'
"#,
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&mock).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&mock, permissions).unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
        .env("AGOV_MCP_AGOV_BIN", &mock)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        writeln!(
            stdin,
            "{}",
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}})
        )
        .unwrap();
        writeln!(
            stdin,
            "{}",
            json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}})
        )
        .unwrap();
        writeln!(
            stdin,
            "{}",
            json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {
                    "name": "agov_coord_list",
                    "arguments": {
                        "since": "2026-07-20T10:00:00Z",
                        "dir": "docs/coordination",
                        "format": "json"
                    }
                }
            })
        )
        .unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let responses: Vec<Value> = stdout
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    assert_eq!(responses.len(), 3);
    assert_eq!(responses[0]["result"]["serverInfo"]["name"], "agov-mcp");
    assert!(responses[1]["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .any(|tool| tool["name"] == "agov_coord_post"));
    assert_eq!(responses[2]["result"]["isError"], false);
    assert_eq!(
        responses[2]["result"]["structuredContent"]["stdout"],
        "mock agov: [coord] [list] [--since] [2026-07-20T10:00:00Z] [--dir] [docs/coordination] [--format] [json]\n"
    );
}

#[test]
fn stdio_server_returns_structured_error_for_failed_agov_call() {
    let fixture = tempfile::tempdir().unwrap();
    let mock = fixture.path().join("agov-mock");
    std::fs::write(
        &mock,
        r#"#!/usr/bin/env sh
printf 'schema failed\n' >&2
exit 1
"#,
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&mock).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&mock, permissions).unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
        .env("AGOV_MCP_AGOV_BIN", &mock)
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
                    "name": "agov_coord_validate",
                    "arguments": { "files": ["bad.md"] }
                }
            })
        )
        .unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();

    assert_eq!(response["result"]["isError"], true);
    assert_eq!(response["result"]["structuredContent"]["status"], 1);
    assert_eq!(
        response["result"]["structuredContent"]["stderr"],
        "schema failed\n"
    );
}
