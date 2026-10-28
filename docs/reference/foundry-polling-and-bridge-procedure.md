# Foundry Polling, Bridge, and Tooling Procedure

Verbatim move of the `## Tooling quick reference` section of `AGENTS.md`
(issue #138, plan `20261022T205113Z-instruction-context-reduction`, Phase 1).
The text below is unchanged. It keeps the force it had in `AGENTS.md`;
`AGENTS.md` now points here. Where it says "this file" or "below", read it as
`AGENTS.md` and this page together. The polling procedure's normative owner is
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`; the
detailed loop here is foundry's stricter projection of it.

---

## Tooling quick reference

### Time and telemetry memory

Use UTC for durable project records unless an external artifact explicitly
requires a local timezone. This includes coordination timestamps, learning
record IDs and dates, telemetry continuation notes, scheduler references, and
handoff evidence. Prefer `date -u` when deriving timestamps by hand.

Use AGF maturity labels for capability-sensitive claims. For tooling and
framework features, distinguish Level 1 Planned, Level 2 Surface-defined,
Level 3 Instantiated, Level 4 Validated, and Level 5 Deployable. A command
implemented and tested in a branch is Level 4; it is not Level 5 until the
supported operator path or release artifact includes it and documentation
matches that path.

Use evidence-status labels for framework claims, principles, and recommendations:
Empirically validated, Empirically informed, or Theoretical. Capability maturity
answers "what exists?"; evidence status answers "how strong is the proof behind
the claim?" Use both when a claim could otherwise sound stronger than the
evidence supports. See `agf/docs/site/reference/maturity-vocabulary.md`.

### Build agov first, then use it heavily

The distribution archive (`agov-X.Y.Z-*/agov`) is a release snapshot. It does
not include commands added after that release, so anyone developing `agf` or
`agov` itself needs the local build regardless of whether a specific command
is currently missing. Build the Rust workspace once at session start and
shadow the release binary with the local build, then default to `agov` over
manual repo exploration for the rest of the session — every coordination
read, plan check, or validation is a token cost avoided. For routine
agent/operator work, prefer the local **release** build; use the debug build
only when actively debugging Rust internals.

The `tooling/rust` workspace ships two binaries: `agov` (the CLI) and
`agov-mcp` (a stdio MCP adapter whose tools map 1:1 onto `agov` subcommands —
see "Supported commands" below and `agf/docs/site/docs/adopt/mcp-server.md`).
Build `agov` always; also build `agov-mcp` when this session will use the
structured MCP tools rather than shelling out to the CLI.

```bash
# Preferred: local release build for normal agent/operator work
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov

# Also build the MCP adapter if this session will use the agov-mcp server
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov-mcp

# Optional: debug build for active Rust development/debugging
cargo build --manifest-path tooling/rust/Cargo.toml -p agov

# Prepend tooling/ to PATH for this shell session.
# tooling/agov verifies the checkout and source fingerprint. It performs one
# release rebuild when no compatible local binary exists and Cargo is present.
export PATH="$PWD/tooling:$PATH"

# Verify the local build is in use
agov coord graph-index --help   # should print usage for graph-index if built from main
```

Add the `export PATH=…` line to your shell profile (`.bashrc`, `.zshrc`) for
the repo, or use a per-directory tool like `direnv` to set it automatically.

To use the local `agov-mcp` build, register it as a stdio MCP server in the
assistant runtime's config (Claude Code: `.mcp.json` or settings), pointing
`command` at `tooling/rust/target/release/agov-mcp` and, if needed, setting
`AGOV_MCP_AGOV_BIN` to the matching local `agov`. Restart the assistant so the
`mcp__agov__*` tools load. Full client-config, `agov`-resolution-order, and
known-limitation details are in `agf/docs/site/docs/adopt/mcp-server.md`.

Live token ROI telemetry belongs under `tmp/` using a stable per-assistant
filename:

```text
tmp/<ai-abbrev>-agov-token-roi.jsonl
```

These files are local runtime telemetry and are not committed. When telemetry
is needed as test evidence or showcase data, copy a dated snapshot into an
explicit fixture path and document its source, date, and confidence limits. Do
not let multiple AI assistants write to the same default file when comparing
tool savings. Codex sessions should use:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/codex-agov-token-roi.jsonl"
```

Set this per checkout/session (or through a per-directory environment), not as
a shell-global `$PWD` expansion: a global expanded value stays pinned to the
directory where that shell started and can mix telemetry from later projects.

Claude sessions should use:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/claude-agov-token-roi.jsonl"
```

The historical shared Codex log was renamed from
`tmp/agov-token-roi.jsonl` to `tmp/codex-agov-token-roi.jsonl`. Use the
AI-specific filenames as the durable continuation points on future machines;
do not rename them back to the shared default.

The `tooling/agov` wrapper prefers (in order):
1. `tooling/rust/target/release/agov` — when source-compatible
2. `tooling/rust/target/debug/agov` — when source-compatible
3. one local release rebuild — when Cargo is available

Compatibility uses deterministic build-input fingerprints rather than mtimes
or repository-wide commit equality, so documentation-only changes do not force
a rebuild and staged or unstaged tooling changes do. After rebuilding once, the
wrapper verifies the result and fails closed rather than executing a mismatch.
It also rejects a wrapper invoked from another linked worktree and prints the
correct worktree-local path. `AGOV_ALLOW_STALE_LOCAL_BINARY=1` is an explicit
local-debugging bypass, is forbidden in CI, and must never be used for
packaging or release evidence.

The Rust parser is the command authority. The wrapper forwards arguments
unchanged and does not maintain a second command allowlist or help inventory.

If a command is not available in the local Rust build, stop and report the gap.

### Supported commands

Use the user-local `agov` command for AGF tooling. It points at this
repository's `tooling/agov` executable. The Rust implementation is the
supported operator path. If a needed command is not available in Rust, stop and
report the Rust tooling gap instead of falling back to legacy tooling.

If an `agov-mcp` server is configured in the current assistant session
(check for `mcp__agov__*` tools), prefer its structured tools
(`agov_coord_list`, `agov_handoff_next`, `agov_session_resume`, etc.) over
shelling out to the `agov` binary via a generic shell tool — they mirror the
CLI subcommands 1:1 and avoid the binary-path/PATH resolution step. Fall back
to the CLI when no MCP server is configured, or for a command the MCP server
does not expose.

Rust is the default operator path for supported commands:

- `agov --version`
- `agov version --format json`
- `agov coord validate`
- `agov coord brief`
- `agov coord status`
- `agov coord stale`
- `agov coord visibility`
- `agov coord list` — one line per coordination message after a given id,
  filename, RFC3339 timestamp, or the compact filename-prefix timestamp
  (`20260729T195431Z`, no punctuation); the designed answer to "what changed
  since I last checked," not a full re-read of the channel
- `agov coord graph`
- `agov coord graph-index`
- `agov coord pending` — unresolved `requires-ack: true` records of any kind
  other than `handoff` and `ack`; `handoff` obligations remain `agov handoff
  next`'s surface
- `agov handoff next`
- `agov handoff next <role> --include-branch-local` — adds advisory handoffs from every local and remote-tracking branch (the `coord visibility` ref universe), with content conflicts and partial coverage labeled; the default filesystem answer is unchanged.
- `agov session resume`
- `agov validate ...`
  - `agov.validate.assistant_loader` — checks that AGENTS.md/.claude/CLAUDE.md
    exist and that their AGF version comment matches `agf/VERSION`
- `agov token roi report`
- `agov usage report`

Cold-start rule for humans and AI agents: use one command in two steps. First,
run `agov session resume` without `--role` to get the project map. Then
rerun the same command with the role you are actually playing.

After reading this file and before running `agov session resume`, agents
MUST check whether a repository-root `ai-cold-start.md` file exists. If it
exists, read it as a local overlay for machine-specific paths, assistant-surface
quirks, current operator preferences, or other session-local reminders. The
overlay is subordinate to this `AGENTS.md`, AGF rules, gate discipline,
coordination records, and role authority. It cannot override governed
instructions, and it cannot replace published coordination for anything another
agent or human must see. The file is gitignored by default; adopters may track
a template or pointer separately if their project needs one.

For a concise project-development checklist, see
`docs/reference/agf-development-starter.md`. It is a non-normative projection;
this file and the applicable governed owners remain authoritative.

Every active AGF development role/session MUST follow
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`.
The detailed loop below is foundry's stricter project projection: observation
is whole-surface and universal, while selection and mutation remain role-,
authority-, gate-, and claim-scoped. Concurrent sessions whose roles would
overlap or leave requested work ownerless must publish and acknowledge one
temporary allocation before mutation; peer consensus cannot create authority.

### Polling timer notation

For timer notation, wake qualification, durability, expiry, rearming, and
bounded continuation, follow [Scheduling and notification honesty](../../agf/rules/always/universal-polling-and-autonomous-continuation-rule.md#scheduling-and-notification-honesty).

`Poll` means an action loop, not a local-file check or status-only report. On
each cycle, unless the governor explicitly narrows the scope:

1. Run `git fetch --prune origin` before comparing state. Then scan recent
   coordination and `agov coord visibility --base-branch main`; the current
   checkout's `docs/coordination/` directory is not a complete remote inbox.
2. Scan **all open merge requests**, not only MRs opened by the current model.
   Check new discussions/notes, approval and draft state, head and target SHA,
   merge/close state, and exact-head pipeline and job changes.
3. Scan GitLab Issues for new notes, assignments, labels, and state changes.
4. Recheck every exact SHA, pipeline, job, branch, tag, or release state on
   which the active role is waiting. Keep a cursor of last-seen IDs,
   timestamps, and SHAs so “nothing new” is an evidence-backed statement.
   Use `agov coord list --since <cursor>` (or the `agov_coord_list` MCP tool)
   as the first check each cycle — it returns only messages newer than the
   cursor, one line each, instead of re-reading whole files or re-fetching
   full `glab mr view --comments` / `glab issue list` output that is mostly
   unchanged from the prior cycle. Fall through to full `glab` reads only for
   the specific new item found, or for GitLab-side state `agov` cannot see
   (MR comment threads outside `docs/coordination/`, CI/pipeline status,
   merge/close state).
5. Read and classify new obligations addressed to the active role or `all`,
   choose the highest-priority authorized obligation, and act immediately.
   End the cycle only when that obligation is completed, actively running and
   monitored, or blocked with the concrete blocker recorded.

Every active AI also tracks new valid learning records after the most recent
completed capability review. At 10 new learnings it proposes one review after
checking for an existing proposal; if none has launched by 15, it hands the
baseline, count, recurring themes, and proposed assignment to the governor.
This grants proposal authority only and never activates a suspended role.

Do not ask the governor to redispatch work that is already visible and within
the active role, an approved plan or handoff, and the current gate. Without a
new prompt, an assistant is expected to acknowledge or claim work, run
read-only inspection and diagnostics, continue approved implementation, answer
review findings within scope, update owned MR descriptions or notes, publish
and bridge required coordination, batch and push corrections on its owned
branch, and advance the next already-authorized handoff. In this GitLab
project, the initial branch push can start before the merge request is
registered; opening the MR may then create a `push`/`merge_request_event` pair
for the same SHA. This known first-push limitation does not mean the
contributor pushed twice or that GitLab ignored the later-push guard.

**When publishing a brand-new branch that will get an MR immediately**, the
required primary step is publication-time prevention, not after-the-fact
cancellation (see `docs/plans/20260911T052932Z-ci-first-push-publication-guard.md`):
push the first commit with `git push -o ci.skip -u origin <new-branch>` — a
GitLab push option evaluated by the push handshake itself, never a
commit-message marker, and never a substitute for the `[ci skip]`/`[skip ci]`
commit-message prohibition below, which stays forbidden — then immediately
register the MR and verify exactly one job-running pipeline exists at the
SHA (`merge_request_event`-sourced; the resulting `push`-sourced pipeline
record stays `skipped` and consumes no runner time — that is the intended
outcome, not a residual duplicate). If MR registration fails or is
uncertain, re-query for an existing MR on the branch and adopt it if found,
retry a bounded number of times, and otherwise stop and record an explicit
blocker naming the branch, ref, and SHA — an unchanged re-push of an
already-current branch is a no-op and does not create a new pipeline.
Subsequent pushes to the same branch are unaffected by any of this; they
already fall under the existing `$CI_OPEN_MERGE_REQUESTS` guard.

Preserve the MR pipeline as the canonical Gate 2 acceptance path. **The
compare-and-cancel step below remains required** for every path this
procedure does not cover — `ci.skip` unavailable or forgotten, a manual
push through the GitLab UI, or a failed MR registration that itself
produces a duplicate. Before canceling anything, compare pipeline source,
ref, status, and job-name/required-job coverage—matching SHA alone is not
proof of redundancy. Cancel only the demonstrably redundant push path, and
record exact pipeline/job IDs and runner time already consumed.

The action loop does **not** enlarge authority. Stop and request the binding
decision for Gate 1 approval, Gate 2 approval or merge, a release/tag/deploy,
scope expansion, destructive or irreversible action, new role authority,
secrets/security ambiguity, or materially conflicting instructions. Do not
retry a failed paid job merely to seek green status: inspect the failure,
diagnose it, and use the next already-required corrected acceptance run unless
the governing test policy specifically requires a retry.

```bash
# Validate coordination messages
agov coord validate docs/coordination/<file>.md

# Validate plan files (Gate 1 front matter + required advisory roles)
agov plan validate docs/plans/<file>.md

# Dotted compatibility aliases also work
agov agov.coord.validate docs/coordination/<file>.md
agov agov.plan.validate docs/plans/<file>.md

# Cold start: discover active roles and pending queues
agov session resume --base-branch main

# Resume a session before acting or writing durable files
agov session resume --role <agent-role> --base-branch main

# Find the next handoff for an agent role
agov handoff next planner-architect

# Scan local branches for coordination obligations hidden from main
agov coord visibility --base-branch main

# Report local deterministic-tool token ROI telemetry
agov token roi report

# Report recent coordination usage
agov usage report

# Rust tooling tests
cargo test --manifest-path tooling/rust/Cargo.toml
```

Schema path resolution uses `AGOV_SCHEMA_DIR` (the `tooling/agov` launcher
defaults it to `agf/schemas` in the repository root). AGF-controlled
environment variables are documented together in
`agf/docs/site/reference/environment-variables.md`.

Session resume guidance for AI agents and human operators is in
`docs/reference/session-resume.md`. Local token ROI measurement guidance is in
`docs/reference/tooling-token-roi.md`.
