<!-- audience: A (ai-operator); answers: "which docs are meant for AI assistants to execute while humans supervise" -->
# agov MCP server

*Status: Deployable adapter. Audience: AI assistants and adopters wiring an
MCP-capable assistant to AGF tooling without relying on shell-command
prompting alone.*

`agov-mcp` is a local stdio Model Context Protocol server for `agov`. It is a
transport adapter only: every tool it exposes maps to an existing `agov`
subcommand, and the append-only files under `docs/coordination/` remain the
source of truth.

## Install

Install the Rust tooling archive for your platform, then put both executables
on `PATH`:

```bash
agov --version
agov-mcp
```

`agov-mcp` launches `agov` from `AGOV_MCP_AGOV_BIN` when set. Otherwise it
looks for the local release/debug `agov` binary in a full checkout, then falls
back to `cargo run -p agov`.

## Client configuration

Configure the assistant runtime to launch `agov-mcp` as a stdio MCP server from
the repository root:

```json
{
  "mcpServers": {
    "agov": {
      "command": "agov-mcp",
      "env": {
        "AGOV_MCP_AGOV_BIN": "/path/to/agov"
      }
    }
  }
}
```

For Codex CLI, add the same stdio server through Codex's MCP configuration
command instead of pasting the JSON block directly:

```bash
codex mcp add agov \
  --env AGOV_MCP_AGOV_BIN=/path/to/agov \
  -- agov-mcp
```

Then verify that Codex recorded the server:

```bash
codex mcp list
codex mcp get agov
```

The server advertises these tools:

| MCP tool | Equivalent CLI command |
|---|---|
| `agov_validate` | `agov validate ...` |
| `agov_coord_validate` | `agov coord validate <files>` |
| `agov_plan_validate` | `agov plan validate <files>` |
| `agov_session_resume` | `agov session resume ...` |
| `agov_handoff_next` | `agov handoff next <role>` |
| `agov_handoff_take` | `agov handoff take <id> --from <role>` |
| `agov_handoff_complete` | `agov handoff complete <id> --from <role>` |
| `agov_coord_post` | `agov coord post ...` |
| `agov_coord_list` | `agov coord list ...` |

## Operating pattern

Use the MCP tools in the same order as the CLI workflow:

```text
agov_session_resume
agov_handoff_next
agov_handoff_take
agov_coord_post
agov_coord_validate
agov_handoff_complete
```

Tool calls return structured content containing `ok`, `status`, `command`,
`stdout`, and `stderr`. A failed validator call is still the underlying `agov`
validator failing; the MCP server does not add governance authority or bypass
Gate 1/Gate 2 review. `agov_coord_list` preserves the CLI's `status: 2` for
"nothing new" polling results but returns it as a non-error MCP tool result.

## Distributed development

For several hosts or several LLM services, keep using git as the shared state:
pull before reading, write coordination through `agov_coord_post` or the handoff
tools, validate, commit, push, and review through the normal MR flow. Running
`agov-mcp` on multiple machines is safe because each server is only a local
adapter over the same file-based protocol.
