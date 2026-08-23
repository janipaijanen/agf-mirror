---
id: adoption-version-drift-check
status: pending-gate1
topic: adoption-version-drift-check
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting independent contribution-auditor review before Gate 1.
      I (planner-architect) author this plan; the originating issue
      (#112, adopter `harbor-workshop`) reports an observation only, no
      proposed implementation was authored by any session of mine to
      co-author against.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Reads two already-repo-local plaintext strings
      (`adoption.framework_version` in a learning-record YAML file
      already tracked in version control, and `agf/VERSION`) and
      reports a mismatch. No credential, network call, or
      security-surface change. Same shape as the already-shipped
      `agov-version-drift-warning` check (issue #114).
---

# Plan: validate `adoption.framework_version` against `agf/VERSION`

Implements GitLab issue [#112](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/112),
an adopter learning record (`harbor-workshop`, 2026-08-10 to
2026-08-21) that has sat in the backlog with no plan since filing.

## The problem

An adopter's `learnings/<codename>.yaml` records
`adoption.framework_version`. After an upgrade updates `agf/VERSION`
but the learning record's `adoption.framework_version` field is not
also updated, `agov validate learnings` passes — the record is
well-formed YAML matching its schema — while the claimed version is
simply wrong. In the reported case this drift persisted for six weeks.

This is more than a stale field: `agf/skills/upgrade/SKILL.md`'s own
Prerequisites section states *"the current framework version is
recorded in the adoption section of `learnings/<codename>.yaml`"* — the
*next* upgrade's changelog-range computation trusts this field. A wrong
starting version means the wrong set of changes is presented at GATE 1
(Step 1 of the upgrade skill), the exact gate whose purpose is the user
reviewing what is about to change.

## Design

Add a validator check, `agov.validate.learnings` (or a new focused
check alongside it — implementer's call, matching whichever placement
keeps the existing module's cohesion, per the existing structure in
`tooling/rust/crates/agov-validators/src/learnings.rs`), comparing each
learning record's `adoption.framework_version` string against the
content of `agf/VERSION` at the repository root.

**Reported as a finding (blocking), not a warning** — per the issue's
own explicit request, and consistent with this project's existing
`agov-version-drift-warning` precedent (issue #114, already shipped):
that check treats binary-vs-tree version mismatch as advisory (a
running binary legitimately predates or postdates an arbitrary tree it
is pointed at), but this check is different — an adopter's own record
claiming a stale version for their own tree is not a legitimate steady
state, it is simply wrong, and it feeds directly into the next
upgrade's Gate 1 changelog-review evidence.

**Exception, per the issue's own suggestion:** if mid-upgrade drift is
a legitimate transient state (the field is updated in Step 4 of the
upgrade skill, after Steps 1-3 have already changed `agf/VERSION` on
disk), an explicit in-progress marker is needed rather than a silent
mismatch window. Resolve this precisely at implementation time by
reading the *actual* current `agf/skills/upgrade/SKILL.md` step
ordering (Step 3 applies file changes including `agf/VERSION`
presumably arriving via the archive copy; Step 4 updates the learning
record) — if Step 3 already updates `agf/VERSION` before Step 4 updates
the learning record, the window between them is real and this plan's
implementation MUST add an explicit `adoption.upgrade_in_progress: true`
marker (or equivalent) that suppresses the check only for that narrow
window, not silently.

## What this plan does NOT do

- Does not change the upgrade skill's step ordering.
- Does not touch the already-shipped `agov-version-drift-warning`
  (binary-vs-tree) check from issue #114 — different check, different
  comparison, different severity model.
- Does not address #96, #111, or #113 — tracked as separate plans.

## Files expected to change

This plan's own diff (Gate 1 candidate): only this plan file.

On the implementation branch, after Gate 1 approval:

- `tooling/rust/crates/agov-validators/src/learnings.rs` — new
  comparison against `agf/VERSION`, surfaced as a blocking finding.
- `agf/skills/upgrade/SKILL.md` Step 4 — if the transient-window
  exception above applies, document the in-progress marker.
- Focused unit tests (see "Test strategy").

## Test strategy

- Positive: learning record with `adoption.framework_version` matching
  `agf/VERSION` — passes.
- Negative: mismatch — fails with a blocking finding naming both
  values.
- If the in-progress marker is implemented: a record carrying it is
  exempted from the check regardless of mismatch; a record without it
  is not.
- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella): no new findings beyond pre-existing,
  unrelated warnings.
- `cargo test -p agov-validators`: must pass, including the new tests.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `tooling/**` — a trigger path.

```
Impact:
  Foundry:          ADDITIVE. This project's own adoption record gains
                     a new mechanical check; no existing validator
                     behavior changes for a record that is already
                     accurate.
  Direct adopters:  PREVENTIVE. An adopter whose learning record is
                     already accurate sees no change. An adopter whose
                     record has drifted (previously undetected) now
                     gets a blocking finding instead of silent
                     staleness — narrows previously-permitted silent
                     drift, does not remove any capability.
  Downstream:       No adopter-produced artifact shape changes; no
                     agent behavior toward an adopter's users changes.
  Migration:        Not required for accurate records. An adopter with
                     a currently-drifted record must correct
                     `adoption.framework_version` once, the same fix
                     the issue itself recommends.
  Evidence:         GitLab issue #112 (harbor-workshop, 2026-08-10 to
                     2026-08-21).
```

## Rollback

Pure additive validator check. Reverting is a one-commit revert with no
schema or data implications — no learning record's content depends on
this check existing.
