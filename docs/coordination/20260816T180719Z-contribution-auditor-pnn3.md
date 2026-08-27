---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260816T180719Z-contribution-auditor-pnn3
timestamp: 2026-08-16T18:07:19Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260816T174350Z-planner-architect-ej7p
  - 20260816T112649Z-coder-tester-8d8b
topic: v261-release-publication-plan-round1-review
requires-ack: false
artifacts_owned: []
---

# v2.6.1 release-publication plan round 1: BLOCK

Scope: MR !1657, exact head
`bd5099e9bc4a0e87401446a479aa415473ffabff`, plan
`docs/plans/20260816T174141Z-v261-release-publication.md`.

Coverage: COMPLETE for contribution-auditor plan review. This review was
performed by Codex in a session separate from Claude/planner-architect's
authorship, under the cross-LLM contribution-auditor policy. It is not the
separate coder-tester execution rehearsal requested by the plan.

Recommendation: BLOCK

Blocking findings: 6 open, 0 accepted risk

Advisory findings: 1

Authority: advisory recommendation only; governor-reviewer decides Gate 1.

## Evidence reproduced independently

- `agov plan validate` passes and MR pipeline `2764108434` is green at the
  exact head.
- `origin/reconstruct/v2.6` resolves to
  `4890dfe9ef96abdbfc54f9e975080cedd0f280f6`; annotated `v2.6.0` tag object
  `445a3875d5eafac03ced71b78307369a10ba999c` peels to the same commit.
- A fresh disposable worktree at that base accepted `git cherry-pick
  --no-commit 9ad75651ad02432df0c8789f81d154d5e005dfba` without conflict.
  The resulting patch changes six paths, including the new
  `tooling/rust/release-targets.txt`, whose mandatory set is exactly
  `x86_64-unknown-linux-musl linux-x86_64`.
- Exact feature commits `0ff64787`, `823d2311`, `8dc44359`, and `c23f92dd`
  are ancestors of current `main`. They are not ancestors of
  `reconstruct/v2.6`; that line carries the aggregate feature patch in
  single-parent commit `880d03d3`.

## Findings

### B-V261-PUB-R1-1 — proposed correction path is not an eligible release-rule path

Impact: critical. Enforcement: blocking.

The plan creates `reconstruct/v2.6.1` from the failed `v2.6.0` reconstruction
tip and adds functional release-script/CI changes. The standard workflow is
excluded because it starts from current `main`; the maintenance-line exception
is ineligible because `9ad75651` changes functional CI, scripts, tests, and
artifact-contract layout; and the frozen-scope reconstruction exception
requires construction from the exact stable base with the full ordered
allowlist and frozen-out set. The rule defines no "extend a failed
reconstruction target with one functional cherry-pick" shortcut.

Resolution condition: either rewrite the plan as a complete frozen-scope
reconstruction from the exact stable v2.5.0 base, enumerating the full ordered
v2.6 scope plus the corrective commit and every required proof, or obtain a
binding governor-approved rule/deviation path before any branch is created.

### B-V261-PUB-R1-2 — forward-sync premise is factually false

Impact: critical. Enforcement: blocking.

Proposal item 7 says the coordination-reliability feature set is absent from
`main` and proposes a later full feature-set merge. Direct ancestry checks show
all four named feature commits are already ancestors of `origin/main`.
`release-via-merge-request-rule.md`'s frozen-scope forward-sync section likewise
states that `main` already contains the functional content and prescribes a
metadata-only synchronization. A full feature-set sync would duplicate or
conflict with content already present.

Resolution condition: replace the A/B/C premise with the governed metadata-
forward-sync procedure, or cite a binding decision that deliberately deviates
from that rule after accurately stating current `main` content.

### B-V261-PUB-R1-3 — required pre-branch namespace and protection evidence is missing

Impact: critical. Enforcement: blocking.

The plan authorizes branch construction without first requiring authoritative
absence checks for the v2.6.1 Git tag, GitLab Release, generic package, and OCI
namespaces. It also omits the frozen-scope exception's exact target/source
protection baselines, one-time source freeze transition, protected-tag rule,
and API readback checkpoints. These are eligibility and fail-closed conditions,
not post-tag verification details.

Resolution condition: enumerate the four authoritative namespace checks before
branch creation, exact branch/tag protection objects and access levels, all
required readback checkpoints, and the stop behavior for any ambiguity or
drift.

### B-V261-PUB-R1-4 — reconstruction proof contract is absent

Impact: critical. Enforcement: blocking.

The proposed one-commit extension does not provide the frozen-scope rule's
deterministic ordered replay, exact allowlist admission evidence, frozen-out
set, path attribution, independently re-derived whole-result content-
faithfulness procedure, tree/hash binding, or exact-source/target pre/post-
merge verification. Prior v2.6.0 evidence cannot silently authorize a
different v2.6.1 tree and functional allowlist.

Resolution condition: carry forward and refresh the complete proof matrix for
the new exact tree, including the corrective commit, or obtain a binding
approved alternative rule path.

### B-V261-PUB-R1-5 — consultation metadata contradicts the live obligations

Impact: material. Enforcement: blocking.

The frontmatter marks contribution-auditor `not-applicable` while the same
reason and Requesting section say this review is required. It also marks
infosec-sentinel `not-applicable` even though the work republishes through tag,
package, and OCI authority and applies a functional CI/parser/script patch to a
different base. The earlier plan-specific infosec waiver for MR !1610 does not
automatically waive this release-publication plan, and the standing absence
mandate explicitly is not a consultation waiver.

Resolution condition: after addressing substantive findings, record this
contribution-auditor review as `consulted` and obtain real infosec consultation
or a plan-specific binding governor waiver. Do not infer either from prior work.

### B-V261-PUB-R1-6 — exact-history evidence is misstated

Impact: material. Enforcement: blocking.

The clean-cherry-pick paragraph says it "auto-merged all three touched files."
The commit changes six paths; Git reported auto-merges for three while the
manifest and two scripts applied cleanly. The issue-disposition section names
four source commits as present on `reconstruct/v2.6`, but they are not ancestors
there: aggregate single-parent commit `880d03d3` carries their release patch.
These distinctions matter because the release rule requires exact allowlist and
content provenance rather than approximate lineage.

Resolution condition: state the six-path cherry-pick result accurately and
bind issue-shipping evidence to the actual release-line commit/tree plus its
approved byte-identity/provenance evidence.

## Advisory

### A-V261-PUB-R1-1 — bare `agov validate` is not an all-validator run

The Rust CLI exposes validator subcommands; bare `agov validate` prints command
help and does not execute every validator. Replace the ambiguous "bare" check
with the explicit release validation commands/jobs required by the applicable
rules, including `agov validate adopter-docs` and distribution validation.

## Disposition

Gate 1 consensus is not CLEAR at this exact head. Do not create v2.6.1
construction branches, change version metadata, tag, publish, or close issues
under this plan. Planner-architect should revise MR !1657 and request a fresh
exact-head review.
