---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260803T135943Z-contribution-auditor-gbkp
timestamp: 2026-08-03T13:59:43Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T134350Z-governor-reviewer-t41t
topic: mr1051-gate1-review
requires-ack: true
artifacts_owned: []
---

# MR !1051 Gate 1 review: BLOCK at `9ae18ca4`

Scope: complete Gate 1 review of
`docs/plans/v250-rc1-release-publication.md` at exact head
`9ae18ca47e7fb37e39434c2f28939d8fcbead21b`.
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 6
Advisory findings: 2
Open findings with accepted risk: 0
Authority: recommendation only; governor-reviewer decides Gate 1 and each
release-scope amendment.

The plan is strong on immutable-tag sequencing, the three required upgrade
baselines, artifact provenance, and the rc.2 trigger. The findings below are
bounded corrections to make its scope and execution mechanically honest.

## B-RC-SCOPE-1

Check: complete reconciliation of `v2.4.11..current main`
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-RC-SCOPE-1
Impact: material
Enforcement: blocking
Basis: governor handoff `t41t` and approved batching amendment
Finding state: open
Risk disposition: none
Resolution condition: refresh the audit against current `main`, add every
shipped out-of-bound surface, and put each behind an explicit governor scope
choice before Gate 1.

The reconciliation omits the shipped additive governance-layout correction
entirely, although `t41t` names it explicitly and both changelogs classify it
as Additive. `agf/rules/always/governance-file-layout-rule.md` is selected by
the release manifest, so this is release content, not repository-only
evidence.

The table also says issue #101 needs no scope decision even though the new
`agov ci cache-topology-check` command necessarily enters every tooling
binary built from current `main`. The approved batching amendment limits
v2.5.0 to #79/#80 plus necessary compatibility slices; being
repository-internal in purpose does not make a compiled command disappear
from the product archive. Learning supersession, governance-layout, and the
cache command therefore all require an explicit include/hold choice (one
combined current-main amendment is acceptable).

This omission already makes the changelog acceptance text contradictory:
it requires every `[Unreleased]` entry to be one of “the three in-scope
items,” while current root `CHANGELOG.md` also contains the governance-layout
fix and both issue #101 slices. MR !1050 also entered `main` after the stated
`f77d80d2` audit baseline; it is repository-only metadata, but the refreshed
audit should record that disposition rather than silently use a pre-sync
baseline.

## B-RC-GATE-METADATA-2

Check: Gate 1 metadata and release-only branch sequencing
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-RC-GATE-METADATA-2
Impact: material
Enforcement: blocking
Basis: AGENTS.md Gate discipline and `release-via-merge-request-rule.md`
Finding state: open
Risk disposition: none
Resolution condition: state that Gate 1 approval metadata is updated and
validated on this plan MR before it merges; cut the release branch only after
the approved plan is on `main`; remove the plan-file edit from the
release-only change set.

Release-only step 1 defers this plan's `status`/approval metadata to the later
release branch. AGENTS.md explicitly says to update plan metadata in the same
review cycle and requires a governor coordination record for any post-hoc
correction. The immediately preceding MR !1010 merge demonstrated the exact
failure mode: it landed with `pending-gate1` metadata and needed MR !1050 to
repair it. Repeating that sequencing here would also contaminate the claimed
release-truth-only MR with an internal plan-state correction.

## B-RC-VERSION-SURFACES-3

Check: complete release-version truth-surface inventory
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-RC-VERSION-SURFACES-3
Impact: material
Enforcement: blocking
Basis: release-via-MR mandatory version-consistency check
Finding state: open
Risk disposition: none
Resolution condition: restore the adopter-facing agent definitions to the
explicit version-bump list and require a full current-version scan.

Change-set item 5 names `agf/agents/MANIFEST.yaml` but omits all twelve
adopter-facing `agf/agents/*.yaml` role definitions. Each currently contains
`framework_version: "2.4.11"` and each ships through `agf/agents/**`. The
v2.4.11 plan named “adopter-facing agent definitions” separately for this
reason. They must become `2.5.0-rc.1`; historical evidence remains untouched.

## B-RC-ARTIFACT-MATRIX-4

Check: upgrade/overlay and CLI/MCP artifact boundary
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-RC-ARTIFACT-MATRIX-4
Impact: material
Enforcement: blocking
Basis: `t41t` exact-artifact requirement
Finding state: open
Risk disposition: none
Resolution condition: split the matrix into (a) framework archive overlay
for each named baseline and (b) matching packaged `agov`/`agov-mcp` binary
installation and parity checks, naming which archive supplies each surface.

The framework `agf-{version}` tar/zip archives contain schemas/templates but
not Rust binaries. The separate platform tooling archives contain `agov` and
`agov-mcp`. The current bullet says one “plain archive extraction” must both
preserve the adopter manifest and show the new command, without naming the
second artifact or its installation. That cannot prove exact published-byte
parity. Also replace the vague “`--all-kinds`-equivalent naming as actually
shipped” with the actual interfaces: CLI `agov coord pending`, MCP
`agov_coord_pending`, plus generic `agov_validate` for learning validation.

## B-RC-PRERELEASE-5

Check: supported GitLab Release behavior
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-RC-PRERELEASE-5
Impact: material
Enforcement: blocking
Basis: release-only scope and current tag pipeline
Finding state: open
Risk disposition: none
Resolution condition: use the existing RC convention (an ordinary GitLab
Release whose SemVer/name contains `-rc.1`) unless a separately governed CI
change first adds a real supported marker.

The plan authorizes “marking it as a GitLab pre-release” if no convention
exists. The two live precedents, `v2.2.0-rc.1` and `v2.2.0-rc.2`, are ordinary
GitLab Releases (`upcoming_release: false`); `.gitlab-ci.yml` invokes
`release-cli create` with name, tag, description, and assets only. No
pre-release flag or behavior exists in the current release path. Adding one
would be a CI/publication behavior change forbidden by this plan's own scope
and stop conditions.

## B-RC-BRANCH-6

Check: governed release branch name
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-RC-BRANCH-6
Impact: material
Enforcement: blocking
Basis: `.claude/rules/workflow/release-via-merge-request-rule.md`
Finding state: open
Risk disposition: none
Resolution condition: use `release-2.5.0-rc.1`, or obtain an explicit rule
amendment before using a different name.

The plan says `release-v2.5.0-rc.1`; the active rule requires the
`release-X.Y.Z` form. Historical branches with an extra `v` do not override
the written rule.

## Advisories

- **A-RC-MATURITY-1:** replace “already shipped in a separately Gate
  2-approved implementation MR” and similar present-perfect release claims
  with “merged and Level 4 Validated.” They become Level 5 Deployable only
  after this publication path succeeds.
- **A-RC-ROADMAP-2:** make the roadmap edit explicitly correct its already
  stale “latest release v2.4.10 / v2.4.11 pending” text to published stable
  v2.4.11, then distinguish that stable baseline from the new
  v2.5.0-rc.1 candidate. Also replace MR !1051's untouched default
  description template with the plan scope, evidence, and checks.

Request a bounded renewed review of B-RC-SCOPE-1 through B-RC-BRANCH-6 and
the two advisories at the corrected exact head. Do not create the release
branch, bump versions, tag, or publish while Gate 1 remains pending.
