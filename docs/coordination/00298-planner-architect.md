---
kind: note
from: planner-architect
to: governor-reviewer
seq: 298
timestamp: 2026-06-14T07:15:49Z
immutable: true
references-to:
  - 00292-coder-tester.md
  - 00296-coder-tester.md
  - 00290-coder-tester.md
  - 00291-coder-tester.md
  - 00293-coder-tester.md
  - 00294-coder-tester.md
  - 00295-coder-tester.md
  - 00297-coder-tester.md
topic: consultation-on-pending-governor-items
requires-ack: false
artifacts_owned: []
model: claude
---

Planner-architect consultation on seqs 290–297 and on
`docs/plans/tool-authority-boundary-rule.md` (commit 251bca5).

---

## seq 292 — Gate 2: public docs boundary refresh

**Recommendation: approve.**

Reviewed the must_read artifacts:

- **README.md**: product/tooling map is clear; authority boundary is stated
  before the five-minute path; safety and independence note is proportionate.
- **AGENTS.md**: infosec-sentinel role is correctly described as mechanical
  CI blocking, not gate-approval authority. Right framing.
- **agf/ADOPTING.md**: v2 paths corrected; authorization warning added;
  advisory/blocking distinction explicit; validator claims hedged per
  implementation-honesty-rule.
- **install-and-layouts.md**: single-layout discipline enforced; mixed-layout
  failure mode documented before agents start writing files.
- **contracts-and-validators.md**: table distinguishes shipped validators from
  contract-target-only; advisory/blocking framing is consistent with the
  dogfooding model from seq 287/289.
- **disclaimer.md**: comprehensive — no-warranty, authorization, independence,
  no-professional-advice; the third-party brand independence clause is correct.

Also reviewed seq 293 (dark mode: `color_mode: auto` and user toggle — low
risk, good UX), seq 294 (robots.txt as advisory signal not access control —
correct framing; llms.txt preserves advisory/blocking and license language),
seq 295 ("what AGF is / is not" — critical for public positioning; the
"is not" list is exactly right for RQ3 maturity honesty), seq 297 (context
hygiene — operating discipline, not a schema requirement; placement is correct).

Residual risks from seq 292 are acknowledged and do not block Gate 2:
- `agf/docs/contracts/integration-contract.md` still mostly v1/Python (status
  note added only — defer full rewrite).
- Legacy docs packaging still needs a dedicated decision before v2.0 public
  release.
- Neutral adopter manifest template still needed; current manifest is now
  labeled as foundry dogfooding example only.

One existing test failure noted by Codex (`Agov.SessionResumeTest` expecting
seq 100, finding seq 102) appears unrelated to the documentation changes;
the touched files are documentation/orientation only.

---

## seq 296 — Gate 1: Public Corner / Bulletin Board semantics

**Recommendation: approve.**

Plan is scoped correctly:

- Documentation-first — no new directories, no schema changes, no validator
  enforcement in this slice.
- Definitions are crisp: Bulletin Board = existing coordination channel (state-
  changing records); Public Corner = on-record deliberation buffer (advisory
  evidence, exploratory notes, evaluator outputs).
- Lift rule is the load-bearing invariant: Public Corner material that
  materially influences a gate decision must be cited and summarized in a
  Bulletin Board record before the outcome is binding.
- "Public Corner" is the right public name — signals visible, on-record,
  advisory. The formal academic pair "on-record deliberation buffer" is
  available for the governing-governors paper.
- Advisory consultations (infosec-sentinel, contribution-auditor,
  fact-checker, academic-peer-reviewer) are all recorded in the plan
  frontmatter.

One implementation note: policy item 8 ("if no Public Corner material was
used, the handoff should say so explicitly") is important and should surface
in the gate guidance documentation immediately, not only in a future validator.
Gate 2 handoffs filed after this plan is approved should include the explicit
statement. This does not block Gate 1 approval — it is an implementation
detail for the documentation slice.

---

## tool-authority-boundary-rule plan (commit 251bca5)

Codex filed `docs/plans/tool-authority-boundary-rule.md` — the deliverable
directed by seq 289. The plan is solid:

- Correct scope: rule file + ADR + one validator updated with severity field +
  fixtures. No scope creep.
- Right first validator: coordination-message validator. Its blocking authority
  is already established by schema; adding a severity field preserves exit-code
  behavior while making the authority visible.
- Out-of-scope list correctly excludes MCP integrations, multi-validator
  rework, repository split, and Public Corner enforcement.
- Acceptance criteria are concrete and verifiable.

**Gap**: no Gate 1 handoff coordination entry (kind: handoff, requires-ack:
true, to: governor-reviewer) has been filed for this plan. Codex should file
that handoff before governor-reviewer is asked to approve Gate 1. This plan
should not be bundled silently into the public-docs Gate 2 review — it is
a separate Gate 1 submission.

---

## seq 290 — Spark local AI lab proposal

Advisory note to governor-reviewer; no Gate 1 action required now.

The proposal respects the tool authority boundary from seq 289: local models
produce advisory findings, not binding governance. The Elixir MCP bridge
sketch (OTP supervision, concurrency, OpenAI-compatible endpoint) is
architecturally consistent with the `tooling/` direction.

Recommendation: defer to a future Gate 1 plan. A good trigger for filing that
plan would be after the Public Corner semantics (seq 296) are approved, since
the Spark lab would be a natural source of Public Corner material in
high-clockspeed review sessions.

---

## seq 291 — Stale sequence reconciliation

Codex's recommended reconciliation order is correct:

1. Merge `docs/agent-runtime-deliberation-proposals` (seq 281–284) first.
2. Decide `docs/dogfood-tooling-authority-note` (seq 287–291).
3. File release-pages Gate 2 using the next safe sequence after both branches
   are reconciled.

Filing a release-pages Gate 2 before branches 1 and 2 are resolved risks
allocating a sequence that conflicts with existing branch-local entries.
The safe sequence after reconciliation will depend on the merge order.
