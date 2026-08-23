---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T202348Z-planner-architect-p45k
timestamp: 2026-08-23T20:23:48Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260823T195011Z-contribution-auditor-psbj.md
topic: mr1847-round1-fixed-request-fresh-audit
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - >-
      Fresh COMPLETE independent audit of MR !1847 at its new exact
      head, following the same isolated-session sourcing pattern
      established across the MR !1817 review chain (a session distinct
      from every session that authored or previously audited this
      plan, including the session that filed issue #115 and the
      session that authored `psbj`).
  must_read:
    - docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md
    - 20260823T195011Z-contribution-auditor-psbj.md
  do_not_touch: []
---

# MR !1847 round 1: all 7 findings fixed, fresh audit requested

`psbj`'s COMPLETE audit (isolated Codex session) found 7 blocking
findings against the exact head reviewed. All 7 verified independently
against the actual repository state (not accepted at face value —
including re-reading the current, not cached, text of
`downstream-impact-rule.md`'s "Adopter docs parity" section, which
turned out to have grown since this plan was first drafted) and fixed
on commit `740a5ad0` (branch `plan/net-new-projection-path-lifecycle`,
pushed):

- **`CA-NNP-R1`** (scope overreach) — fixed by introducing a dedicated
  `normativeOwnerEntry` schema definition used only by
  `surfaces.normative_owner`; every other row keeps the plain,
  unmodified `surfaceEntry`. The validator's flag-gated relaxation is
  narrowed to match.
- **`CA-NNP-R2`** (deferred contract decisions) — both decided: whole-
  `surfaceEntry` granularity (final, not merely recommended), and a
  concrete `.gitlab-ci.yml` correlation mechanism keyed off each
  changed manifest's `plan_id` field and that plan's own
  `status: pending-gate1` marker.
- **`CA-NNP-R3`** (stale plan-stage evidence accepted at Gate 2) —
  fixed: a new check, unconditional on `--allow-planned`, rejects
  `stage: planned` paired with a path that already resolves. The
  earlier "harmless" framing is retracted; a dedicated negative test is
  specified.
- **`CA-NNP-R4`** (adopter documentation parity omitted) — fixed:
  `Adopter docs parity: updated`, citing the rule-text subsection
  itself (adopter-shipped via `agf/rules/**`) as the disposition, per
  `downstream-impact-rule.md`'s exact three-outcome contract.
- **`CA-NNP-R5`** (missing gate-self-test artifact requirement) —
  fixed: added an explicit Gate 2 requirement for a reproducible
  negative CLI demonstration (exact command, expected/actual
  output/exit code, pipeline log reference).
- **`CA-NNP-R6`** (unfilled MR description) — fixed: replaced with a
  description conforming to `merge-request-description-rule.md`'s six
  required headings, synced to GitLab.
- **`CA-NNP-R7`** (overstated compatibility/rollback claims) — fixed:
  the compatibility analysis now states the exact bounded collision
  risk (only `normative_owner.stage` set to a non-enum value on an
  existing manifest could newly fail), and rollback now gives
  lifecycle-aware steps for a manifest already depending on
  `--allow-planned` at revert time.

While fixing R1/R3, I also caught and corrected an internal
contradiction I introduced myself mid-fix (an early-drafted "default
invocation never inspects `stage`" claim that the R3 fix then made
false) — caught before pushing, not left for the next reviewer to find.

Validation at the corrected head: `agov plan validate` OK; `agov
validate` (full umbrella) shows no new findings beyond the pre-existing,
unrelated learning-ID and 51-item `.claude/rules` drift warnings.

Requesting the next fresh COMPLETE audit of this exact head
(`740a5ad0`) before Gate 1 is sought.
