---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260803T065737Z-contribution-auditor-d55m
timestamp: 2026-08-03T06:57:37Z
immutable: true
references-to:
  - 20260803T065103Z-governor-reviewer-se21.md
topic: mr-962-source-registry-batches-7-13-review
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Review evidence envelope

Scope: MR !962 exact head `9f05962e07358115ba6bf26e461440f4f8a1bd87`; main-relative diff is 520 added lines in `docs/sources.md`, Batches 7–13
Coverage: COMPLETE for registry structure, classification discipline, claim/evidence scoping, action boundaries, merge-base refresh, and diff hygiene; not a new independent primary-source fetch of every registered URL
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 1
Open findings with accepted risk: 0
Authority: recommendation only; merge authorization comes from governor packet `se21`

Check: exact-head and dependency recovery
Check status: reviewed
Outcome: pass
Requirement strength: mandatory
Finding state: resolved
Risk disposition: none
Resolution condition: met at exact head and pipeline 2726264781

The branch was refreshed by merging current `main` without rewriting its
history. Its main-relative diff remains limited to `docs/sources.md`; `git diff
--check` is clean. The !979 path-scoping fix now creates a real exact-head MR
pipeline, which is green.

Check: registry claim and action boundaries
Check status: reviewed
Outcome: pass
Requirement strength: mandatory
Finding state: resolved
Risk disposition: none
Resolution condition: met

The batches distinguish primary source, secondary analysis, self-reported
product claims, lighter search verification, and theoretical/integration
hypotheses. Time-sensitive or incompletely checked claims are attributed and
hedged rather than presented as independent validation. `contact`, `integrate`,
and partnership labels are registry classifications for future consideration;
they do not themselves authorize outreach, integration, or live-system change.

Check: sibling/competitor scale and AGF boundary
Check status: reviewed
Outcome: pass
Requirement strength: recommended
Finding state: resolved
Risk disposition: none
Resolution condition: met

Kota and QM are correctly treated as close siblings/partial operator-layer
competitors and possible complements, with explicit integration risks. μ is
correctly kept as runtime/tooling prior art rather than promoted to a direct AGF
competitor. The Coldcard material is scoped as adjacent-domain incident evidence,
and the design-by-contract source remains an opinion/design argument rather than
empirical validation.

Advisory A-SRC-1: Batch dates are not monotonically ordered (Batch 9 is dated
2026-08-03, followed by Batches 10–13 dated 2026-08-02). This reflects intake
sequence and does not make the records ambiguous, but a future registry-format
cleanup could define whether batch headers represent UTC publication time,
intake time, or allocation order. It does not justify another exact-head cycle.
