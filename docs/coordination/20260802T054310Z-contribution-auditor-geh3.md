---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260802T054310Z-contribution-auditor-geh3
timestamp: 2026-08-02T05:43:10Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260729T092303Z-governor-reviewer-bn8g
  - 20260731T065654Z-governor-reviewer-88mk
  - 20260731T081904Z-governor-reviewer-tken
topic: mr913-v250-contract-core-plan-review
requires-ack: false
artifacts_owned: []
---

# MR !913 v2.5.0 contract-core plan review: ADAPT / BLOCK

Scope: `docs/plans/v250-rc1-contract-core.md` in MR !913 at exact head
`d90c44d14b7e8a92b186cef914d6150c9910f488`, base
`7c2e48849c112011c486dfef7646e35057cd563f`.

Coverage: COMPLETE

Recommendation: BLOCK pending revision

Blocking findings: 4 open, unaccepted

Advisory findings: 1

Open findings with accepted risk: 0

Authority: recommendation only; the human governor-reviewer decides Gate 1.

Disclosure: this is an arm's-length read-only review by Codex in a separate
session and product/model stack from Claude's planner-authored plan. It did not
edit MR !913, its issues, or any implementation branch. Different products do
not by themselves prove factual independence; every finding below is tied to
current source, history, or tracker evidence.

## Verified before findings

- MR !913's exact-head pipeline `2724213743` passed. The plan itself passes
  `agov plan validate` when streamed from the exact head.
- The approved v2.5.0 SoW and later release-batching amendment do authorize a
  v2.5.0 authority/acknowledgement contract core. This review does not reopen
  that release decision.
- The #79 overwrite hazard and #80 accepted-but-untracked flag are real and
  still reproduce in current source.
- Excluding #76 and the parked trust-chain themes matches the later batching
  decision.

## BLOCK 1 — #79 leaves its central contract undecided

Check: B1-manifest-contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B1

Impact: material

Enforcement: blocking

Basis: issue #79 requires the child plan to select the canonical adopter-owned
path plus configuration/default and migration rules; Gate 1 must bound the
implementation decision.

Finding state: open

Risk disposition: none

Resolution condition: revise the plan to name one canonical adopter-owned
manifest path, its default and override precedence, the exact legacy migration
state machine, and the archive-upgrade mechanism used by the acceptance test.

Evidence:

- The scope says the implementation may either retain
  `agf/agents/MANIFEST.yaml` with upgrade-time protection or relocate it to a
  new path. Acceptance criterion 3 simultaneously mandates removal of that
  path and creation of `MANIFEST.example.yaml`. Those are different product
  contracts, not interchangeable implementation details.
- No new canonical adopter path is selected. `governance/MANIFEST.yaml` is
  only a candidate.
- The existing `AGOV_MANIFEST_PATH` override is already implemented in
  `agov-core::manifest_path()` and documented in the environment-variable
  reference. The plan says to "add configurable/default" support without
  defining how the existing override participates in precedence or migration.
- The plan says it will make ownership mechanically real, but it adds no
  upgrade apply/planner command. If the old in-archive path may remain, the
  plan must name the actual mechanism that prevents a normal archive extract
  from overwriting it. If relocation is chosen, say so and test that exact
  path instead.

## BLOCK 2 — #80's recommended all-kind obligation model is not implementable as specified

Check: B2-ack-contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B2

Impact: material

Enforcement: blocking

Basis: issue #80 explicitly requires recipient, resolution, queue, migration,
and CLI semantics to be designed consistently before implementation.

Finding state: open

Risk disposition: none

Resolution condition: select one contract at plan head and define the complete
permitted-kind matrix, resolution graph, historical cutoff/migration behavior,
and CLI presentation/operation semantics, including how cycles are impossible.

Evidence:

- Option 1 is described as tracking every schema kind. That includes `ack`
  itself. An `ack` with `requires-ack: true` can therefore require another ack,
  and the plan supplies no termination or cycle rule.
- `handoff next`, `handoff take`, and `handoff complete` are structurally
  handoff-specific. `render_handoff_summary()` reads the `handoff` object and
  its task/must-read/do-not-touch fields; non-handoff kinds do not have that
  object. Merely broadening `find_pending()` would produce an empty handoff
  task and would not define a generic acknowledgement workflow.
- Current `main` contains 371 non-handoff coordination records with
  `requires-ack: true`: 226 reviews, 86 notes, 40 decision records, 10 lesson
  proposals, 4 completions, 3 acks, and 2 fast-track waivers. The plan says
  not to silently reinterpret history but does not define a cutoff,
  grandfathering rule, diagnostic cardinality, or resolution path. A naive
  implementation would manufacture a very large historical pending queue.
- The SemVer section calls either option additive even though option 2 would
  reject combinations accepted by the published schema. "Silently inert" is
  evidence of a defect, not evidence that removing a schema-valid contract is
  non-breaking. Select option 1 with bounded semantics, or explicitly govern
  option 2's compatibility/deprecation consequence.

## BLOCK 3 — work package 3 is stale and already satisfied

Check: B3-current-reproduction

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B3

Impact: material

Enforcement: blocking

Basis: child-plan scope must reproduce against its exact base; already-shipped
work cannot be authorized again as an unimplemented release package.

Finding state: open

Risk disposition: none

Resolution condition: remove work package 3, or replace it only with an exact
current-main reproduction of a genuinely remaining #67/#78 defect and update
the issue ledger accordingly.

Evidence:

- The plan states that two adopter-facing legacy Pages URLs remain in
  `retrospective.md` and `learning-system.md`. On MR !913's exact base both use
  `janipaijanen.gitlab.io/agents-govern-framework/...`; there are zero
  `agent-governance` matches in the shipped `agf/` tree.
- Commit `d9dfe51e` corrected both URLs on 2026-07-29 and is already contained
  in v2.4.6 through v2.4.11. "Re-verify at implementation time" cannot rescue
  a false planning-time scope claim.
- The older #67 fact-checker/advisory-role validator defect is also already
  fixed on current main: `plan_validator.rs` accepts active manifest roles and
  has a direct `fact-checker` acceptance test. If some graph/advisory remainder
  still exists, this plan has not named or reproduced it.

## BLOCK 4 — work package 4 proposes already-shipped behavior without a #80 dependency

Check: B4-concurrency-slice

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B4

Impact: material

Enforcement: blocking

Basis: the approved SoW permits only the minimal unshipped #33 slice that #80
actually requires; it does not require reimplementation of existing behavior.

Finding state: open

Risk disposition: none

Resolution condition: remove the already-shipped validator scope; identify a
concrete unshipped #80 dependency if one exists. Route any remaining
per-writer/global-ordering prose clarification through a separately accurate
documentation slice or record the SoW dependency as already satisfied.

Evidence:

- Directory-aware duplicate-ID detection already exists in
  `validate_coord_files()` through `duplicate_id_errors_in_dirs()`. It scans
  sibling Markdown files and reports a duplicate when only one target file is
  passed.
- `coord_validate_blocks_duplicate_ids_in_same_directory` directly tests the
  exact behavior MR !913 proposes. The rule already says validators MUST
  compare against valid sibling records.
- This behavior entered in commit `40c77ac3` and is contained in every tag from
  v2.3.0 onward, including v2.4.11. It is not v2.5.0 work.
- #80 already relies on existing coordination identity and reference
  resolution just as current handoff tracking does; MR !913 does not show a
  new causal need for collision detection.
- Two agents choosing the same predictable branch name and receiving a loud
  Git rejection demonstrates concurrent branch naming, not a readable-ID
  collision. The plan partially concedes this, but still says issue #33's
  "not yet observed" statement is no longer accurate. The ID failure remains
  theoretical on the cited evidence.

## ADVISORY 1 — publish meaningful MR metadata

MR !913's description is the untouched contribution template, including the
literal `Closes #` placeholder, and it has no labels. The API currently reports
no structural closing issue, so this is not a premature-closure blocker.
Nevertheless, a four-package Gate 1 request needs a rendered summary, exact
issues, evidence, boundaries, and an explicit statement that it does not close
them. Add the `plan` label and replace the template before renewed review.

## Required renewed review boundary

Any correction changes the decision surface materially. Request a renewed
independent review against the new exact head. Do not treat the passing
pipeline or schema-valid front matter as resolving B1–B4; those checks do not
validate factual freshness or contract completeness.
