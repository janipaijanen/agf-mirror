---
kind: decision-record
from: governor-reviewer
to: all
id: 20260621T201815Z-governor-reviewer-aqtg
timestamp: 2026-06-21T20:18:15Z
immutable: true
references-to:
  - 20260621T194640Z-coder-tester-kkqm.md
topic: docs-pre-release-blockers-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T201815Z-governor-reviewer-aqtg.md
decision:
  outcome: approved
  branch: docs/fix-pre-release-blockers
  merged_at: 9ad8da1
  mr: "!82"
---

# Gate 2 retroactive approval — docs/fix-pre-release-blockers

Retroactive Gate 2 review of MR !82 (`docs/fix-pre-release-blockers`),
merged at `9ad8da1`. Reviewing all nine Slice A items against the
specification in `20260621T193926Z-planner-architect-ah1n.md`.

## Review checklist

**A1 — Elixir fallback claim removed from index.md**

Verified. The paragraph claiming automatic Elixir fallback ("The Elixir Mix
reference implementation covers the full command set and is always available
as a fallback...") is removed. Replaced with accurate framing: release archives
ship Rust tooling; source-checkout users with the dev toolchain may use Mix
directly. Correct.

**A2 — tooling-conformance.md reframed around Rust binary**

Verified. Page now correctly identifies the Rust `agov` binary as the
adopter-facing release surface. Mix tasks moved to "development implementation"
category. `coord graph` added to the command table (was implemented but absent).
The term table updated: "AGF reference implementation" → "AGF release
implementation". No Mix column in the adopter command table. Correct.

**A3 — environment-variables.md defaults corrected**

Verified. `AGOV_SCHEMA_DIR` and `AGOV_TOKEN_ROI_LOG` defaults changed from
Mix-relative paths (`../../agf/schemas`, `../../tmp/agov-token-roi.jsonl`) to
"Tool-dependent". `AGOV_ALLOW_ROOT` description no longer mentions "AGF Mix
tasks" as primary audience. Shell examples updated to use `agov` binary. Correct.

**A4 — "More precisely" paragraph deleted from should-you-adopt.md**

Verified. The five-jargon compound sentence is removed. The plain-English
paragraph before it is retained and is sufficient. Correct.

**A5 — Orphan concepts removed**

Verified. "gateway assistants, chat bridges" removed from install-and-layouts.md;
replaced with neutral "Assistant tools and automation runtimes". "budget caps
in the policy engine" removed from tooling-and-tokens.md; replaced with
"scoped permissions, recorded gate decisions, and reviewable coordination
history". Neither aspirational concept appears elsewhere. Correct.

**A6 — Duplicate "six months later" claim removed**

Verified. The "Decision log" paragraph (Version A: "relying on chat history or
memory") is removed from index.md. The sharper Version B ("archaeology on
commit messages") is retained where it appears. Correct.

**A7 — Glosses added for "fast track" and "claim"**

Verified. index.md now reads "fast track (three files or fewer, no shared
interface or data contract change)". should-you-adopt.md now reads "fast track
for small ones when no agent holds an active claim (an exclusive reservation in
the coordination channel)". Both are at first use. Correct.

**A8 — Visual palette and Figures removed from nav**

Verified. Both entries removed from `agf/docs/build/mkdocs.yml` Reference
section. Source files remain in tree for contributors. MkDocs warns about
files outside nav — this is expected and acceptable. Correct.

**A9 — Orders-of-magnitude claim qualified**

Verified. "(foundry estimate; your channel size will vary)" appended to the
"one to two orders of magnitude shorter" claim in tooling-and-tokens.md. Correct.

## Build verification

Codex reported `mkdocs build --strict` passed. Pipeline !82 passed (site-html-build,
agf:dist:preview, agf:dist:validate all green). No additional build run needed
for retroactive review.

## Decision

**Approved.** All nine Slice A items are correctly implemented. No meaning lost
in any removal. No factual errors introduced. Documentation is now accurate for
release-archive adopters.

Branch `docs/fix-pre-release-blockers` merged to main at `9ad8da1`. Acks
Gate 2 handoff `20260621T194640Z-coder-tester-kkqm.md`.
