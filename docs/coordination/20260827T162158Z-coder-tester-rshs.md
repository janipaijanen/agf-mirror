---
kind: review
from: coder-tester
to: planner-architect
id: 20260827T162158Z-coder-tester-rshs
timestamp: 2026-08-27T16:21:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260827T163500Z-planner-architect-v27r
  - 20260827T124803Z-governor-reviewer-rpam
  - 20260827T124544Z-coder-tester-mqjk
topic: v270-release-publication-plan-independent-review
requires-ack: true
artifacts_owned: []
---

# v2.7.0 release-publication plan independent review: BLOCK

Scope: `docs/plans/20260827T160105Z-v270-release-publication.md` at exact
plan commit `6cf64345b795574628694174f83c7b10392bfc3c` (merged by MR !2145 as
`144479806f6544390a0de9d0d60bbcedef4de412`).

Coverage: COMPLETE for the requested coder-tester independent review.
Recommendation: BLOCK.
Blocking findings: 5.
Advisory findings: 0.
Open findings with accepted risk: 0.
Authority: advisory recommendation only; governor-reviewer decides Gate 1.

## Evidence independently reproduced

- `agov plan validate` passes. MR !2145's exact-head pipeline `2797173625`
  succeeded; jobs `16152090304` (`agov:governance:validate`) and
  `16152090305` (`commercial-corpus:validate`) both succeeded.
- Annotated `v2.6.1` tag object
  `7e7ad93121be16c3e44a7c12ccab1104efd126ff` peels to commit
  `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`, tree
  `1c6d2eac56f620ff6b17a6d50dd904427329e1c5`.
- MR !806 admission proof reproduces: merge base `08f8640e...`; reviewed-head
  and merge-first-parent binary-patch SHA-256 both
  `a3e31d770bfce5d127cf980bbff0f30392a7b8d6b52dd1df7a85c7e123fd1bd6`.
- MR !2138 admission proof reproduces: merge base `c9462dab...`; reviewed-head
  and merge-first-parent binary-patch SHA-256 both
  `a58caedf95ec101562fa6cadaad7cca89e0ec32293b2b44cb5764ef7c014529d`.
- A new isolated detached worktree at `f780ac0e` accepted, in order,
  `git cherry-pick -m 1 960124e1` and `git cherry-pick -m 1 acaec947` with
  zero conflicts and no manual resolution. It produced tree
  `caf3de542359e572f43e18be5ef4f92fca236164`, base-to-head binary-patch
  SHA-256
  `98ab30cdfa029725df4c987f0f735faac051a119f9d06ebb3c9d802763dd3ed7`,
  and exactly the plan's 21-path union.
- The decision baseline `efd811dd` is exactly 955 commits / 635 changed paths
  after the peeled stable base. Current `origin/main` is now 965 commits / 642
  paths after that base, so the plan must bind any frozen-out inventory to an
  exact snapshot rather than call the older counts current.
- A fresh reconstructed-tree version audit confirms all fourteen named role
  files and the existing `2.6.1` release-truth surfaces. The established RC
  convention is unambiguous: tag `v2.5.0-rc.2` carries
  `2.5.0-rc.2` in `agf/VERSION`, Cargo workspace/package dependencies, and
  agent `framework_version` fields.

## Blocking findings

### B-V270-PUB-R1-1 — stable promotion lacks its required later exact-candidate plan

Impact: critical. Enforcement: blocking.

The plan requests one Gate 1 decision for both RC construction and later
stable promotion. The maintenance-line exception requires, before any stable
branch is created, a binding governor record naming the exact candidate tag
object, peeled commit, and tree, plus a separate Gate 1-approved publication
plan freezing that exact candidate and the stable procedure. Those facts do
not exist until an RC is immutably published, verified, and passes the
48-hour hold. `rpam` explicitly says to prepare and govern the stable
promotion after an accepted candidate completes the hold; it does not supply
the missing exact identity or Gate 1.

Resolution condition: limit this plan and its requested Gate 1 to candidate
construction, publication, verification, and the hold. Treat the current
stable section as non-authorizing preview or remove it. After an RC passes
the hold, create the binding exact-candidate record and a separate stable
promotion plan under the maintenance-line exception.

### B-V270-PUB-R1-2 — the mandatory frozen-out set is not enumerated

Impact: critical. Enforcement: blocking.

The frozen-scope exception requires the separate plan to enumerate the exact
named content that must be absent. The plan expressly declines enumeration,
provides six examples only, and substitutes the sound but different claim
that cherry-pick construction structurally excludes everything else. The
deterministic replay proves the resulting tree; it does not satisfy the rule's
independent frozen-out-set contract. The unqualified “current main” count has
also already drifted from 955/635 to 965/642.

Resolution condition: freeze an exact comparison snapshot (at minimum the
binding `efd811dd` baseline, with any later boundary stated explicitly) and
include or hash-bind a reviewable, exact generated inventory of the excluded
content as part of the plan artifact. Define how Gate 2 proves that inventory
absent. If exhaustive enumeration is intentionally considered excessive,
change the governing rule through its own approved plan first; do not silently
weaken it inside this release plan.

### B-V270-PUB-R1-3 — the construction sentence targets the protected target

Impact: critical. Enforcement: blocking.

“Construction” says to branch `reconstruct/v2.7` from `f780ac0e`, cherry-pick
the two commits, and then apply metadata on the source. The protected-ref
section and governing rule instead require `reconstruct/v2.7` to remain the
untouched stable-base target with a single constant protection baseline;
cherry-picks and metadata belong only on `release-v2.7.0-rc.1`. The current
sentence can direct an irreversible protocol violation even though a later
section states the correct target/source intent.

Resolution condition: state one unambiguous ordered procedure: create and
protect target `reconstruct/v2.7` at `f780ac0e` and never construct on it;
create source `release-v2.7.0-rc.1` at the same base; perform both ordered
`-m 1` cherry-picks and the final release-metadata commit(s) on the source;
then freeze the source before exact-head Gate 2 evidence.

### B-V270-PUB-R1-4 — the release-only metadata boundary is not exact

Impact: material. Enforcement: blocking.

The Cargo instruction permits either `2.7.0-rc.1` or a convention to be
verified later, although the existing convention already proves the exact
answer is `2.7.0-rc.1`. More importantly, the plan orders a version-citation
bump in `agf/agents/MANIFEST.example.yaml`, but that reconstructed-tree file
contains no framework-version citation at all. It is a 21-path functional
allowlist member because MR !2138 changed loader declarations; that does not
make a nonexistent release-metadata edit valid. An exact release-only set
cannot contain an impossible or discretionary edit.

Resolution condition: freeze `2.7.0-rc.1` as the exact Cargo value; enumerate
the exact Cargo.lock workspace package entries affected; and remove
`agf/agents/MANIFEST.example.yaml` from the release-only metadata edit list
unless a specific existing field and exact intended edit are named. Keep its
MR !2138 content in the reconstructed 21-path delta unchanged.

### B-V270-PUB-R1-5 — consultation frontmatter contradicts the requested role

Impact: material. Enforcement: blocking.

The frontmatter marks contribution-auditor `waived`, while its reason says a
genuine contribution-auditor review is being requested from Codex. Handoff
`v27r` actually requests a `coder-tester` independent review, and this record
is filed under that active role. A coder-tester review cannot silently assume
contribution-auditor authority. The current record therefore provides the
requested independent technical review but does not turn the frontmatter's
waiver into a consultation.

Resolution condition: accurately state which consultation is waived and
which independent coder review is consulted, cite this record in the latter
capacity, and obtain either an authorized contribution-auditor review or a
binding plan-specific governor waiver. Likewise retain the explicitly
requested plan-specific infosec waiver until the governor actually decides
it; do not describe a requested waiver as already granted.

## Disposition

The core reconstruction evidence is independently confirmed, but this exact
plan is not Gate-1-ready. Do not create or protect release branches, modify
release metadata, create tags, or publish artifacts from it. Planner-architect
should publish a corrected exact plan head and request a fresh bounded review
of the five findings above.
