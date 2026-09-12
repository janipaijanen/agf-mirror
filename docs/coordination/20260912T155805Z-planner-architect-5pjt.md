---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260912T155805Z-planner-architect-5pjt
timestamp: 2026-09-12T15:58:05Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T151723Z-planner-architect-31pr
  - 20260912T152459Z-planner-architect-mjkr
  - 20260912T153453Z-coder-tester-p5mj
  - 20260912T154603Z-planner-architect-2h7r
requires-ack: true
topic: v2101-ruw-gate1-request
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 1 for docs/plans/20260912T155648Z-v2101-ruw-generic-artifact-validation.md.
    - Confirm the v2.10.1 version-number choice (PATCH for MINOR-shaped content) per the plan's honesty note, or direct a v2.11.0/other number instead.
  must_read:
    - docs/plans/20260912T155648Z-v2101-ruw-generic-artifact-validation.md
  do_not_touch:
    - resuming v2.11/RUW implementation work before a separate explicit decision
---

# Gate 1 request: redesignate RUW to v2.10.1, scope = low-customer-coupling artifact validation

Per Jani's direct instruction: "plan 2.10.1 with low-customer-coupling
pieces and then implement with codex. hold the RUW (2.11) work, and
make 2.10.1 RUW." Claude transcribes the instruction; authority is the
human's.

The plan requests exactly what was asked: v2.10.1 becomes the active
release-under-work, carrying only the pieces of the labdns distillation
(A1 + A3 + A4's structural subset) that Codex and I both assessed as
genuinely low-customer-coupling — generic capability useful on its own
merits, not gated on the customer's pace or requiring AGF to track it.
A2, A4's domain/governance-authority subset, A5, A6, and A7 are
explicitly deferred, not silently dropped.

v2.11/RUW's existing work stays exactly as the `31pr` pause left it —
no new commits, claims preserved — pending a separate future decision
to resume it. This request does not touch that work.

Once Gate 1 is decided, coder-tester (Codex) is the intended
implementer per Jani's instruction; I retain planning and build-review.
Requesting your Gate 1 decision, plus explicit confirmation or
correction of the v2.10.1 vs. v2.11.0 version-number choice named in
the plan's "Why v2.10.1" section — I flagged the SemVer tension rather
than resolving it silently.
