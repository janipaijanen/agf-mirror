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
fn tools_list_includes_all_six_new_coordination_tools_with_correct_schemas() {
    let fixture = tempfile::tempdir().unwrap();
    let mock = fixture.path().join("agov-mock");
    std::fs::write(
        &mock,
        "#!/usr/bin/env sh\nif [ \"$1\" = \"--version\" ]; then exit 0; fi\nprintf 'mock\\n'\n",
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
    writeln!(
        child.stdin.as_mut().unwrap(),
        "{}",
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})
    )
    .unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    let tools = response["result"]["tools"].as_array().unwrap();

    let expected = [
        "agov_coord_status",
        "agov_coord_visibility",
        "agov_coord_stale",
        "agov_coord_graph_index",
        "agov_coord_publication_check",
        "agov_coord_brief",
    ];
    for name in expected {
        let tool = tools
            .iter()
            .find(|tool| tool["name"] == name)
            .unwrap_or_else(|| panic!("tools/list is missing {name}"));
        assert!(
            tool["inputSchema"]["type"] == "object",
            "{name} schema must be an object type"
        );
    }

    // B5: the graph-index tool's schema exposes no `output`/file-path
    // property at all -- proving exclusion, not merely omission from a
    // test that happens not to exercise it.
    let graph_index = tools
        .iter()
        .find(|tool| tool["name"] == "agov_coord_graph_index")
        .unwrap();
    assert!(graph_index["inputSchema"]["properties"]
        .get("output")
        .is_none());

    // publication_check's base_ref is required, per the frozen contract.
    let publication_check = tools
        .iter()
        .find(|tool| tool["name"] == "agov_coord_publication_check")
        .unwrap();
    assert_eq!(
        publication_check["inputSchema"]["required"],
        json!(["base_ref"])
    );
}

#[test]
fn new_tool_arguments_forward_to_correct_cli_flags() {
    let fixture = tempfile::tempdir().unwrap();
    let mock = fixture.path().join("agov-mock");
    std::fs::write(
        &mock,
        r#"#!/usr/bin/env sh
if [ "$1" = "--version" ]; then exit 0; fi
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

    let cases: &[(&str, Value, &str)] = &[
        (
            "agov_coord_status",
            json!({ "recent": 3, "roles": true }),
            "mock agov: [coord] [status] [--recent] [3] [--roles]\n",
        ),
        (
            "agov_coord_visibility",
            json!({ "base_branch": "main", "branch": ["a", "b"], "format": "json" }),
            "mock agov: [coord] [visibility] [--base-branch] [main] [--branch] [a] [--branch] [b] [--format] [json]\n",
        ),
        (
            "agov_coord_publication_check",
            json!({ "base_ref": "origin/main", "subject_prefix": "coordination:" }),
            "mock agov: [coord] [publication-check] [--base-ref] [origin/main] [--subject-prefix] [coordination:]\n",
        ),
        (
            "agov_coord_brief",
            json!({ "id_format": "ulid", "kind": "note" }),
            "mock agov: [coord] [brief] [--id-format] [ulid] [--kind] [note]\n",
        ),
    ];

    for (name, arguments, expected_stdout) in cases {
        let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
            .env("AGOV_MCP_AGOV_BIN", &mock)
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
                "params": { "name": name, "arguments": arguments }
            })
        )
        .unwrap();
        drop(child.stdin.take());

        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        let response: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(response["result"]["isError"], false, "{name} call failed");
        assert_eq!(
            response["result"]["structuredContent"]["stdout"], *expected_stdout,
            "{name} argument forwarding mismatch"
        );
    }
}

#[test]
fn coord_post_authority_and_arguments_unchanged_by_this_plan() {
    // Slice 2 adds only read-path tools; this proves the pre-existing
    // mutating tool's argument forwarding is untouched.
    let fixture = tempfile::tempdir().unwrap();
    let mock = fixture.path().join("agov-mock");
    std::fs::write(
        &mock,
        r#"#!/usr/bin/env sh
if [ "$1" = "--version" ]; then exit 0; fi
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
    writeln!(
        child.stdin.as_mut().unwrap(),
        "{}",
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "agov_coord_post",
                "arguments": {
                    "from": "coder-tester",
                    "to": "governor-reviewer",
                    "kind": "note",
                    "topic": "t"
                }
            }
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
        "mock agov: [coord] [post] [--from] [coder-tester] [--to] [governor-reviewer] [--kind] [note] [--topic] [t]\n"
    );
}

/// CLI/MCP registry contract test, per Slice 2's requirement: enumerate
/// the real `agov coord --help` subcommands and assert the intended
/// coverage set matches the MCP registry, so future CLI additions cannot
/// silently drift out of sync again. Deliberately-unexposed CLI surface
/// (`graph`, and `graph-index --output`) is named here with its reason,
/// not merely absent.
#[test]
fn mcp_coordination_tool_registry_matches_live_cli_subcommands() {
    let test_exe = std::env::current_exe().unwrap();
    let profile_dir = test_exe.parent().unwrap().parent().unwrap().to_path_buf();
    let is_release = profile_dir
        .file_name()
        .is_some_and(|name| name == "release");
    let workspace_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut build = Command::new(env!("CARGO"));
    build.args(["build", "--quiet", "-p", "agov"]);
    if is_release {
        build.arg("--release");
    }
    let status = build.current_dir(&workspace_root).status().unwrap();
    assert!(status.success(), "cargo build -p agov failed");
    let agov_bin = profile_dir.join("agov");
    assert!(agov_bin.is_file());

    let output = Command::new(&agov_bin)
        .args(["coord", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();

    let live_subcommands: Vec<String> = help
        .lines()
        .skip_while(|line| !line.starts_with("Commands:"))
        .skip(1)
        .take_while(|line| !line.trim().is_empty())
        .filter_map(|line| line.trim_start().split_whitespace().next())
        .map(str::to_string)
        .filter(|name| name != "help")
        .collect();

    // Every live `agov coord` subcommand must be accounted for: either
    // exposed as an MCP tool (mirrored via `agov_coord_<name>`, with `-`
    // mapped to `_`), or explicitly named here as deliberately unexposed.
    let deliberately_unexposed = ["graph"];
    let mcp_exposed: Vec<String> = live_subcommands
        .iter()
        .filter(|name| !deliberately_unexposed.contains(&name.as_str()))
        .map(|name| format!("agov_coord_{}", name.replace('-', "_")))
        .collect();

    let mock = std::env::temp_dir().join(format!(
        "agov-mock-registry-contract-{}",
        std::process::id()
    ));
    std::fs::write(
        &mock,
        "#!/usr/bin/env sh\nif [ \"$1\" = \"--version\" ]; then exit 0; fi\nprintf 'mock\\n'\n",
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
    writeln!(
        child.stdin.as_mut().unwrap(),
        "{}",
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})
    )
    .unwrap();
    drop(child.stdin.take());
    let list_output = child.wait_with_output().unwrap();
    std::fs::remove_file(&mock).ok();
    assert!(list_output.status.success());
    let list_response: Value = serde_json::from_slice(&list_output.stdout).unwrap();
    let published: Vec<String> = list_response["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap().to_string())
        .filter(|name| name.starts_with("agov_coord_"))
        .collect();

    for expected in &mcp_exposed {
        assert!(
            published.contains(expected),
            "live `agov coord` subcommand set implies MCP tool {expected}, but it is missing from the registry \
             (live subcommands: {live_subcommands:?})"
        );
    }

    for name in &deliberately_unexposed {
        assert!(
            live_subcommands.contains(&name.to_string()),
            "deliberately-unexposed subcommand {name} is no longer part of the live CLI -- update this test"
        );
        assert!(
            !published.contains(&format!("agov_coord_{name}")),
            "{name} is listed as deliberately unexposed but has an MCP tool anyway"
        );
    }
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
