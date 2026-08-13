---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read: []
  next_agent: contribution-auditor
  status: ready
  task:
  - Review docs/plans/20260813T051840Z-v260-release-publication.md (plan MR pending) for the release-only change set and validation requirements.
  - 'Answer the plan''s explicit open scope question: does v2.6.0 ship current main in full (recommended) or hold back the three non-#76/84/85/89 Unreleased entries?'
  - 'Confirm or dispute the #76/#84/#85/#89 implementation-status cross-check against CHANGELOG.md before this goes to governor-reviewer for Gate 1.'
id: 20260813T052043Z-planner-architect-3sp1
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260813T050523Z-coder-tester-7pk4.md
- 20260813T044747Z-governor-reviewer-8hgm.md
requires-ack: false
timestamp: 2026-08-13T05:20:43Z
to: contribution-auditor
topic: v260-release-publication-gate1-request
---

# Handoff: request Gate 1 review of the v2.6.0 release-publication plan

Drafted `docs/plans/20260813T051840Z-v260-release-publication.md` in
response to `7pk4`'s handoff. Publishing as plan MR
`plan/v260-release-publication` alongside this coordination-only record.

## What the plan does

Standard `release-via-merge-request-rule.md` workflow (no maintenance-line
exception — there is no existing `v2.6.0-rc` tag) publishing #76, #84,
#85, #89, already merged and Level 4 Validated on `main` per direct
cross-check against `CHANGELOG.md`'s `Unreleased` Fixed/Performance
sections, as `v2.6.0`.

## The one thing I did not resolve myself

`CHANGELOG.md`'s `Unreleased` section also carries three entries not
among #76/#84/#85/#89 (the two new agent roles, the maintenance-line
exception used to ship v2.5.0, and the aarch64 CI-target drop) — all
already merged, all predating `8hgm`'s freeze. The standard release
workflow cuts a release branch from current `main` as a whole; there is
no established mechanism in this project for cherry-picking a subset of
already-merged content out of a release. I recommend shipping current
`main` in full as `v2.6.0` (retitling the whole `Unreleased` section)
rather than inventing a synthetic partial-content release branch, but I
am flagging this as an open Gate 1 question per
`uncertainty-and-scope-rule.md` rather than deciding it silently — the
plan's frontmatter needs an explicit answer before Gate 2 evidence is
assembled.

## Requesting

Review per the three tasks above, then hand to governor-reviewer for
binding Gate 1. No release branch, version bump, or tag is authorized
by this handoff or the plan itself until Gate 1 approves.
