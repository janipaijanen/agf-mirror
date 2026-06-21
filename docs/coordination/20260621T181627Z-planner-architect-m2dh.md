---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260621T181627Z-planner-architect-m2dh
timestamp: 2026-06-21T18:16:27Z
immutable: true
references-to:
  - 20260621T180612Z-planner-architect-dh37.md
topic: rule-gate-posture-improvements-gate2
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T181627Z-planner-architect-m2dh.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review rule/gate-posture-improvements branch (2 substantive commits) for Gate 2 approval
    - Merge if approved
  must_read:
    - MR_DESCRIPTION.md (on rule/gate-posture-improvements branch)
  do_not_touch:
    - docs/coordination/
---

# Gate 2 handoff: rule/gate-posture-improvements

Research-driven rule and docs improvements from external governance article scan.
Branch: `rule/gate-posture-improvements` (2 substantive commits + session closeout note)

## What was built

**`agf/rules/always/high-risk-operations-rule.md`** (+ `.claude/rules/` copy):
- Added per-area consequence/blast-radius statements to each high-risk bullet.
  Rationale: consequence-based framing improves compliance vs stop-only rules
  (Amazon/Register article, June 2026 — normalization of deviance under repeated
  approval tasks).

**`agf/rules/workflow/systematic-code-review-rule.md`** (+ `.claude/rules/` copy):
- Added "Gate types" section with structural vs advisory enforcement table.
  Honest about the gap: advisory gates rely on review discipline; structural gates
  (pre-commit hooks, CI validators) enforce regardless of agent cooperation.
  Rationale: tenureai.dev article — gate enforcement transparency matters for
  adopters deciding where to invest in automation.

**`agf/docs/site/docs/adopt/tooling-and-tokens.md`**:
- Added "Why meter at the governance gate" section before the evidence model.
  Covers the chokepoint principle: gate-level metering produces cost attribution
  that API billing cannot (cost per role, per gate pass). Cites r-lopes blog.

**`agf/docs/site/docs/concepts/roles-and-gates.md`**:
- Added "Structural and advisory enforcement" section with gate-type table and
  link to contracts-and-validators. Consistent with the systematic-code-review
  gate-types addition.

## Verification

- `mkdocs build --strict`: clean (confirmed twice in the session that produced this branch)
- Both rule copies verified in sync (`diff` showed only provenance frontmatter difference)
- No new locales, no new interactive elements, no security boundary changes

## Notes

- Dual-copy rule (agf/rules/ + .claude/rules/): both copies updated. The only
  diff between them is the `provenance: verified-agf-v2` frontmatter in agf/rules/.
- The coordination note `20260621T180612Z-planner-architect-dh37.md` on this branch
  is a session closeout note (kind: note, to: all) — not a substantive change.
