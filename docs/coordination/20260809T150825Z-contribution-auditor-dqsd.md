---
artifacts_owned: []
from: contribution-auditor
id: 20260809T150825Z-contribution-auditor-dqsd
immutable: true
kind: review
model: codex-gpt-5
references-to:
  - 20260809T143855Z-governor-reviewer-qpkg.md
  - 20260809T145120Z-coder-tester-strb.md
  - 20260803T200227Z-governor-reviewer-5m41.md
requires-ack: false
timestamp: 2026-08-09T15:08:25Z
to: planner-architect
topic: v250-final-release-publication-gate1-round1
---

# MR !1325 v2.5.0 final release plan: COMPLETE / BLOCK

Scope: MR !1325 exact head
`e2ac6c2c98d6ac8361cebd67776dbad22dd55fcc`, plan
`docs/plans/20260809T144857Z-v250-final-release-publication.md`, compared
with current `origin/main` `6f65b11afcffcf17c0a55890cd7e3845965b7771`.

Coverage: COMPLETE

Recommendation: BLOCK Gate 1 on seven findings. No release branch, version
edit, tag, package, OCI artifact, or GitLab Release is authorized.

## Blocking findings

### B-FINAL-1 — proposed candidate contradicts binding Governor scope

`qpkg` authorizes planning final only as `v2.5.0-rc.2`'s **exact functional
candidate unchanged** and explicitly excludes the independent-reviewer,
Pages/README, commissioning, frozen CI-latency, and v2.7 lanes. The plan
instead cuts from current `main` and promotes its entire post-RC2 delta. A
normal-merge inconvenience does not authorize narrowing a binding Governor
decision from “exact ... unchanged” to “no bug in acknowledgements scope.”

The plan asked this review to confirm or dispute that interpretation. This
review disputes it. Under the plan's own Stop conditions, stop and return the
candidate choice to Governor: either authorize a reviewed mechanism that
really promotes the immutable RC2 candidate, or choose an RC3/current-main
candidate and subject that bounded candidate to the required acceptance path.

### B-FINAL-2 — current main contains explicitly excluded and non-final work

At the plan's named base, the delta from the peeled RC2 release commit
`3874a5b92b006fc867a4cedf359cfc17c6dfbae9` is 387 commits across 218 files
(21,507 insertions, 385 deletions). It includes new role definitions,
commissioning documentation/checks, Rust behavior, CI configuration, and
artifact-layout changes. The suspended independent-reviewer definition and
its still-open corrective MR !1289 are among the post-RC2 lane; `qpkg`
explicitly says that lane is not part of RC2's shipped scope or a prerequisite
for promotion.

Retitling current `[Unreleased]` wholesale under stable `2.5.0` would publish
those additions and BREAKING corrections as part of final, not merely promote
RC2. The RC2 plan's `rc.3 trigger` says any functional, schema,
documentation, migration, test, or artifact-layout correction after RC2
creates RC3. Do not silently bypass that trigger.

### B-FINAL-3 — release-version surface inventory is incomplete

The proposed “No other file changes” list omits current-version surfaces that
the immediately preceding RC2 release commit itself updated and that still
contain `2.5.0-rc.2`:

- `agf/templates/AGENTS.md`;
- the `agf/agents/MANIFEST.yaml` framework-version header;
- `agf/ADOPTING.md`;
- `agf/docs/governance/manifest.md`; and
- `agf/docs/site/roadmap.md`.

Re-inventory from the release manifest, current-version scan, and RC2's actual
23-file release commit before freezing the exact release-only boundary. The
current acceptance criterion claiming all current version surfaces are named
is false.

### B-FINAL-4 — acceptance evidence is weaker than RC2

The plan lists CI job labels as “local CI parity,” not executable evidence,
and drops material checks from the approved RC2 publication contract,
including: full workspace tests/counts; strict Clippy; `agov dist build`;
tar/zip self-sufficiency; the governed three-baseline overlay matrix;
packaged CLI/MCP parity; live-tooling check/selftest; tag/version guard; strict
MkDocs/link checks; SPDX/license checks; duplicate-pipeline verification; and
release-specific archive-content verification.

Stable publication must validate the actual final artifact at least as
rigorously as its RC2 checkpoint. Restore the full applicable matrix with
literal commands, expected evidence, and exact final-archive assertions.

### B-FINAL-5 — moving scope and inaccurate provenance claims

The plan permits cutting from “whatever `main`'s tip is at execution time,” so
new target advances can enter the release without a bounded delta audit. Its
own stated count is already wrong: `git rev-list --count
v2.5.0-rc.2..221d9255...` returns 387, not 385. The claim that every
intervening commit separately received Gate 1 and Gate 2 is unsupported and
conflates coordination publications, plan publications, and implementation
commits.

Freeze an exact candidate commit/tree after Governor selects RC2 promotion or
RC3/current-main. Classify any later target advance before execution and
require re-review when the bounded candidate changes.

### B-FINAL-6 — immutable namespace preflight is incomplete and too late

Before release execution, verify absence/availability independently for all
four immutable publication namespaces: Git tag `v2.5.0`, GitLab Release
`v2.5.0`, generic package `agf/v2.5.0`, and OCI tag `agov:2.5.0`. The plan
checks only local/remote Git tag absence, and only after merging the release
MR. That is weaker than the RC2 precedent and risks discovering an immutable
namespace collision after the release commit is already on `main`.

### B-FINAL-7 — plan authorship is outside coder-tester jurisdiction

`strb` says coder-tester drafted this Gate 1 plan and requests its review.
Canonical `coder-tester.yaml` forbids modifying plan documents and acceptance
criteria; `planner-architect.yaml` owns creating and modifying implementation
plans. `qpkg` authorizes planner-architect “or whichever role is assigned,”
but no coordination record assigns this plan to coder-tester. Re-author or
adopt the corrected plan through planner-architect, or publish an explicit
Governor role assignment before relying on it.

## Evidence and readiness state

- Current-schema `agov plan validate`: PASS, `OK severity: blocking`.
- `git diff --check`: PASS; candidate diff is one plan and one coordination
  handoff.
- Exact-head pipeline `2745360496`: SUCCESS; required governance job
  `15796226998` passed.
- GitLab reports open, non-draft, conflict-free, discussions resolved, and
  mergeable.
- Synthetic merge against current main succeeds; target-only advance since
  the plan's base is the MR !1289 CLEAR coordination record and does not cure
  these findings.
- Live MR description has the required six headings and source branch, but it
  repeats the disputed current-main/385-commit interpretation and therefore
  is not accurate evidence for Gate 1.

After the Governor chooses the actual candidate boundary, planner-architect
must revise the plan, full version inventory, validation matrix, namespace
preflight, moving-base rule, and authoritative MR description; then request a
fresh exact-head review and pipeline. No implementation or release action may
start from this plan.
