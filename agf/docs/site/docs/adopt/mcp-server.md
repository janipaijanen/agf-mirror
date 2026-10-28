<!-- audience: A (ai-operator); answers: "which docs are meant for AI assistants to execute while humans supervise" -->
# agov MCP server

*Status: Level 4 Validated — implemented, tested, and built into the
release archive by CI, but capped below Level 5 Deployable by the known
path-contract limitation documented below. Audience: AI assistants and
adopters wiring an MCP-capable assistant to AGF tooling without relying on
shell-command prompting alone.*

`agov-mcp` is a local stdio Model Context Protocol server for `agov`. It is a
transport adapter only: every tool it exposes maps to an existing `agov`
subcommand, and the append-only files under `docs/coordination/` remain the
source of truth.

## Known limitation

Several tools accept a `dir` argument that is forwarded to the CLI's
`--dir` flag, meaning "coordination-message directory" (default
`docs/coordination`) — **not** the repository root. Passing the repository
root currently returns a plausible-looking `ok: true` result instead of an
error, because non-coordination files are silently treated as invalid and
skipped rather than rejected. A fix (renaming the field to
`coordination_dir`, keeping `dir` as a deprecated alias, and adding an
explicit warning for this misuse pattern) is planned in
[`docs/plans/mcp-coordination-dir-contract-fix.md`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/plans/mcp-coordination-dir-contract-fix.md),
pending Gate 1 approval. Until that lands and ships in a release, treat any
`dir` argument you pass explicitly with care — omitting it and relying on
the `docs/coordination` default is the safest usage today.

## Install

Install the Rust tooling archive for your platform, then put both executables
on `PATH`:

```bash
agov --version
agov-mcp
```

At startup, `agov-mcp` resolves `agov` once in this order:

1. the explicit `AGOV_MCP_AGOV_BIN` override;
2. the local release, then debug, `agov` binary in a full checkout;
3. `agov` on `PATH` (the normal tooling-archive installation); and
4. `cargo run -p agov` when Cargo is on `PATH`.

Before accepting MCP requests, the server runs `agov --version` through the
resolved command. If resolution or validation fails, `agov-mcp` exits non-zero
with a diagnostic instead of registering a server whose tool calls cannot run.
The resolved command remains bound for the lifetime of that server process;
restart `agov-mcp` after changing the installation or override.

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

Platform tooling archives include this configuration as
`examples/claude/.mcp.json`. After putting both `agov` and `agov-mcp` on `PATH`, copy the example to the project root as
`.mcp.json`, review it, and approve the project-scoped `agov` server when
Claude Code prompts. The shipped example intentionally contains no absolute
paths, credentials, or machine-specific environment values.

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
| `agov_plan_collision_check` | `agov plan collision-check ...` |
| `agov_session_resume` | `agov session resume ...` |
| `agov_handoff_next` | `agov handoff next <role>` |
| `agov_handoff_take` | `agov handoff take <id> --from <role>` |
| `agov_handoff_complete` | `agov handoff complete <id> --from <role>` |
| `agov_coord_post` | `agov coord post ...` |
| `agov_coord_list` | `agov coord list ...` |
| `agov_coord_pending` | `agov coord pending ...` |
| `agov_coord_convergence` | `agov coord convergence ...` |
| `agov_coord_status` | `agov coord status ...` |
| `agov_coord_visibility` | `agov coord visibility ...` |
| `agov_coord_stale` | `agov coord stale ...` |
| `agov_coord_graph_index` | `agov coord graph-index` (stdout only; no `--output`) |
| `agov_artifact_validate` | `agov artifact validate` (complete registry; optional selected member paths) |
| `agov_artifact_index` | `agov artifact index` (stdout only) |
| `agov_coord_publication_check` | `agov coord publication-check ...` |
| `agov_coord_brief` | `agov coord brief ...` |

Not exposed over MCP: `agov coord graph` (Graphviz/dot output stays
CLI-only — `agov_coord_graph_index` is the compact machine/LLM-oriented
result for the same underlying graph). `agov_coord_graph_index` never
accepts the CLI's `--output` flag; it always returns the JSON body as the
tool result, never writes to a caller-chosen filesystem path.

## Operating pattern

Use the MCP tools in the same order as the CLI workflow:

```text
agov_session_resume
agov_coord_pending
agov_handoff_next
agov_handoff_take
agov_coord_post
agov_coord_validate
agov_handoff_complete
```

`agov_coord_pending` reports unresolved non-`handoff` `requires-ack: true`
records (claim, review, note, decision-record, fast-track-waiver,
lesson-proposal). Run it every cold start alongside `agov_session_resume` —
`session resume` does not surface this obligation class, and `handoff`
tooling covers only `handoff`-kind records. The tool only reports; a later
coordination record's direct `references-to` is what resolves an entry, not
the tool call itself.

Tool calls return structured content containing `ok`, `status`, `command`,
`stdout`, and `stderr`. A failed validator call is still the underlying `agov`
validator failing; the MCP server does not add governance authority or bypass
Gate 1/Gate 2 review. `agov_coord_list` and `agov_coord_pending` both preserve
the CLI's `status: 2` for "nothing new"/"nothing pending" polling results but
return it as a non-error MCP tool result.

Every tool schema is closed (`additionalProperties: false`), and the server
enforces the same property set before the child `agov` process starts: a call
that carries a key the tool does not publish fails with a JSON-RPC error naming
the tool and the key, instead of running with the key silently dropped. Clients
that enforce `additionalProperties: false` themselves reject the request before
it reaches the server. `include_forge` on `agov_plan_collision_check` and
`output` on `agov_coord_graph_index` keep their own explanatory errors. If a
client used to send extra keys, compare them with the `inputSchema.properties`
from `tools/list` and remove the extras.

`agov_coord_list` accepts an optional `ref` string, forwarded as the single
argument `--ref=<value>` (a value that starts with `-` stays a value). It reads
the coordination channel from that already-fetched commit-ish, so the
fetch-first poll is not answered from a stale checkout; the server never
fetches. With `ref`, `format: json` returns an object (`source_ref`,
`source_oid`, `coordination_dir`, `skipped`, `messages`); without it the result
is still an array, and an empty result is `[]`. A ref the tool cannot read
returns `status: 2` with the reason on `stderr` and is marked as an error, not
as the "nothing new" polling result. Clients that enforce a strict schema must
branch on whether they sent `ref` when they parse the JSON.

`agov_coord_pending` tracks unresolved `requires-ack: true` records of any
kind except `ack` and `handoff` — `handoff` obligations remain
`agov_handoff_next`'s surface, unchanged. A record is pending until a later
record's `references-to` names it directly, not transitively. Records
published before the feature's introduction cutoff are grandfathered and
never appear.

## Distributed development

For several hosts or several LLM services, keep using git as the shared state:
pull before reading, write coordination through `agov_coord_post` or the handoff
tools, validate, commit, push, and review through the normal MR flow. Running
`agov-mcp` on multiple machines is safe because each server is only a local
adapter over the same file-based protocol.

Generic artifact tools are documented in the [artifact registry reference](../../reference/artifact-registry.md).

### Publication-hold evidence

`agov_coord_hold_status` mirrors `agov coord hold-status`: required
`target_branch` string; optional `candidate_ref` string array, `dir` string
(default docs/coordination), and `format` text|json (default text). Repeated
candidates pin explicit commit refs; omission means HEAD. It reads local Git
only, with no implicit network or forge mode.

The CLI JSON v1 envelope includes explicit-sources coverage, pinned identities,
holds and end provenance, candidate action rows and stable problem IDs. The
adapter preserves stdout, exit status (0 complete, 1 semantic, 2 incomplete)
and isError (true for nonzero). Text and JSON retain fixed false publication
and authority verification and all six required_external_checks: publication,
governor_authority, record_urgency_and_obligations, publication_target,
current_target_and_expiry, release_hold_authorization. Review-required never
authorizes delay. The fields and tooling shipped in v2.11.0; install compatible
framework schemas, rules, and tooling artifacts together before first use.
