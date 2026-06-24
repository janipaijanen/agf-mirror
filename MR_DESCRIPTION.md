# learn: incorporate harbor-workshop v1-to-v2 switchover learning (#16)

Branch: `learn/harbor-workshop-v1-v2-switchover`

## Summary

Incorporates the learning record submitted in issue #16 by adopter
`harbor-workshop` (2026-06-24). Added as `20260624T120000Z-q7wj` to
`agf/learnings/foundry.yaml`.

**Finding:** AGF v1-to-v2 switchovers need a migration checklist and
role-reviewed learning cleanup. The friction during migration was not copying
files — it was finding every active instruction surface still teaching the old
layout, old numbering scheme, or local machine-specific tool paths. A later
role review also caught that the adopter's own learning record could pass YAML
validation while still carrying stale v1 framework metadata and retired role
paths.

**Recommendation recorded:** Add an upstream v1-to-v2 switchover checklist
covering path retirement (agents/, agents-govern/), role migration into agf/agents/,
loader and cold-start instruction updates, coordination ID migration to readable
IDs via `agov coord brief`, adopter validation (`agov validate adoption/agents/
learnings/infosec`), active-doc scan for old paths, and a full role review
(infosec-sentinel, contribution-auditor, fact-checker) of the adopter's own
learning records before submission.

## Files changed

| File | Change |
|---|---|
| `agf/learnings/foundry.yaml` | Added learning `20260624T120000Z-q7wj` (single-adopter, harbor-workshop) |

## Validation

```
$ agov validate learnings
OK agov.validate.learnings
```

Pre-existing WARN entries on bare timestamp IDs (L018–L021 era) unchanged.

## Review checklist

- [ ] Learning content faithfully captures the issue #16 submission
- [ ] ID format: `YYYYMMDDThhmmssZ-<4-char-random>` ✓
- [ ] Evidence ref recorded: `harbor-workshop:2026-06-24:agf-v2-switchover`
- [ ] `agov validate learnings` passes (no new errors)
- [ ] No `Co-authored-by` trailers in commit
