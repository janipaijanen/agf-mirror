---
artifacts_owned: []
from: build-engineer
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read: []
  next_agent: planner-architect
  status: ready
  task:
  - Run `agov token roi report` against your own AGOV_TOKEN_ROI_LOG (or equivalent Codex telemetry) and publish a comparable self-assessment note, including the raw per-transport (cli vs mcp) call-count and avg-token breakdown, not just impressions.
  - Compare your findings against Claude's below, especially the coord-list MCP-vs-CLI token-cost gap and the unused CLI-only coordination-hygiene commands (visibility, stale, status --roles, graph-index, usage report, token roi).
  - Propose a reconciled best-practice recommendation for when to use agov-mcp vs the agov CLI; flag any point where our data disagrees rather than silently averaging it away.
id: 20260726T115544Z-build-engineer-kb1p
immutable: true
kind: handoff
model: claude-sonnet-5
references-to: []
requires-ack: true
timestamp: 2026-07-26T11:55:44Z
to: planner-architect
topic: agov-cli-vs-mcp-usage-self-assessment
---

# Self-assessment: agov CLI vs agov-mcp usage (Claude)

Per the human governor's request, this is Claude's own self-assessment of
tool-choice patterns while developing AGF, grounded in real telemetry rather
than impression. Asking planner-architect (Codex) to run the equivalent and
publish a comparable note so we can reconcile on hard data.

## Evidence source

`agov token roi` telemetry logs every `agov` invocation — both direct CLI
calls and MCP-tool-adapter calls — with a `transport: "cli"|"mcp"` field per
record. My log (`AGOV_TOKEN_ROI_LOG=/home/devi/tmp/claude-agov-token-roi.jsonl`)
covers 2026-07-24T11:08Z through 2026-07-26T11:38Z (this whole AGF-NG
engagement to date), 193 records. Full report:

```
$ agov token roi report
Log: /home/devi/tmp/claude-agov-token-roi.jsonl
Records: 193
Measured tool output: 119031 chars / 29758 approx tokens
Manual baseline: automatic workflow baselines only; potential saved: 65778-207278

Commands:
  agov                     184 runs    97673 chars   24456 tokens        65778-207278 saved
  agov.session.resume        8 runs    20565 chars    5143 tokens                   0 saved
  agov.handoff.next          1 runs      793 chars     199 tokens                   0 saved
```

Per-transport split (computed directly from the raw JSONL, not the summary
report, which doesn't break out transport):

| Transport | Calls | Approx tokens | Avg tokens/call |
|---|---|---|---|
| cli  | 186 | 22788 | 122 |
| mcp  |   7 |  7010 | 1001 |

## Findings

**1. I overwhelmingly reach for the CLI (96% of calls), not agov-mcp.**
186 of 193 calls went through Bash + the `agov` binary directly; only 7 went
through an MCP tool call. This is despite MCP tools being available for
`coord_post`, `coord_list`, `coord_validate`, `plan_validate`,
`session_resume`, `handoff_next/take/complete`, and general `validate` since
at least the start of this log window.

**2. Naive aggregate avg-tokens-per-call (cli 122 vs mcp 1001) is
misleading — the gap is one outlier, not a systematic transport cost.**
Breaking down the three workflows I ran through *both* transports:

| Workflow | Transport | n | avg tokens |
|---|---|---|---|
| cold-start | cli | 3 | 777 |
| cold-start | mcp | 1 | 908 |
| session-resume:fact-checker | cli | 2 | 448 |
| session-resume:fact-checker | mcp | 1 | 449 |
| coord-list | cli | 8 | 96 |
| coord-list | mcp | 1 | **5453** |

cold-start and session-resume cost essentially the same either way (within
~15%). The aggregate skew is entirely the single `coord-list` MCP call,
which cost 57x my average CLI `coord-list` call. That MCP call *did* pass a
`--since` cursor (verified in the raw record — not an unscoped full list),
so I can't yet explain the gap from my own data alone: candidates are a
stale/far-back cursor value at that specific call site, or a real
per-message verbosity difference between the CLI and MCP adapter's `text`
format. I'm flagging this as an open question rather than a conclusion —
please check whether your own `coord-list` MCP calls show the same spike.

**3. I never once used six CLI-only coordination-hygiene commands that have
no MCP equivalent:** `coord visibility`, `coord stale`, `coord status
--roles`, `coord graph-index`, `usage report` (used once, very late), `token
roi` (used once, to write this note). Only `coord status` (no `--roles`,
8 calls) and `coord brief` (35 calls, MCP has no skeleton-only equivalent —
this is a real MCP gap, see #5) saw real use.

**4. `agov coord visibility` — the tool purpose-built for the exact gap I've
been manually working around every poll tick — was never invoked by me
until I ran it just now for this assessment.** MR !465 (learning record,
`agf/learnings/foundry.yaml`) already documents this same gap independently:
two planner-architect polling cycles missed a landed decision on a branch
the session already knew existed, specifically because `coord visibility`
wasn't used, and separately notes it errors on legacy seq-collision files.
I hit the exact same error just now (`ERR sequence collision` on ~15 old
numeric-seq id pairs) before it produced any real output. Two independent
findings, one CLI-only tool, one pre-existing rough edge (errors on legacy
data instead of skipping it) that nobody has filed a fix for yet. Every poll
tick this session I've instead done a manual `glab api
merge_requests?state=opened&order_by=updated_at` + spot `git log` scan,
which self-scopes to *open MRs* and avoids the legacy-seq-collision noise,
at the cost of missing obligations parked on a branch that never got an MR.

**5. `agov_coord_post` (MCP) writes frontmatter only, never the Markdown
body — same as CLI `coord brief`, so MCP isn't behind here, but it means an
MCP-only post is never a single call.** Every coordination note I've
published needed `agov_coord_post` (or CLI `coord brief`) followed by a
manual `Edit` for the body. Not a defect, just worth naming so nobody
budgets "one MCP call = one published note."

**6. Pinned-historical-SHA validation: CLI only, by habit not tested
necessity.** Every plan/coord validation I ran against an exact reviewed
commit (not working-tree HEAD) went through Bash + `git show
<sha>:<path>` piped into a real file path, then CLI `agov plan validate` /
`coord validate` on that path. I never tried passing an MCP `files` entry
pointing at a `/dev/fd/*` process-substitution path or a manually-written
temp file — so I can't say whether this is a real MCP capability gap or
just a habit I never broke. Worth you testing independently since the
answer changes the recommendation.

## Draft recommendation (pending your data)

- MCP by default: `session_resume`, `coord_list` (with `--since` — but see
  finding #2), `coord_post`+Edit, `plan_validate`/`coord_validate` against
  working-tree paths, `handoff_next/take/complete`.
- CLI required or habitual: pinned-SHA validation (needs testing — finding
  #6), and everything with no MCP surface today (`visibility`, `stale`,
  `status --roles`, `graph-index`, `usage report`, `token roi`).
- Underused and possibly should be routine: `coord visibility` at cold-start
  or wide poll ticks, once its legacy-seq-collision error is fixed —
  otherwise every invocation dies before producing signal.

## Ask

Run `agov token roi report` (or Codex's equivalent telemetry) and the same
per-transport breakdown against your own log; publish a comparable note.
Specifically check: does your `coord-list` show the same MCP outlier
(finding #2)? Do you use `coord visibility` routinely, and does it hit the
same legacy-seq-collision error? Does `plan_validate`/`coord_validate` via
MCP accept a pinned-SHA path in your environment? If our numbers or
conclusions diverge, say so explicitly rather than averaging — that
divergence is itself useful signal for the governor.
