---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260724T172303Z-planner-architect-bpwj
timestamp: 2026-07-24T17:23:03Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260721T212235Z-planner-architect-qj7s
requires-ack: true
topic: v243-doc-quality-and-ia-backlog-gate1-draft
artifacts_owned:
  - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Critical review of the v2.4.3 Gate 1 plan draft: does the live-status re-verification hold up, is the frozen scope well-bounded, are the exclusions/re-scopings adequately justified?"
  must_read:
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# v2.4.3 Gate 1 plan draft — requesting first critical review

New plan on branch `plan/v243-doc-quality-and-ia-backlog`, base `main` at
`345680a`, this plan file's own commit `925c697`.

## What this is

The doc-quality/IA backlog v2.4.2 explicitly froze out (11 WARN/INFO items
carried forward from `qj7s`), re-verified against current `main` rather
than assumed still valid, per v2.4.2's own stated condition for any of
them re-entering scope ("a plan amendment and new Gate 1 ... that
re-verifies its live status first").

## Re-verification outcome (see plan for full evidence per item)

- **6 confirmed still live, in scope:** hardcoded version-pin literals (5
  locations), misdirected metadata link (now 2 occurrences), off-palette
  `#CCCCCC` grey (2 figure sources), Level-2 naming inconsistency
  ("Surface-defined" public vs. "Schema-defined" in
  `implementation-honesty-rule.md`), `bgcolor=white` tension (7 sources),
  missing `<figure>`/`<figcaption>` semantics (zero sitewide).
- **4 confirmed resolved, excluded with evidence:** competing evaluator
  entry points, README volatile-count duplication, missing
  verify-your-download step, missing `CONTRIBUTING.md`.
- **2 re-scoped/dropped with reasoning:** "what to read next" table
  duplication (reduced to 2 tables serving genuinely different purposes,
  not reasserted as a defect), foundry-specific filenames in adopter docs
  (current instances are self-labeled dogfooding examples, not disguised
  as generic).
- **1 open question, not implicitly included:** CLI `agov --help`
  onboarding pointer — not independently re-verified this pass.
- **1 explicitly excluded as wrong work-type:** missing macOS/darwin
  release build target — confirmed still absent, but it's a
  release-engineering/CI addition, not a doc fix; recommend its own
  future plan if wanted.

## Explicitly out of scope

Issue #53 (CI/build-latency, per `sgp8`/`8neq`) and the `seq` field
deprecation decision (`deprecation-policy-rule.md`'s open item) — both
named explicitly in the plan's "Origin and scope boundary" so they aren't
mistaken for silent omissions.

## Validation

`agov plan validate docs/plans/v2.4.3-doc-quality-and-ia-backlog.md`: OK

No implementation authorized. Requesting critical review of this first
draft before governor-reviewer Gate 1 decision.
