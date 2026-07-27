//! agov-mcp: stdio MCP adapter for agov.
//!
//! Parity invariant: every MCP tool below wraps an existing `agov` CLI
//! subcommand and must not expose standalone governance capability.
//!
//! | MCP tool | CLI command |
//! |---|---|
//! | `agov_validate` | `agov validate ...` |
//! | `agov_coord_validate` | `agov coord validate <files>` |
//! | `agov_plan_validate` | `agov plan validate <files>` |
//! | `agov_session_resume` | `agov session resume ...` |
//! | `agov_handoff_next` | `agov handoff next <role>` |
//! | `agov_handoff_take` | `agov handoff take <id> --from <role>` |
//! | `agov_handoff_complete` | `agov handoff complete <id> --from <role>` |
//! | `agov_coord_post` | `agov coord post ...` |
//! | `agov_coord_list` | `agov coord list ...` |

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::Command;

const PROTOCOL_VERSION: &str = "2025-06-18";

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

fn main() {
    // `#[used]` retains the marker in compiler output, but a cross-linker may
    // still garbage-collect its section when no reachable code references it.
    // Keep the exact bytes reachable without changing version output.
    std::hint::black_box(BUILD_PROVENANCE_MARKER);

    let args: Vec<String> = std::env::args().skip(1).collect();
    if matches!(args.as_slice(), [arg] if arg == "--version" || arg == "-V") {
        println!(
            "agov-mcp {} ({} {})",
            env!("CARGO_PKG_VERSION"),
            build_commit(),
            if build_dirty() { "dirty" } else { "clean" }
        );
        return;
    }
    if matches!(args.as_slice(), [arg] if arg == "--help" || arg == "-h") {
        println!(
            "agov-mcp — stdio MCP server for agov\n\nUsage: agov-mcp [--help | --version]\n\nPlace agov and agov-mcp on PATH, or set AGOV_MCP_AGOV_BIN to the agov executable.\nDocumentation: https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/docs/site/docs/adopt/mcp-server.md"
        );
        return;
    }
    if matches!(args.as_slice(), [command, flag, format] if command == "version" && flag == "--format" && format == "json")
    {
        println!(
            "{}",
            serde_json::to_string(&version_info()).expect("serialize version info")
        );
        return;
    }
    if !args.is_empty() {
        eprintln!("agov-mcp: unsupported arguments");
        std::process::exit(2);
    }
    if let Err(err) = run_stdio() {
        eprintln!("agov-mcp: {err}");
        std::process::exit(1);
    }
}

#[derive(Serialize)]
struct VersionInfo {
    version: &'static str,
    commit: &'static str,
    dirty: bool,
    source_dirty: bool,
}

fn build_commit() -> &'static str {
    option_env!("AGOV_GIT_SHA").unwrap_or("unknown")
}

fn build_dirty() -> bool {
    option_env!("AGOV_GIT_DIRTY").unwrap_or("false") == "true"
}

fn version_info() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        commit: build_commit(),
        dirty: build_dirty(),
        source_dirty: option_env!("AGOV_SOURCE_DIRTY").unwrap_or("false") == "true",
    }
}

fn run_stdio() -> Result<(), String> {
    validate_agov_startup()?;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        let line = line.map_err(|err| err.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let response = handle_message(&line);
        if let Some(response) = response {
            writeln!(
                stdout,
                "{}",
                serde_json::to_string(&response).map_err(|err| err.to_string())?
            )
            .map_err(|err| err.to_string())?;
            stdout.flush().map_err(|err| err.to_string())?;
        }
    }

    Ok(())
}

fn handle_message(line: &str) -> Option<JsonRpcResponse> {
    let request: JsonRpcRequest = match serde_json::from_str(line) {
        Ok(request) => request,
        Err(err) => {
            return Some(JsonRpcResponse::error(
                Value::Null,
                -32700,
                format!("parse error: {err}"),
            ));
        }
    };

    request.id.as_ref()?;
    let id = request.id.unwrap_or(Value::Null);
    match request.method.as_str() {
        "initialize" => Some(JsonRpcResponse::result(
            id,
            json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "agov-mcp",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }),
        )),
        "tools/list" => Some(JsonRpcResponse::result(id, json!({ "tools": tools() }))),
        "tools/call" => match call_tool(request.params.unwrap_or(Value::Null)) {
            Ok(result) => Some(JsonRpcResponse::result(id, result)),
            Err(message) => Some(JsonRpcResponse::error(id, -32602, message)),
        },
        _ => Some(JsonRpcResponse::error(
            id,
            -32601,
            format!("method not found: {}", request.method),
        )),
    }
}

fn call_tool(params: Value) -> Result<Value, String> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| "tools/call requires params.name".to_string())?;
    // Gate 2 review `32qd` (narrowed BLOCK 3): the allowlist must be the
    // real acceptance gate, checked BEFORE `tool_cli_args`, not an
    // optional lookup performed after. Resolving to the canonical
    // `&'static str` here and using that canonical value for both
    // `tool_cli_args` and `run_agov` means a hypothetical future
    // `tool_cli_args` match arm added without a matching
    // `MCP_TOOL_ALLOWLIST` entry is unreachable: this line rejects the
    // request before any such arm could run, so there is exactly one
    // source of truth for "which tools exist," not three that can drift.
    let canonical_tool = agov_core::roi::MCP_TOOL_ALLOWLIST
        .iter()
        .find(|allowed| **allowed == name)
        .copied()
        .ok_or_else(|| format!("unknown tool: {name}"))?;
    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let cli_args = tool_cli_args(canonical_tool, &args)?;
    let execution = run_agov(&cli_args, Some(canonical_tool));

    Ok(json!({
        "content": [
            {
                "type": "text",
                "text": tool_text(&execution)
            }
        ],
        "isError": !execution.ok,
        "structuredContent": execution
    }))
}

fn tool_text(execution: &ToolExecution) -> String {
    if execution.ok || execution.stderr.trim().is_empty() {
        execution.stdout.clone()
    } else {
        execution.stderr.clone()
    }
}

/// Spawn the child `agov` process. When `tool_name` is `Some`, set the
/// fixed transport-attribution markers (v2.4.0 Workstream C) on the
/// child environment. `tool_name` must already be a canonical
/// `&'static str` resolved from `agov_core::roi::MCP_TOOL_ALLOWLIST`
/// (see `call_tool`) — never a borrowed slice of the incoming MCP
/// request string — so the marker value set here is always this
/// adapter's own constant. The child validates the tuple again
/// independently before persisting anything.
fn run_agov(args: &[String], tool_name: Option<&'static str>) -> ToolExecution {
    let command = match agov_command() {
        Ok(command) => command,
        Err(err) => {
            return ToolExecution {
                ok: false,
                status: 127,
                command: args.to_vec(),
                stdout: String::new(),
                stderr: err,
            }
        }
    };
    let mut child = Command::new(command.program);
    child
        .args(&command.prefix)
        .args(args)
        .current_dir(command.cwd);
    if let Some(name) = tool_name {
        child
            .env(agov_core::roi::MCP_TRANSPORT_MARKER, "mcp")
            .env(agov_core::roi::MCP_ADAPTER_MARKER, "agov-mcp")
            .env(agov_core::roi::MCP_TOOL_MARKER, name);
    }
    let output = child.output();

    match output {
        Ok(output) => ToolExecution {
            ok: output.status.success() || is_expected_non_error_status(args, output.status.code()),
            status: output.status.code().unwrap_or(1),
            command: args.to_vec(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        },
        Err(err) => ToolExecution {
            ok: false,
            status: 127,
            command: args.to_vec(),
            stdout: String::new(),
            stderr: format!("could not execute agov: {err}"),
        },
    }
}

fn is_expected_non_error_status(args: &[String], status: Option<i32>) -> bool {
    status == Some(2)
        && args.first().map(String::as_str) == Some("coord")
        && args.get(1).map(String::as_str) == Some("list")
}

fn validate_agov_startup() -> Result<(), String> {
    let command = agov_command()?;
    let output = Command::new(&command.program)
        .args(&command.prefix)
        .arg("--version")
        .current_dir(&command.cwd)
        .output()
        .map_err(|err| {
            format!(
                "agov startup validation failed for {}: {err}",
                command.program.display()
            )
        })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "agov startup validation failed for {} with status {}: {}",
            command.program.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn executable_on_path(name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|directory| directory.join(name))
            .find(|candidate| candidate.is_file())
    })
}

fn agov_command() -> Result<AgovCommand, String> {
    if let Some(bin) = std::env::var_os("AGOV_MCP_AGOV_BIN") {
        return Ok(AgovCommand {
            program: PathBuf::from(bin),
            prefix: Vec::new(),
            cwd: agov_core::repo_root(),
        });
    }

    let root = agov_core::repo_root();
    let release = root
        .join("tooling")
        .join("rust")
        .join("target")
        .join("release")
        .join("agov");
    if release.is_file() {
        return Ok(AgovCommand {
            program: release,
            prefix: Vec::new(),
            cwd: root,
        });
    }

    let debug = root
        .join("tooling")
        .join("rust")
        .join("target")
        .join("debug")
        .join("agov");
    if debug.is_file() {
        return Ok(AgovCommand {
            program: debug,
            prefix: Vec::new(),
            cwd: root,
        });
    }

    if let Some(agov) = executable_on_path(if cfg!(windows) { "agov.exe" } else { "agov" }) {
        return Ok(AgovCommand {
            program: agov,
            prefix: Vec::new(),
            cwd: root,
        });
    }

    let cargo = executable_on_path(if cfg!(windows) { "cargo.exe" } else { "cargo" })
        .ok_or_else(|| {
            "could not resolve agov; tried AGOV_MCP_AGOV_BIN, repository release/debug builds, agov on PATH, and cargo on PATH"
                .to_string()
        })?;
    Ok(AgovCommand {
        program: cargo,
        prefix: vec![
            "run".to_string(),
            "--quiet".to_string(),
            "--manifest-path".to_string(),
            root.join("tooling")
                .join("rust")
                .join("Cargo.toml")
                .to_string_lossy()
                .to_string(),
            "-p".to_string(),
            "agov".to_string(),
            "--".to_string(),
        ],
        cwd: root,
    })
}

fn tool_cli_args(name: &str, args: &Value) -> Result<Vec<String>, String> {
    match name {
        "agov_validate" => {
            let mut cli = vec!["validate".to_string()];
            if let Some(check) = optional_string(args, "check")? {
                cli.push(check);
            }
            push_strings(args, "paths", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_validate" => {
            let mut cli = vec!["coord".to_string(), "validate".to_string()];
            push_required_strings(args, "files", &mut cli)?;
            Ok(cli)
        }
        "agov_plan_validate" => {
            let mut cli = vec!["plan".to_string(), "validate".to_string()];
            push_required_strings(args, "files", &mut cli)?;
            Ok(cli)
        }
        "agov_session_resume" => {
            let mut cli = vec!["session".to_string(), "resume".to_string()];
            push_optional_flag(args, "role", "--role", &mut cli)?;
            push_optional_flag(args, "base_branch", "--base-branch", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            Ok(cli)
        }
        "agov_handoff_next" => {
            let mut cli = vec!["handoff".to_string(), "next".to_string()];
            cli.push(required_string(args, "role")?);
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            Ok(cli)
        }
        "agov_handoff_take" => {
            let mut cli = vec!["handoff".to_string(), "take".to_string()];
            cli.push(required_string(args, "id")?);
            push_required_flag(args, "from", "--from", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            Ok(cli)
        }
        "agov_handoff_complete" => {
            let mut cli = vec!["handoff".to_string(), "complete".to_string()];
            cli.push(required_string(args, "id")?);
            push_required_flag(args, "from", "--from", &mut cli)?;
            push_optional_flag(args, "to", "--to", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_post" => {
            let mut cli = vec!["coord".to_string(), "post".to_string()];
            push_optional_flag(args, "id_format", "--id-format", &mut cli)?;
            push_required_flag(args, "from", "--from", &mut cli)?;
            push_required_flag(args, "to", "--to", &mut cli)?;
            push_required_flag(args, "kind", "--kind", &mut cli)?;
            push_required_flag(args, "topic", "--topic", &mut cli)?;
            push_repeated_flag(args, "refs", "--ref", &mut cli)?;
            push_repeated_flag(args, "artifacts", "--artifact", &mut cli)?;
            push_repeated_flag(args, "tasks", "--task", &mut cli)?;
            push_optional_flag(args, "model", "--model", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_list" => {
            let mut cli = vec!["coord".to_string(), "list".to_string()];
            push_optional_flag(args, "since", "--since", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            push_optional_flag(args, "format", "--format", &mut cli)?;
            Ok(cli)
        }
        other => Err(format!("unknown tool: {other}")),
    }
}

fn push_required_strings(args: &Value, key: &str, cli: &mut Vec<String>) -> Result<(), String> {
    let values = string_array(args, key)?;
    if values.is_empty() {
        return Err(format!("{key} must contain at least one item"));
    }
    cli.extend(values);
    Ok(())
}

fn push_strings(args: &Value, key: &str, cli: &mut Vec<String>) -> Result<(), String> {
    cli.extend(string_array(args, key)?);
    Ok(())
}

fn push_required_flag(
    args: &Value,
    key: &str,
    flag: &str,
    cli: &mut Vec<String>,
) -> Result<(), String> {
    let value = required_string(args, key)?;
    cli.push(flag.to_string());
    cli.push(value);
    Ok(())
}

fn push_optional_flag(
    args: &Value,
    key: &str,
    flag: &str,
    cli: &mut Vec<String>,
) -> Result<(), String> {
    if let Some(value) = optional_string(args, key)? {
        cli.push(flag.to_string());
        cli.push(value);
    }
    Ok(())
}

fn push_repeated_flag(
    args: &Value,
    key: &str,
    flag: &str,
    cli: &mut Vec<String>,
) -> Result<(), String> {
    for value in string_array(args, key)? {
        cli.push(flag.to_string());
        cli.push(value);
    }
    Ok(())
}

fn required_string(args: &Value, key: &str) -> Result<String, String> {
    optional_string(args, key)?.ok_or_else(|| format!("{key} is required"))
}

fn optional_string(args: &Value, key: &str) -> Result<Option<String>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) if !value.trim().is_empty() => Ok(Some(value.clone())),
        Some(Value::String(_)) => Err(format!("{key} must be non-empty")),
        Some(_) => Err(format!("{key} must be a string")),
    }
}

fn string_array(args: &Value, key: &str) -> Result<Vec<String>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(values)) => values
            .iter()
            .map(|value| match value {
                Value::String(item) if !item.trim().is_empty() => Ok(item.clone()),
                Value::String(_) => Err(format!("{key} must not contain empty strings")),
                _ => Err(format!("{key} must contain only strings")),
            })
            .collect(),
        Some(_) => Err(format!("{key} must be an array of strings")),
    }
}

fn tools() -> Vec<Value> {
    vec![
        tool(
            "agov_validate",
            "Run agov validate, optionally scoped to one validator.",
            json!({
                "type": "object",
                "properties": {
                    "check": { "type": "string" },
                    "paths": { "type": "array", "items": { "type": "string" } }
                }
            }),
        ),
        tool(
            "agov_coord_validate",
            "Validate coordination message files.",
            json!({
                "type": "object",
                "required": ["files"],
                "properties": {
                    "files": { "type": "array", "items": { "type": "string" }, "minItems": 1 }
                }
            }),
        ),
        tool(
            "agov_plan_validate",
            "Validate Gate 1 plan files.",
            json!({
                "type": "object",
                "required": ["files"],
                "properties": {
                    "files": { "type": "array", "items": { "type": "string" }, "minItems": 1 }
                }
            }),
        ),
        tool(
            "agov_session_resume",
            "Summarize AGF session state.",
            json!({
                "type": "object",
                "properties": {
                    "role": { "type": "string" },
                    "base_branch": { "type": "string" },
                    "dir": { "type": "string" }
                }
            }),
        ),
        tool(
            "agov_handoff_next",
            "Show the next pending handoff for a role.",
            json!({
                "type": "object",
                "required": ["role"],
                "properties": {
                    "role": { "type": "string" },
                    "dir": { "type": "string" }
                }
            }),
        ),
        tool(
            "agov_handoff_take",
            "Acknowledge and take a pending handoff.",
            json!({
                "type": "object",
                "required": ["id", "from"],
                "properties": {
                    "id": { "type": "string" },
                    "from": { "type": "string" },
                    "dir": { "type": "string" }
                }
            }),
        ),
        tool(
            "agov_handoff_complete",
            "Mark a pending handoff complete.",
            json!({
                "type": "object",
                "required": ["id", "from"],
                "properties": {
                    "id": { "type": "string" },
                    "from": { "type": "string" },
                    "to": { "type": "string" },
                    "dir": { "type": "string" }
                }
            }),
        ),
        tool(
            "agov_coord_post",
            "Write and validate a coordination message.",
            json!({
                "type": "object",
                "required": ["from", "to", "kind", "topic"],
                "properties": {
                    "id_format": { "type": "string", "enum": ["readable", "ulid"] },
                    "from": { "type": "string" },
                    "to": { "type": "string" },
                    "kind": { "type": "string" },
                    "topic": { "type": "string" },
                    "refs": { "type": "array", "items": { "type": "string" } },
                    "artifacts": { "type": "array", "items": { "type": "string" } },
                    "tasks": { "type": "array", "items": { "type": "string" } },
                    "model": { "type": "string" },
                    "dir": { "type": "string" }
                }
            }),
        ),
        tool(
            "agov_coord_list",
            "List coordination messages after a cursor.",
            json!({
                "type": "object",
                "properties": {
                    "since": { "type": "string" },
                    "dir": { "type": "string" },
                    "format": { "type": "string", "enum": ["text", "json"] }
                }
            }),
        ),
    ]
}

fn tool(name: &str, description: &str, input_schema: Value) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema
    })
}

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    fn result(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: Value, code: i64, message: String) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(JsonRpcError { code, message }),
        }
    }
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

struct AgovCommand {
    program: PathBuf,
    prefix: Vec<String>,
    cwd: PathBuf,
}

#[derive(Debug, Serialize)]
struct ToolExecution {
    ok: bool,
    status: i32,
    command: Vec<String>,
    stdout: String,
    stderr: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_list_contains_only_cli_backed_tools() {
        let names: Vec<String> = tools()
            .into_iter()
            .map(|tool| tool["name"].as_str().unwrap().to_string())
            .collect();

        assert_eq!(
            names,
            vec![
                "agov_validate",
                "agov_coord_validate",
                "agov_plan_validate",
                "agov_session_resume",
                "agov_handoff_next",
                "agov_handoff_take",
                "agov_handoff_complete",
                "agov_coord_post",
                "agov_coord_list",
            ]
        );
    }

    /// Parity guard for the Workstream C transport-attribution boundary:
    /// the tool names this adapter publishes (and therefore may set as the
    /// AGOV_MCP_TOOL marker after tool_cli_args validation) must exactly
    /// match the closed allowlist `resolve_transport` accepts on the child
    /// side. A drift in either direction would silently drop `tool`
    /// attribution for the drifted name.
    #[test]
    fn published_tools_match_core_mcp_tool_allowlist() {
        let published: Vec<String> = tools()
            .into_iter()
            .map(|tool| tool["name"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(published, agov_core::roi::MCP_TOOL_ALLOWLIST.to_vec());
    }

    /// Gate 2 review `d8h5` BLOCK 3, third leg: `published_tools_match_
    /// core_mcp_tool_allowlist` only proves `tools()` (the published MCP
    /// tool list) matches `MCP_TOOL_ALLOWLIST`. It says nothing about
    /// `tool_cli_args`'s own `match name { ... }` arms -- a third,
    /// independent source of truth for "which tool names exist." This
    /// test proves `tool_cli_args` accepts every allowlisted name with
    /// minimally-valid arguments (so the match has a real arm for each,
    /// not just a coincidentally-overlapping subset) and rejects any name
    /// outside the allowlist.
    #[test]
    fn tool_cli_args_accepts_exactly_the_allowlisted_names() {
        let minimal_args: &[(&str, Value)] = &[
            ("agov_validate", json!({})),
            ("agov_coord_validate", json!({ "files": ["a.md"] })),
            ("agov_plan_validate", json!({ "files": ["a.md"] })),
            ("agov_session_resume", json!({})),
            ("agov_handoff_next", json!({ "role": "coder-tester" })),
            (
                "agov_handoff_take",
                json!({ "id": "x", "from": "coder-tester" }),
            ),
            (
                "agov_handoff_complete",
                json!({ "id": "x", "from": "coder-tester" }),
            ),
            (
                "agov_coord_post",
                json!({
                    "from": "coder-tester",
                    "to": "governor-reviewer",
                    "kind": "note",
                    "topic": "t"
                }),
            ),
            ("agov_coord_list", json!({})),
        ];

        assert_eq!(
            minimal_args.len(),
            agov_core::roi::MCP_TOOL_ALLOWLIST.len(),
            "this test's fixture must cover every allowlisted name exactly once"
        );
        for (name, args) in minimal_args {
            assert!(
                agov_core::roi::MCP_TOOL_ALLOWLIST.contains(name),
                "{name} is not in MCP_TOOL_ALLOWLIST -- fixture/allowlist drift"
            );
            assert!(
                tool_cli_args(name, args).is_ok(),
                "tool_cli_args should accept allowlisted name {name}"
            );
        }

        assert!(
            tool_cli_args("agov_totally_unrecognized_tool", &json!({})).is_err(),
            "tool_cli_args must reject a name outside the allowlist"
        );
    }

    #[test]
    fn coord_post_maps_to_cli_arguments() {
        let args = tool_cli_args(
            "agov_coord_post",
            &json!({
                "from": "coder-tester",
                "to": "governor-reviewer",
                "kind": "handoff",
                "topic": "test",
                "refs": ["20260720T100000Z-coder-tester-abcd"],
                "tasks": ["Ship it"],
                "dir": "docs/coordination"
            }),
        )
        .unwrap();

        assert_eq!(
            args,
            vec![
                "coord",
                "post",
                "--from",
                "coder-tester",
                "--to",
                "governor-reviewer",
                "--kind",
                "handoff",
                "--topic",
                "test",
                "--ref",
                "20260720T100000Z-coder-tester-abcd",
                "--task",
                "Ship it",
                "--dir",
                "docs/coordination",
            ]
        );
    }

    #[test]
    fn malformed_tool_arguments_return_json_rpc_error() {
        let response = handle_message(
            r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"agov_handoff_next","arguments":{}}}"#,
        )
        .unwrap();

        assert_eq!(response.error.unwrap().message, "role is required");
    }

    #[test]
    fn coord_list_exit_two_is_polling_empty_not_transport_error() {
        let args = vec!["coord".to_string(), "list".to_string()];

        assert!(is_expected_non_error_status(&args, Some(2)));
    }
}
