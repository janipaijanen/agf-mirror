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
fn tools_list_includes_all_seven_new_coordination_tools_with_correct_schemas() {
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
        "agov_coord_convergence",
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
            "agov_coord_convergence",
            json!({ "series": "series-id", "dir": "docs/coordination", "format": "json" }),
            "mock agov: [coord] [convergence] [--series] [series-id] [--dir] [docs/coordination] [--format] [json]\n",
        ),
        (
            "agov_coord_status",
            json!({ "recent": 3, "roles": true }),
            "mock agov: [coord] [status] [--recent] [3] [--roles]\n",
        ),
        (
            "agov_coord_visibility",
            json!({ "base_branch": "main", "branch": "a", "format": "json" }),
            "mock agov: [coord] [visibility] [--base-branch] [main] [--branch] [a] [--format] [json]\n",
        ),
        (
            "agov_coord_stale",
            json!({ "dir": "docs/coordination" }),
            "mock agov: [coord] [stale] [--dir] [docs/coordination]\n",
        ),
        (
            "agov_coord_graph_index",
            json!({ "dir": "docs/coordination" }),
            "mock agov: [coord] [graph-index] [--dir] [docs/coordination]\n",
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

fn build_real_agov_bin() -> std::path::PathBuf {
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
    let bin = profile_dir.join("agov");
    assert!(bin.is_file());
    bin
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

/// Item 12 (B-MR806-G2-3): the same input through the CLI and through the
/// MCP tool call must produce the same output, for each of the seven newly
/// added tools -- against the real compiled `agov` binary, not a mock.
///
/// `status`/`visibility`/`stale`/`graph-index` run against a small,
/// isolated fixture directory rather than the live `docs/coordination/`:
/// this repository is actively written to by other concurrent sessions,
/// so two separate process invocations a moment apart can legitimately
/// observe different real state (a race, not a CLI/MCP divergence) --
/// and `graph-index`'s edge list has no documented stable ordering
/// guarantee across separate process runs even for identical input,
/// which a two-file fixture with exactly one edge sidesteps entirely.
/// `publication-check` and `brief` don't depend on `docs/coordination/`
/// file contents (git history and pure generation, respectively), so
/// they run against the real repository.
#[test]
fn real_cli_and_mcp_produce_equivalent_output_for_all_seven_new_tools() {
    let agov_bin = build_real_agov_bin();
    let root = workspace_root();

    let run_cli = |args: &[&str], cwd: &std::path::Path| -> String {
        let output = Command::new(&agov_bin)
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .args(args)
            .current_dir(cwd)
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    let run_mcp = |name: &str, arguments: Value, cwd: &std::path::Path| -> String {
        let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
            .env("AGOV_MCP_AGOV_BIN", &agov_bin)
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .current_dir(cwd)
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
        let response: Value = serde_json::from_slice(&output.stdout).unwrap();
        // Deliberately not asserting `isError == false`: a real-state
        // read may legitimately exit non-zero (e.g. `coord stale`
        // finding a genuine stale claim). `structuredContent.stdout` is
        // the underlying command's real stdout regardless of exit
        // status -- the equivalence claim is about stdout content.
        response["result"]["structuredContent"]["stdout"]
            .as_str()
            .unwrap_or_else(|| panic!("{name}: no structuredContent.stdout in {response}"))
            .to_string()
    };

    // Isolated, two-file fixture: fixed content, one cross-reference (so
    // `graph-index` has exactly one edge -- no ordering ambiguity).
    let fixture = tempfile::tempdir().unwrap();
    let coord_dir = fixture.path().join("docs").join("coordination");
    std::fs::create_dir_all(&coord_dir).unwrap();
    std::fs::write(
        coord_dir.join("20260101T000000Z-coder-tester-fix1.md"),
        "---\nkind: note\nfrom: coder-tester\nto: governor-reviewer\nid: 20260101T000000Z-coder-tester-fix1\nimmutable: true\nreferences-to: []\nrequires-ack: false\ntimestamp: 2026-01-01T00:00:00Z\ntopic: equivalence-fixture\n---\n\n# Fixture one\n",
    )
    .unwrap();
    std::fs::write(
        coord_dir.join("20260101T000100Z-governor-reviewer-fix2.md"),
        "---\nkind: note\nfrom: governor-reviewer\nto: coder-tester\nid: 20260101T000100Z-governor-reviewer-fix2\nimmutable: true\nreferences-to:\n  - 20260101T000000Z-coder-tester-fix1\nrequires-ack: false\ntimestamp: 2026-01-01T00:01:00Z\ntopic: equivalence-fixture-response\n---\n\n# Fixture two, references fixture one\n",
    )
    .unwrap();
    let coord_dir_str = coord_dir.to_str().unwrap();

    let isolated_cases: &[(&str, Vec<&str>, Value)] = &[
        (
            "agov_coord_convergence",
            vec![
                "coord",
                "convergence",
                "--series",
                "missing-series",
                "--dir",
                coord_dir_str,
                "--format",
                "json",
            ],
            json!({ "series": "missing-series", "dir": coord_dir_str, "format": "json" }),
        ),
        (
            "agov_coord_status",
            vec!["coord", "status", "--dir", coord_dir_str, "--recent", "3"],
            json!({ "dir": coord_dir_str, "recent": 3 }),
        ),
        (
            "agov_coord_stale",
            vec!["coord", "stale", "--dir", coord_dir_str],
            json!({ "dir": coord_dir_str }),
        ),
        (
            "agov_coord_graph_index",
            vec!["coord", "graph-index", "--dir", coord_dir_str],
            json!({ "dir": coord_dir_str }),
        ),
    ];
    for (name, cli_args, mcp_arguments) in isolated_cases {
        let cli_stdout = run_cli(cli_args, fixture.path());
        let mcp_stdout = run_mcp(name, mcp_arguments.clone(), fixture.path());
        assert_eq!(
            cli_stdout, mcp_stdout,
            "{name}: CLI and MCP output diverged"
        );
    }

    // Visibility needs Git state as well as fixed files. Use an isolated
    // repository so concurrent branch/coordination publication cannot race
    // the two invocations or turn this parity check into an estate scan.
    let git_fixture = tempfile::tempdir().unwrap();
    let git_root = git_fixture.path();
    for argv in [
        vec!["init", "-b", "main"],
        vec!["config", "user.name", "Parity Test"],
        vec!["config", "user.email", "parity@example.invalid"],
        vec!["commit", "--allow-empty", "-m", "base"],
        vec!["checkout", "-b", "parity"],
    ] {
        assert!(Command::new("git")
            .args(argv)
            .current_dir(git_root)
            .output()
            .unwrap()
            .status
            .success());
    }
    std::fs::create_dir_all(git_root.join("docs/coordination")).unwrap();
    for entry in std::fs::read_dir(&coord_dir).unwrap() {
        let entry = entry.unwrap();
        std::fs::copy(
            entry.path(),
            git_root.join("docs/coordination").join(entry.file_name()),
        )
        .unwrap();
    }
    for argv in [vec!["add", "docs"], vec!["commit", "-m", "fixture"]] {
        assert!(Command::new("git")
            .args(argv)
            .current_dir(git_root)
            .output()
            .unwrap()
            .status
            .success());
    }
    let cli_stdout = run_cli(
        &[
            "coord",
            "visibility",
            "--base-branch",
            "main",
            "--branch",
            "parity",
            "--format",
            "text",
        ],
        git_root,
    );
    let mcp_stdout = run_mcp(
        "agov_coord_visibility",
        json!({"base_branch":"main","branch":"parity","format":"text"}),
        git_root,
    );
    assert_eq!(
        cli_stdout, mcp_stdout,
        "agov_coord_visibility: CLI and MCP output diverged"
    );

    // `publication-check` depends only on git commit history for the
    // already-fetched `origin/main`, not on `docs/coordination/` file
    // contents or any hash-ordered collection -- safe against the real
    // repository.
    let cli_stdout = run_cli(
        &[
            "coord",
            "publication-check",
            "--base-ref",
            "origin/main",
            "--dir",
            "docs/coordination",
        ],
        &root,
    );
    let mcp_stdout = run_mcp(
        "agov_coord_publication_check",
        json!({ "base_ref": "origin/main", "dir": "docs/coordination" }),
        &root,
    );
    assert_eq!(
        cli_stdout, mcp_stdout,
        "agov_coord_publication_check: CLI and MCP output diverged"
    );

    // `agov_coord_brief` generates a fresh id/timestamp on every
    // invocation, so CLI and MCP calls cannot be byte-identical -- compare
    // structurally instead: same echoed from/to/kind/topic fields.
    let brief_args = [
        "coord",
        "brief",
        "--from",
        "coder-tester",
        "--to",
        "governor-reviewer",
        "--kind",
        "note",
        "--topic",
        "cli-mcp-equivalence-check",
    ];
    let cli_brief = run_cli(&brief_args, &root);
    let mcp_brief = run_mcp(
        "agov_coord_brief",
        json!({
            "from": "coder-tester",
            "to": "governor-reviewer",
            "kind": "note",
            "topic": "cli-mcp-equivalence-check"
        }),
        &root,
    );
    for field in [
        "from: coder-tester",
        "to: governor-reviewer",
        "kind: note",
        "topic: cli-mcp-equivalence-check",
    ] {
        assert!(
            cli_brief.contains(field),
            "CLI brief output missing {field}: {cli_brief}"
        );
        assert!(
            mcp_brief.contains(field),
            "MCP brief output missing {field}: {mcp_brief}"
        );
    }
}

#[test]
fn channel_candidate_evidence_matches_real_cli_and_mcp() {
    let bin = build_real_agov_bin();
    let schema = workspace_root().join("agf/schemas");
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    let git = |args: &[&str]| {
        assert!(Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .unwrap()
            .status
            .success());
    };
    git(&["init", "-b", "main"]);
    git(&["config", "user.name", "Test"]);
    git(&["config", "user.email", "test@example.invalid"]);
    std::fs::create_dir_all(root.join("docs/coordination-archive")).unwrap();
    let record = "---\nkind: handoff\nfrom: planner-architect\nto: coder-tester\nid: fixture-handoff\ntimestamp: 2026-09-10T10:00:00Z\nimmutable: true\nreferences-to: []\ntopic: parity\nrequires-ack: true\nhandoff:\n  next_agent: coder-tester\n  status: blocked\n  task: [test]\n  must_read: []\n  do_not_touch: []\n---\n";
    std::fs::write(root.join("docs/coordination-archive/old.md"), record).unwrap();
    git(&["add", "."]);
    git(&["commit", "-m", "base"]);
    git(&["checkout", "-b", "feat/candidate"]);
    std::fs::write(
        root.join("docs/coordination-typo.md"),
        record.replace("fixture-handoff", "fixture-sibling"),
    )
    .unwrap();
    git(&["add", "."]);
    git(&["commit", "-m", "candidate"]);
    let direct = Command::new(&bin)
        .args([
            "coord",
            "visibility",
            "--base-branch",
            "main",
            "--branch",
            "feat/candidate",
            "--format",
            "json",
        ])
        .env("AGOV_SCHEMA_DIR", &schema)
        .current_dir(root)
        .output()
        .unwrap();
    assert_eq!(direct.status.code(), Some(1));
    let stdout = String::from_utf8(direct.stdout).unwrap();
    let evidence: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(evidence["archive_baseline"]["count"], 1);
    assert_eq!(evidence["archive_baseline"]["lifecycle"], "unassessed");
    assert_eq!(
        evidence["candidate_diagnostics"][0]["path"],
        "docs/coordination-typo.md"
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
        .env("AGOV_MCP_AGOV_BIN", &bin)
        .env("AGOV_SCHEMA_DIR", &schema)
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    writeln!(child.stdin.as_mut().unwrap(), "{}", json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"agov_coord_visibility","arguments":{"base_branch":"main","branch":"feat/candidate","format":"json"}}})).unwrap();
    drop(child.stdin.take());
    let out = child.wait_with_output().unwrap();
    let value: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["result"]["structuredContent"]["stdout"], stdout);
    assert_eq!(value["result"]["structuredContent"]["status"], 1);
    assert_eq!(value["result"]["isError"], true); // existing warning-envelope convention
}

#[test]
fn recipient_session_matches_real_cli_and_mcp_without_selection_changes() {
    let bin = build_real_agov_bin();
    let schema = workspace_root().join("agf/schemas");
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    let git = |args: &[&str]| {
        assert!(Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .unwrap()
            .status
            .success());
    };
    git(&["init", "-b", "main"]);
    git(&["config", "user.name", "Test"]);
    git(&["config", "user.email", "test@example.invalid"]);
    git(&["commit", "--allow-empty", "-m", "base"]);
    git(&["checkout", "-b", "feat/session"]);
    let dir = root.join("docs/coordination");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("fixture-handoff.md"), "---\nkind: handoff\nfrom: planner-architect\nto: coder-tester\nid: fixture-handoff\ntimestamp: 2026-09-10T10:00:00Z\nimmutable: true\nreferences-to: []\ntopic: parity\nrequires-ack: true\nsession: author-A\nhandoff:\n  next_agent: coder-tester\n  for_session: recipient-B\n  status: blocked\n  task: [test]\n  must_read: []\n  do_not_touch: []\n---\n").unwrap();
    git(&["add", "."]);
    git(&["commit", "-m", "handoff"]);
    for session in [None, Some("recipient-B"), Some("author-A")] {
        for (name, mut argv, mut args, status) in [
            (
                "agov_handoff_next",
                vec!["handoff", "next", "coder-tester"],
                json!({"role":"coder-tester"}),
                0,
            ),
            (
                "agov_coord_visibility",
                vec![
                    "coord",
                    "visibility",
                    "--base-branch",
                    "main",
                    "--branch",
                    "feat/session",
                    "--format",
                    "json",
                ],
                json!({"base_branch":"main","branch":"feat/session","format":"json"}),
                1,
            ),
        ] {
            if let Some(token) = session {
                argv.extend(["--session", token]);
                args["session"] = json!(token);
            }
            let direct = Command::new(&bin)
                .args(argv)
                .env("AGOV_SCHEMA_DIR", &schema)
                .current_dir(root)
                .output()
                .unwrap();
            assert_eq!(direct.status.code(), Some(status));
            let stdout = String::from_utf8(direct.stdout).unwrap();
            assert_eq!(stdout.contains("elsewhere"), session == Some("author-A"));
            if name == "agov_handoff_next" {
                assert!(stdout.contains("Status:      blocked"));
            }
            let mut child = Command::new(env!("CARGO_BIN_EXE_agov-mcp"))
                .env("AGOV_MCP_AGOV_BIN", &bin)
                .env("AGOV_SCHEMA_DIR", &schema)
                .current_dir(root)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            writeln!(child.stdin.as_mut().unwrap(), "{}", json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":args}})).unwrap();
            drop(child.stdin.take());
            let out = child.wait_with_output().unwrap();
            let value: Value = serde_json::from_slice(&out.stdout).unwrap();
            assert_eq!(value["result"]["structuredContent"]["stdout"], stdout);
            assert_eq!(value["result"]["structuredContent"]["status"], status);
            assert_eq!(value["result"]["isError"], status != 0);
        }
    }
}
