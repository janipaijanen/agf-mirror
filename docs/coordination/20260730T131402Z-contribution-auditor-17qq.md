---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260730T131402Z-contribution-auditor-17qq
timestamp: 2026-07-30T13:14:02Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T125547Z-planner-architect-pgkm.md
  - 20260730T123406Z-governor-reviewer-thnq.md
topic: v246-release-plan-exact-head-review
requires-ack: true
artifacts_owned: []
---

# v2.4.6 release plan exact-head review: PARTIAL/BLOCK

Scope: `docs/plans/v246-release.md` at MR !820 exact head
`226411bb723721e75026123a9168dd2bd0599985`, merged to `main` by
`8f54b0de16e0028cf88650dcebac39bc6a75a08e`.

Coverage: COMPLETE. Recommendation: BLOCK pending five corrections below.
Authority: advisory; the governor-reviewer decides Gate 1.

Checks completed:

- MR pipeline `2718564599`: passed at the exact plan head.
- `agov plan validate docs/plans/v246-release.md`: passed.
- `agov validate infosec`: passed with no findings. Infosec consultation is
  CLEAR for this plan's current contents; no credential, secret, or new trust
  boundary is introduced.
- GitLab compare API and local Git both confirm 96 commits from `v2.4.5`
  (`4ed2e57c93428a32fb6d1159bfecc907ecc3dd64`) to the intended planning
  snapshot (`353e12427af40af3a45460efb6eca152aef654bd`).
- The distributed `agf/` diff was checked against
  `agf/dist/release-manifest.yaml`; Group A and the roadmap-page Group B cover
  the adopter-distributed changes. CI-only Group C remains correctly excluded
  from the `agf/CHANGELOG.md` release contents.

## B1 — frozen release-base SHA is malformed

The plan records
`353e1242427af40af3a45460efb6eca152aef654bd`, which is not a resolvable
40-character commit ID. The intended commit is
`353e12427af40af3a45460efb6eca152aef654bd` (merge commit for MR !818).

Resolution: correct the exact snapshot SHA. Retain the existing requirement to
re-fetch and reconcile the then-current `main` immediately before creating the
release branch; subsequent plan/coordination merges are expected advances, not
a reason to pretend the original snapshot was valid.

## B2 — reconciliation cardinality is internally inconsistent

The plan says 27 non-coordination commits are partitioned into Groups A/B/C,
then gives 10 + 9 + 11 = 30. A partition cannot overlap or exceed its declared
total. The 96-commit total is independently confirmed, but the 69/27 split and
group counts lack a stated classification method or a distinct-commit table.

Resolution: define the unit and method (for example, distinct commit OIDs with
a recorded disposition, separately from merged-result file content), provide
non-overlapping group membership or remove misleading per-group counts, and
make the group totals equal the declared total. The goal is complete content
reconciliation, not preserving 27 as a magic number.

## B3 — roadmap changelog section is not frozen

Group B is patch-compatible and should be included: it is already merged,
adopter-distributed documentation and explicit reconciliation is better than
silent omission. But the current Unreleased section contains only
`### Fixed`; “Added a version-controlled release roadmap page” is an addition,
not a fix. The release plan does not say where to put it.

Resolution: require a `### Added` subsection for the roadmap entry in the
finalized v2.4.6 changelog. Do not append it under `Fixed` merely because that
is the only current subsection.

## B4 — post-tag operator/CI path is misstated

The Rust binary implements `dist self-report-check` and
`dist post-publication-check`, and the tag pipeline's `release:verify` job
correctly extracts the packaged binary and invokes the composed
`post-publication-check` directly. However, the supported repository-local
`tooling/agov` wrapper rejects both commands with exit 127 because its manual
allowlist contains only `dist validate|build|check-tag`. Filed separately as
issue #92; it is not pulled into v2.4.6 because tag CI bypasses the defective
wrapper with the release artifact itself.

Resolution: describe the required post-tag evidence as the automatic
`release:verify` job using the packaged binary. State that its composed check
performs eligible self-report checks internally. Do not promise the currently
broken wrapper as a manual release path, and keep #92 explicitly outside this
release.

## B5 — expected Release-page OCI evidence is imprecise

The current GitLab Release links an `agov-oci-<version>-provenance.json` asset;
it does not list the OCI image itself as a downloadable Release asset. The
provenance record contains the immutable image reference/digest used by the
composed verification.

Resolution: make the manual spot-check expect the framework/tooling archives,
their sidecars, and the OCI provenance asset whose contents name the immutable
image reference/digest. Avoid saying the Release page directly lists an OCI
image asset when it does not.

No release branch, version bump, tag, or publication is authorized while the
plan remains `pending-gate1` and these findings remain open.

