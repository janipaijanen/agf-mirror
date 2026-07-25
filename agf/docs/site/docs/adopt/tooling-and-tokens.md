<!-- audience: A (ai-operator); answers: "which docs are meant for AI assistants to execute while humans supervise" -->
# Tooling and token savings

*Status: Released (v2.1 documentation). Audience: AI assistants adding
deterministic tooling to the inner agent loop, with developers and
administrators supervising setup and reviewing the evidence.*

## The premise

AI agents use context tokens every time they read governance files to
reconstruct coordination state: which handoffs are pending, which artifacts are
claimed, whether a coordination message is schema-valid. When those questions
are answered by deterministic tooling instead, the agent reads a compact summary
rather than scanning the full file set. Those tokens are then available for
judgment, design, and review — the work that actually benefits from AI
reasoning.

This page documents the AGF tooling, what it replaces, how to set it up, and
what the evidence base for the savings looks like.

## What the tooling does

The optional `agov` command is the adopter-facing CLI for AGF validators and
coordination helpers. It is a static binary (~5–12 MB) with no runtime
dependency. Download the archive for your platform, unpack it, and run `agov`.

## Current command maturity

Maturity levels follow the
[maturity vocabulary](../../reference/maturity-vocabulary.md): Level 5 is
deployable and in active use; Level 1 is planned.

### Level 5 — Deployable (in production use)

This is the canonical command inventory. Each command below is implemented,
tested, documented, and present in the supported v2.4.2 operator path. A future
command that exists only in a branch or design must be listed separately at its
actual maturity level rather than added to this table by analogy.

| Command | What it replaces |
|---|---|
| `agov --version` / `agov version --format json` | Inferring build provenance from file paths, timestamps, or `git log`. |
| `agov coord validate <file>...` | Agent reads a coordination file and judges schema compliance. Validation is deterministic; agents should not improvise it. |
| `agov plan validate <file>...` | Agent reads a Gate 1 plan file and checks front-matter. |
| `agov validate` | Agent reads adoption, agent definitions, learning records, adopter-docs parity, and copied-assistant drift warnings to spot structural problems across the framework tree. It does not validate every plan or coordination file. |
| `agov validate adopter-docs` | Agent or release reviewer checks whether adopter-relevant framework/tooling changes since the latest reachable `v*` tag include downstream-facing docs/templates or a visible changelog waiver. |
| `agov coord brief [--id-format readable\|ulid] [--from <role>] [--kind <kind>]` | Agent guesses a coordination ID, makes up a filename, or asks a human for the next ID. The command generates a valid, non-colliding ID with the correct filename format. |
| `agov handoff next <role>` | Agent reads handoff messages, mentally subtracts resolved items, and selects the pending one for its role. The command returns the relevant handoff in one invocation. |
| `agov coord status [--recent N] [--roles]` | Agent counts pending handoffs, reads claim messages, and scans the channel for channel health. The `--roles` flag adds a per-role activity summary for session orientation. |
| `agov session resume [--role <role>]` | Agent reads recent coordination files, the manifest, branch status, and claims to reconstruct its starting state for the session. This is potentially the highest-token-cost activity in the agent loop because it happens at every session boundary. |
| `agov coord stale` | Agent identifies expired artifact claims by reading all claim messages and comparing timestamps. |
| `agov coord visibility` | Agent compares branches for coordination obligations that exist off the base branch. |
| `agov coord graph` | Agent reads coordination files to reconstruct which roles communicated with which, then manually renders a flow summary. The command produces a Graphviz dot map in one invocation. |
| `agov coord graph-index` | Agent scans all coordination files to produce a structured JSON index of handoffs, references, and pending obligations — replaces ad-hoc mental reconstruction. |
| `agov token roi report` | Agent summarizes local deterministic-tool telemetry instead of estimating savings by hand. |
| `agov usage report [--last <n>]` | Agent counts message kinds, active senders, inactive active-status agents, and unknown senders. |

For adoption work, `agov usage report --last 50` is the quickest answer to
"which roles have actually been active recently?"

## How to set it up

### Option A — Full checkout (recommended)

If you have a full checkout, build the binary once per machine. Prefer the
release build for normal development, agent, and operator work; use the debug
build only when actively debugging Rust internals.

```bash
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov
```

Then add it to your PATH:

```bash
ln -sfn "$PWD/tooling/rust/target/release/agov" ~/.local/bin/agov
```

### Option B — Release archive

Download the `agov` archive for your platform from the
[project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases),
unpack it, and place the `agov` executable on PATH.

### Option C — CI integration

```yaml
validate-governance:
  image:
    name: registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.4.2
    entrypoint: [""]
  script:
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/coordination/*.md' |
        xargs -0r agov coord validate
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/plans/*.md' |
        xargs -0r agov plan validate
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

Pin the image to an exact version tag or digest. The release archive remains a
supported alternative when your CI environment does not allow project registry
images.

See [Contracts and validators](../advanced/contracts-and-validators.md) for the
full CI wiring guide, including how to declare checks as blocking vs advisory.

`agov` is optional tooling. If you do not install it, keep the same governance
obligations: manually review changed plan and coordination front matter,
confirm assistant loaders point at `agf/`, and record the manual checks in the
MR. When `agov validate` is available, treat its output as framework coverage;
for complete governance coverage also run `agov plan validate` and
`agov coord validate` on the relevant files.

## Why meter at the governance gate

"You cannot optimise what you do not meter, and you cannot meter what flows
through no chokepoint."

Every agent invocation passes through one or more AGF gates: coordination
validation, handoff routing, session resume. Capturing token consumption at
those chokepoints produces attribution data that API-level billing cannot: cost
per role, per coordination message kind, per gate pass. API billing shows total
spend; gate-level metering shows which governance activity caused it.

This matters for efficiency decisions. "AI is expensive" is not actionable.
"Session-resume accounts for 40% of coordination overhead and runs 12 times
per day" is. The `agov token roi report` command reads the local JSONL telemetry
and surfaces savings per command, giving teams the visibility to reduce
coordination overhead without reducing coordination discipline.

Governance also enables greater agent autonomy safely: scoped permissions,
recorded gate decisions, and reviewable coordination history make the
consequences of missteps bounded and visible.

## Token savings: the evidence model

AGF does not claim exact token counts — token consumption varies by model,
context window state, and whether the agent needs to read the full file or a
summary. The honest model is:

```
estimated_tokens_saved =
  tokens(manual_context_that_would_have_been_read)
  - tokens(tool_output_that_was_read)
  - tokens(extra_prompt_needed_to_invoke_the_tool)
```

Where estimates are needed for tooling planning, use:

```
approx_tokens = ceil(character_count / 4)
```

**What the foundry project has measured:** The AGF foundry project (which
develops the framework itself) runs the `agov` tooling in every agent polling
loop. The `agov coord validate` and `agov coord brief` commands each run in
under 100 milliseconds and produce output that is one to two orders of magnitude
shorter than reading the underlying coordination files with an AI agent (foundry
estimate; your channel size will vary).

The telemetry system collects saved-token estimates in a local JSONL file. Live
token ROI telemetry belongs under `tmp/` using a stable per-assistant filename:

```text
tmp/<ai-abbrev>-agov-token-roi.jsonl
```

These files are local runtime telemetry and are not committed. When telemetry
is needed as test evidence or showcase data, copy a dated snapshot into an
explicit fixture path and document its source, date, and confidence limits.
Teams running multiple assistants in the same checkout should split the logs by
agent so later analysis does not mix streams accidentally:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/codex-agov-token-roi.jsonl"
# or, for a Claude-run shell:
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/claude-agov-token-roi.jsonl"
```

To print accumulated records:

```bash
agov token roi report --log tmp/codex-agov-token-roi.jsonl
```

To disable telemetry:

```bash
export AGOV_TOKEN_ROI=0
```

See the [environment variable reference](../../reference/environment-variables.md)
for the full list of telemetry controls.

## What is not in scope

- Registry publishing or code signing (a separate future decision)
- Windows x64 packaged release (preview CI artifact exists; downloadable
  release not yet available)

## See also

- [Contracts and validators](../advanced/contracts-and-validators.md) — wiring
  `agov` checks into CI as blocking or advisory gates.
- [Tooling conformance](../../reference/tooling-conformance.md) — the
  command-surface contract that any AGF-compatible implementation must pass.
- [Environment variables](../../reference/environment-variables.md) — telemetry
  controls and path overrides.
- [Your first governed change](../start/first-governed-change.md) — where to
  start before worrying about token savings.
