---
id: poll-and-coordinate-skill
status: pending-gate1
topic: poll-and-coordinate-skill
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting Codex's independent review — Claude authors both plan
      and content here.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Documentation and skill-authoring only; no code, no schema, no
      credential or security-surface change.
---

# Plan: `agf/skills/poll-and-coordinate/` — a single invokable skill for
# the coordination poll cycle, plus two corrections to the canonical
# adopter-facing procedure it operationalizes

Human governor requested this directly ("we need to make those an
agf-level single skill or command") after this session repeatedly
hand-wrote poll instructions inline in scheduled-wakeup prompts rather
than invoking one canonical, reusable procedure — and after two real
poll-process gaps surfaced this session (learning
`20260823T081929Z-gkhn`): a stale local checkout read instead of the
freshly fetched ref, and an unpaginated MR list silently truncated to 30
results, both hiding real work for multiple ticks.

## What already exists (verified before writing this plan)

`agf/docs/site/docs/operate/poll-and-coordinate.md` is already the
canonical, adopter-facing, "Released" procedure for this exact cycle —
comprehensive on scope (coordination, MRs, issues, pipelines, cadence,
terminal outcomes) but silent on the two specific failure modes this
session just hit. There is no `agf/skills/` entry that operationalizes
it into an invokable step-by-step skill the way `upgrade/` and
`learning/` operationalize their respective workflows — every poll this
session ran from ad-hoc inline prompt text instead.

## Decision requested

1. Patch `poll-and-coordinate.md` (the single canonical source per
   `marketing-tone-rule.md`'s message-integrity principle — corrections
   land there, not duplicated elsewhere) with a new "Common pitfalls"
   subsection under step 3/4 naming both failure modes concretely, with
   the fix for each.
2. Add `agf/skills/poll-and-coordinate/SKILL.md`, matching the existing
   `upgrade`/`learning` skill shape (YAML frontmatter with `name`,
   `origin: upstream`, `description`; numbered steps; explicit gates),
   operationalizing the canonical doc into an invokable procedure rather
   than duplicating its content — the skill points to and follows the
   doc, it does not restate it.
3. Add one row to `agf/skills/README.md`'s skill table.

## Design: what the skill adds beyond the existing doc

The doc describes the *what* and *why* (scope, cadence semantics,
terminal-outcome definition). The skill adds the *invokable procedure*:
a single entry point (`Skill: poll-and-coordinate`) that:

- Names the exact commands to run, in order, for the two GitLab-specific
  pitfalls the doc's generic "GitLab adapter" section doesn't spell out
  mechanically:
  - Local-file governance checks (`agov coord status`, `agov coord
    pending`, etc.) run from a working tree just reset to the freshly
    fetched ref (`git fetch` + `git reset --hard origin/<branch>` in a
    disposable worktree, or equivalent) — never from a persistent
    checkout that may be on an unrelated branch or stale relative to the
    fetch.
  - MR/issue listing via `glab api ... --paginate` (or an explicit
    `per_page` large enough for the known backlog size) — never a bare
    `glab mr list` with its silent default page cap.
- States the same act-or-reschedule discipline already in this session's
  memory (`feedback-polling-timer-reset`), generalized: classify findings,
  act within role/gate authority, or record the concrete blocker, then
  either continue the cycle or stop.
- Is explicitly the *reusable* procedure any future poll tick invokes
  by name, replacing inline ad-hoc prompt text with "invoke
  poll-and-coordinate, watching X and Y" — matching how `upgrade` and
  `learning` are already invoked by name rather than re-explained each
  time.

## Why this stays a skill, not a rule or a validator

This is a *procedure* an agent follows when polling — the same category
as `upgrade`/`learning`, not a *constraint* on already-decided content
(a rule) or a *mechanical pass/fail check* (a validator). No blocking
authority is claimed or implied; per `tool-authority-boundary-rule.md`
a skill is advisory guidance for the agent invoking it, same as every
other skill in `agf/skills/`.

## Files expected to change

- `agf/docs/site/docs/operate/poll-and-coordinate.md` — one new
  subsection naming both pitfalls concretely.
- `agf/skills/poll-and-coordinate/SKILL.md` — new skill file.
- `agf/skills/README.md` — one new table row.

No code, no schema, no migration, no new dependency.

## Test strategy

Documentation/skill authoring — no unit tests apply. Verification is:
`agov validate` passes (adopter-docs parity, no schema violations);
`agov plan validate` passes on this plan; manual read-through confirming
the skill's steps match the canonical doc's scope exactly (no drift
between the two).

## Rollback

Pure addition plus one doc subsection; revert is a one-commit revert
with no data or state implications.
