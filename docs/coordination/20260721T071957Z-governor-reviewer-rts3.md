---
kind: decision-record
from: governor-reviewer
to: all
model: human-governor-via-claude-session
id: 20260721T071957Z-governor-reviewer-rts3
timestamp: 2026-07-21T07:19:57Z
immutable: true
references-to:
  - 20260721T004357Z-governor-reviewer-e5ba
  - 20260721T064902Z-coder-tester-2hsp
  - 20260721T071426Z-coder-tester-j5s2
topic: contribution-auditor-trigger-wording-supersedes-e5ba
requires-ack: false
artifacts_owned: []
---

# Supersedes e5ba's Contribution Auditor trigger clause: "external" is dropped

The human governor, in this active session, reviewed the "external"-vs-
"upstream or downstream" wording tension directly and approved dropping
"external" entirely, independently of and prior to seeing `j5s2`'s report
of the same clarification reaching a different concurrent session. Two
independent sessions converging on the same correction from the same
governor is treated as corroborating, not conflicting, evidence.

## What this supersedes

`e5ba` (2026-07-21T00:43:57Z) recorded the Gate 1 approval of
`adopter-applicability-catalog` revision 3 and, in the same paragraph,
resolved the Contribution Auditor trigger discrepancy in favor of "first
external contribution." That specific clause — and only that clause — is
superseded here. `e5ba`'s Gate 1 approval of the plan itself, the
four-question semantics model, and everything else in that record stands
unchanged.

`2hsp`'s Gate 2 review was correct to flag MR !289's mutable
implementation as disagreeing with the only durable decision-record that
existed at the time it was written (`e5ba`). This record is the durable
correction `2hsp` asked for.

## Why "external" is dropped, not just reworded

"External" collides with `always/external-anonymization-rule.md`'s own,
different, already-defined meaning of "external" (the codebase-publication
boundary: issues, MRs, LLM API calls crossing outside the project). Reusing
the same word here for "an unfamiliar contributor" was importing a term
this project's own rule corpus had already claimed for something else.

## Settled wording

`j5s2`'s proposal correctly identified a second problem beyond the
"external" collision: "upstream or downstream" is this framework's own
internal shorthand, and a bare `activation_condition` field value read out
of context (by tooling, or by an adopter who hasn't yet absorbed that
vocabulary) shouldn't require it. But `j5s2`'s proposed field value
("Activate when the first contribution is submitted to either the
framework or an adopter project") is a full sentence, inconsistent with
this schema's established terse style for `activation_condition`
("always", "project has a CI pipeline", "project submits learning
records").

Settled, terse, self-explanatory, no jargon required:

    activation_condition: "first contribution to the framework or an adopter project"

Explanatory prose (comments, docs page prose) may retain the directional
shorthand for readers who already have it: "upstream (to the framework) or
downstream (to an adopter project)."

## Required correction

Coder-tester: update all four locations flagged by `2hsp` and `j5s2` to
the settled wording above — `agf/agents/contribution-auditor.yaml`
(header comment and `activation_condition` field), `agf/framework.md`,
`agf/ADOPTING.md`, `agf/docs/site/docs/adopt/applicability-catalog.md`.
Remove "pending governor reconciliation" / "open wording question"
language now that this record exists. Push a new head so CI evidence
covers the corrected content (per `2hsp`'s BLOCK 2 — note the two
coordination-only commits since the last content commit correctly did not
trigger new pipelines per `.gitlab-ci.yml`'s documented
`coordination:`-commit-title skip rule; the next content-touching commit
will).
