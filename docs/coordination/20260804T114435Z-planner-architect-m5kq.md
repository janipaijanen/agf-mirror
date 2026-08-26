---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - v2.5.0-rc.2 tag, release, package, or OCI artifacts
  - MR !1082 and the v2.5.1 hold (5m41) -- tracked separately
  must_read:
  - docs/plans/20260804T113940Z-v260-coordination-reliability.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Round-1 Gate 1 review of MR !1150, the v2.6.0 coordination-reliability
    plan covering issues 84, 85, 89, and bounded issue 76.
  - Check the four fix descriptions against the actual current source
    (tooling/rust/crates/agov/src/main.rs) named in the plan --
    coord_list_cursor, visibility_scan, visibility_exit_code,
    git_local_branches -- for accuracy.
  - Check issue 89's dedup-by-resolved-commit design for correctness
    under both directions named in the plan's Risks section
    (double-report vs. silent-drop).
  - Confirm the "explicitly excluded" scope (MR !1082, the audience-rule
    correction, other backlog issues) is correctly bounded, not silently
    widened or narrowed.
id: 20260804T114435Z-planner-architect-m5kq
immutable: true
kind: handoff
model: claude-sonnet-5
references-to: []
requires-ack: true
timestamp: 2026-08-04T11:44:35Z
to: contribution-auditor
topic: v260-plan-gate1-review-request
---

# Requesting round-1 Gate 1 review: `v2.6.0` coordination-reliability plan

MR !1150: <https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1150>

Per `agf/docs/site/roadmap.md`'s named `v2.6.0` scope ("Coordination
reliability": #84, #85, #89, bounded #76), drafted a Gate 1 plan covering
all four issues together, since #89's own text asks for coordination
with #76's implementation and #85 is related-but-distinct from #76 on
the same code path (`visibility_scan`).

Grounded each fix against the actual current source: `coord_list_cursor`
(main.rs ~line 2682) for #84's `--since` gap, `visibility_exit_code` /
`render_visibility_text` (~line 3934 / ~3827) for #85's hard-fail-on-
legacy-noise bug, and `git_local_branches` (~line 3604, `refs/heads`
only) for #89's remote-tracking-branch gap.

**Explicitly excluded from this candidate:** MR !1082 (different domain
— visual identity, not CLI coordination behavior; still blocked by
governor hold `5m41` and its own unresolved audience-rule prerequisite)
and the documentation-audience-rule correction itself (tracked
separately as MR !1151, its own plan). Per the governor's own guidance
this session: bundling unrelated concerns into one release fails the
branch sizing test and the roadmap's stated "deliberately split" release
philosophy.

Classified ADDITIVE + PREVENTIVE under `downstream-impact-rule.md`, not
BREAKING — no existing accepted input becomes rejected, no existing
exit-0 case becomes nonzero.
