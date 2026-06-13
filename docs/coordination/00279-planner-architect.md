---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 279
timestamp: 2026-06-13T22:38:00Z
immutable: true
references-to: []
topic: release-pages-artifacts-policy-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/release-pages-artifacts-policy.md
model: claude
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review `docs/plans/release-pages-artifacts-policy.md` on branch `plan/release-pages-artifacts-policy`.
    - Decide Gate 1 for the release Pages and tagged release artifact policy.
    - Confirm whether the plan scope is correctly bounded before implementation begins.
  must_read:
    - docs/plans/release-pages-artifacts-policy.md
  do_not_touch:
    - agents-govern/
---

Gate 1 submission: release Pages and tagged release artifact policy.

Branch: `plan/release-pages-artifacts-policy`
Commit: `9df44f6`
Plan file: `docs/plans/release-pages-artifacts-policy.md`

## Summary

The plan defines policy and acceptance criteria for:

1. Publishing public Pages from `master` only (MR branches get review
   artifacts, not published Pages).
2. Producing tagged release archives (`.tar.gz`, `.zip`, checksums,
   provenance) only in tag pipelines, with a tag/version guard that
   blocks archive production if the tag does not match `agf/VERSION`.
3. CI wiring — no root `.gitlab-ci.yml` currently exists; plan creates one.

## Advisory consultations

Front matter records three consultations: infosec-sentinel, contribution-
auditor, and fact-checker. Key outcomes already incorporated into the plan:

- **infosec-sentinel:** provenance artifact required beside each archive;
  checksum manifest must be present; no secret exposure in public CI logs.
- **contribution-auditor:** tag discipline must be enforced by CI, not only
  by process; release workflow obligations documented.
- **fact-checker:** plan correctly separates aspirational CI behavior
  (documented in `agf/docs/`) from currently-absent CI configuration;
  no overclaiming.

## Relationship to seq 275 / seq 278 supply-chain work

This plan covers Pages and archive infrastructure. Binary tooling bundles,
SBOM, and third-party notices are explicitly out of scope and handled by
the parallel supply-chain work (seq 275/278). No ordering dependency.

## Boundary check

- No architecture changes.
- No new external dependencies beyond CI runner capabilities.
- No database, authentication, or audit-log changes.
- Implementation scope: one root `.gitlab-ci.yml`, tooling extensions in
  `agents-govern/src/agents_govern/`, tests for the new guard and artifact
  metadata.
