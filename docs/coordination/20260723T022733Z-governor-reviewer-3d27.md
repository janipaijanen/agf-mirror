---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260723T022733Z-governor-reviewer-3d27
timestamp: 2026-07-23T02:27:33Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T022449Z-governor-reviewer-qkks
topic: visible-assistant-attribution-on-gitlab
requires-ack: false
artifacts_owned: []
---

# Decision: make assistant identity visible on shared-account GitLab activity

Issue #58 must include a bounded assistant-attribution deliverable. GitLab
currently renders notes from Claude, Codex, and the governor under the same
account author, which hides the acting assistant in ordinary issue/MR reading.

The v2.4.2 plan must define and apply a concise visible convention for
assistant-authored GitLab notes and descriptions. The working convention is a
first heading or byline containing both execution identity and AGF role, for
example:

- `### Claude · planner-architect`
- `### Codex · contribution-auditor`
- `### Jani · governor-reviewer`

When a GitLab note bridges a durable coordination record, it also names that
record ID; the record's `from` and `model` fields remain the structured
provenance. Assistant identity and AGF role are separate concepts and must not
be conflated.

The plan must state the platform boundary: a byline improves attribution and
readability but is not authentication or non-repudiation. Distinct native
GitLab author names/avatars require separate GitLab user or service accounts
and credentials. That higher-cost option must be separately assessed for token
security, account lifecycle, bot disclosure, and maintenance before adoption;
v2.4.2 must not create accounts or credentials implicitly.

Acceptance evidence should demonstrate that a reader can distinguish Claude,
Codex, and human/governor contributions in issue and MR timelines without
opening raw coordination files, while retaining links to the durable audit
trail.
