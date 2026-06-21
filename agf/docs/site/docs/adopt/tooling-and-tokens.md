# Tooling and token savings

*Status: Provisional (v2.0 documentation). Audience: developers and
administrators who have the framework installed and want to reduce AI token
consumption by adding deterministic tooling to the inner agent loop.*

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

The `agov` command is the adopter-facing CLI for AGF validators and coordination
helpers. It has two implementations that coexist:

- **Rust binary** (`tooling/rust/`) — the preferred path for supported commands.
  Static binary (~5–12 MB), no runtime dependency. Phase 1 commands are in
  production use on the AGF foundry project itself.
- **Elixir Mix reference implementation** (`tooling/elixir/`) — the full command
  set, including commands not yet ported to Rust. The `agov` wrapper falls
  through to Mix automatically when the Rust binary is absent or the command
  is not yet supported in Rust.

You do not choose between them. The wrapper chooses for you based on what is
available.

## Current command maturity

Maturity levels follow the
[implementation honesty model](../concepts/evidence-and-learnings.md):
Level 5 is deployable and in active use; Level 1 is planned.

### Level 5 — Deployable (Rust, in production use)

| Command | What it replaces |
|---|---|
| `agov coord validate <file>...` | Agent reads a coordination file and judges schema compliance. Validation is deterministic; agents should not improvise it. |
| `agov plan validate <file>...` | Agent reads a Gate 1 plan file and checks front-matter. |
| `agov validate` | Agent reads adoption, agent definitions, and learning records to spot structural problems across the governance tree. |
| `agov coord brief [--id-format readable\|ulid] [--from <role>] [--kind <kind>]` | Agent guesses a coordination ID, makes up a filename, or asks a human for the next ID. The command generates a valid, non-colliding ID with the correct filename format. |
| `agov handoff next <role>` | Agent reads handoff messages, mentally subtracts resolved items, and selects the pending one for its role. The command returns the relevant handoff in one invocation. |

### Level 1 — Planned (Rust, Phase 2A)

The following commands exist in the Elixir reference implementation and are in
operational use today via Mix. A Gate 1 plan (`docs/plans/rust-agov-phase2a.md`)
has been submitted to port them to Rust. When that plan is implemented, these
will reach Level 5 via parity testing.

| Command | What it replaces |
|---|---|
| `agov session resume [--role <role>]` | Agent reads recent coordination files, the manifest, branch status, and claims to reconstruct its starting state for the session. This is potentially the highest-token-cost activity in the agent loop because it happens at every session boundary. |
| `agov coord status [--recent N]` | Agent counts pending handoffs, reads claim messages, and scans the channel for channel health information. |
| `agov coord stale` | Agent identifies expired artifact claims by reading all claim messages and comparing timestamps. |

Until the Rust port lands, these three commands are available via Mix for
projects that have Elixir installed. The `agov` wrapper routes to Mix when the
Rust binary does not support the command.

## How to set it up

### Option A — Use the `agov` wrapper (recommended)

If you have a full checkout, the `tooling/agov` wrapper handles routing. Add it
to your PATH:

```bash
ln -sfn "$PWD/tooling/agov" ~/.local/bin/agov
```

The wrapper prefers the Rust binary when available. If the Rust binary is not
built, it falls through to the compiled Elixir escript, then to Mix. Build the
Rust binary once per machine:

```bash
cd tooling/rust && cargo build -p agov
```

After that, `agov coord validate`, `agov coord brief`, `agov handoff next`, and
the other supported commands use Rust automatically.

### Option B — Framework-only release archive

A release archive containing only `agf/` has schemas and templates but not
the tooling executable. In that case, download the optional packaged `agov`
tooling archive from the release page, or use the wrapper with Mix as the
backend.

### Option C — CI integration

```yaml
validate-governance:
  image: hexpm/elixir:<tag>
  script:
    - ln -sfn "$PWD/tooling/agov" /usr/local/bin/agov
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/coordination/*.md' |
        xargs -0r agov coord validate
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/plans/*.md' |
        xargs -0r agov plan validate
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

See [Contracts and validators](../advanced/contracts-and-validators.md) for the
full CI wiring guide, including how to declare checks as blocking vs advisory.

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
develops the framework itself) runs the Rust tooling in every agent polling
loop. The `agov coord validate` and `agov coord brief` commands each run in
under 100 milliseconds and produce output that is one to two orders of magnitude
shorter than reading the underlying coordination files with an AI agent.

The local telemetry proof-of-concept (`mix agov.token.roi`) collects saved-token
estimates in a local JSONL file (`tmp/agov-token-roi.jsonl`) and can report
them on request. Rust Phase 1 commands also append compatible records to the
same log. To print accumulated records:

```bash
cd tooling/elixir && mix agov.token.roi report
```

To disable telemetry:

```bash
export AGOV_TOKEN_ROI=0
```

See the [environment variable reference](../../reference/environment-variables.md)
for the full list of telemetry controls.

## Why Rust

The Elixir tooling bundles a 50–70 MB ERTS runtime. The Rust binary is a
static 5–12 MB executable with no runtime dependency — download and run,
no Elixir install required. For adopters who do not use Elixir and want to
integrate the validators into their CI without pulling in a language runtime,
the Rust binary is the lower-friction path.

The Elixir Mix implementation remains the authoritative reference and is not
going away. Rust Phase 1 passes a byte-for-byte parity test against the Elixir
output for all supported commands; that parity check is a blocking CI gate.
Both run side-by-side; the wrapper picks the best available implementation.

## What is not in scope

- Removing the Elixir tooling (a separate future decision)
- Registry publishing or code signing (a separate future decision)
- Windows x64 distribution (Phase 3, not yet planned)
- Token ROI reporting in Rust (Phase 2B)

## See also

- [Contracts and validators](../advanced/contracts-and-validators.md) — wiring
  `agov` checks into CI as blocking or advisory gates.
- [Tooling conformance](../../reference/tooling-conformance.md) — the
  command-surface contract that any AGF-compatible implementation must pass.
- [Environment variables](../../reference/environment-variables.md) — telemetry
  controls and path overrides.
- [Your first governed change](../start/first-governed-change.md) — where to
  start before worrying about token savings.
