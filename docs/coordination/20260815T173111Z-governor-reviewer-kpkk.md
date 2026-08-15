---
kind: decision-record
from: governor-reviewer
to: all
id: 20260815T173111Z-governor-reviewer-kpkk
timestamp: 2026-08-15T17:31:11Z
model: human
immutable: true
references-to:
  - 20260815T170632Z-planner-architect-6h4p.md
  - 20260815T165311Z-governor-reviewer-4skj.md
  - 20260815T165601Z-coder-tester-bg2j.md
  - 20260814T074427Z-governor-reviewer-33br.md
  - 20260813T154648Z-governor-reviewer-2r8r.md
topic: v260-release-publication-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/20260813T051840Z-v260-release-publication.md
---

# Binding v2.6.0 eligibility, consultation-waiver, and Gate 1 decision

This record transcribes Jani's explicit in-session approval, “it matches.
please proceed,” of the self-contained three-part composite decision presented
after the reconciled handoff `6h4p`. The three decisions below are separate;
none is inferred from a platform merge action or assembled by combining
partial authority across earlier records.

## Decision 1: self-contained frozen-scope eligibility

For v2.6.0, the frozen feature scope is exactly GitLab issues **#76, #84,
#85, and #89**.

The immutable stable base is the annotated `v2.5.0` tag object
`b9215bd65aec59bea45f1576d789d00ca3f3e4ae`, peeled to commit
`f1932a1a486b772ca4353c7fa71c2279132fe0dc`.

The standard release workflow is excluded because current `main` contains
four unrelated approved change groups outside the frozen scope. The
maintenance-line workflow is excluded because no pre-existing
`v2.6.0-rc*` maintenance line exists.

Governor decision `2r8r` is the binding conditional allowlist authority for
exact commit `3456c1fa`. That commit may enter this v2.6.0 reconstruction only
if the bounded necessity procedure in Decision 3 produces its specified
disk-exhaustion or runner-capacity result. GitLab MR !1171's merge metadata is
platform provenance only; it does not originate or substitute for this
governor authority. No other prerequisite or out-of-scope commit is allowed.

## Decision 2: plan-specific semantic infosec waiver

The semantic infosec-sentinel consultation requested by this plan is waived
for MR !1492 at the exact head named below. The configured CI actor can supply
only its governed mechanical checks and cannot perform semantic review of the
release-lineage decision. All configured mechanical checks remain mandatory.

This waiver matches the boundary established by `33br`: it is plan-specific,
does not reattribute coder-tester evidence to infosec-sentinel, supplies no
substantive security approval, expands no role authority, and creates no
precedent for silently substituting future semantic consultations.

## Decision 3: exact-head Gate 1 approval and next authorized action

Gate 1 is approved for MR !1492 at exact head
`823bad6de8c28f996b65489a549062985ed99167`. Exact-head pipeline
`2762809467` succeeded. The approved plan is
`docs/plans/20260813T051840Z-v260-release-publication.md`.

Before creating any actual reconstruction branch or consuming the
`3456c1fa` exception, coder-tester is authorized and instructed to run the
approved bounded necessity procedure once:

1. From the exact `v2.5.0` peeled base, create an unprotected throwaway branch
   that is not merged anywhere.
2. Temporarily reinstate `aarch64-unknown-linux-musl` in the tag-simulating
   Rust tooling-release target loop, without cherry-picking `3456c1fa`.
3. Trigger the relevant tooling-release job manually on that non-protected,
   non-tag ref and publish the pipeline URL, exact SHA, runner class, and
   decisive log evidence in a coordination completion record.
4. A `no space left on device` or equivalent runner-capacity failure on the
   shared `saas-linux-small-amd64` runner conclusively activates `3456c1fa`.
   Reliable success stops execution and returns to the governor without that
   commit. Any unrelated, unavailable-runner, timeout, or ambiguous outcome is
   inconclusive and likewise returns to the governor.

This approval authorizes only Gate 1, the stated plan-metadata reflection, and
the bounded throwaway necessity procedure. It does **not** authorize Gate 2,
merging MR !1492, creating the reconstruction or release branches, version
bumps, tags, releases, publication, deployment, issue closure, or OpenClaw
work. Those actions remain behind their existing gates and explicit human
decisions.
