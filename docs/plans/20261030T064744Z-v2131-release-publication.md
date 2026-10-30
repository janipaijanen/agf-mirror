---
id: 20261030T064744Z-v2131-release-publication
status: pending-gate1
topic: v2131-standard-main-release-publication
candidate_release: v2.13.1
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: Draft awaiting exact freeze and independent release-plan review.
  - role: infosec-sentinel
    status: pending
    reason: Fresh publisher residual-risk and SPDX dispositions required; no prior exception carries forward.
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
  - 20261029T220500Z-v213-release-publication
  - 20261029T204751Z-v213-release-freeze-preparation
  - 20261029T192922Z-release-publisher-api-amendment
  - 20261028T194451Z-ci-reproducibility-pins
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Draft: publish AGF v2.13.1 through the standard main-branch path

**Planning only; not release readiness or permission to execute.** Root must
replace every pending field, refresh the inventory at the final construction
base, resolve prerequisite dispositions and complete independent consultation
before publishing an approved plan. No version/tag/package/Release/protection
mutation has been performed by preparation of this draft.

## Candidate and freeze

Candidate patch: the twelve-path Pages reader-journey correction approved by
pksk/gnqj, MR !3394 reviewed head `6fc3e6de` (full identity and final Gate 2
record to be supplied by root). Its scope is nine explanatory pages, MkDocs
navigation, the narrowly amended CTA guard, and the projection manifest. It
changes navigation and documentation/acceptance checks, not an AGF command,
API, rule, schema, dependency, platform or installation contract.

**Pending before approval:** exact verified !3394 merge SHA/tree/parents,
Gate 2 record and successful exact-head pipeline, and final content-freeze SHA.
Do not treat an MR head or this draft's historical observation as that freeze.
Inventory observation used here is main `a5d13ad9`, fully recorded in
`/tmp/agf-v2131-release-version-inventory.json`; later coordination-only target
advances are not implicitly admitted. Enumerate every intervening merge/path,
verify its authority and tree, and freeze the concrete final current-main tip.
Any unrelated product change requires explicit release-content reconciliation.

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

Preferred bounded disposition for this already-approved Pages-only release:
obtain a **fresh explicit v2.13.1-only checklist exception**, naming these exact
three files, their observed header state, the unchanged path-based CC-BY-SA-4.0
license and absence of a generated-file exclusion. Require independent
licensing/security review and a new binding governor decision before metadata
Gate 2. It must say the all-file header checklist is not fully satisfied, carry
an exact candidate/time boundary, grant no general exception or header-edit
permission, and keep tgrk's separate repair proposal tracked. Neither dta3 nor
this draft supplies that decision. No known third-party/license conflict is
waived by accepting these three historical omissions.

The alternative, if the governor chooses it, is a separate three-file
header-only Gate 1 plan following tgrk, then independent review and Gate 2 before
freezing. That would need explicit release-scope reconciliation and a named
plan dependency. Do not silently implement repairs in the Pages MR or metadata
MR. Current recommendation preserves the approved twelve-path feature scope
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

Gate 1 may authorize bounded credential-free replay of that exact install,
base and actual post-install filesystem scans using a verified scanner image
and fresh dated database, and tool/CA inventory. No project mounts, forwarded
credentials/environment, custom shipped builder image, dependency/CI edits or
production POST. Retain report hashes, scanner/package/database/time/architecture
scope, findings and exclusions. The old 2026-10-29 HIGH/CRITICAL-zero reports
remain immutable historical evidence; do not relabel them as today's scan.

Before metadata Gate 2, require independent review and a fresh governor record
accepting only the observed mutable-package/tool/CA and availability residual
for this exact v2.13.1 cut, with an explicit expiry no later than the delegation.
No known-vulnerability waiver is implied: any new finding requires remediation
or its own evidence-based disposition; no failure is suppressed. A material
base/package/client/control change returns to a separately approved scope.

## Exact release-truth inventory

At observation a5d13ad9, the checked literal inventory has **36 current-version
occurrences on 36 lines across 27 files (A)**; **11 latest-published occurrences
on 9 lines across 3 files (B)**; and **61 historical/external occurrences on 60
lines across 17 files (C)**. Counts are observations, not a replacement for the
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

Proposed repository entry: identify !3394's verified merge, the two independently
checked CTA destinations and negative controls, unchanged CSS/feature-summary
contract, and exact validation evidence. Link the adopter section rather than
invent a second scope. Do not mark unrelated issues closed or promise conversion,
performance, compatibility or independent-adopter benefits. Report actual test
counts/ignores after validation; no copied count is a new pass.

## Separate gates and publication controls

1. **Before Gate 1:** replace pending freeze/merge/plan-review fields; reconcile
   main since v2.13.0 (the previous three status-sync paths are historical
   publication truth, not new feature scope); review current claims/open MRs,
   collisions, full release estate and exact candidate pipeline/Pages result.
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
4. **Prepare metadata:** use a clean isolated existing worktree/branch
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
   availability and tag policies; run canonical namespace-check `--phase pre-push`
   bound to the actual live main merge. On success, annotate `v2.13.1` on that
   merge and push once in one action, with no unnecessary extra confirmation.
   The merged metadata MR supplies standard-path tag authorization. Never move,
   delete or recreate the tag. If cut crosses UTC midnight before tagging,
   reconcile the changelog date and re-review rather than publish a false date.

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

Required before this draft may be approved:

- [ ] Exact !3394 Gate 2/merge/parents/tree/pipeline and Pages verification.
- [ ] Exact frozen content/main construction SHA, all intervening-path inventory,
      release-estate review and refreshed A/B/C inventory.
- [ ] Fresh independent contribution and security consultations, with no pending
      advisory row left in approved metadata.
- [ ] Governor choice/record for the three inherited headers; tgrk repair remains
      separately tracked, not silently added to Pages scope.
- [ ] Agreed evidence refresh/expiry and fresh v2.13.1-only mutable-APK residual
      disposition before metadata Gate 2; any findings separately resolved.
- [ ] Current claims/collisions, UTC cut-date choice, expected job matrix/runner
      availability and exact protected-tag policy/creation scope.
- [ ] Replace all pending fields and unverified short IDs; reviewed plan must
      name its own new binding approval, not inherit v2.13.0 authority.
