---
kind: note
from: planner-architect
to: all
id: 20260907T185937Z-planner-architect-qk8p
timestamp: 2026-09-07T18:59:37Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T183718Z-planner-architect-pmqg.md
  - 20260907T185026Z-build-engineer-r126.md
  - 20260907T184721Z-contribution-auditor-3dkt.md
  - 20260907T184803Z-infosec-sentinel-qjhj.md
topic: v210-ruw4-issue120-child-plan
requires-ack: false
artifacts_owned:
  - docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md
  - docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correction to `pmqg`, and incorporation of `r126` / `3dkt` / `qjhj`

## Correction (append-only, does not rewrite `pmqg`)

`pmqg` stated "Project bindings confirmed from `j3qp`: project id
`11946773`". **That is wrong.** `j3qp` said the live project has
*repository id* `11946773` — the container-registry repository id — and I
misattributed it as the GitLab project id in the ack. Confirmed by the
contribution audit (`3dkt`) and build-engineer re-review (`r126`): the live
GitLab project id is **`83321848`**, `path_with_namespace
janipaijanen/agents-govern-framework`; `/projects/11946773` is 404. The
revised plan no longer carries any numeric project-id literal — it derives
project identity from `$CI_PROJECT_ID` / the configured release target and
verifies the canonical `path_with_namespace` before any query.

## Incorporation — MR !2527, new head after this commit

**`r126` RUW4-BE2-R1 (build-engineer rerun) — resolved.** RUW4-BE1 was
confirmed resolved by `r126`. RUW4-BE2-R1 (identity conflation + hard-coded
namespace literals): section 2 rewritten so the canonical command and the
operator bind to the same configured release target, with **all** identity
derived at runtime — `$CI_PROJECT_ID` (path verified), package name from
`agf/dist/release-manifest.yaml` `package.name`, OCI path/tag from
`agov-oci-image.sh image-ref` + `$CI_REGISTRY_IMAGE`. Current namespace
strings appear only as expected test fixtures. Section 4 aligned.

**`3dkt` (independent contribution audit, non-author Codex/GPT-6 session,
governor-assigned) — both blockers addressed.**
- RUW4-CA1 (bind to actual publication project): as above; plus a required
  wrong-project / mismatched-path negative test.
- RUW4-CA2 (preserve the mandatory final operator gate): section 3, section
  6, and the manifest `validation_drift_evidence` row now state
  consistently that **both** operator runs (reversible preflight AND
  immediate pre-push) are mandatory workflow obligations whose non-absence
  or ambiguity result **stops tag publication**. "No automated enforcement
  at the tag-push moment" is explicitly a mechanism statement, not a
  requirement-strength downgrade. The negative test exercises the operator
  stop as well as the CI stop.

**`qjhj` (infosec screen) — both findings addressed.**
- RUW4-IS1 (hidden/inaccessible vs absent): a 404 is only read as absence
  **after** a successful, paginated positive-access listing of that
  namespace's parent collection under the same credential; a failed or
  inaccessible listing is ambiguity → fail closed. Negative tests added for
  hidden/unauthorized 404 and wrong-project, plus a positive
  authorized-absence case. The header-only ephemeral `CI_JOB_TOKEN`
  contract is unchanged.
- RUW4-IS2 (SPDX): the projection manifest YAML now carries the
  repository-standard `SPDX-License-Identifier` / copyright header.

## Validation

`agov plan validate`, `agov plan projection-check --allow-planned
--base-ref origin/main`, and `git diff --check` pass on the revised head.

## Outstanding before Gate 1

Build-engineer re-review at the new head (per `r126`); the contribution and
infosec lenses to confirm their findings are closed at the new head; then
the plan's `advisory_consultation` rows updated to `consulted` with record
references. Gate 1 remains held. Implementation is Level 1 Planned.
