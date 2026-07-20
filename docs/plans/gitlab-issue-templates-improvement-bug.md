---
id: gitlab-issue-templates-improvement-bug
status: pending-gate1
topic: gitlab-issue-templates
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: static markdown template files only; no secrets, code, or security-relevant content
  - role: contribution-auditor
    status: not-applicable
    reason: internal tooling/process artifact, not an external contribution
---

# Plan note: add `improvement.md` and `bug.md` GitLab issue templates (issue #31)

**Date:** 2026-07-20T07:55:03Z
**Status:** fast-track — process note, not a full Gate 1 plan (per
simplicity-rule: "two template files" = fix/edit-directly tier)

## Why this is fast-tracked, not a heavyweight plan

Two new static markdown files, additive, no code, no schema, no migration,
no security surface. This matches the simplicity-rule's "Typo, comment, doc
update" / "single-file bug fix" tier. This note exists only to record the
scope decision and confirm the target files don't collide with anything
already present, per this repo's plan-file discipline (docs/plans/ is where
design decisions are timestamped, even small ones).

## What already exists (checked before planning, to avoid duplication)

`.gitlab/issue_templates/` already contains four templates: `Bug Report.md`,
`Rule Proposal.md`, `Learning.md`, `External Review.md`. These are scoped
narrowly to **upstream AGF framework contribution** workflows (reporting a
docs/guidance bug in the framework itself, proposing a new rule, submitting
a learning record, external fit-assessment reviews) — not general AGF-NG
project issue triage for arbitrary features/bugs.

Issue #31 asks for `improvement.md` and `bug.md` (lowercase, no space) with a
materially different structure: `bug.md` covers Environment (AGF+agov
version, install method, OS/arch, agent runtime), Component, Steps to
reproduce, Expected, Actual, Suspected layer (code defect | spec ambiguity |
agent misinterpretation | unknown) — broader than the existing `Bug
Report.md`, which is scoped only to documentation-guidance bugs ("incorrect
guidance, broken cross-reference, stale count"). `improvement.md` has no
existing counterpart at all (`Rule Proposal.md` is a different, narrower
artifact — a rule to submit upstream, not a general improvement idea for
this project).

**Conclusion:** no collision. The two new templates fill a real gap
(general project issue intake) distinct from the four existing
framework-contribution templates. No rename or consolidation of the
existing four is in scope.

## Affected files

| File | Action |
|---|---|
| `.gitlab/issue_templates/improvement.md` | Create |
| `.gitlab/issue_templates/bug.md` | Create |

## Content shape (from the issue, English only)

**`improvement.md`:** Summary / Problem / Proposal / Scope / Evidence status
(theoretical \| empirically informed \| validated) / Lock-in impact (does
this change make AGF/agov harder to remove?) / Alternatives considered /
Acceptance criteria. Apply label via GitLab quick action (`/label
~improvement`).

**`bug.md`:** Summary / Environment (AGF+agov version, install method,
OS/arch, agent runtime involved) / Component / Steps to reproduce / Expected
/ Actual / Suspected layer (code defect \| spec ambiguity \| agent
misinterpretation \| unknown). Apply label via GitLab quick action (`/label
~bug`).

Labels `~improvement` and `~bug` already exist (confirmed created earlier
this session) — no label creation step needed.

## Migration needed

No.

## Test strategy

Manual verification only (no test framework applies to static GitLab
templates):

- Both files present on the default branch.
- Both selectable in GitLab's "Choose a template" issue-creation dropdown
  (GitLab auto-discovers `.gitlab/issue_templates/*.md`; no registration
  step needed beyond the file existing).
- Visual check: quick-action label lines use correct syntax
  (`/label ~improvement`, `/label ~bug`) on their own line so GitLab parses
  them as quick actions rather than literal text.

## Risks

None of note. Static content addition.

## Rollback

Delete the two files.

## Out of scope (per issue #31)

MR templates, `CONTRIBUTING.md` changes.
