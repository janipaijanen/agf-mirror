#![cfg(unix)]

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

#[test]
fn help_describes_mcp_server_and_installation() {
    for flag in ["--help", "-h"] {
        let output = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
            .arg(flag)
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("stdio MCP server for agov"));
        assert!(stdout.contains("AGOV_MCP_AGOV_BIN"));
        assert!(stdout.contains("agf/docs/site/docs/adopt/mcp-server.md"));
    }
}

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
if [ "$1" = "--version" ]; then
  printf 'agov mock\n'
  exit 0
fi
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

#[test]
fn stdio_server_resolves_agov_from_path() {
    let fixture = tempfile::tempdir().unwrap();
    let mock = fixture.path().join("agov");
    std::fs::write(
        &mock,
        r#"#!/bin/sh
if [ "$1" = "--version" ]; then
  printf 'agov mock\n'
  exit 0
fi
printf 'path agov:'
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
        .env_remove("AGOV_MCP_AGOV_BIN")
        .env("PATH", fixture.path())
        .current_dir(fixture.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    writeln!(
        child.stdin.as_mut().unwrap(),
        "{}",
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": { "name": "agov_session_resume", "arguments": {} }
        })
    )
    .unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(response["result"]["isError"], false);
    assert_eq!(
        response["result"]["structuredContent"]["stdout"],
        "path agov: [session] [resume]\n"
    );
}

#[test]
fn stdio_server_keeps_startup_resolution_after_path_candidate_disappears() {
    let fixture = tempfile::tempdir().unwrap();
    let first = fixture.path().join("first");
    let second = fixture.path().join("second");
    std::fs::create_dir_all(&first).unwrap();
    std::fs::create_dir_all(&second).unwrap();
    for (directory, identity) in [(&first, "first"), (&second, "second")] {
        let mock = directory.join("agov");
        std::fs::write(
            &mock,
            format!(
                "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then exit 0; fi\nprintf '{identity} agov\\n'\n"
            ),
        )
        .unwrap();
        let mut permissions = std::fs::metadata(&mock).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&mock, permissions).unwrap();
    }

    let path = std::env::join_paths([&first, &second]).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
        .env_remove("AGOV_MCP_AGOV_BIN")
        .env("PATH", path)
        .current_dir(fixture.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());

    writeln!(
        child.stdin.as_mut().unwrap(),
        "{}",
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}})
    )
    .unwrap();
    let mut initialized = String::new();
    stdout.read_line(&mut initialized).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&initialized).unwrap()["id"],
        1
    );

    std::fs::remove_file(first.join("agov")).unwrap();
    writeln!(
        child.stdin.as_mut().unwrap(),
        "{}",
        json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": { "name": "agov_session_resume", "arguments": {} }
        })
    )
    .unwrap();
    let mut called = String::new();
    stdout.read_line(&mut called).unwrap();
    let response: Value = serde_json::from_str(&called).unwrap();
    assert_eq!(response["result"]["isError"], true);
    assert_eq!(response["result"]["structuredContent"]["status"], 127);
    assert!(!called.contains("second agov"));

    drop(child.stdin.take());
    assert!(child.wait().unwrap().success());
}

#[test]
fn stdio_server_fails_at_startup_when_agov_is_unresolvable() {
    let fixture = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
        .env_remove("AGOV_MCP_AGOV_BIN")
        .env("PATH", "")
        .current_dir(fixture.path())
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("could not resolve agov"));
    assert!(stderr.contains("AGOV_MCP_AGOV_BIN"));
    assert!(stderr.contains("agov on PATH"));
    assert!(stderr.contains("cargo on PATH"));
}
