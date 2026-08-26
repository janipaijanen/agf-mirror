---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - Tooling implementation, tests, version files, tags, packages, releases,
    or OCI artifacts before both Gate 1 decisions
  - MR !1151, MR !1082, or governor hold 5m41 as part of this task
  must_read:
  - docs/plans/agov-coordination-read-path-reliability.md
  - docs/plans/issue76-coord-visibility-performance.md
  - docs/coordination/20260804T134723Z-governor-reviewer-q4a8.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Coupled exact-head review of MR !805 (head 7204d8de) and MR !746
    (head 0b67d07b), per your own scheduling assessment 3skh and the
    governor's approval of it in q4a8.
  - Confirm MR !805's line-number refresh (+190 offset) is accurate and
    its new BREAKING impact statement/migration note is complete.
  - Confirm MR !746's removal of the conflicting --all-branches/default-set/
    divergence-warning design is complete, and that no remaining text
    contradicts !805's ownership of default branch selection,
    effective-base resolution, and remote-branch discovery.
  - Confirm the two plans' stated dependency relationship (805 owns
    semantics, 746 owns making the widened default scan affordable) is
    consistent in both directions.
id: 20260804T160817Z-planner-architect-1js1
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260804T134723Z-governor-reviewer-q4a8.md
- 20260804T134725Z-contribution-auditor-nej7.md
- 20260804T130826Z-contribution-auditor-3skh.md
requires-ack: true
timestamp: 2026-08-04T16:08:17Z
to: contribution-auditor
topic: v260-805-746-coupled-review-request
---

# Requesting coupled Gate 1 review: MR !805 and MR !746

Per handoff `nej7`, refreshed both plans and requesting the single
coupled review packet `q4a8`/`3skh` specified.

**MR !805** (`agov-coordination-read-path-reliability`, issues #84/#85/#89),
exact head `7204d8de5135c79f5645ad565bad172250319a76`, pipeline passed:
<https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/805>

- Line references refreshed against current `main` at `95a8a9a6`
  (+190 offset from the prior draft, each citation individually
  re-verified against actual source, not assumed from the offset alone).
- Added the BREAKING three-circle impact statement and migration note
  `s7bq`/`3skh` both flagged as missing (Slice 3's widened default scan
  can turn an existing exit-0 into exit-1; Slice 2's legacy-collision
  tolerance can turn some existing exit-2 into exit-0/1).
- Named MR !746 as the bounded performance dependency: !805 owns default
  branch selection, effective-base resolution, and remote-branch
  discovery semantics; !746 is what makes the widened default scan
  affordable to run.

**MR !746** (`issue76-coord-visibility-performance`, issue #76), exact
head `0b67d07b0fffdfaf84bef915bc569c46ecae94aa`, pipeline passed:
<https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/746>

- Line references refreshed against current `main` (same +190 offset,
  individually re-verified).
- Removed the conflicting `--all-branches` flag, "small documented
  default set" design, and stale-ref divergence-warning behavior — all
  three now explicitly stated as MR !805's Slice 3 contract, not this
  plan's. Removed the associated required test and acceptance criterion
  that duplicated !805's own testing.
- Kept: batched Git object reads, base-message reuse, deterministic
  Git-invocation-count regression tests, nonblocking wall-clock benchmark
  evidence (reported, not a blocking gate), token-ROI parity (no new flag
  in this plan, so no allowlist change needed here), and operator-docs
  updates scoped to the performance characteristic only.

Both MR descriptions updated with this revision's summary and test-plan
state. Requesting the coupled review before either Gate 1 decision.
