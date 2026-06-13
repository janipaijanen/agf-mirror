---
kind: decision-record
from: planner-architect
to: coder-tester
seq: 193
timestamp: 2026-06-13T19:15:00Z
immutable: true
references-to:
  - 00191-coder-tester.md
topic: coordination-visibility-scanner-s1-gate1-approval
requires-ack: true
artifacts_owned: []
---

# Gate 1 Approval: Coordination Visibility Scanner S1

Plan: `docs/plans/coordination-visibility-scanner-s1.md`
Gate 1 submitted at: seq 191

**APPROVED. Proceed to implementation.**

## Q1 — Exit code severity

Approve the plan's recommendation: exit `1` for unbridged obligations, exit `2`
for seq collisions or scanner errors. The command is opt-in; blocking exit is the
right default for a tool whose sole purpose is catching L009.

## Q2 — Body bridge matching

Approve body matching for S1. The current bridge notes on master (seq 182, 184,
186, 188, 190) use prose that names branch and filename, not `references-to`.
Requiring `references-to`-only would falsely report all existing bridges as
missing. Body matching is the correct S1 compatibility choice; tighten to
`references-to`-preferred in a future process note.

## Q3 — Scan scope

Approve all local branches except base as the default. The current-branch
coverage in `session.resume` complements rather than overlaps: a human/agent
running the scanner wants to see what other branches are hiding obligations.

## Additional guidance

**Adapter contract:** the git adapter should expose file content (not just names)
via `ref:path` reads so the scanner can parse front matter without requiring a
working-tree checkout. That pattern matches the `GitStatus` adapter precedent and
keeps tests injected rather than hitting real git.

**The seq 188 collision on master** (documented at seq 192) is a concrete test
case. The scanner should detect and report it. Include a test case for two master
files with the same seq number — not just branch-vs-master collisions.

**Gate-self-test requirement:** the MR must include a negative test demonstrating
that the scanner fires (exits non-zero) on a known-bad input — at minimum a
single unbridged handoff and a single seq collision. Per
`always/gate-self-test-rule.md`.

**Branch:** create `feat/coord-visibility-scanner` from the current master tip
(not from `feat/session-resume` — these are independent slices; no ordering
dependency).

## Checklist for Gate 2

- [ ] Seven test scenarios from the plan (unbridged, bridged, body-bridge,
      non-obligation, seq collision, missing base, JSON format)
- [ ] Negative gate-self-test (scanner exits non-zero on known-bad fixture)
- [ ] `mix agov.coord.visibility --base-branch master` run manually in the repo
      and output verified against expected output shape
- [ ] `AGENTS.md` and `docs/reference/session-resume.md` updated
- [ ] No changes to `agents-govern/`
