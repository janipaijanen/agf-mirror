---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260824T042339Z-governor-reviewer-1m61
timestamp: 2026-08-24T04:23:39Z
model: human-governor
immutable: true
references-to:
  - 20260823T201631Z-planner-architect-dda7.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1817-ai-landing-row-redesign
requires-ack: true
artifacts_owned: []
---

# MR !1817: `dda7` resolved by redesign, not a second waiver

The human governor answers `dda7`'s question (does `am2p`'s one-time,
non-precedential waiver extend to the `.claude/rules/` mirror path):
**no second waiver is granted.** Instead, the plan's `ai_landing_loaders`
projection row is redesigned to cite only the canonical
`agf/rules/workflow/unassigned-work-discovery-rule.md` path — already
covered by `am2p`'s existing waiver — plus `AGENTS.md`.

This does not weaken the technical commitment to create the
`.claude/rules/` mirror; it remains in "Files expected to change" and is
still created verbatim in the same Gate 2 implementation commit. It is
simply no longer independently cited in the projection manifest as its
own landing surface, since it is a byte-identical downstream copy of an
already-covered path, and its ongoing sync is tracked by the existing,
generic `agov validate` `assistant_drift` check.

Planner-architect implemented this at exact head
`f4568aa410d32fb591f61312a830d749fc1245f8`: `docs/reviews/rule-
projection/20260823T151423Z-unassigned-work-discovery-rule.yaml`'s
`ai_landing_loaders` row now cites `agf/rules/workflow/unassigned-
work-discovery-rule.md` and `AGENTS.md`, with `.claude/rules/workflow/
unassigned-work-discovery-rule.md` moved to `not_applicable` with a
disclosed reason. `agov plan projection-check --base-ref origin/main`
on the corrected manifest, re-run and verified (not assumed), still
fails at 4 locations — but all four now cite the single file `am2p`
already waives, down from 3 citations of that file plus 1 citation of a
second, uncovered file. No open governor question remains on this row.

This decision does not itself approve Gate 1 for MR !1817 — the fresh
COMPLETE contribution-auditor audit already requested elsewhere for this
plan is unaffected and still required before Gate 1 is sought.
