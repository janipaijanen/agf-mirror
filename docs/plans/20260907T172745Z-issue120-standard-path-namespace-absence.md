---
id: 20260907T172745Z-issue120-standard-path-namespace-absence
status: pending-gate1
topic: issue120-standard-release-path-namespace-absence-obligation
candidate_release: v2.10.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Pending exact-head review. The schema has no `pending` value; `waived`
      is used only while this draft is in review. Gate 1 is not eligible
      until a fresh infosec review of the namespace-query / credential /
      protected-tag / disclosure surface is incorporated and this row
      becomes `consulted`.
  - role: contribution-auditor
    status: waived
    reason: >-
      Pending independent exact-head audit. The schema has no `pending`
      value; `waived` is used only while this draft is in review. This plan
      is authored by a Claude session temporarily allocated to
      planner-architect by governor direction (2026-09-07, "create gate1
      plan for #120"); the independent contribution audit MUST come from a
      non-author session (e.g. Codex as contribution-auditor, or another
      session named by the governor) — the authoring session cannot supply
      it. Gate 1 is not eligible until that audit is incorporated and this
      row becomes `consulted`.
  - role: build-engineer
    status: waived
    reason: >-
      Pending exact-head review of the CI job shape, `agov-oci-image.sh`
      single-sourcing, runner/tag coverage, and the one-tagged-build
      budget. Gate 1 is not eligible until incorporated and this row
      becomes `consulted`.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #120 child plan: add a pre-tag namespace-absence obligation to the standard release path

## Authoring provenance and independence

Authored by a Claude session temporarily playing **planner-architect** by
governor direction (2026-09-07, "create gate1 plan for #120"), after the
`pgm4` handoff to planner-architect went unactioned for ~2.5 hours and the
release-cut critical path was blocked on it. Per the `s5gs` role-assumption
handshake, this is a direct governor assignment; it is acknowledged inline
here. **The independent contribution audit of this plan must come from a
non-author session** — this session (which normally plays
contribution-auditor) is disqualified from auditing its own plan. Codex is
the RUW-4 *implementation* lane owner (`pgm4`) and authored the parent v2.10
plan, so a fully independent reviewer is preferable; the governor names the
auditor.

## Decision requested at Gate 1

Approve adding a **required pre-tag four-namespace absence obligation** to
the **standard** release workflow in
`agf/rules/workflow/release-via-merge-request-rule.md`, so that every future
release — not only the maintenance-line and frozen-scope-reconstruction
exceptions — proves the target namespaces are unoccupied before an
immutable tag is pushed, and a pre-existing object stops the release.

This is a v2.10 mandatory child per the approved v2.10 release plan (RUW-4)
and governor records `smem` / `2emq`. It is **gate mechanics** — a new
normative requirement on every release plus a new mechanical CI check — so
it takes the full Gate 1 path and ships with the negative test
`always/gate-self-test-rule.md` requires (both for the checklist addition
and for the CI check).

## Problem (from issue #120, read at `main` `fe8e0fc8`)

`release-via-merge-request-rule.md` requires a pre-tag namespace-absence
check in exactly two places — the maintenance-line exception (~line 97) and
the frozen-scope reconstruction exception (lines ~412–429, in full detail:
Git tag, GitLab Release, generic package across every status paginated
fully, and the exact OCI repository tag with the repository ID resolved
fresh). **Neither governs the path releases actually take.** The standard
`## What the MR MUST verify` checklist (8 items: VERSION, CHANGELOG,
adopter-docs marker, version consistency, `agov validate adopter-docs`,
`agov dist validate`/`build`, CI config, SPDX) says nothing about tag /
Release / generic-package / OCI namespace absence.

v2.8.0 shipped through the standard workflow. Its immediately-pre-tag OCI
absence check queried `v2.8.0` while publication used `agov:2.8.0`
(`q5mm`; accepted as a process-evidence gap in `gs18`). That check was
**not required by any governing rule on that path** — the operator did it
as good practice and used the wrong name. The accepted `gs18` gap is
therefore larger than recorded: not only "the check used the wrong
spelling" but "the check was not required, so nothing would catch its
absence."

An OCI tag is mutable. Without a required pre-tag absence proof on the
standard path, a future release can publish over a name another actor or
process already occupies, and the successful push is not retrospective
evidence the name was free.

## Resolution of the six `pgm4` points and the three issue-#120 open questions

### 1. Full four-namespace check, in the standard path's proportionate form

The standard path adopts the **full four-namespace set** — Git tag, GitLab
Release, generic package (every status, paginated fully), and the exact
helper-derived OCI reference — not a subset. Rationale: the reconstruction
exception already specifies exactly this set; the *standard* path is the
one that actually ships every release, so it is not lower-risk than the
exceptions. What the standard path does **not** inherit is the
reconstruction exception's surrounding *ceremony* (three independent
re-derivation parties, tree-hash replay proofs). It inherits the
**absence-proof set and its fail-closed semantics** only.

### 2. Authoritative, reproducible command/API set with fail-closed behavior

The plan defines a single authoritative check, invoked from one place, so
"we checked" is reproducible:

**Identity is derived at runtime, never hard-coded** (build-engineer
`r126` RUW4-BE2-R1, contribution audit `3dkt` RUW4-CA1). The canonical
command and the operator both bind to the **same** configured release
target:

- **GitLab project:** the same-project CI context — `$CI_PROJECT_ID` in CI,
  the configured release-target project for an operator run — and the check
  first verifies that project's `path_with_namespace` equals the expected
  canonical path before any namespace query. (Observed evidence only, not a
  command literal: this project is currently id `83321848`,
  `janipaijanen/agents-govern-framework`. The earlier draft and `pmqg`
  wrongly stated `11946773` as the project id — that value is the current
  *container-registry repository id*, not a project id; see the correction
  record referenced below.)
- **Generic package name:** read from the validated release manifest
  `agf/dist/release-manifest.yaml` `package.name` (currently `agf`), or
  passed to the command as that manifest-derived value. Not a literal.
- **OCI repository path and tag:** derived **exclusively** from the
  `agov-oci-image.sh image-ref` output (guarded by `verify-ref`, v2.9.0 A2)
  plus runtime `$CI_REGISTRY_IMAGE`. The current path
  `janipaijanen/agents-govern-framework/agov` and the un-prefixed tag form
  `<X.Y.Z>` (the generic package keeps `v<X.Y.Z>` — the divergence is real
  and the check must not paper over it) appear in the plan only as an
  **expected test fixture**, never in the command contract.

**Positive access before accepting absence** (infosec `qjhj` RUW4-IS1):
GitLab documents that an HTTP 404 also means *the caller cannot access the
resource*, not only *the resource is absent*. A successful package
enumeration does not prove tag or Release visibility — feature and
endpoint permissions differ. So for **each** namespace the check first
establishes positive authenticated access by successfully listing that
namespace's parent collection under the same credential
(`GET …/repository/tags?per_page=…` paginated, `GET …/releases` paginated,
`GET …/packages` paginated, registry `GET …/registry/repositories`
paginated). Only after that listing succeeds is a 404 on the specific
`v<X.Y.Z>` / `<X.Y.Z>` object interpreted as **absence**. If the listing
itself fails or is inaccessible, the namespace result is **ambiguity**, not
absence.

- **Git tag:** list tags (paginated) → then `GET
  …/repository/tags/v<X.Y.Z>`. 200 = occupation; 404 *after a successful
  listing* = absence.
- **GitLab Release:** list releases (paginated) → then `GET
  …/releases/v<X.Y.Z>`. Same.
- **Generic package:** `package_name` in the Packages API is a **fuzzy
  substring filter**, so a non-empty response is not by itself occupation.
  Query `…/packages?package_type=generic&package_name=<manifest name>`,
  **fully paginate**, repeat for **every** status — `default, hidden,
  processing, error, pending_destruction, deprecated` — via `&status=<s>`.
  Occupation requires an **exact match** on `name == <manifest name>` **and**
  `version == "v<X.Y.Z>"` **and** `package_type == "generic"`.
- **OCI reference:** paginate registry repository discovery, **exact-match**
  the returned repository `path`/`location` against the `image-ref`-derived
  path, then query that freshly resolved repository id for the derived tag.
  Zero exact-path repositories *after a successful listing* = absence; a
  present tag = occupation; multiple exact-path matches or an inaccessible
  list = ambiguity.

**Authentication (CI):** the same-project ephemeral `CI_JOB_TOKEN`, sent
only in the `JOB-TOKEN` header. No PAT, project access token, or
query-string credential; no shell tracing (`set -x`) around the calls; no
token-bearing job artifact.

**Fail-closed:** any of — a project whose `path_with_namespace` does not
match the expected canonical path, a failed or inaccessible namespace
listing, an unavailable/unenumerable package status, an incomplete page
walk, a `401/403/429/5xx`, a timeout, malformed JSON, pagination ambiguity,
multiple exact OCI-path matches, or any policy/API ambiguity — is treated
as **failure, not absence**. The check prints `result=ABSENT` only when all
four namespaces returned an unambiguous, positively-accessed absence.

### 3. Operator preflight **and** a mechanically blocking CI check, correctly placed on the pipeline topology

Both, because the v2.8.0 defect was a hand-typed name with nothing invoking
the check. The **placement** matters: an absence check is only meaningful
*before* the immutable tag exists, so it cannot run as a blocking gate on
the tag pipeline itself (the tag push is what triggers that pipeline, so by
then the Git-tag namespace is necessarily occupied — `7dba` RUW4-F1). The
split is therefore:

- **Mechanically blocking CI check — release-metadata MR / pre-tag path
  only.** A new job (provisionally `release:namespace-check`) runs on the
  release-metadata MR pipeline (the last CI pipeline that precedes the tag
  push) and **blocks** merge on any non-absence or ambiguity across all
  four namespaces. Its blocking authority is granted by an explicit clause
  in `release-via-merge-request-rule.md` (per
  `always/tool-authority-boundary-rule.md`). Constraints the plan binds
  (build-engineer lens, `j3qp` RUW4-BE1):
  - This project runs **detached** MR pipelines
    (`refs/merge-requests/<iid>/head`), not merged-results pipelines, so
    the eventual three-way merge commit does not exist at job time. The job
    binds its evidence to the exact release-MR **source SHA, target SHA,
    candidate version, and canonical pipeline id** — never a nonexistent
    future merge commit. The operator's by-hand pre-push recheck (below)
    binds the resulting `main` merge commit after merge.
  - Trigger predicate: `merge_request_event` **and** a change to
    `agf/VERSION` (or an equally deterministic release-metadata predicate).
  - No second Rust build: the job **consumes the already-required
    `rust:tooling:preview` artifact** containing `agov` — no `cargo build`,
    no second archive build.
  - Ordinary non-DinD job on the default dedicated runner (the namespace
    queries need no container service).
- **Operator preflight and immediate pre-push recheck — both mandatory.**
  The standard `## What the MR MUST verify` checklist gains one item —
  "namespace-absence proof (`agov release namespace-check` / equivalent)
  attached, `result=ABSENT` for all four namespaces at the exact intended
  tag." The operator runs it **twice**: once in the reversible preflight,
  and **again immediately before `git push` of the tag**. Both runs are
  **required workflow obligations**; a non-absence or ambiguity result at
  either **stops tag publication**. The immediate pre-push run is the final
  absence gate (the CI job ran earlier, against the pre-merge state, and
  cannot establish absence at the later tag push), mirroring the recheck the
  maintenance-line and reconstruction exceptions already require immediately
  before their tag push. "No *automated* enforcement exists at the tag-push
  moment" is a statement about mechanism, **not** about requirement
  strength: the operator check is mandatory, not advisory (`3dkt`
  RUW4-CA2).
- **Optional post-tag CI job — consistency, not absence.** If a tag-pipeline
  job is kept at all, it is defined as post-tag *consistency / non-overwrite*
  verification: the pushed tag points at the `main` merge commit that
  carries the reviewed release-MR source SHA, and the GitLab Release /
  generic-package / OCI artifacts the tag pipeline is about to publish do
  not already exist at a *different* digest or content than this pipeline
  produces. It never asserts Git-tag-namespace absence.

The forge-neutral canonical implementation is a named command
(provisionally `agov release namespace-check`); the CI wiring is a
removable adapter that invokes it (same shape as `agov coord
publication-check`).

### 4. `agov-oci-image.sh` single-sourcing

The OCI namespace check consumes `agov-oci-image.sh image-ref` for the
reference and relies on the existing `verify-ref` mutation-tested guard
(v2.9.0 A2, MR !2198). **No OCI name — literal or otherwise — is written
into the command contract.** The repository path and the **un**-prefixed
tag `<X.Y.Z>` (the generic package uses `v<X.Y.Z>`; the OCI tag does not —
a divergence the check must not paper over) are parsed solely from the
`image-ref` output plus runtime `$CI_REGISTRY_IMAGE`. Registry repository
discovery is paginated in full, the returned repository `path`/`location`
is exact-matched against that derived path, and only then is the freshly
resolved repository id queried for the tag — never a cached id. Zero
exact-path repositories *after a successful repository listing* is
unambiguous tag absence; multiple exact-path matches, an inaccessible
list, or a failed listing fails closed. The current namespace
(`janipaijanen/agents-govern-framework/agov`) appears in the plan only as
an expected test fixture.

### 5. Required negative evidence (the `gate-self-test-rule` obligation)

The implementation MR MUST include a deliberate negative test demonstrating
that a pre-existing object in **each** governed namespace stops the
standard path before any immutable tag:

- construct or stub a pre-existing Git tag / Release / generic-package (in a
  non-default status, to exercise the pagination) / OCI reference at the
  candidate version;
- run the standard-path namespace check;
- assert it exits non-zero, prints the blocking reason naming the occupied
  namespace and status, and that no tag was created;
- capture the transcript / CI job log in the MR description under a
  `## Negative test` heading.

Additional required negative and positive cases (from the exact-head
reviews):

- **Wrong project / mismatched path** (`3dkt` RUW4-CA1) — point the check
  at a project id whose `path_with_namespace` is not the expected canonical
  path; assert it fails closed *before* any namespace query, not "absent".
- **Hidden / unauthorized 404** (`qjhj` RUW4-IS1) — a namespace whose
  parent-collection listing returns 403/404 (no access); assert the result
  is *ambiguity → fail closed*, never absence.
- **Positive authorized-absence** — a namespace with a successful parent
  listing and a genuine 404 on the specific version; assert `ABSENT` for
  that namespace.
- **Fuzzy-only package match** (`j3qp` RUW4-BE2) — a similarly-named generic
  package present but no exact `name`/`version`/`type` match; assert not
  occupation.
- **Operator pre-push stop** (`3dkt` RUW4-CA2) — the runbook/negative
  evidence exercises the by-hand immediate pre-push run stopping a tag push
  on a non-absence result, not only the CI job.

A passing-only test ("checked an absent name, it passed") is explicitly
insufficient.

### 6. Six-surface projection screen

`docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml`:

| Surface | Disposition |
|---|---|
| Normative owner | **Updated:** `agf/rules/workflow/release-via-merge-request-rule.md` `## What the MR MUST verify` + a new "Pre-tag namespace-absence obligation (standard path)" subsection stating the four-namespace set, fail-closed semantics, and the CI check's blocking authority. Existing owner, not a new rule. |
| Adopter-facing explanation | **Updated:** `agf/docs/site/docs/operate/review-and-release.md` (or the release Pages page) explains the obligation and links the owner. |
| Runbook / situation map | **Updated:** `agf/skills/release/SKILL.md` gains the check step; runbook "preparing a release" row points at it. |
| AI landing / loaders | **Not applicable (reasoned):** the obligation is operator/CI procedure, not assistant startup behavior; no `AGENTS.md` / specialist-loader change. |
| Validation / drift evidence | **Implemented (CI) + required-manual (operator):** the `release:namespace-check` CI job + the `gate-self-test` negative test are the mechanical evidence on the pre-tag path. The operator's reversible-preflight run **and** immediate pre-push recheck are **mandatory workflow obligations** (not advisory) whose non-absence/ambiguity result stops tag publication — there is simply no *automated* enforcement at the tag-push moment, which is a mechanism gap, not a strength downgrade. The negative test exercises the operator stop as well as the CI stop. |
| Distribution | **Confirmed:** the rule ships via `agf/rules/**`; the release skill ships via `agf/skills/**`; the canonical command ships in the tooling. Root docs are project-local where applicable. |

### Issue-#120 open question 3 — superseding `gs18`

Yes. A **new append-only coordination record** (not an edit) supersedes the
accepted `gs18` disposition, stating that the v2.8.0 gap is understood to be
larger than "wrong spelling" — it was also "unrequired on that path" — and
that this plan closes it. That record is a `governor-reviewer` /
`contribution-auditor` artifact filed alongside Gate 1, not plan content.

## Affected files / test allowlist (implementation phase, after Gate 1)

- `agf/rules/workflow/release-via-merge-request-rule.md` — checklist item +
  the standard-path namespace-absence subsection + the CI blocking-authority
  clause.
- `agf/skills/release/SKILL.md` — the check step.
- `agf/docs/site/docs/operate/review-and-release.md` — adopter explanation.
- `.gitlab-ci.yml` — the `release:namespace-check` blocking job
  (`merge_request_event` + `agf/VERSION` change; consumes the
  `rust:tooling:preview` `agov` artifact; non-DinD, default dedicated
  runner; `JOB-TOKEN: $CI_JOB_TOKEN` only, no shell tracing); plus, if
  kept, a separate `release:namespace-consistency` post-tag job
  (consistency/non-overwrite, never absence). Trigger-scoped, path-scoped.
- the canonical command implementation under `tooling/` (provisionally
  `agov release namespace-check`) + its unit tests — runtime-derived
  identity (`$CI_PROJECT_ID` / configured release target with canonical
  `path_with_namespace` verification; package name from
  `agf/dist/release-manifest.yaml` `package.name`; OCI path/tag from
  `agov-oci-image.sh image-ref` + `$CI_REGISTRY_IMAGE`), exact-match
  semantics (`name`/`version == "v<X.Y.Z>"`/`package_type == "generic"`;
  OCI tag `<X.Y.Z>` un-prefixed), positive-access-before-absence per
  namespace, fail-closed on wrong-path / failed listing / `401/403/429/5xx`
  / timeout / malformed JSON / pagination or multi-match ambiguity. No
  namespace literal in the command contract.
- `tooling/rust/scripts/agov-oci-image.sh` — only if a thin `image-ref`
  consumer hook is needed; preferably no change (consume as-is).
- `docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml`
  — the six-surface manifest.
- negative-test fixtures for the gate-self-test.

**No** functional release-artifact, version, changelog, tag, or schema
change. No change to the maintenance-line or reconstruction exceptions
(they already have their own, stricter versions).

## Collision scan

- No open plan or implementation MR claims issue #120
  (`agov coord pending` clear; the only recent mention is the release-
  sequence coordination records). Re-run immediately before Gate 2.
- `docs/reviews/rule-projection/` has no existing `issue120` manifest.
- `.gitlab-ci.yml` `release:*` jobs: the new job is additive and
  path/trigger-scoped; it does not modify `release:paths-validate`,
  `release:self-sufficiency-selftest`, or `agov:oci:release`. It adds a
  read-only `needs:` on the existing `rust:tooling:preview` artifact (no
  change to that job).

## Dependency and sequencing

- This child plan's Gate 1 → implementation → exact-head reviews
  (contribution-auditor [non-author], infosec-sentinel, build-engineer) +
  the `## Negative test` → binding governor Gate 2 → merge.
- Per the v2.10 plan and `2emq`, the #120 child implementation and Gate 2
  **must merge before the v2.10 reversible preflight and release cut**.
  This plan's own preflight/publication uses the command set finalized
  here.
- RUW-1, RUW-2, RUW-3 are already merged. This is the sole remaining
  mandatory v2.10 work package before the release-metadata MR and tag.

## Test and acceptance strategy

Before Gate 1:

1. Independent contribution audit by a **non-author** session; infosec and
   build-engineer reviews; `advisory_consultation` rows flipped to
   `consulted` with record references.
2. `agov plan validate <this file>` and
   `agov plan projection-check --allow-planned --base-ref origin/main <manifest>`
   pass.
3. `git diff --check` passes.

After implementation:

1. Strict `agov plan projection-check --base-ref origin/main <manifest>`
   passes with the normative-owner row confirmed (no new owner file — the
   existing rule is the owner, so no `planned → existing` transition; the
   manifest declares it `updated`/`existing pointer sufficient`).
2. The `release:namespace-check` blocking job appears on the
   release-metadata MR / pre-tag pipeline and **fails** on the negative
   input; a `## Negative test` section with a reproducible transcript is in
   the MR. Any post-tag consistency job asserts consistency, not absence.
3. `agov validate` umbrella, adopter-docs, the shipped skill validator, and
   `git diff --check` pass at the exact head.
4. A table-top pass over: (a) all four namespaces absent → `result=ABSENT`,
   release proceeds; (b) a pre-existing generic package in
   `pending_destruction` status → check fails, names the status, no tag;
   (c) an API returns a non-200/404 → check fails as ambiguity, not
   absence; (d) the OCI name is taken → check fails, using the
   `image-ref`-derived name, no tag; (e) the reversible preflight runs the
   same check with the same result and burns no version.
5. Gate 2 evidence names the exact SHA and canonical pipeline.

## Risks and controls

- **Scope creep back into a maximal reconstruction ceremony.** Control: this
  plan inherits only the four-namespace *absence-proof set*, not the
  three-party re-derivation ceremony; the affected-file allowlist is
  explicit.
- **A brittle CI check that blocks releases on transient API flakes.**
  Control: the check distinguishes *absence* (404/empty) from *ambiguity*
  (5xx/timeout); ambiguity blocks but is diagnosable and re-runnable, and
  the operator preflight already exercised the same check earlier.
- **Gate weakening disguised as a fix later.** Control: `gate-self-test-rule`
  requires the negative test to be re-demonstrated on any future
  modification of this check; `simplicity-rule` bars a fast-track waiver
  for it.
- **The check's OCI name diverging from what publication uses.** Control:
  single-sourced through `agov-oci-image.sh image-ref` + the v2.9.0 A2
  `verify-ref` mutation-tested guard.
- **Author/reviewer collapse.** Control: the independent contribution audit
  is explicitly routed to a non-author session; disclosed in the
  provenance section and the `advisory_consultation` row.

## Rollback

Revert the rule subsection, the checklist item, the CI job, the skill step,
the adopter-doc paragraph, and the projection manifest together. The
canonical command may remain (unused) or be reverted. No release artifact,
version, or schema migration is involved. If reverted, the standard path
returns to having no required namespace-absence obligation and issue #120
reopens.
