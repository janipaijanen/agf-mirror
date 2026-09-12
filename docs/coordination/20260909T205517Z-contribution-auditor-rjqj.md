---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260909T205517Z-contribution-auditor-rjqj
timestamp: 2026-09-09T20:55:17Z
model: codex-gpt5
immutable: true
references-to:
  - 20260909T203827Z-planner-architect-khm3
  - 20260909T193208Z-coder-tester-nj1j
  - 20260909T194219Z-governor-reviewer-j8bq
topic: v211-release-publication-child-gate1-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 release-publication child — exact-head Gate 1 audit

Scope: MR !2646, exact source head
`c0a06e92852313c133212400262b47424e18abad`.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 5

Advisory findings: 1

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only; the
governor-reviewer decides Gate 1 and every later release boundary.

Automated evidence: plan and handoff validate; exact-head canonical pipeline
`2834670442` is green; `git diff --check`, contribution heuristics, and InfoSec
validator pass (two known pre-existing learning-corpus warnings); SPDX placement
is correct; no credential value or protected content appears in the diff.

## V211-RP-1 — protected-tag policy is checked too late

Check: rule soundness

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: V211-RP-1

Impact: critical

Enforcement: blocking

Basis: approved parent plan's “Protected-tag policy checkpoints” and `nj1j`

Finding state: open

Risk disposition: none

The parent requires the exact `v2.11.0` rule, Maintainers-only create access,
to exist and match **before preflight**, then be re-read immediately before
push. The child first checks it in Step 5b, after the metadata MR merged. Live
API evidence also shows the v2.10.0 rule exists but v2.11.0 is currently absent.

Resolution condition: create/read/record the exact policy before any versioned
draft or namespace preflight in Steps 3–4, and retain the Step 5 re-read.

## V211-RP-2 — validations are not bound to the metadata draft that is committed

Check: technical plausibility

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: V211-RP-2

Impact: material

Enforcement: blocking

Basis: approved reversible-preflight sequence and release-via-MR rule

Finding state: open

Risk disposition: none

Step 3 says to run validations from a clean checkout and only then create the
metadata commit. That can validate the old 2.10 tree rather than the proposed
2.11 metadata. The required sequence is: apply the exact allowlisted metadata
draft without committing; run every reversible validation against that dirty
draft; record the draft tree/diff; commit exactly those already-tested bytes;
prove the worktree is clean and the commit tree equals the tested draft; then
run namespace preflight on that clean unpublished commit.

Resolution condition: make that byte/tree-binding sequence explicit.

## V211-RP-3 — the derived inventory omits live adoption metadata and is not an exact allowlist

Check: evidence quality and boundary integrity

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: V211-RP-3

Impact: material

Enforcement: blocking

Basis: parent requirement for an exact metadata allowlist and live-version inventory

Finding state: open

Risk disposition: none

The audit command excludes all of `agf/learnings`, so it cannot discover the
top-level live adoption `framework_version` in
`agf/learnings/foundry.yaml`, which the v2.10 release updated without rewriting
historical learning entries. It also does not itself derive non-version Group B
status rows or the #129 roadmap correction. “Expected members” plus an open
ended “any text” is not the exact metadata allowlist the parent requires.

Resolution condition: enumerate the exact permitted paths and the precisely
bounded cells within mixed historical/live files; include the foundry top-level
adoption metadata; define separate deterministic discovery for Group B and the
#129 status-only correction; fail on any final diff outside that allowlist.

## V211-RP-4 — the MR acceptance-job list is wildcarded and inaccurate

Check: technical plausibility

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: V211-RP-4

Impact: material

Enforcement: blocking

Basis: parent requirement for actual verified job names and exact-head Gate 2 evidence

Finding state: open

Risk disposition: none

Step 4 uses `rust:tooling:*`, which includes tag-only and optional/manual jobs,
and names selftests that do not necessarily run for a version-only diff. It
also omits mandatory jobs that ran on the last release MR. Pipeline
`2829685860` provides the concrete regression baseline: its required non-manual
jobs included `agf:dist:validate`, `agf:dist:preview`, `site-html-build`,
`agf:flyer:verify`, `agov:governance:validate`, `agov:oci:mr-build-scan`,
`commercial-corpus:validate`, `dependency:cve-scan`,
`release:assets-verify-preview`, `release:namespace-check`,
`release:paths-validate`, `rust:agov:validate`, `rust:tooling:clippy`,
`rust:tooling:deny`, `rust:tooling:test`, `rust:tooling:preview`, and
`tooling:live-surface-validate`; Windows preview jobs were optional/manual.

Resolution condition: derive and state the exact mandatory matrix for the
release diff from current CI rules, separately identify tag-only and optional
jobs, and require every expected job at the exact MR head.

## V211-RP-5 — anonymous post-publication proof is not executable

Check: rule soundness

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: V211-RP-5

Impact: material

Enforcement: blocking

Basis: parent anonymous-byte verification requirement

Finding state: open

Risk disposition: none

`glab release view` is an authenticated metadata view and does not verify
anonymous asset retrieval or downloaded bytes. The implemented operator command
is `agov dist post-publication-check`; CI's `release:verify` is the tag-pipeline
counterpart. Step 7 must name their exact inputs/evidence and make completion
conditional on both the tag pipeline and the independent anonymous download
check.

Resolution condition: specify the actual command, URLs/download directory,
expected provenance/asset/digest checks, and the completion record's exact
identities.

## V211-RP-A1 — “independent agreements” is misleading wording

Check: evidence quality

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Finding: V211-RP-A1

Impact: minor

Enforcement: advisory

Basis: audit clarity

Finding state: open

Risk disposition: none

Per-file, category, and total counts are three reconciliation dimensions, not
three independent reviewers or independent data sources. Rename them so the
control is not overstated.

Resolution condition: call them three-way reconciliation checks and retain all
three stop conditions.

After revision, rerun the contribution and infosec consultations at the new
exact head. The separate infosec-sentinel review remains required; this record
reports mechanical screening evidence only.
