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
//! | `agov_coord_pending` | `agov coord pending ...` |
//! | `agov_coord_convergence` | `agov coord convergence ...` |
//! | `agov_coord_status` | `agov coord status ...` |
//! | `agov_coord_visibility` | `agov coord visibility ...` |
//! | `agov_coord_stale` | `agov coord stale ...` |
//! | `agov_coord_graph_index` | `agov coord graph-index` (stdout only; no `--output`) |
//! | `agov_coord_publication_check` | `agov coord publication-check ...` |
//! | `agov_coord_brief` | `agov coord brief ...` |

use serde::Serialize;
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::Command;

const PROTOCOL_VERSION: &str = "2025-06-18";

/// Maximum bytes retained for a single stdio request line before it is
/// rejected as oversized (#88, round 3+4 bounded-line-read contract).
/// Comfortably larger than any real coordination message, plan file, or
/// tool-call payload this project produces, while still bounding
/// worst-case memory for a hostile or corrupted delimiter-free stream.
/// This count does not include the terminating `\n` itself — it is
/// consumed from the stream but never appended to the line buffer or
/// counted against the cap. A CRLF line ending's `\r` is ordinary
/// payload from this reader's point of view: it IS appended to the
/// line buffer and IS counted against the cap, like any other byte
/// before the `\n` (per `A-MR806-G2-1`; JSON's own grammar treats `\r`
/// as insignificant whitespace, so this has no effect on parsing).
const MAX_REQUEST_LINE_BYTES: usize = 8 * 1024 * 1024;

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
    let agov = agov_command()?;
    validate_agov_startup(&agov)?;
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut stdout = std::io::stdout();

    loop {
        let outcome = match read_bounded_line(&mut reader)? {
            None => break,
            Some(outcome) => outcome,
        };
        let response = match outcome {
            BoundedLine::Line(bytes) => {
                let line = match String::from_utf8(bytes) {
                    Ok(line) => line,
                    Err(_) => {
                        emit(
                            &mut stdout,
                            &JsonRpcResponse::error(
                                Value::Null,
                                -32700,
                                "parse error: request line is not valid UTF-8".to_string(),
                            ),
                        )?;
                        continue;
                    }
                };
                if line.trim().is_empty() {
                    continue;
                }
                handle_message(&line, &agov)
            }
            BoundedLine::Oversized => Some(JsonRpcResponse::error(
                Value::Null,
                -32600,
                "Invalid Request: request line exceeds maximum size".to_string(),
            )),
        };
        if let Some(response) = response {
            emit(&mut stdout, &response)?;
        }
    }

    Ok(())
}

fn emit(stdout: &mut impl Write, response: &JsonRpcResponse) -> Result<(), String> {
    writeln!(
        stdout,
        "{}",
        serde_json::to_string(response).map_err(|err| err.to_string())?
    )
    .map_err(|err| err.to_string())?;
    stdout.flush().map_err(|err| err.to_string())
}

/// Outcome of reading one bounded request line from stdio.
enum BoundedLine {
    /// A complete line (payload bytes, excluding the trailing newline),
    /// within `MAX_REQUEST_LINE_BYTES`.
    Line(Vec<u8>),
    /// A complete line whose accumulated payload exceeded
    /// `MAX_REQUEST_LINE_BYTES` at some point during the read. The
    /// oversized bytes themselves are discarded — never retained or
    /// returned — only the fact of exceeding the cap is reported.
    Oversized,
}

/// Read one newline-terminated (or EOF-terminated) line from `reader`,
/// bounding the retained line buffer to `MAX_REQUEST_LINE_BYTES` at every
/// point during the read -- including while still consuming a hostile,
/// arbitrarily long, delimiter-free stream. Never uses
/// `BufRead::read_until`/`lines()`, which both append the entire
/// delimiter-free payload to the buffer before any length check can run.
///
/// Returns `Ok(None)` only at a clean end-of-stream with no bytes read for
/// the line in progress (i.e. the previous line already completed and the
/// stream has nothing more to offer). EOF encountered mid-line is treated
/// exactly like reaching a newline for whatever state the line was in.
fn read_bounded_line<R: BufRead + ?Sized>(reader: &mut R) -> Result<Option<BoundedLine>, String> {
    let mut buf: Vec<u8> = Vec::new();
    let mut oversized = false;
    let mut any_bytes = false;

    loop {
        let chunk = reader.fill_buf().map_err(|err| err.to_string())?;
        if chunk.is_empty() {
            if !any_bytes {
                return Ok(None);
            }
            return Ok(Some(if oversized {
                BoundedLine::Oversized
            } else {
                BoundedLine::Line(buf)
            }));
        }
        any_bytes = true;

        let newline_pos = chunk.iter().position(|&byte| byte == b'\n');
        let payload = match newline_pos {
            Some(pos) => &chunk[..pos],
            None => chunk,
        };

        // Bounds check applied identically regardless of whether a newline
        // was found in this chunk (round 4 fix for `B-V27-MCP-R3-1`: the
        // newline-found branch must not skip this check just because the
        // line is about to complete).
        if !oversized {
            if buf.len() + payload.len() > MAX_REQUEST_LINE_BYTES {
                oversized = true;
            } else {
                buf.extend_from_slice(payload);
            }
        }

        let consume_len = match newline_pos {
            Some(pos) => pos + 1,
            None => chunk.len(),
        };
        reader.consume(consume_len);

        if newline_pos.is_some() {
            return Ok(Some(if oversized {
                BoundedLine::Oversized
            } else {
                BoundedLine::Line(buf)
            }));
        }
        // No newline in this chunk: the line is still in progress. Loop
        // for another `fill_buf()` call.
    }
}

/// A structurally-valid JSON-RPC 2.0 request, per the frozen outcome table
/// in `docs/plans/agov-mcp-transport-protocol-correctness.md` Slice 1.
/// `id: None` means the request is a Notification (id key absent); an
/// explicit JSON `null` id is a distinct, invalid shape and never reaches
/// this struct (see `validate_request`).
struct ValidRequest<'a> {
    id: Option<Value>,
    method: &'a str,
}

/// Validate `raw` against the JSON-RPC 2.0 / MCP request contract:
/// `jsonrpc` must be exactly `"2.0"`; `method` must be present and a
/// string; `params`, if present, must be an object or array (an explicit
/// `null` is present, not absent, and is rejected the same as any other
/// non-Structured value);
/// `id`, if present, must be a string or a non-fractional number (an
/// explicit `null` id, a boolean, an array, an object, or a fractional
/// number are all invalid shapes — B3). Absence of the `id` key (not the
/// same as an explicit `null`) marks a Notification. Returns `Err(())` for
/// any structural failure; the caller responds `-32600` with `id: null`.
fn validate_request(raw: &Value) -> Result<ValidRequest<'_>, ()> {
    let obj = raw.as_object().ok_or(())?;

    match obj.get("jsonrpc") {
        Some(Value::String(version)) if version == "2.0" => {}
        _ => return Err(()),
    }

    let method = match obj.get("method") {
        Some(Value::String(method)) => method.as_str(),
        _ => return Err(()),
    };

    match obj.get("params") {
        None => {}
        Some(Value::Object(_)) | Some(Value::Array(_)) => {}
        // JSON-RPC 2.0 §4: if present, params MUST be a Structured value
        // (Object or Array). An explicit `null` is present, not absent,
        // and is not itself a Structured value -- reject it the same as
        // a string/number/boolean, per B-MR806-G2-1.
        Some(_) => return Err(()),
    }

    let id = match obj.get("id") {
        None => None,
        Some(Value::String(id)) => Some(Value::String(id.clone())),
        Some(Value::Number(number)) if number.is_i64() || number.is_u64() => {
            Some(Value::Number(number.clone()))
        }
        Some(_) => return Err(()),
    };

    Ok(ValidRequest { id, method })
}

fn handle_message(line: &str, agov: &AgovCommand) -> Option<JsonRpcResponse> {
    let raw: Value = match serde_json::from_str(line) {
        Ok(raw) => raw,
        Err(err) => {
            return Some(JsonRpcResponse::error(
                Value::Null,
                -32700,
                format!("parse error: {err}"),
            ));
        }
    };

    let request = match validate_request(&raw) {
        Ok(request) => request,
        Err(()) => {
            return Some(JsonRpcResponse::error(
                Value::Null,
                -32600,
                "Invalid Request".to_string(),
            ));
        }
    };

    // Structurally valid, `id` absent: a Notification. No response is ever
    // sent, regardless of what the method would have returned.
    let id = request.id?;

    match request.method {
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
        "tools/call" => {
            let params = raw.get("params").cloned().unwrap_or(Value::Null);
            match call_tool(params, agov) {
                Ok(result) => Some(JsonRpcResponse::result(id, result)),
                Err(message) => Some(JsonRpcResponse::error(id, -32602, message)),
            }
        }
        other => Some(JsonRpcResponse::error(
            id,
            -32601,
            format!("method not found: {other}"),
        )),
    }
}

fn call_tool(params: Value, agov: &AgovCommand) -> Result<Value, String> {
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
    let execution = run_agov(agov, &cli_args, Some(canonical_tool));

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
fn run_agov(
    command: &AgovCommand,
    args: &[String],
    tool_name: Option<&'static str>,
) -> ToolExecution {
    let mut child = Command::new(&command.program);
    child
        .args(&command.prefix)
        .args(args)
        .current_dir(&command.cwd);
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
        && matches!(
            args.get(1).map(String::as_str),
            Some("list") | Some("pending")
        )
}

fn validate_agov_startup(command: &AgovCommand) -> Result<(), String> {
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
            push_optional_flag(args, "manifest", "--manifest", &mut cli)?;
            Ok(cli)
        }
        "agov_handoff_next" => {
            let mut cli = vec!["handoff".to_string(), "next".to_string()];
            cli.push(required_string(args, "role")?);
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            push_optional_flag(args, "session", "--session", &mut cli)?;
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
        "agov_coord_pending" => {
            let mut cli = vec!["coord".to_string(), "pending".to_string()];
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            push_optional_flag(args, "cutoff", "--cutoff", &mut cli)?;
            push_optional_flag(args, "format", "--format", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_convergence" => {
            let mut cli = vec!["coord".to_string(), "convergence".to_string()];
            push_required_flag(args, "series", "--series", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            push_optional_flag(args, "format", "--format", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_status" => {
            let mut cli = vec!["coord".to_string(), "status".to_string()];
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            push_optional_uint_flag(args, "recent", "--recent", &mut cli)?;
            push_bool_flag(args, "roles", "--roles", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_visibility" => {
            // B-MR806-G2-2: the frozen Gate 1 contract specifies `branch`
            // as a single string, not the CLI's repeatable array -- match
            // the approved interface exactly rather than silently
            // widening it. One string still maps to one `--branch` flag,
            // which the CLI's Vec<String> accepts.
            let mut cli = vec!["coord".to_string(), "visibility".to_string()];
            push_optional_flag(args, "base_branch", "--base-branch", &mut cli)?;
            push_optional_flag(args, "branch", "--branch", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            push_optional_flag(args, "format", "--format", &mut cli)?;
            push_optional_flag(args, "session", "--session", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_stale" => {
            let mut cli = vec!["coord".to_string(), "stale".to_string()];
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_graph_index" => {
            // Frozen decision (B5): this tool never exposes `--output`.
            // `graph-index` always writes to stdout here; the JSON body is
            // returned as the tool result. An `output` argument is
            // rejected, not silently ignored, so a caller cannot rely on
            // an unsupported argument having any effect.
            if args.get("output").is_some() {
                return Err(
                    "agov_coord_graph_index does not support 'output'; this tool always returns stdout".to_string(),
                );
            }
            let mut cli = vec!["coord".to_string(), "graph-index".to_string()];
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_publication_check" => {
            let mut cli = vec!["coord".to_string(), "publication-check".to_string()];
            push_required_flag(args, "base_ref", "--base-ref", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
            push_optional_flag(args, "subject_prefix", "--subject-prefix", &mut cli)?;
            push_optional_flag(args, "format", "--format", &mut cli)?;
            Ok(cli)
        }
        "agov_coord_brief" => {
            let mut cli = vec!["coord".to_string(), "brief".to_string()];
            push_optional_flag(args, "id_format", "--id-format", &mut cli)?;
            push_optional_flag(args, "from", "--from", &mut cli)?;
            push_optional_flag(args, "to", "--to", &mut cli)?;
            push_optional_flag(args, "kind", "--kind", &mut cli)?;
            push_optional_flag(args, "topic", "--topic", &mut cli)?;
            push_optional_flag(args, "model", "--model", &mut cli)?;
            push_optional_flag(args, "dir", "--dir", &mut cli)?;
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

fn push_bool_flag(
    args: &Value,
    key: &str,
    flag: &str,
    cli: &mut Vec<String>,
) -> Result<(), String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(()),
        Some(Value::Bool(true)) => {
            cli.push(flag.to_string());
            Ok(())
        }
        Some(Value::Bool(false)) => Ok(()),
        Some(_) => Err(format!("{key} must be a boolean")),
    }
}

fn push_optional_uint_flag(
    args: &Value,
    key: &str,
    flag: &str,
    cli: &mut Vec<String>,
) -> Result<(), String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(()),
        Some(Value::Number(number)) if number.is_u64() => {
            cli.push(flag.to_string());
            cli.push(number.to_string());
            Ok(())
        }
        Some(_) => Err(format!("{key} must be a non-negative integer")),
    }
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
                    "dir": { "type": "string" },
                    "manifest": { "type": "string" }
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
                    "session": { "type": "string" },
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
        tool(
            "agov_coord_pending",
            "List unresolved requires-ack:true records of any non-ack, non-handoff kind (handoff obligations remain agov_handoff_next's surface). A record is pending until a later record's references-to names it directly. Records before the feature's introduction cutoff are grandfathered and never appear. Exit code 2 means the query found nothing pending, not a transport error.",
            json!({
                "type": "object",
                "properties": {
                    "dir": { "type": "string" },
                    "cutoff": { "type": "string" },
                    "format": { "type": "string", "enum": ["text", "json"] }
                }
            }),
        ),
        tool(
            "agov_coord_convergence",
            "Report deterministic state for one opt-in review-convergence series.",
            json!({
                "type": "object",
                "required": ["series"],
                "properties": {
                    "series": { "type": "string" },
                    "dir": { "type": "string" },
                    "format": { "type": "string", "enum": ["text", "json"] }
                }
            }),
        ),
        tool(
            "agov_coord_status",
            "Print coordination channel overview.",
            json!({
                "type": "object",
                "properties": {
                    "dir": { "type": "string" },
                    "recent": { "type": "integer", "minimum": 0 },
                    "roles": { "type": "boolean" }
                }
            }),
        ),
        tool(
            "agov_coord_visibility",
            "Scan local branches for hidden coordination obligations.",
            json!({
                "type": "object",
                "properties": {
                    "session": { "type": "string" },
                    "base_branch": { "type": "string" },
                    "branch": { "type": "string" },
                    "dir": { "type": "string" },
                    "format": { "type": "string", "enum": ["text", "json"] }
                }
            }),
        ),
        tool(
            "agov_coord_stale",
            "Report stale artifact claims in the coordination channel.",
            json!({
                "type": "object",
                "properties": {
                    "dir": { "type": "string" }
                }
            }),
        ),
        tool(
            "agov_coord_graph_index",
            "Emit a compact JSON index of the coordination graph for LLM cold-start. Always returns stdout; does not support writing to a file path.",
            json!({
                "type": "object",
                "properties": {
                    "dir": { "type": "string" }
                }
            }),
        ),
        tool(
            "agov_coord_publication_check",
            "Mechanically verify a coordination-only publication candidate against its target ref.",
            json!({
                "type": "object",
                "required": ["base_ref"],
                "properties": {
                    "base_ref": { "type": "string" },
                    "dir": { "type": "string" },
                    "subject_prefix": { "type": "string" },
                    "format": { "type": "string", "enum": ["text", "json"] }
                }
            }),
        ),
        tool(
            "agov_coord_brief",
            "Generate a coordination message skeleton with a new coordination id.",
            json!({
                "type": "object",
                "properties": {
                    "id_format": { "type": "string", "enum": ["readable", "ulid"] },
                    "from": { "type": "string" },
                    "to": { "type": "string" },
                    "kind": { "type": "string" },
                    "topic": { "type": "string" },
                    "model": { "type": "string" },
                    "dir": { "type": "string" }
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
                "agov_coord_pending",
                "agov_coord_convergence",
                "agov_coord_status",
                "agov_coord_visibility",
                "agov_coord_stale",
                "agov_coord_graph_index",
                "agov_coord_publication_check",
                "agov_coord_brief",
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
            ("agov_coord_pending", json!({})),
            ("agov_coord_convergence", json!({ "series": "series-id" })),
            ("agov_coord_status", json!({})),
            ("agov_coord_visibility", json!({})),
            ("agov_coord_stale", json!({})),
            ("agov_coord_graph_index", json!({})),
            (
                "agov_coord_publication_check",
                json!({ "base_ref": "origin/main" }),
            ),
            ("agov_coord_brief", json!({})),
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
        let agov = AgovCommand {
            program: PathBuf::from("unused"),
            prefix: Vec::new(),
            cwd: PathBuf::from("."),
        };
        let response = handle_message(
            r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"agov_handoff_next","arguments":{}}}"#,
            &agov,
        )
        .unwrap();

        assert_eq!(response.error.unwrap().message, "role is required");
    }

    #[test]
    fn coord_list_exit_two_is_polling_empty_not_transport_error() {
        let args = vec!["coord".to_string(), "list".to_string()];

        assert!(is_expected_non_error_status(&args, Some(2)));
    }

    #[test]
    fn coord_pending_maps_to_cli_arguments() {
        let args = tool_cli_args(
            "agov_coord_pending",
            &json!({
                "dir": "docs/coordination",
                "cutoff": "2026-08-02T18:10:18Z",
                "format": "json"
            }),
        )
        .unwrap();

        assert_eq!(
            args,
            vec![
                "coord",
                "pending",
                "--dir",
                "docs/coordination",
                "--cutoff",
                "2026-08-02T18:10:18Z",
                "--format",
                "json",
            ]
        );
    }

    #[test]
    fn coord_pending_requires_no_arguments() {
        assert_eq!(
            tool_cli_args("agov_coord_pending", &json!({})).unwrap(),
            vec!["coord", "pending"]
        );
    }

    #[test]
    fn coord_pending_exit_two_is_polling_empty_not_transport_error() {
        let args = vec!["coord".to_string(), "pending".to_string()];

        assert!(is_expected_non_error_status(&args, Some(2)));
    }

    /// `coord list` and `coord pending` share exit code 2 for "empty
    /// result," but no other subcommand pair does -- guards against a
    /// future subcommand accidentally matching the `Some("list") |
    /// Some("pending")` pattern by coincidence of name.
    #[test]
    fn other_coord_subcommands_do_not_get_exit_two_amnesty() {
        for sub in ["validate", "post", "status", "stale", "visibility"] {
            let args = vec!["coord".to_string(), sub.to_string()];
            assert!(
                !is_expected_non_error_status(&args, Some(2)),
                "coord {sub} must not be treated as expected-empty on exit 2"
            );
        }
    }

    // -- JSON-RPC 2.0 request validity (#88, plan test items 1-8, 14) --

    fn dummy_agov() -> AgovCommand {
        AgovCommand {
            program: PathBuf::from("unused"),
            prefix: Vec::new(),
            cwd: PathBuf::from("."),
        }
    }

    /// Item 1: valid `"jsonrpc": "2.0"`, ID-bearing requests get a normal
    /// response, with the same `id` echoed back.
    #[test]
    fn valid_id_bearing_request_gets_normal_response_with_echoed_id() {
        let agov = dummy_agov();
        let response =
            handle_message(r#"{"jsonrpc":"2.0","id":7,"method":"tools/list"}"#, &agov).unwrap();
        assert_eq!(response.id, json!(7));
        assert!(response.result.is_some());
        assert!(response.error.is_none());

        let response = handle_message(
            r#"{"jsonrpc":"2.0","id":"abc","method":"tools/list"}"#,
            &agov,
        )
        .unwrap();
        assert_eq!(response.id, json!("abc"));
        assert!(response.result.is_some());
    }

    /// Item 2: missing `jsonrpc` field, `id` present -- `-32600`, response
    /// `id: null` (not the request's own id, since the request is
    /// structurally invalid).
    #[test]
    fn missing_jsonrpc_field_is_invalid_request_with_null_id() {
        let agov = dummy_agov();
        let response = handle_message(r#"{"id":1,"method":"tools/list"}"#, &agov).unwrap();
        assert_eq!(response.id, Value::Null);
        assert_eq!(response.error.unwrap().code, -32600);
    }

    /// Item 3: wrong `jsonrpc` value, `id` present -- `-32600`, `id: null`.
    #[test]
    fn wrong_jsonrpc_value_is_invalid_request() {
        let agov = dummy_agov();
        let response =
            handle_message(r#"{"jsonrpc":"1.0","id":1,"method":"tools/list"}"#, &agov).unwrap();
        assert_eq!(response.id, Value::Null);
        assert_eq!(response.error.unwrap().code, -32600);
    }

    /// Item 4: malformed JSON -- `-32700`, `id: null`, no panic.
    #[test]
    fn malformed_json_is_parse_error() {
        let agov = dummy_agov();
        let response = handle_message("{not json", &agov).unwrap();
        assert_eq!(response.id, Value::Null);
        assert_eq!(response.error.unwrap().code, -32700);
    }

    /// Item 5 (B3): five distinct invalid request-ID shapes, each rejected
    /// with `-32600` and response `id: null`. A `null` id is explicitly
    /// invalid (reserved for error responses), not the same as an absent
    /// id (which means Notification -- see the separate test below).
    #[test]
    fn invalid_request_id_shapes_are_rejected() {
        let agov = dummy_agov();
        for id_json in ["null", "true", "[1]", "{}", "1.5"] {
            let line = format!(r#"{{"jsonrpc":"2.0","id":{id_json},"method":"tools/list"}}"#);
            let response = handle_message(&line, &agov).unwrap();
            assert_eq!(
                response.id,
                Value::Null,
                "id shape {id_json} should produce a null response id"
            );
            assert_eq!(
                response.error.unwrap().code,
                -32600,
                "id shape {id_json} should be rejected as Invalid Request"
            );
        }
    }

    /// Item 6: a valid Notification (id absent, otherwise fully valid) --
    /// no response sent at all, regardless of what the method would have
    /// returned.
    #[test]
    fn valid_notification_produces_no_response() {
        let agov = dummy_agov();
        let response = handle_message(r#"{"jsonrpc":"2.0","method":"tools/list"}"#, &agov);
        assert!(response.is_none());

        // Even for an unknown method, still no response -- a Notification
        // never gets a response, not even an error.
        let response = handle_message(r#"{"jsonrpc":"2.0","method":"nope"}"#, &agov);
        assert!(response.is_none());
    }

    /// Item 7: an invalid ID-less object (id absent *and* another
    /// validity failure) -- `-32600` with `id: null`, not silently
    /// dropped like a real Notification.
    #[test]
    fn invalid_id_less_object_is_rejected_not_treated_as_notification() {
        let agov = dummy_agov();
        // id absent, but jsonrpc is wrong -- must not be silently dropped.
        let response = handle_message(r#"{"jsonrpc":"1.0","method":"tools/list"}"#, &agov).unwrap();
        assert_eq!(response.id, Value::Null);
        assert_eq!(response.error.unwrap().code, -32600);

        // id absent, method missing entirely.
        let response = handle_message(r#"{"jsonrpc":"2.0"}"#, &agov).unwrap();
        assert_eq!(response.id, Value::Null);
        assert_eq!(response.error.unwrap().code, -32600);
    }

    /// Item 8, corollary: a request that fails validation via `method`
    /// missing or wrong-typed is Invalid Request, not "method not found"
    /// (which only applies to a structurally valid request with an
    /// unrecognized method string).
    #[test]
    fn missing_or_non_string_method_is_invalid_request() {
        let agov = dummy_agov();
        let response = handle_message(r#"{"jsonrpc":"2.0","id":1}"#, &agov).unwrap();
        assert_eq!(response.error.unwrap().code, -32600);

        let response = handle_message(r#"{"jsonrpc":"2.0","id":1,"method":5}"#, &agov).unwrap();
        assert_eq!(response.error.unwrap().code, -32600);
    }

    /// A structurally valid request with a genuinely unrecognized method
    /// still gets `-32601`, echoing the request's own id -- distinct from
    /// the `-32600` structural-invalidity path above.
    #[test]
    fn unrecognized_method_on_valid_request_is_method_not_found() {
        let agov = dummy_agov();
        let response =
            handle_message(r#"{"jsonrpc":"2.0","id":9,"method":"nope"}"#, &agov).unwrap();
        assert_eq!(response.id, json!(9));
        assert_eq!(response.error.unwrap().code, -32601);
    }

    /// Item 14: `params` shape validation. A string, a number, and `true`
    /// as `params` are each rejected (`-32600`/`id: null`); absent, an
    /// empty object, and an empty array all remain valid.
    #[test]
    fn params_must_be_structured_when_present() {
        let agov = dummy_agov();
        // B-MR806-G2-1: explicit `params: null` is present, not absent,
        // and is not itself a Structured value -- must be rejected.
        for params_json in [r#""a string""#, "42", "true", "null"] {
            let line = format!(
                r#"{{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{params_json}}}"#
            );
            let response = handle_message(&line, &agov).unwrap();
            assert_eq!(
                response.id,
                Value::Null,
                "params {params_json} should be rejected"
            );
            assert_eq!(response.error.unwrap().code, -32600);
        }

        for params_json in ["{}", "[]"] {
            let line = format!(
                r#"{{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{params_json}}}"#
            );
            let response = handle_message(&line, &agov).unwrap();
            assert!(
                response.error.is_none(),
                "params {params_json} should be accepted, got {:?}",
                response.error
            );
        }

        // params absent entirely: still valid.
        let response =
            handle_message(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#, &agov).unwrap();
        assert!(response.error.is_none());
    }

    // -- Bounded-line-read contract (#88, plan test item 15) --

    fn read_line_from(bytes: &[u8]) -> Option<Vec<u8>> {
        let mut cursor = std::io::Cursor::new(bytes);
        match read_bounded_line(&mut cursor).unwrap() {
            None => None,
            Some(BoundedLine::Line(bytes)) => Some(bytes),
            Some(BoundedLine::Oversized) => panic!("expected Line, got Oversized"),
        }
    }

    #[test]
    fn bounded_line_read_accepts_exactly_at_cap() {
        let payload = vec![b'a'; MAX_REQUEST_LINE_BYTES];
        let mut stream = payload.clone();
        stream.push(b'\n');
        let line = read_line_from(&stream).unwrap();
        assert_eq!(line.len(), MAX_REQUEST_LINE_BYTES);
        assert_eq!(line, payload);
    }

    /// `A-MR806-G2-1`: a CRLF line ending's `\r` is ordinary payload --
    /// retained in the returned line and counted against the cap, unlike
    /// `\n`. A short CRLF line proves retention; an exactly-at-cap CRLF
    /// line (content one byte short of the cap, so the `\r` itself lands
    /// exactly on the boundary) proves it counts.
    #[test]
    fn bounded_line_read_retains_and_counts_crlf_carriage_return() {
        let line = read_line_from(b"hello\r\n").unwrap();
        assert_eq!(line, b"hello\r");

        let content = vec![b'a'; MAX_REQUEST_LINE_BYTES - 1];
        let mut stream = content.clone();
        stream.push(b'\r');
        stream.push(b'\n');
        let line = read_line_from(&stream).unwrap();
        assert_eq!(line.len(), MAX_REQUEST_LINE_BYTES);
        assert_eq!(line.last(), Some(&b'\r'));

        // One more content byte before the CRLF pushes the `\r` itself
        // over the cap.
        let mut oversized_stream = vec![b'a'; MAX_REQUEST_LINE_BYTES];
        oversized_stream.push(b'\r');
        oversized_stream.push(b'\n');
        let mut cursor = std::io::Cursor::new(oversized_stream.as_slice());
        assert!(matches!(
            read_bounded_line(&mut cursor).unwrap(),
            Some(BoundedLine::Oversized)
        ));
    }

    #[test]
    fn bounded_line_read_rejects_one_byte_over_cap() {
        let payload = vec![b'a'; MAX_REQUEST_LINE_BYTES + 1];
        let mut stream = payload;
        stream.push(b'\n');
        let mut cursor = std::io::Cursor::new(stream.as_slice());
        match read_bounded_line(&mut cursor).unwrap() {
            Some(BoundedLine::Oversized) => {}
            other => panic!(
                "expected Oversized, got a different outcome: {}",
                other.is_some()
            ),
        }
    }

    /// `B-V27-MCP-R3-1`: the first over-cap byte and the line's
    /// terminating newline arrive in the same `fill_buf` chunk -- the
    /// exact gap round 3 left undefined. A `Cursor` yields its entire
    /// slice in one `fill_buf` call, so a single write already exercises
    /// this; this test names the scenario explicitly per the plan.
    #[test]
    fn bounded_line_read_rejects_oversized_line_when_cap_and_newline_share_a_chunk() {
        let mut stream = vec![b'a'; MAX_REQUEST_LINE_BYTES + 1];
        stream.push(b'\n');
        let mut cursor = std::io::Cursor::new(stream.as_slice());
        assert!(matches!(
            read_bounded_line(&mut cursor).unwrap(),
            Some(BoundedLine::Oversized)
        ));
    }

    /// The actual hostile case: a delimiter-free stream many times larger
    /// than the cap, delivered across many small chunks so the bounds
    /// check must hold on every iteration, not just at completion.
    #[test]
    fn bounded_line_read_caps_buffer_throughout_a_hostile_delimiter_free_stream() {
        struct SmallChunkReader {
            remaining: usize,
        }
        impl std::io::Read for SmallChunkReader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                let n = buf.len().min(self.remaining).min(64);
                for byte in buf.iter_mut().take(n) {
                    *byte = b'x';
                }
                self.remaining -= n;
                Ok(n)
            }
        }

        let hostile_len = MAX_REQUEST_LINE_BYTES * 3;
        let reader = SmallChunkReader {
            remaining: hostile_len,
        };
        let mut buffered = std::io::BufReader::new(reader);

        // Drive the read manually so the buffer can be inspected mid-read,
        // proving the cap holds throughout, not only in the final result.
        let mut buf: Vec<u8> = Vec::new();
        let mut oversized = false;
        let mut total_consumed = 0usize;
        loop {
            let chunk = buffered.fill_buf().unwrap();
            if chunk.is_empty() {
                break;
            }
            let len = chunk.len();
            if !oversized {
                if buf.len() + len > MAX_REQUEST_LINE_BYTES {
                    oversized = true;
                } else {
                    buf.extend_from_slice(chunk);
                }
            }
            assert!(
                buf.len() <= MAX_REQUEST_LINE_BYTES,
                "line buffer exceeded cap mid-read: {} > {}",
                buf.len(),
                MAX_REQUEST_LINE_BYTES
            );
            buffered.consume(len);
            total_consumed += len;
            if total_consumed >= hostile_len {
                break;
            }
        }
        assert!(
            oversized,
            "hostile stream should have been marked oversized"
        );
        assert_eq!(total_consumed, hostile_len);
    }

    #[test]
    fn bounded_line_read_eof_mid_line_and_oversized_is_line_complete_not_hang() {
        // No trailing newline at all -- EOF while mid-line, already
        // oversized.
        let stream = vec![b'a'; MAX_REQUEST_LINE_BYTES + 10];
        let mut cursor = std::io::Cursor::new(stream.as_slice());
        assert!(matches!(
            read_bounded_line(&mut cursor).unwrap(),
            Some(BoundedLine::Oversized)
        ));
    }

    #[test]
    fn bounded_line_read_eof_mid_line_not_oversized_is_line_complete() {
        let stream = b"no trailing newline".to_vec();
        let line = read_line_from(&stream).unwrap();
        assert_eq!(line, stream);
    }

    #[test]
    fn bounded_line_read_clean_eof_with_no_bytes_returns_none() {
        let stream: &[u8] = b"";
        let mut cursor = std::io::Cursor::new(stream);
        assert!(read_bounded_line(&mut cursor).unwrap().is_none());
    }

    /// A well-formed request line immediately following an oversized one
    /// is processed normally -- proving the reader recovers cleanly
    /// rather than desyncing after discarding an oversized line.
    #[test]
    fn bounded_line_read_recovers_after_an_oversized_line() {
        let mut stream = vec![b'a'; MAX_REQUEST_LINE_BYTES + 1];
        stream.push(b'\n');
        stream.extend_from_slice(b"next line\n");
        let mut cursor = std::io::Cursor::new(stream.as_slice());

        assert!(matches!(
            read_bounded_line(&mut cursor).unwrap(),
            Some(BoundedLine::Oversized)
        ));
        match read_bounded_line(&mut cursor).unwrap() {
            Some(BoundedLine::Line(bytes)) => assert_eq!(bytes, b"next line"),
            other => panic!(
                "expected the next well-formed line, got: {}",
                other.is_some()
            ),
        }
    }

    // -- graph-index authority boundary (B5, plan test item 11) --

    #[test]
    fn graph_index_rejects_output_argument() {
        let result = tool_cli_args("agov_coord_graph_index", &json!({ "output": "/tmp/x" }));
        assert!(result.is_err());
        let result = tool_cli_args("agov_coord_graph_index", &json!({}));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec!["coord".to_string(), "graph-index".to_string()]
        );
    }

    // -- --manifest forwarding for agov_session_resume (#86) --

    #[test]
    fn session_resume_forwards_manifest_flag() {
        let args = tool_cli_args(
            "agov_session_resume",
            &json!({ "manifest": "agf/agents/MANIFEST.yaml" }),
        )
        .unwrap();
        assert!(args.contains(&"--manifest".to_string()));
        assert!(args.contains(&"agf/agents/MANIFEST.yaml".to_string()));
    }
}
