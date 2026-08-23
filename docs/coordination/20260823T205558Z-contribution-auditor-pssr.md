---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260823T205558Z-contribution-auditor-pssr
timestamp: 2026-08-23T20:55:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T204154Z-planner-architect-tsqg.md
topic: learning-record-backlog-plan-batch-exact-head-review
requires-ack: true
artifacts_owned: []
---

# Learning-record backlog plan batch: COMPLETE / BLOCK

Scope:

- MR !1855 at `677279f11e43b6b10280ca944025ccc3574f0ab5`
- MR !1856 at `1e48fc3f9d59907c0432970b913fb2e1a5d26ea0`
- MR !1857 at `f472f994773a89778103567f07765df054e14414`
- MR !1858 at `8c244c9fb3b6b61dc30837fd733932080d3a621e`

Coverage: COMPLETE

Recommendation: **BLOCK all four before Gate 1**

Blocking findings: 11 open

Advisory findings: 1

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. The
governor-reviewer decides Gate 1.

Independence: this Codex session did not author the four plans or their
implementation designs. Earlier issue-label maintenance did not contribute
design substance to these plans.

## MR !1855 — upgrade overlay and provenance protection

### `CA-LRB-UPG-1` — the five classes are neither exhaustive nor operationally consistent

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: Gate 1 must approve the data-loss-prevention contract before coding

Finding state: open

Risk disposition: none

The proposed three-tree classifier does not cover the full presence/content
state space. For example, when `previous != new` and `local == new`, the path
is neither upstream-only, adopter-only, overlapping (the plan requires all
three to differ), removed-upstream, nor local-only. Both-removed paths,
local deletions, directory/file transitions, and symlink states are also not
specified. The action table then says overlapping files are both manual-merge
findings and candidates to overwrite from the archive, depending on an
undefined “side.” This leaves the implementation to decide the exact behavior
whose purpose is preventing silent loss.

Resolution condition: define an exhaustive truth table over path presence,
content equality, file type, and symlink state; give each state one result and
one permitted operation; make every unclassified state fail closed. Align the
skill actions with that table and add table-driven tests.

### `CA-LRB-UPG-2` — trusted-root and archive provenance boundaries are absent

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: the plan reads release archives and authorizes overwrite/removal of local files

Finding state: open

Risk disposition: none

The CLI accepts three arbitrary directories but does not say which release
manifest controls enumeration, how the previous and new pristine releases are
authenticated, or how paths are confined to their roots. It does not define
behavior for absolute/parent-traversal manifest entries, symlinks escaping a
root, duplicate/case-colliding paths, or a tampered new archive. The plan itself
says this write path warrants infosec review, yet no reviewed threat boundary
or acceptance evidence is supplied.

Resolution condition: specify release-identity/provenance inputs, authoritative
manifest selection, canonical path confinement, symlink policy, collision
handling, and fail-closed errors. Add adversarial fixtures and record the
infosec disposition before Gate 1.

### `CA-LRB-UPG-3` — a restrictive upgrade contract is misclassified and incompletely documented

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md`

Finding state: open

Risk disposition: none

The plan adds confirmation requirements and prevents operations previously
permitted by the shipped skill, but calls the change PREVENTIVE/ADDITIVE. The
rule explicitly classifies a restriction of previously permitted behavior as
BREAKING. The plan also defers the exact adopter documentation file until
implementation and says rollback is consequence-free even after adopters rely
on the new command/skill contract.

Resolution condition: classify the behavioral change as BREAKING, name the
exact migration and adopter-operating documentation paths now, require the
Gate 2 docs-parity outcome, and describe rollback after adoption rather than
only before use.

## MR !1856 — adoption version drift

### `CA-LRB-VER-1` — “each learning record” confuses live adoption state with imported evidence

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: validator scope must not rewrite or reject historical adopter evidence

Finding state: open

Risk disposition: none

The proposed check compares every learning record with the current tree. At
the reviewed head, `agf/VERSION` is `2.6.1`, while
`agf/learnings/harbor-workshop.yaml` records `2.5.0` and
`agf/learnings/foundry.yaml` records `2.0.0-provisional`. Those are imported or
historical evidence records, not necessarily the live adoption record for this
checkout. The plan would make the repository's existing evidence corpus fail
or encourage rewriting the version at which evidence was observed.

Resolution condition: define how the validator selects exactly the live
adoption record for an adopter checkout and excludes imported/historical
evidence. Add current-corpus regression tests proving historical records remain
unchanged and valid.

### `CA-LRB-VER-2` — the transient lifecycle decision is already real but deferred past Gate 1

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: Gate 1 design completeness

Finding state: open

Risk disposition: none

The current upgrade skill applies archive files in Step 3 and updates
`adoption.framework_version` in Step 4, so the mismatch window the plan treats
as hypothetical is real. The plan delegates the marker name, schema, owner,
cleanup, crash recovery, and suppression scope to implementation. That is a
new lifecycle contract, not a local coding choice.

Resolution condition: decide the exact lifecycle at Gate 1, including marker
shape/location, who sets and clears it, interrupted-upgrade recovery, maximum
suppression scope, and default failure behavior. Name schema, template, skill,
documentation, and tests that change.

### `CA-LRB-VER-3` — blocking behavior and a possible record field lack the required downstream disposition

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md` adopter-docs-parity and BREAKING boundary

Finding state: open

Risk disposition: none

A new blocking validator narrows previously accepted records, and the proposed
exception may add a learning-record field. The plan labels this merely
PREVENTIVE, names no adopter-facing operating/template guidance, and claims no
schema or data implication on rollback.

Resolution condition: classify the restriction correctly, give a migration
for existing live drift, name adopter docs/template/schema surfaces, require a
Gate 2 docs-parity outcome, and make rollback cover records carrying the new
lifecycle state.

## MR !1857 — distribution ignore protection

### `CA-LRB-IGN-1` — the proposed Git command misses tracked ignored paths

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: issue #113 acceptance requires effective ignore-rule detection

Finding state: open

Risk disposition: none

`git check-ignore -q <path>` normally suppresses tracked files from its output.
Once an adopter has force-added a path, the proposed check can report it as not
ignored even though the effective rule still matches it. The design must use
the index-independent semantics (`--no-index`) when testing the rule itself and
must distinguish Git errors from “not ignored.”

Resolution condition: define the exact command/API semantics and add fixtures
for tracked, untracked, force-added, negated, and nested-ignore cases, including
`-v --no-index` attribution.

### `CA-LRB-IGN-2` — the release manifest contains globs, exclusions, and optional entries, not an enumerable path list

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: implementation must have a deterministic coverage boundary

Finding state: open

Risk disposition: none

`agf/dist/release-manifest.yaml` mixes literal `path` entries with recursive
`glob` entries, exclusions, forbidden patterns, and optional content. The plan
says to check “every path declared as distributed” but does not define which
tree expands globs, whether absent matches fail, how excludes win, or what
happens outside a Git worktree. Different choices change both coverage and
false-positive behavior.

Resolution condition: specify deterministic manifest expansion and exclusion
precedence against the adopted tree, optional/absent behavior, repository-root
detection, and non-Git behavior. Test each manifest entry form.

### `CA-LRB-IGN-3` — enforcement and migration are deferred or misclassified

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: Gate 1 completeness and `downstream-impact-rule.md`

Finding state: open

Risk disposition: none

The plan leaves default umbrella enforcement, a dedicated command, and a
`--skip-dist-visibility` bypass to the implementer. Those options create
different governance guarantees. A new blocking check also restricts a
previously accepted checkout, so the rule classifies it as BREAKING, not merely
ADDITIVE.

Resolution condition: choose the canonical invocation and bypass authority at
Gate 1, state behavior in CI/release/install flows, classify the restriction
correctly, and provide the exact migration and docs-parity evidence.

## MR !1858 — review topology declaration

### `CA-LRB-TOP-1` — “blinded” is presented as sufficient independence

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: issue #82 requires correlated reviews not to be presented as independent

Finding state: open

Risk disposition: none

The proposed `blinded independent review` category tests only whether a reviewer
saw another answer. Two reviews may still share the implementing session,
prompt, supplied framing, context, model lineage/vendor, or evidence selection.
Calling that one condition “independent” can recreate the same overclaim the
plan aims to prevent.

Resolution condition: either name the field narrowly (for example,
`blinded-to-prior-answers`) or require an independence disclosure covering the
material dimensions and state which dimensions are necessary for each claim.
Coordinate terminology with issue #47 rather than silently defining a weaker
framework-wide meaning.

### `CA-LRB-TOP-2` — external-review topology is incorrectly tied to adopter confidence levels

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: `learning.schema.json`, `schemas/invariants.yaml`, and the External Review template

Finding state: open

Risk disposition: none

The External Review template explicitly says it is not an adopter Learning
Record. Learning confidence levels describe single-adopter, multi-adopter,
longitudinal, and industry-corroborated evidence; the enforced threshold only
counts evidence items. The plan nevertheless says blinded model reviews count
for those levels, which can be read as allowing multiple models in one exercise
to promote adopter confidence.

Resolution condition: state explicitly that review topology never turns one
adopter/event into multi-adopter or longitudinal evidence and does not by
itself increment `confidence.evidence_count`. If a separate cross-model
corroboration contract is desired, define it separately and update its schema
or invariant rather than attaching it informally to adopter levels.

### Advisory `CA-LRB-TOP-A1` — “required” remains unenforced

The change is documentation-only and the plan explicitly rejects mechanical
validation. That is a valid maturity choice, but the result is Level 2 guidance,
not a guaranteed declaration. Describe omissions as detectable only in manual
review and avoid claiming the field is universally present.

## Verified evidence

- Each MR is plan-only, one file, and `git diff --check` is clean.
- `agov plan validate` passes independently at all four exact heads.
- Exact-head pipelines `2783539230`, `2783539355`, `2783539504`, and
  `2783539707` passed.
- Live MR descriptions are populated and the four MRs are conflict-free.
- Issue #82, #96, #111, #112, and #113 source records were compared with the
  plans; the findings above concern unresolved implementation contracts, not
  the legitimacy of the reported adopter problems.

Correct each MR independently on a plan-only head and request renewed review;
the smaller plans need not wait for the larger MR !1855 correction.
