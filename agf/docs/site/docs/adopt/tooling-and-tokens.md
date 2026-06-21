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
helpers. It is a static binary (~5–12 MB) with no runtime dependency. Download
the archive for your platform, unpack it, and run `agov`.

## Current command maturity

Maturity levels follow the
[implementation honesty model](../concepts/evidence-and-learnings.md):
Level 5 is deployable and in active use; Level 1 is planned.

### Level 5 — Deployable (in production use)

| Command | What it replaces |
|---|---|
| `agov coord validate <file>...` | Agent reads a coordination file and judges schema compliance. Validation is deterministic; agents should not improvise it. |
| `agov plan validate <file>...` | Agent reads a Gate 1 plan file and checks front-matter. |
| `agov validate` | Agent reads adoption, agent definitions, and learning records to spot structural problems across the governance tree. |
| `agov coord brief [--id-format readable\|ulid] [--from <role>] [--kind <kind>]` | Agent guesses a coordination ID, makes up a filename, or asks a human for the next ID. The command generates a valid, non-colliding ID with the correct filename format. |
| `agov handoff next <role>` | Agent reads handoff messages, mentally subtracts resolved items, and selects the pending one for its role. The command returns the relevant handoff in one invocation. |
| `agov coord status [--recent N] [--roles]` | Agent counts pending handoffs, reads claim messages, and scans the channel for channel health. The `--roles` flag adds a per-role activity summary for session orientation. |
| `agov session resume [--role <role>]` | Agent reads recent coordination files, the manifest, branch status, and claims to reconstruct its starting state for the session. This is potentially the highest-token-cost activity in the agent loop because it happens at every session boundary. |
| `agov coord stale` | Agent identifies expired artifact claims by reading all claim messages and comparing timestamps. |
| `agov coord visibility` | Agent compares branches for coordination obligations that exist off the base branch. |
| `agov token roi report` | Agent summarizes local deterministic-tool telemetry instead of estimating savings by hand. |
| `agov usage report [--last <n>]` | Agent counts message kinds, active senders, inactive active-status agents, and unknown senders. |

For adoption work, `agov usage report --last 50` is the quickest answer to
"which roles have actually been active recently?"

## How to set it up

### Option A — Full checkout (recommended)

If you have a full checkout, build the binary once per machine:

```bash
cd tooling/rust && cargo build -p agov
```

Then add it to your PATH:

```bash
ln -sfn "$PWD/tooling/rust/target/debug/agov" ~/.local/bin/agov
```

### Option B — Release archive

Download the `agov` archive for your platform from the
[project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases),
unpack it, and place the `agov` executable on PATH.

### Option C — CI integration

```yaml
validate-governance:
  image: debian:stable-slim
  script:
    - tar -xzf agov-<version>-x86_64-unknown-linux-musl.tar.gz
    - install -m 0755 agov-<version>-x86_64-unknown-linux-musl/agov /usr/local/bin/agov
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/coordination/*.md' |
        xargs -0r agov coord validate
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/plans/*.md' |
        xargs -0r agov plan validate
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

See [Contracts and validators](../advanced/contracts-and-validators.md) for the
full CI wiring guide, including how to declare checks as blocking vs advisory.

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

Governance also enables greater agent autonomy safely: a controlled blast radius
(budget caps in the policy engine, audit log for every gate pass) means agents
can be trusted with more latitude because the consequences of missteps are
bounded and visible.

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
shorter than reading the underlying coordination files with an AI agent.

The telemetry system collects saved-token estimates in a local JSONL file
(`tmp/agov-token-roi.jsonl`). To print accumulated records:

```bash
agov token roi report
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
