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

- **Git tag:** `glab api projects/:id/repository/tags/v<X.Y.Z>` — a 200 is
  non-absence (fail); a 404 is absence.
- **GitLab Release:** `glab api projects/:id/releases/v<X.Y.Z>` — same.
- **Generic package:** `glab api "projects/:id/packages?package_type=generic&package_name=<name>&package_version=v<X.Y.Z>"`
  paged in full, then repeated for **every** status that can occupy the
  name — `default, hidden, processing, error, pending_destruction,
  deprecated` — via `&status=<s>`. Any non-empty result at any status is
  non-absence.
- **OCI reference:** the reference comes from `agov-oci-image.sh image-ref`
  (never a hand-typed name; `verify-ref` from v2.9.0 A2 already rejects a
  non-byte-identical name). The registry is queried for that exact
  reference with the repository ID resolved fresh at check time.

**Fail-closed:** an unavailable status, an incomplete page walk, an
unenumerable state, a non-200/404 response, or any policy/API ambiguity is
treated as **failure, not absence**. The check prints `result=ABSENT` only
when every one of the four namespaces returned an unambiguous absence.

### 3. Operator preflight **and** a mechanically blocking CI check

Both, because the v2.8.0 defect was a hand-typed name with nothing invoking
the check:

- **Operator preflight:** the standard `## What the MR MUST verify`
  checklist gains one item — "namespace-absence proof (`agov release
  namespace-check` / equivalent) attached, `result=ABSENT` for all four
  namespaces at the exact intended tag." Runs in the reversible preflight
  and again immediately before the tag push.
- **Mechanical CI check:** a new job (provisionally `release:namespace-check`)
  runs on the release-metadata MR pipeline and the tag pipeline, invokes
  the single authoritative check above, and **blocks** on any non-absence
  or ambiguity. Its blocking authority is granted by an explicit clause in
  `release-via-merge-request-rule.md` (per
  `always/tool-authority-boundary-rule.md` — a check may only block when a
  named rule grants it that authority).

The forge-neutral canonical implementation is a named command
(provisionally `agov release namespace-check`); the CI wiring is a
removable adapter that invokes it (same shape as `agov coord
publication-check`).

### 4. `agov-oci-image.sh` single-sourcing

The OCI namespace check consumes `agov-oci-image.sh image-ref` for the name
and relies on the existing `verify-ref` mutation-tested guard (v2.9.0 A2,
MR !2198). No hand-typed OCI name enters the check. The repository ID is
resolved fresh at check time, not cached.

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
| Validation / drift evidence | **Implemented:** the `release:namespace-check` CI job + the `gate-self-test` negative test are the mechanical evidence; advisory where the check is operator-run in the reversible preflight. |
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
- `.gitlab-ci.yml` — the `release:namespace-check` job (release-metadata MR
  + tag pipeline triggers only; path-scoped).
- the canonical command implementation under `tooling/` (provisionally
  `agov release namespace-check`) + its unit tests.
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
  `release:self-sufficiency-selftest`, or `agov:oci:release`.

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
2. The `release:namespace-check` job appears in the release-metadata MR /
   tag pipelines and **fails** on the negative input; a `## Negative test`
   section with a reproducible transcript is in the MR.
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
