---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260907T173329Z-planner-architect-3g2g
timestamp: 2026-09-07T17:33:29Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T144721Z-coder-tester-pgm4.md
  - 20260907T140640Z-governor-reviewer-smem.md
  - 20260907T151325Z-governor-reviewer-2emq.md
  - 20260907T045703Z-coder-tester-jqrm.md
topic: v210-ruw4-issue120-child-plan
requires-ack: true
artifacts_owned:
  - docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md
  - docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of the RUW-4 / issue #120 child plan: adding a required pre-tag four-namespace absence obligation to the standard release path in release-via-merge-request-rule.md."
    - "Before Gate 1 can be issued: route the independent contribution-auditor audit to a NON-author session (this plan is authored by a Claude session; the normal contribution-auditor Claude cannot audit its own plan), plus infosec-sentinel and build-engineer exact-head reviews. The three advisory_consultation rows are marked waived-pending and must become consulted first."
  must_read:
    - docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md
    - docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml
    - docs/coordination/20260907T144721Z-coder-tester-pgm4.md
  do_not_touch:
    - "Implementation, tests, .gitlab-ci.yml, release metadata, tags, packages, OCI publication, or deployment before child Gate 1 approval"
    - "Propeller MR !2508 or Stage 4 activation surfaces"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# RUW-4 / issue #120 child Gate 1 plan — ready for review

This discharges the `pgm4` handoff to planner-architect. The planner-architect
lane (`jqrm` temp allocation, Codex) did not action `pgm4` for ~2.5 hours and
RUW-4 is on the v2.10 release-cut critical path, so the governor directed this
Claude session in-session (2026-09-07, "create gate1 plan for #120") to author
the child plan. Per the `s5gs` role-assumption handshake this is a direct
governor assignment, acknowledged in the plan's provenance section.

## What the plan decides

A **required pre-tag four-namespace absence obligation** on the **standard**
release path — not only the maintenance-line and frozen-scope-reconstruction
exceptions, which already carry their own stricter versions. Every future
release proves Git tag, GitLab Release, generic package (every status,
paginated fully), and the exact `agov-oci-image.sh image-ref`-derived OCI
reference are unoccupied before an immutable tag is pushed; a pre-existing
object stops the release.

## Resolution of the six `pgm4` points

1. **Full four-namespace set**, in the standard path's proportionate form —
   inherits the reconstruction exception's absence-proof set and fail-closed
   semantics, not its three-party re-derivation ceremony.
2. **Authoritative reproducible command/API set** defined (glab api for tag /
   release / generic-package-per-status; registry query for the helper-derived
   OCI ref), **fail-closed** on any unavailable status, incomplete pagination,
   unenumerable state, non-200/404 response, or API ambiguity —
   `result=ABSENT` only when all four are unambiguously absent.
3. **Both** an operator preflight checklist item **and** a mechanically
   blocking CI check (`release:namespace-check`), the CI check's blocking
   authority granted by an explicit clause in the owner rule per
   `tool-authority-boundary-rule`.
4. **Single-sourced** through `agov-oci-image.sh image-ref` + the existing
   v2.9.0 A2 `verify-ref` mutation-tested guard; repository ID resolved fresh.
5. **Required negative test** per `gate-self-test-rule` in the implementation
   MR: a pre-existing object in **each** governed namespace (including a
   non-default generic-package status, to exercise pagination) must stop the
   standard path before any immutable tag, with a reproducible `## Negative
   test` transcript.
6. **Six-surface projection manifest** filed at
   `docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml`
   (owner = existing release rule, updated; skill + operator page updated;
   AI-loaders not-applicable with reasons; CI job + negative test = drift
   evidence; distribution confirmed). `agov plan validate` and
   `agov plan projection-check --allow-planned --base-ref origin/main` both
   pass.

## Also in the plan

- Exact affected-file / test allowlist (owner rule, release skill, operator
  page, `.gitlab-ci.yml` job, canonical `agov release namespace-check`
  command + unit tests, projection manifest, negative-test fixtures). No
  functional release-artifact, version, changelog, tag, or schema change.
- Collision scan (no plan or MR claims #120; no existing `issue120` projection
  manifest; the new CI job is additive and trigger-scoped).
- Rollback: revert rule subsection + checklist item + CI job + skill step +
  adopter paragraph + manifest together; no release artifact or schema
  migration involved.
- Issue #120 open question 3: a **new append-only** coordination record (not
  an edit) should supersede the accepted `gs18` v2.8.0 disposition, recording
  that the v2.8.0 gap was "unrequired on that path", not only "wrong
  spelling". That record is a governor/auditor artifact filed alongside this
  Gate 1, not plan content.

## Independence note

Authoring a Gate 1 plan is planner-architect work; the independent
contribution audit of this plan **must** come from a non-author session. Codex
authored the parent v2.10 plan and owns the RUW-4 implementation lane, so a
fully independent third reviewer is preferable — the governor names the
auditor. Disclosed in the plan and in the `advisory_consultation` row.

## Sequencing

Per `2emq` and the v2.10 plan, the #120 child implementation and its Gate 2
must merge **before** the v2.10 reversible preflight and release cut. RUW-1,
RUW-2, RUW-3 are merged; this is the sole remaining mandatory v2.10 work
package before the release-metadata MR and `v2.10.0` tag.
