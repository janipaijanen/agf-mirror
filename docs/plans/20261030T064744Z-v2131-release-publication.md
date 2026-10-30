---
id: 20261030T064744Z-v2131-release-publication
status: approved
gate1_approved_by: 20261030T075341Z-governor-reviewer-bmaq
gate1_approved_at: "2026-10-30T07:53:41Z"
topic: v2131-standard-main-release-publication
candidate_release: v2.13.1
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: Final independent contribution audit cleared the exact freeze, estate, matrix and bounded sequence; bmaq records the evidence.
  - role: infosec-sentinel
    status: consulted
    reason: Final independent security review cleared the exact plan and fresh publisher/header evidence; bmaq supplies fresh bounded dispositions, with final metadata-head rechecks required.
affected_paths:
  - AGENTS.md
  - CHANGELOG.md
  - agf/ADOPTING.md
  - agf/CHANGELOG.md
  - agf/VERSION
  - agf/agents/MANIFEST.yaml
  - agf/agents/academic-peer-reviewer.yaml
  - agf/agents/academic-visual-reviewer.yaml
  - agf/agents/build-engineer.yaml
  - agf/agents/capability-developer.yaml
  - agf/agents/coder-tester.yaml
  - agf/agents/contribution-auditor.yaml
  - agf/agents/deployment-engineer.yaml
  - agf/agents/documentation-estate-steward.yaml
  - agf/agents/documentation-writer.yaml
  - agf/agents/fact-checker.yaml
  - agf/agents/governor-reviewer.yaml
  - agf/agents/improvement-propeller.yaml
  - agf/agents/independent-implementation-reviewer.yaml
  - agf/agents/infosec-sentinel.yaml
  - agf/agents/planner-architect.yaml
  - agf/agents/web-ux-reviewer.yaml
  - agf/docs/governance/governance.md
  - agf/docs/governance/manifest.md
  - agf/docs/site/roadmap.md
  - agf/learnings/foundry.yaml
  - agf/templates/AGENTS.md
  - agf/templates/learning-record.yaml
  - docs/plans/20261030T064744Z-v2131-release-publication.md
  - docs/reviews/documentation-estate/20261030-v2131-release-content-freeze.md
  - docs/reviews/next-release-freeze-evidence/20261030-v2131-apk-residual-risk.md
  - docs/reviews/rule-projection/20261030T064744Z-v2131-release-publication.yaml
  - docs/roadmap.md
  - tooling/rust/Cargo.lock
  - tooling/rust/Cargo.toml
depends_on:
  - 20261030T054200Z-pages-reader-journey-patch
  - 20261030T072047Z-v2131-readme-journey-followup
  - 20261029T220500Z-v213-release-publication
  - 20261029T204751Z-v213-release-freeze-preparation
  - 20261029T192922Z-release-publisher-api-amendment
  - 20261028T194451Z-ci-reproducibility-pins
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260916T002012Z-roadmap-refresh-post-v2101-issue130
  - 20261021T220000Z-retire-windows-support
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T045214Z-remove-elixir-references
  - 20261023T080431Z-instruction-context-followup
  - 20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20261025T192020Z-supersedes-adopter-docs
  - 20261027T045944Z-record-only-publication-termination
  - 20261027T230355Z-v212-release-freeze-corrections
  - 20261027T235807Z-v212-release-publication
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Publish AGF v2.13.1 through the standard main-branch path

**Gate 1 approved by bmaq.** Publish this approved plan and binding decision
before implementation. The exact product freeze and release-specific
conditions are recorded below. The audit-inclusive construction checkpoint,
metadata Gate 2, protected-tag/namespace checks and publication evidence remain
separate future obligations; this approval grants no metadata merge or tag.

## Candidate and freeze

The reviewed Pages portion is MR !3394 at
`6fc3e6de1bd52e299a0fec8dd3127f56ad4b0e7c`, merged without squash as
`bb407c35969ccb251a32f5fe32179e58600223a7`. Its actual first parent is
`ec1022d4ac8b6ea110008959b3289c2269785672`, second parent is that reviewed
head, and tree is `7b694db43c8272660ef50e6826b4eacafb1be965`. The exact
12-path delta is the nine explanatory pages, MkDocs navigation, narrowly amended
CTA guard and projection manifest under pksk/gnqj. Gate 2 p8ms was published
unchanged at `280384a0d8472c5a2ee8c75d8c7cb9594b5315a4` on the approved
shared plan branch before the handoff and implementation merges. MR pipeline
3007344107 passed all seven required jobs. Main pipeline 3007394646 passed all
seven required jobs, including Pages job 17599605482, at 07:10:39Z.

Fresh anonymous reads at 07:16:59Z verified Home, documentation index,
Commission, Should you adopt and Release information at source `bb407c35969c`:
HTTP 200, six navigation groups, the corrected homepage headline, both exact
CTA destinations, and removal of the obsolete commissioning command. The live
site correctly remains development documentation after v2.13.0. Evidence:
`/home/dev/s/tmp/codex-pages-live-bb407c35.json`. This completes the prior
Pages deployment hold, not a v2.13.1 artifact publication.

**Product-content freeze approved by bmaq:**
`93e203466fbb4fdc50f926a7b7c7490bfbb0fdc9`, tree
`e55f8d90a6ad481c7cc4a6d85b17a50494134d38`. This is the verified !3397
merge: first parent `bb407c35969ccb251a32f5fe32179e58600223a7`, second
parent `7b79aab16c8e4ebe92629c1129e8611e4aeb6b29`. Child plan
`20261030T072047Z-v2131-readme-journey-followup` has Gate 1 8p5m and
explicit overlap reapproval gs85. Required exact-head pipeline 3007494056
passed all three jobs. Binding Gate 2 ksm2 was shared unchanged at
`a7391274fc3856b1298b9a8e851767cbe8be6080` before the 07:43:22Z merge.
Actual parents, tree and six-path delta were verified: only README's two
paragraphs, its plan/projection and 8p5m/km7d/gs85. Both independent reviewers
cleared the exact corrected head. README now routes the full prompt to
Commission and limits coexistence to the proposed/Theoretical pattern.

The independently prepared exact-source addendum closes F1/F2:
`/tmp/agf-v2131-estate-93e20346-addendum.md`, SHA-256
`6ee2110eeca1c3e9a64a6e2b60c7ce7e7269bc50d0c3dc3cc3a7b3f750ea0fec`.
Its fresh inventory classifies 357 paths, 43 projection manifests and eleven
post-release first-parent commits. The 42 prior whole-tree projection results
plus the independently checked child manifest are retained; no new 43-manifest
whole-tree run is falsely claimed. A/B/C remains 36/11/62 occurrences across
27/3/18 files. Inventory SHA-256
`440f201ce8f79d178f92fe4a70d3d023e86b5c1c31fb655afa33902d0aae91b8`;
A/B/C inventory SHA-256
`2c95d2874530496ee383784f71e7607a1335f21c21983ef94f9ad3944099662e`;
source-continuity JSON SHA-256
`5816f7e3dfa3097ef85f43c7472c266c4603c088040b909d91bdd3657101f3cb`.
All eight publisher inputs and B-status surfaces remain unchanged. Main
pipeline 3007506312 passed all three required governance/commercial jobs after
the child merge; it did not deploy Pages. The already-verified live Pages bytes
remain unchanged by this README-only product delta.

The user's uncommitted root README was untouched. The metadata MR excludes
README and remains 29 product paths. Only the Pages slice plus this README
projection correction are new product content; no command, API, rule, schema,
dependency or platform change. Both final independent consultations are clear; bmaq binds this product freeze
and the stated dispositions.

The ten-commit post-v2.13.0 crosswalk through bb407c35 is in the estate inventory;
!3397 is the sole subsequent main advance, explicitly classified above. The
later construction checkpoint may admit only the specifically authorized
plan/decision/report additions below, with exact identities and unchanged product
bytes. Any other main advance returns to release-content reconciliation.

The complete read-only estate assessment at `bb407c35` is
`/tmp/agf-v2131-estate-bb407c35-assessment.md`, SHA-256
`4f322c08a93bc0fb895c4b8ae4b376db717784421be96d3fb9cfda4ca49193d1`.
It classifies 356 paths and ten post-v2.13.0 first-parent commits, checks 42
projection manifests, and records F1/F2 open at its pre-child cutoff. The frozen
report must add the actual child resolution/source rather than relabel this
historical assessment. Fresh A/B/C at bb407c35 is A36/27files, B11/3,
C62/18; the extra C occurrence/file is audit history. Refresh after the merge.

Explicit release-owner dispositions approved by bmaq for inherited findings: preserve six historical learning-ID warnings and two contribution-text
heuristic warnings as disclosed advisory evidence, without rewriting history or
endorsing the flagged heuristic; defer seven unchanged local assistant-loader
projection warnings to separately governed repair, without calling the copies
canonical; defer two unchanged README count warnings (51 versus 52 rules,
16 versus 17 agent definitions), without claiming those counts are current.
The child excludes those count edits; this patch fixes its identified
setup/compatibility regressions and does not claim a warning-free estate.
`framework_config` remains explicitly skipped, not passed. Header checklist
and mutable-APK conditions stay separate below. No finding is implicitly closed.

The additional historical plan dependencies acknowledge the concrete overlaps
reported by the changed-plan collision check. This release touches only the
named current-version fields, new changelog entry and later verified-status
cells on their shared paths. It preserves those plans' rule/agent/loader bodies,
role status, roadmap feature dispositions and historical records; it does not
reimplement, activate, supersede or declare completion of their work. In
particular, MANIFEST/agent version metadata cannot activate work-item-steward or
change any other role. Claims and current open MRs are rechecked before editing;
a live overlapping mutation outside these cells requires fresh coordination.
The 204 older plans without affected_paths remain mechanically unassessed, not
presumed closed. Metadata-only limits and an independent estate review supplement
that coverage gap rather than claiming the collision command proved closure.

Standard topology is mandatory: branch `release-2.13.1` from reviewed `main`,
metadata-only MR back to `main`, then annotate the actual approved merge.
No maintenance or reconstruction exception applies. The canonical owner is
`agf/rules/workflow/release-via-merge-request-rule.md`; the release skill is
`agf/skills/release/SKILL.md`. Use actual foundry paths below rather than its
unconfigured generic VERSION/README examples.

The user's existing full governor delegation ss2k expires
**2026-10-30T18:00:00Z**. Binding gates and risk decisions must be recorded within
that authority; this plan does not extend it or require another user permission
round for already delegated decisions. Root owns integration/publication;
existing independent sessions supply review. No extra sessions or timers.

## Preconditions requiring fresh explicit disposition

### Three inherited SPDX-header omissions

`CHANGELOG.md`, `agf/CHANGELOG.md`, and `agf/templates/learning-record.yaml`
lack authored-file SPDX headers. `dta3` accepted this only for its exact v2.13.0
metadata merge; `tgrk` is a repair proposal, not approval or completed repair.
The Pages plan deliberately excludes header work. A successful validator is not
proof that the release owner's all-file SPDX checklist is fully satisfied.

Chosen disposition in bmaq for this Pages/README release:
a **fresh explicit v2.13.1-only checklist exception**, naming these exact
three files, their observed header state, the unchanged path-based CC-BY-SA-4.0
license and absence of a generated-file exclusion. Require independent
licensing/security review and a new binding governor decision before metadata
Gate 2. It must say the all-file header checklist is not fully satisfied, carry
an exact candidate/time boundary, grant no general exception or header-edit
permission, and keep tgrk's separate repair proposal tracked. bmaq supplies the fresh conditional disposition; final metadata Gate 2 binds
and rechecks the actual candidate. dta3 supplies no authority for this release. No known third-party/license conflict is
waived by accepting these three historical omissions.

The alternative, if the governor chooses it, is a separate three-file
header-only Gate 1 plan following tgrk, then independent review and Gate 2 before
freezing. That would need explicit release-scope reconciliation and a named
plan dependency. Do not silently implement repairs in the Pages MR or metadata
MR. Current recommendation preserves the Pages slice and its separately approved
two-paragraph README correction
and leaves repair separate; it does not declare the files header-compliant.

### Mutable APK publisher residual

`ja28` is expressly limited to v2.13.0 and is not reusable for v2.13.1, even
though its time window has not expired. Confirm the actual publisher/CI bytes
remain the reviewed `alpine:3.24.3` linux/amd64 manifest
`sha256:c26612752c78b18faacd63c7d50f4592c2e7b0f25e0e20e82bd74e465c57ab0d`,
`apk add --no-cache curl jq unzip`, then `tar gzip`, and the create-only helper
`tooling/rust/scripts/create-release.sh`. Record exact file hashes and diff
against v2.13.0. The successful v2.13.0 publication/q5r4 is historical production
evidence, not proof about the next mutable install or future API outcome.

Pre-Gate-1 read-only diagnostics may replay that exact install in disposable
credential-free containers, scan the base and actual post-install filesystems
using a verified scanner image and fresh dated database, and record tool/CA and
repository inventories under an owned temporary directory. These are assessment
artifacts, not product implementation or authority to publish tracked reports. No project mounts, forwarded
credentials/environment, custom shipped builder image, dependency/CI edits or
production POST. Retain report hashes, scanner/package/database/time/architecture
scope, every severity of finding and exclusions. Preserve full-severity scan
JSON; HIGH/CRITICAL-zero does not waive any other known finding. The old
2026-10-29 HIGH/CRITICAL-zero reports
remain immutable historical evidence; do not relabel them as today's scan.

Before metadata Gate 2, require independent review and a fresh governor record
accepting only the observed mutable-package/tool/CA and availability residual
for this exact v2.13.1 cut, with an explicit expiry no later than the delegation.
Bind that decision to the sanitized report hash, scanner image digest/version,
DB timestamp, exact base digest/architecture, installed package/tool/CA and
repository inventory, and frozen `.gitlab-ci.yml`/`create-release.sh` hashes.
The acceptance must remain active until the entire release:create job reaches
terminal success, including both APK installs, every package upload, Release
POST and follow-up GET/response verification; a pre-expiry tag or POST alone
does not satisfy that condition.
No known-vulnerability waiver is implied: any new finding requires remediation
or its own evidence-based disposition; no failure is suppressed. A material
base/package/client/control change returns to a separately approved scope.

### Current independent assessment (not acceptance)

Independent security review cleared the exact diagnostic publisher report
`/tmp/agf-v2131-publisher-refresh-20261030/report.md`, SHA-256
`7f2ad87905d77fb41e1049114bac0cef68ec2836c29361ecd8ebc63e533ead41`,
and evidence manifest `fc8ce119707cb374d99cdcd3f0019d12d30c8cf13e479f8559684a822769a141`.
Trivy 0.76.1 with DB updated 2026-10-30T01:01:11Z and downloaded 07:03:01Z
reported zero findings at every severity in 16 base / 34 runtime packages.
All eight publisher/CI inputs match v2.13.0 at the Pages baseline; the report
records exact scanner/base identities, package/tool/CA/repository inventories
and replay limits. Its corrected wording distinguishes the single recorded
`command -v` path from successful tool version executions. No rescan was needed.
The independent recommendation supports only the fresh bounded dispositions
above, rechecked against the eventual frozen source and final metadata candidate.
It grants no risk acceptance or gate approval. The child merge and exact-source addendum resolve the README findings; final exact-plan consultation is clear and bmaq supplies the binding decision.

## Evidence publication and construction ordering

Before Gate 1, inspection, read-only replay/scanning, an uncommitted estate
assessment and version inventory may be prepared under `/tmp`. No tracked report
is authored or committed by that diagnostic permission. Gate 1 reviews these
concrete assessments and pins the product-content freeze to the verified Pages
implementation merge, while explicitly authorizing the next audit-only step.

After Gate 1, use branch `evidence/v2131-release-freeze` for exactly these three
already-enumerated audit artifacts: the documentation-estate freeze report,
the APK residual-risk report and the release rule-projection manifest. The
projection manifest accurately labels planned versus completed status work;
it must not claim the later B-cell synchronization already happened. Obtain
independent review, required CI and ordinary Gate 2 for this evidence MR. The
reports must state their actual observed product baseline and limitations.

Merge that audit-only MR before constructing `release-2.13.1`. At the final
construction checkpoint, enumerate the exact intervening plan, approval and
report commits/paths since the pinned product freeze, verify their authority and
actual trees, and prove no additional product bytes changed. Re-run the A/B/C
inventory and record the actual current main SHA in a binding governor
construction decision before starting the metadata branch. Only the specifically
named plan/decision/report additions are eligible for that reconciliation;
unrelated work requires a new freeze. This separates product-content freeze
from the later audit-inclusive construction base without a Gate 1/report cycle.

The metadata MR remains exactly 29 product metadata paths (27 A paths and two
changelogs); it must not absorb audit reports or header repairs. Later B-cell
status synchronization remains on its own branch/MR after verified publication.
A required new substantive report path or changed product baseline needs a plan
amendment before mutation, not an implicit allowance.

## Exact release-truth inventory

At verified product baseline 93e20346, the checked literal inventory has **36 current-version
occurrences on 36 lines across 27 files (A)**; **11 latest-published occurrences
on 9 lines across 3 files (B)**; and **62 historical/external occurrences on 61
lines across 18 files (C)**. Counts are observations, not a replacement for the
named field boundaries. Both label and URL contain the version on two B lines.
Re-run and reconcile at the actual freeze; classify new history rather than
blanket-replacing every `2.13.0`.

A changes to `2.13.1` in the metadata MR only:

| Path | Exact field / observed lines | Count |
| --- | --- | --- |
| `AGENTS.md` | 4 | 1 |
| `agf/ADOPTING.md` | 333 | 1 |
| `agf/VERSION` | 1 | 1 |
| `agf/agents/MANIFEST.yaml` | 11 | 1 |
| `agf/agents/academic-peer-reviewer.yaml` | 17 | 1 |
| `agf/agents/academic-visual-reviewer.yaml` | 17 | 1 |
| `agf/agents/build-engineer.yaml` | 36 | 1 |
| `agf/agents/capability-developer.yaml` | 36 | 1 |
| `agf/agents/coder-tester.yaml` | 15 | 1 |
| `agf/agents/contribution-auditor.yaml` | 29 | 1 |
| `agf/agents/deployment-engineer.yaml` | 15 | 1 |
| `agf/agents/documentation-estate-steward.yaml` | 42 | 1 |
| `agf/agents/documentation-writer.yaml` | 15 | 1 |
| `agf/agents/fact-checker.yaml` | 28 | 1 |
| `agf/agents/governor-reviewer.yaml` | 15 | 1 |
| `agf/agents/improvement-propeller.yaml` | 8 | 1 |
| `agf/agents/independent-implementation-reviewer.yaml` | 34 | 1 |
| `agf/agents/infosec-sentinel.yaml` | 26 | 1 |
| `agf/agents/planner-architect.yaml` | 15 | 1 |
| `agf/agents/web-ux-reviewer.yaml` | 33 | 1 |
| `agf/docs/governance/governance.md` | 5 | 1 |
| `agf/docs/governance/manifest.md` | 11, 38, 76 | 3 |
| `agf/learnings/foundry.yaml` | 38 | 1 |
| `agf/templates/AGENTS.md` | 3 | 1 |
| `agf/templates/learning-record.yaml` | 8 | 1 |
| `tooling/rust/Cargo.lock` | 13, 36, 48, 61, 72 | 5 |
| `tooling/rust/Cargo.toml` | 14, 20, 21, 22 | 4 |

The 17 agent-definition fields, MANIFEST comment, root/template loader comments,
live foundry adoption field, template/example versions and governance current
content fields are metadata only. Do not change role status or played_by,
assistant behavior, rule bodies or historical learning entries.

Cargo.toml changes its workspace version and three internal dependency versions.
Regenerate Cargo.lock through Cargo, allowing exactly the five workspace package
version changes (agov, agov-core, agov-coord, agov-validators, agov-mcp). The sixth
literal `2.13.0` in Cargo.lock is **external bitflags** at observation line 194:
leave it and all external dependency versions/checksums byte-identical. Any other
lock change stops for diagnosis, not an automatic dependency update.

The release MR has 29 product metadata paths: these 27 A paths plus the two
changelogs. Do not alter README (including the user's root-checkout edits),
`.claude/CLAUDE.md`'s existing pointer, derived distribution manifests, packaged
binary inputs other than their version metadata, older releases or historical
coordination/plan/review records. The proposed evidence/plan/projection paths in
front matter are audit artifacts, not extra feature scope.

B stays unchanged during metadata preparation/merge:

| File | Latest-published fields preserved until verified publication |
| --- | --- |
| `agf/docs/governance/manifest.md` | Current verified-release paragraph (observed line 18), tagged date on line 19 |
| `agf/docs/site/roadmap.md` | Status, Observed at, annotated tag object/peeled baseline, current-release paragraph/date/job count/verification evidence and Release link (observed lines 15–27) |
| `docs/roadmap.md` | Corresponding status/time/tag/paragraph/date/job count/new completion-record citation/Release link (observed lines 10–25) |

A separate post-publication status-sync MR updates only these live B and
companion cells to actual v2.13.1 evidence. Preserve unrelated roadmap rows,
Improvement Propeller maturity, historical release sections and earlier records.
The literal count is not the full semantic status-sync boundary. Do not copy
v2.13.0's 23-job count or q5r4 citation as evidence for the new release.

## Changelog text proposed for review

Both current Unreleased sections are empty at the observed base. Leave an empty
Unreleased section; add a new v2.13.1 section dated at the actual UTC cut. Do not
reinterpret or rewrite the existing v2.13.0 release body/cross-reference.

Proposed adopter entry (final wording reviewed at metadata Gate 2):

> **Clearer documentation setup path.** Home and the documentation index use
> Decide → Set up → First governed change → Operate, with six navigation groups
> preserving all 46 destinations and existing links. Setup separates framework
> files from optional agov tooling, retains collision/checksum/provenance checks,
> uses general validation and explains manual review. Loader/runtime and
> third-party-tooling guidance now match their canonical sources and dated
> evidence limits. No command, governance authority, platform or dependency
> changes. Adopter docs: updated (`index.md`, `docs/index.md`, `use-cases.md`,
> `docs/start/should-you-adopt.md`, `docs/start/commission.md`,
> `docs/start/adopter-faq.md`, `docs/adopt/install-and-layouts.md`,
> `docs/adopt/third-party-plugin-coexistence.md`, `docs/concepts/position.md`).

Proposed repository entry: identify !3394's verified merge and the separately
verified !3397 two-paragraph README projection correction, the two independently
checked CTA destinations and negative controls, unchanged CSS/feature-summary
contract, and exact validation evidence. Link the adopter section rather than
invent a second scope. Do not mark unrelated issues closed or promise conversion,
performance, compatibility or independent-adopter benefits. Report actual test
counts/ignores after validation; no copied count is a new pass.

## Separate gates and publication controls

1. **Before Gate 1:** replace pending freeze/merge/plan-review fields; reconcile
   main since v2.13.0 (the previous three status-sync paths are historical
   publication truth, not new feature scope); review current claims/open MRs,
   collisions, the read-only estate assessment and exact candidate pipeline/Pages
   result. Pin the product freeze and approve the audit-carrier sequence above.
   Publish the approved plan and binding governor record before implementation.
   Fresh header and APK dispositions must be independently reviewed and recorded
   before final metadata Gate 2; this plan itself accepts no residual risk.
2. **Gate 1 scope:** approve candidate patch, enumerated metadata/changelog edits,
   bounded evidence refresh, later B-only status synchronization and the existing
   automatic Pages side effects described below. It may also authorize creation
   of one exact `v2.13.1` protected-tag rule while the tag is absent. It grants no
   metadata merge or tag authority. Keep status pending until actual approval.
3. **Tag protection:** exact rule must restrict creation to Maintainers/access40.
   Read back its complete object. Fully paginate all protected-tag policies and
   evaluate every exact/wildcard match for v2.13.1; broader user/group/deploy-key
   access, ambiguous policy or incomplete coverage stops. If an existing exact
   rule differs, stop; do not edit/delete it or any other rule. Recheck unchanged
   exact object and effective matching policy set pre-push and post-publication.
4. **Prepare metadata:** first complete the approved evidence MR and binding
   final construction checkpoint above. Then use a clean isolated existing worktree/branch
   `release-2.13.1` from the exact reviewed current-main base. Do not touch the
   dirty root checkout. Edit only A and new changelog sections. Validate exact
   drafted bytes; commit only the reviewed allowlist, verify clean state and
   tested tree equality. Rebuild clean default binaries after the metadata
   commit for provenance; dirty-draft artifacts are layout/test evidence only.
5. **Namespace preflight:** from the clean committed release checkout, run
   `agov release namespace-check --phase preflight` with authenticated configured
   project identity 83321848 / janipaijanen/agents-govern-framework, live main
   target, package from `agf/dist/release-manifest.yaml`, and OCI reference derived
   through the governed helper with verify-ref agreement. Require positive
   collection access, complete pagination including every package status and
   all four namespaces ABSENT. No cached registry repository ID or manual 404
   reinterpretation. Credential stays in authorized environment, never argv,
   logs, output artifacts or chat. Recheck actual endpoint/project identity.
6. **Publish metadata MR:** first-push `git push -o ci.skip`, immediately open MR
   to main, verify one skipped push record and one canonical MR pipeline. Do not
   use commit-message skip markers. Derive expected jobs from actual current CI
   and diff, including mandatory release:namespace-check and preview artifacts;
   require exact-head success and independent metadata/security/content review.
7. **Gate 2:** publish binding exact-head governor decision only after both fresh
   residual dispositions, all required checks/artifact evidence, source/target
   SHA and reviewed merge strategy hold. Merge without squash. Prove main merge
   first parent is the reviewed target, second is approved source, and tree is
   the accepted tested metadata tree. Any intervening product change requires
   re-freeze; audit-only advance still needs exact reconciliation/acceptance.
8. **Immediate authorized tag action:** reobserve authority/expiry, runner
   availability and tag policies. Do not push unless a conservative duration
   budget makes the entire release:create job reach terminal success before
   expiry (installs, uploads, POST and follow-up verification): reserve
   at least two hours, or twice the latest observed complete tag-pipeline duration
   plus current queue allowance, whichever is longer. With an 18:00Z expiry,
   16:00Z is therefore the latest possible cutoff, and evidence may require an
   earlier cutoff. Unknown capacity/duration or insufficient margin stops before
   tagging for renewed authority, not a grandfathered late run. Then run canonical
   namespace-check `--phase pre-push`
   bound to the actual live main merge. On success, annotate `v2.13.1` on that
   merge and push once in one action, with no unnecessary extra confirmation.
   The merged metadata MR supplies standard-path tag authorization. Never move,
   delete or recreate the tag. If cut crosses UTC midnight before tagging,
   reconcile the changelog date and re-review rather than publish a false date.

After tag push, monitor until the entire release:create job reaches terminal
success inside the accepted window, including installs, uploads, POST and
follow-up verification. If unexpected delay threatens expiry, cancel or suspend
queued or running risk-bearing work while authority remains active and preserve
the immutable tag/any partial assets; reconcile ambiguous outcomes read-only.
Do not launch or retry expired work. A pre-expiry tag never grants post-expiry execution. The
margin is an operating precaution, not a guarantee of queue or network timing.

Planning capacity observation at 07:22Z: prior complete v2.13.0 tag pipeline
3006335980 took 28 minutes 5 seconds wall time (created 23:26:32Z, finished
23:54:37Z); its reported job duration is 1599 seconds and queue eight seconds.
The primary and active DinD runners were online. Ten protected-tag policies
were fully enumerated in one page; no exact v2.13.1 rule existed. These are
preparatory observations only, saved in
`/home/dev/s/tmp/codex-v2131-release-capacity-observation.json`. They support
planning the two-hour minimum reserve, not permission to tag or proof of future
capacity/protection. Repeat all required identity, policy, queue and expiry
checks at their actual pre-push checkpoints.

Namespace-check JSON must bind project/path/version/source/live target/time;
the mandatory MR job also binds the canonical pipeline. Neither check reserves
namespaces or eliminates races. The create-only publisher independently checks
tag commit identity and positive Release access before uploads and again before
its single ref-free POST. It does not check `.protected`; tag protection is the
operator prerequisite above. No update/delete/redirect following/POST retry.
Postconditions detect observed disagreement, not atomic transactionality.
Package upload/OCI/Release steps can leave a partial outcome; retain evidence.

## Acceptance matrix and evidence limits

Use current checked scripts/CI as command authority; no new dependencies or
changes to runtime, tests, CI, release image or scan policy in this plan.

- Release-mode workspace/all-feature compatibility tests, configured clippy and
  cargo-deny; all applicable agov validators, adopter-docs and assistant-loader
  checks. Preserve disclosed ignores and advisory warnings as such.
- Strict MkDocs, existing hook/tooling suites, commissioning first-screen and
  real link/CTA checks, projection checks, whitespace and release-path/license
  checks. Preserve independent desktop/mobile/keyboard review at the actual
  accepted content; no claiming visual acceptance from build success alone.
- `agov dist validate` and `dist build`; both framework archives' shipped tests
  and supported self-sufficiency checks; mandatory linux-x86_64 tooling archive,
  checksums/provenance and agov/agov-mcp self-reports. Clean version 2.13.1/source
  identity must agree; opt-in dist-profile diagnostic remains absent by default.
- Reuse canonical CI preview outputs; verify their actual bytes, sidecars and
  provenance/source/pipeline identities. Unpublished OCI MR scan must pass.
  That OCI scan covers the shipped image, not the publisher's mutable runtime;
  the fresh bounded APK report supplies its separately limited evidence.
- Derive job coverage from current `.gitlab-ci.yml`; do not copy the old 23-job
  matrix as a fixed requirement. No manual paid retry solely to seek green:
  diagnose, then use the next required corrected acceptance run or specific
  governing retry policy. Do not start duplicate local Rust/CI builds without
  evidence need. Ensure runner capacity before the irreversible tag push.
- Full release-content-freeze report reconciles all estate dimensions and
  inherited warnings. Update the structured projection manifest for the later
  live-status pages without introducing new normative text or loader changes.

### Job matrix derived from the frozen CI rules

Read-only derivation `/tmp/agf-v2131-release-job-matrix.md`, SHA-256
`61085634278a4f56ef2f1f18502ba83507d706f98bbf5f10b5516bb9ab88a960`,
evaluates the exact 29-path metadata diff against the frozen CI bytes. Recompute
at the real metadata head/target; any different path set invalidates this matrix.

The MR requires these 17 automatic, non-allow-failure jobs: `site-html-build`,
`agf:dist:preview`, `rust:tooling:preview`, `dependency:cve-scan`,
`agf:flyer:verify`, `agf:dist:validate`, `agov:oci:mr-build-scan`,
`release:namespace-check`, `release:assets-verify-preview`,
`tooling:live-surface-validate`, `release:paths-validate`,
`agov:governance:validate`, `rust:agov:validate`, `rust:tooling:deny`,
`commercial-corpus:validate`, `rust:tooling:test`, `rust:tooling:clippy`.

The tag requires 23 automatic, non-allow-failure jobs. Its 17 test jobs are:
`dependency:cve-scan`, `agf:flyer:verify`, `agf:dist:validate`,
`release:assets-selftest`, `agov-oci-image:selftest`,
`tooling:live-surface-validate`, `tooling:rust-cache-policy-shellcheck`,
`release:self-sufficiency-selftest`, `release:paths-validate`,
`agov:governance:validate`, `governance:changed-files:selftest`,
`rust:agov:validate`, `rust:tooling:deny`, `commercial-corpus:validate`,
`rust:tooling:test`, `rust:tooling:oci-pull-test`, `rust:tooling:clippy`.
The build job is `rust:tooling:release`; the five deploy-chain jobs are
`agf:dist:release`, `release:assets-verify-pre-oci`, `agov:oci:release`,
`release:create`, `release:verify`.

No manual jobs exist in this CI. The sole allow-failure job, `ci:pins:advisory`,
matches neither the metadata paths nor tags and is correctly absent. Ordinary
workspace tests exclude ignored cases; the tag's OCI-pull job explicitly runs
only ignored `oci_pull` and `composed_` families, not every ignored platform,
manual or profiling test. Mandatory jobs can still disclose advisory flyer,
commercial or validator output. Ordinary jobs need dedicated
`podman/linux/amd64` runners, nested OCI jobs need `dind`; no shared fallback.
Pages is the separately admitted default-main side effect, not MR/tag acceptance.

## Anonymous verification and Pages status synchronization

Require every expected tag job, including package/OCI publish, release:create
and release:verify, to pass at the annotated tag's peeled merge. Independently
run `agov dist post-publication-check` against the v2.13.1 public Release API
using fresh anonymous actual downloads in an owned temporary directory.
Validate all archive bytes/checksums/provenance, source commit, build and package
identities, both executable self-reports and OCI reference→manifest/config digest
chain with the supported helper. Reobserve exact annotated object/tagger/message,
peeled merge and protection/matching policy set. Retain sanitized evidence;
remove temporary downloads/credentials and never convert an unavailable
anonymous job API into claimed job-state proof.

Publish completion with new exact identities, archive hashes, Release/generic
package/OCI references, current tag pipeline/jobs, protection and anonymous-byte
verification. Only then prepare the B-plus-companion status-sync MR, using actual
UTC observation/publication times, new completion record and actual job count.
Require separate exact-head Gate 2 before merge. Verify its actual merge and
ordinary Pages deployment, then fresh anonymous site text/version/link state
before announcing release completion. An artifact publication followed by a
failed status sync remains published but not fully synchronized; fix status
forward without altering tag or artifacts.

Existing main-branch Pages CI automatically builds/deploys when metadata paths
change. Gate 1 must explicitly admit that existing side effect after metadata
Gate 2, with no CI change/manual deploy. It can expose v2.13.1 content version
before verified artifacts exist. Release-info's pre-tag mismatch or post-tag
latest-tag wording is source/Git provenance, **not a publication-completion
guard**. All authored B/latest-published fields stay at verified v2.13.0 until
anonymous verification and completion. Status-sync later deploys verified new
release status. No early announcement or proof inferred from a Pages build.

## Failure handling and unresolved approval fields

Before tag push, correct/review the reversible metadata branch or stop.
Any occupied/ambiguous namespace, changed protection, failed check, missing job,
expired delegation/risk disposition or unexpected content stops publication.
After push, preserve the immutable tag/artifacts, stop completion/status sync on
failure, reconcile uncertain POST by read-only inspection and never blindly
retry publication. A correction requires a separately authorized new version;
this plan does not authorize a v2.13.2 release or editing a published Release.

Approval readiness:

- [x] Exact Pages Gate 2/merge/parents/tree/pipeline and live verification recorded.
- [x] Exact README child Gate 1/amendment/Gate 2, merge and all required checks;
      F1/F2 resolved in a fresh source-bound estate addendum.
- [x] Product freeze candidate 93e20346, full estate/ABC inventory, explicit
      inherited-advisory dispositions and separate audit/construction sequence.
- [x] Independent design, runtime evidence, estate and job-matrix reviews received;
      final exact-plan consultation must resolve both front-matter rows.
- [x] Proposed fresh three-header checklist exception and mutable-APK decision are
      concrete and reviewed; the binding governor record must choose them, and
      final metadata Gate 2 must bind/recheck its exact candidate before merge.
- [x] Current claims/open MRs scanned; UTC cut date 2026-10-30; derived 17/23 job
      matrix, capacity observation, conservative cutoff and exact-tag-rule scope.
- [x] Binding Gate 1 bmaq and final consulted metadata prepared for publication.
      No implementation starts before that shared record and plan. The later audit-inclusive
      construction SHA, metadata Gate 2, namespace/protection/expiry checks and
      actual publication evidence are future gated checkpoints, not falsely
      claimed complete by approving this plan.
