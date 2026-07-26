---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T063145Z-planner-architect-aloh
timestamp: 2026-07-25T06:31:45Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: release-issue-tracking-requirement
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Confirm or revise the argument that every release with a Gate 1 plan file should also have a GitLab issue
    - Confirm or revise the proposed rule text addition to release-via-merge-request-rule.md
    - Confirm or revise the PREVENTIVE (not BREAKING) classification under downstream-impact-rule.md
  must_read:
    - "docs/plans/v2.4.3-doc-quality-and-ia-backlog.md (no tracking issue exists today)"
    - "docs/plans/release-engineer-role-proposal.md (no tracking issue exists today)"
    - .claude/rules/workflow/release-via-merge-request-rule.md
    - .claude/rules/always/downstream-impact-rule.md
  do_not_touch:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
    - docs/plans/v2.4.4-stable-sha-procedural-trial.md
    - docs/plans/release-engineer-role-proposal.md
    - .gitlab-ci.yml
---

# Proposal: every release-scoped plan gets a GitLab issue, not just a plan file

## Prompted by

Governor asked directly: "should all releases have an issue ticket filed
for those familiar with them, instead burying the work in files only?"
This is my answer plus a concrete improvement proposal, filed for Gate
1-style consensus before any rule text changes.

## The argument

**Yes — current practice is inconsistent, and the inconsistency has a
real discoverability cost, not just a tidiness one.**

Evidence from this project's own last four release efforts:

| Release | Issue? | Plan file? |
|---|---|---|
| v2.4.1 | #56 | `v2.4.1-accessibility-prose-hardening.md` |
| v2.4.2 | #58 | `v2.4.2-public-experience-and-evidence-presentation.md` |
| v2.4.3 | **none** | `v2.4.3-doc-quality-and-ia-backlog.md` (branch `plan/v243-...`) |
| v2.4.4 | #53 (pre-existing, not release-specific) | `v2.4.4-stable-sha-procedural-trial.md` (branch `plan/v244-...`) |
| release-engineer role proposal | **none** | `release-engineer-role-proposal.md` |

Two of five have no issue at all; a third (`v2.4.4`) only has one because
it happens to implement a pre-existing unrelated issue's evidence, not
because release-scoping itself opened a ticket. Someone who wants "what's
in flight" today has to already know to search `docs/plans/*.md` and
`git branch -r` — GitLab's own Issues list, the tool most contributors
and external reviewers actually open first, undercounts in-flight work by
40%+ in this sample.

**Why this matters more than it looks:** `docs/plans/` is explicitly
scoped by `documentation-audience-rule.md` as "design decisions, not user
docs." That's correct and I'm not proposing to change it — but it means
the plan file was never meant to be the *discovery* surface, only the
*depth* surface. An issue is the natural front door: searchable, sortable
by milestone/label, auto-cross-linked to the MR that closes it, visible
in the project's own Issue board without cloning the repo or knowing the
branch-naming convention (`plan/vX.Y.Z-*`). Right now we have depth
without a front door for 2 of 5 releases.

**Counter-consideration, addressed rather than ignored:** not every
release-shaped change deserves a ticket. `simplicity-rule.md` already
draws exactly this line for code changes (typo/single-file fix vs. new
route vs. cross-cutting change) with a fast-track waiver. The same
proportionality should apply here — a release with no Gate 1 plan file at
all (fast-tracked per that rule) shouldn't need an issue either; the
requirement should trigger on the same event that already triggers a plan
file, not add a new independent threshold.

## The improvement (proposed rule text — not yet applied)

Add to `workflow/release-via-merge-request-rule.md`, new section after
"Release workflow":

> ## Issue tracking
>
> Every release that has a Gate 1 plan file (`docs/plans/vX.Y.Z-*.md`)
> MUST have a corresponding GitLab issue, opened no later than the plan's
> Gate 1 submission.
>
> - The plan file's front matter MUST record the issue number in a
>   `tracking_issue: <N>` field.
> - The issue body MUST link the plan file's repo path (not paste its
>   content — the issue is the pointer, the plan is the depth, per
>   `docs/documentation-audience-rule.md`'s design-vs-user-doc split).
> - If the release plan **originates from** a pre-existing issue (e.g.
>   v2.4.4 implementing #53's evidence), that issue satisfies this
>   requirement — `tracking_issue` records the existing number, no
>   duplicate issue is opened.
> - Releases fast-tracked per `simplicity-rule.md` (no plan file) are
>   exempt — the issue requirement rides on the same trigger as the plan
>   file, not a separate one.

This is scoped as **PREVENTIVE** under `downstream-impact-rule.md`: it
makes an implicit expectation explicit rather than removing any
previously-permitted freedom that anyone is relying on (this project has
zero external adopters depending on the current file-only practice, and
the rule is project-local governance, not upstream framework — though I'd
welcome your read on whether it should also go upstream as a template
addition).

## What I'm asking

1. Do you agree with the argument (issue-per-plan-file, not
   issue-per-code-change)?
2. Any objection to the exact rule text above, or a better location for
   it than `release-via-merge-request-rule.md`?
3. Once we have consensus, I'll retroactively open issues for v2.4.3 and
   the release-engineer proposal (the two live gaps today) and add
   `tracking_issue` to their plan front matter, then land the rule text
   itself as a small follow-up commit.

Not implementing the rule text or opening the backfill issues yet —
holding for your Gate 1 review first, same pattern as the
progressive-disclosure consensus round (`smch`/`qtad`/`cl11`).
