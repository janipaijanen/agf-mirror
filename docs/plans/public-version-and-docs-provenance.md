---
id: public-version-and-docs-provenance
status: pending-gate1
topic: public-version-and-docs-provenance
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "must review authority boundaries, scope, compatibility, and acceptance evidence before Gate 1"
  - role: infosec-sentinel
    status: not-applicable
    reason: "the CI-driven sentinel's rule-granted mechanical credential/cross-record checks are not triggered by this plan-only documentation artifact; a separately labeled AI security-lens review covers design risks without impersonating or replacing CI"
  - role: fact-checker
    status: consulted
    reason: "must verify the exact release tag, release commit, live Pages commit, and all public version wording before Gate 2"
---

# Plan: public version and documentation provenance

## Trigger and observed problem

The governor accepted the model in which `agf/VERSION` remains the single
source of truth for the semantic framework version, while immutable release
identity and mutable documentation-build identity remain separately derived.
Promotion work is parked until public pages state these identities without
ambiguity.

The current surfaces mix three different facts:

- `agf/VERSION` and the latest GitLab release identify AGF `2.4.3`;
- the immutable `v2.4.3` tag identifies release commit
  `7a43ad687e45ae2a36f6c8c157f08d2127faaae7`;
- the live Pages site was built from later `main` commit
  `ba2f29c2da760cf2d2ca8e4c23c65389a2d699e0`; and
- current `main` has already replaced the former per-page `Released (v2.1
  documentation)` convention with generated release-information links, but
  one authored page still says `Deployable (v2.0)` and two meta-descriptions
  still describe the obsolete versioned-label convention.

The generated release-info hook reads `agf/VERSION` and the current build
commit. That correctly reports content version and build provenance, but it
does not presently distinguish the latest immutable release from development
documentation built after that release.

## Decision proposed for Gate 1

Keep `agf/VERSION` as the only authored semantic-version scalar. Do not add a
second manually maintained current-version file.

Treat the public identity as a generated tuple with separate authorities:

| Public field | Authority | Meaning |
|---|---|---|
| Framework version | `agf/VERSION` | semantic content/API version declared by the repository |
| Latest release tag | Git tag/release metadata | immutable release name, expected to equal `v${VERSION}` |
| Release source commit | commit targeted by that tag | immutable source used for released artifacts |
| Live docs branch and commit | Pages CI checkout | exact mutable documentation build |
| Documentation status | comparison of build commit with release tag | exact release docs, or development docs after/divergent from the release |

The default Pages site remains a mutable build from `main`, but must call
itself development documentation whenever its build commit is not exactly the
release tag commit. The immutable tag/source URL remains the release source.

## Scope

### 1. Define one release/build provenance contract

Extend the generated release information so page body text, header chrome, and
the AI evaluation dossier consistently expose:

- framework version;
- latest release tag and immutable release commit;
- live docs branch and full build commit;
- whether the build is exactly at the release tag; and
- canonical links to the GitLab release and immutable tagged source.

Use plain labels such as:

> Latest release: v2.4.3
>
> Release source: tag v2.4.3, commit 7a43ad6…
>
> Live docs build: main, commit ba2f29c2…
>
> Status: development documentation after v2.4.3

Never describe the mutable Pages URL itself as an immutable release artifact.

### 2. Derive provenance without a second version truth

Implementation must begin with a bounded probe that tests whether the Pages
job's actual shallow branch checkout can reliably resolve the latest release
tag and its peeled commit. No other implementation scope begins until that
result is recorded. The preferred resolution order is:

1. local Git tag data, after making tag availability explicit in CI;
2. deterministic CI-provided values sourced from the same Git refs; then
3. an explicit `unknown` result that preserves the build identity and avoids a
   false release claim.

Do not query a mutable external API during documentation rendering and do not
introduce a hand-edited release manifest merely to duplicate `VERSION` or tag
state. If the preferred derivation is infeasible, return to Gate 1 with the
evidence and a revised provenance design.

GitLab currently has no protected-tag rule for `v*`. That pre-existing
repository trust risk also affects the existing release pipeline and is not
created or remediated by this documentation plan. The governor may track tag
protection as a separate repository-configuration decision; implementation
must not silently expand this plan to change project permissions.

Tag pipelines must continue to fail when `CI_COMMIT_TAG` differs from
`v$(cat agf/VERSION)`. A default-branch docs build may share the release's
semantic version while clearly being later development content.

### 3. Remove misleading authored status versions

Inventory every current-looking static version label in source documentation,
the dossier assembly, generated output checks, screenshots, and public landing
copy. Correct the remaining `Deployable (v2.0)` label and the two
meta-descriptions of the obsolete `Released (vX documentation)` convention.
Use a non-versioned lifecycle status such as `Released documentation`,
retaining a version only when it is an explicitly historical statement such as
`Introduced in v2.1`.

Generated `agf/site-html` is not an authority. Either regenerate it through its
documented workflow when required or ensure tests cannot mistake it for source
truth; do not hand-edit generated pages to hide drift.

### 4. Make the result extractor- and accessibility-friendly

The tuple must appear in visible page-body text on the release-information page
and in the dossier's release/provenance section. Header presentation may remain
concise but must not be the only source. Labels and link text must preserve
meaning in plain-text extraction and screen-reader output.

### 5. Add deterministic acceptance evidence

Tests and validation must prove:

1. `agf/VERSION` contains one valid semantic version and remains the only
   authored current-version scalar;
2. on an exact tag build, release tag is `v${VERSION}`, release commit equals
   the docs build commit, and status says release documentation;
3. on a later `main` build, the release tuple remains pinned to the tag while
   branch/commit identify the newer build and status says development docs;
4. when tag provenance is unavailable, output says `unknown` rather than
   inferring or fabricating a release commit;
5. built HTML and plain-text extraction produce one unambiguous answer for
   latest release and a separately labeled live build;
6. the dossier embeds the same tuple rather than only linking to it;
7. source scanning rejects current-looking versioned lifecycle or maturity
   labels—including `Released (vX documentation)` and `Deployable (vX)`—except
   allow-listed prose that explicitly labels the version as historical; and
8. existing tag/VERSION release guards and strict MkDocs builds remain green.

The implementation handoff must include the exact tested commit, representative
rendered text, extraction output, and the tag-resolution behavior observed in
CI.

## Files likely affected after Gate 1

- `agf/docs/build/hooks/release_info.py`
- dossier generation and release/provenance source pages
- header partial and any release-info styling
- pages containing static versioned lifecycle labels
- `.gitlab-ci.yml` only if tag availability or deterministic inputs must be
  made explicit
- a new focused hook test such as
  `agf/docs/build/hooks/test_release_info.py`, plus build/extraction tests and
  documentation of the provenance contract

Exact files are to be confirmed during implementation. No release, tag,
`agf/VERSION` bump, promotion publication, mirror, or external repository is in
scope.

## Review and authority

- Contribution-auditor reviews the authority model, minimality, and testable
  scope before approval.
- The CI-driven infosec-sentinel has no applicable plan-stage mechanical check;
  Claude's separately labeled security-lens review is advisory evidence only
  and does not impersonate or replace the sentinel.
- Governor-reviewer alone approves Gate 1 and Gate 2.
- Fact-checker is activated for the exact implementation SHA before Gate 2 and
  verifies public claims against Git refs, release metadata, and rendered
  output.
- Coder-tester may implement only after a validated governor-reviewer Gate 1
  approval record changes this plan to `approved`.

## Rollback

The implementation must remain one reviewable feature branch. If provenance
derivation proves unreliable, revert the generated presentation changes and
retain the existing `VERSION` and tag/VERSION release guard. No tag or released
artifact is rewritten.

## Completion criteria

This plan is complete only when the approved implementation is merged, public
Pages have rebuilt successfully, the live tuple is independently verified,
and a completion record states that promotion planning may resume. Until then,
promotion remains parked.
