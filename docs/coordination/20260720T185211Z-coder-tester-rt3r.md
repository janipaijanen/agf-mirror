---
kind: review
from: coder-tester
to: planner-architect
id: 20260720T185211Z-coder-tester-rt3r
timestamp: 2026-07-20T18:52:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T152054Z-planner-architect-q4dj.md
  - 20260720T152811Z-coder-tester-rqn4.md
topic: nutshell-flyer-ci-cd-pedagogy-addendum
requires-ack: false
artifacts_owned: []
---

# Review addendum: CI/CD framing for student flyer

Severity: INFO. The governor noted that third-year engineering bachelor
students can be assumed to know the CI/CD concept. A pedagogical AI lecturer
reviewed the flyer direction with that constraint and recommends using CI/CD
as a familiar mental model for **mechanical rule enforcement**, while keeping
human gate judgment explicit.

## Recommended flyer block

Suggested heading:

**CI/CD Checks The Paperwork, Humans Make The Decisions**

Suggested copy:

> AGF can plug into CI/CD to run rule checks automatically: schemas, required
> records, validation commands, and whether the ceremony is complete.
>
> CI/CD does not decide whether the design is good, ethical, or ready to ship.
> Students and reviewers make those judgments at the two human gates.

## Boundary wording

Use wording like:

- "CI blocks missing or invalid evidence."
- "CI/CD validates the mechanical rules."
- "Tools check that required records and validation commands exist and pass."

Avoid wording like:

- "CI approves."
- "CI governs."
- "CI ensures quality."

The student-facing division of labor should remain:

- **Student/human:** decides intent, acceptability, responsibility, and risk.
- **AI assistant:** prepares code, records, validation runs, and summaries.
- **Tools/CI:** check schemas, required records, tests, links, and configured
  contracts.

## Visual placement

Place the CI/CD note as a narrow side callout near the "AI does / tools check"
message, labeled something like **Automated Rule Enforcement**.

The callout should sit beside or under a simple flow:

```text
Student decides -> AI prepares work + records -> CI/CD validates rules -> Human gate decides
```

This lets students connect AGF to a concept they already know without
mistaking CI/CD for an approval or ethics engine.
