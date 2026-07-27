<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Architecture Decision Records

Record significant architecture decisions using the template below.
See [framework.md Section 5.2](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/framework.md)
for the theory behind each required field.

## Quick start

```bash
cp docs/adr/_template.md docs/adr/YYYYMMDD-short-title.md
# Fill in all sections. Commit with status: Proposed.
# After review and approval, update status to: Accepted.
```

## Format

Each Architecture Decision Record (ADR) is a Markdown file named `YYYYMMDD-short-title.md` with these
required sections:

| Section | Purpose |
|---------|---------|
| **Status** | `Proposed`, `Accepted`, or `Superseded (by YYYYMMDD-...)` |
| **Context** | Forces at play — facts, not opinions |
| **Decision** | The chosen option, stated clearly |
| **Alternatives considered** | All options with pros/cons (including the chosen one) |
| **Counter-arguments sought** | Who was consulted for dissent, and what they said |
| **Strongest objection** | The best argument against, with rebuttal |
| **Consequences** | Positive and negative outcomes |
| **Confidence and reevaluation triggers** | How sure, and when to revisit |

## Lifecycle

1. **Proposed** — draft, open for review
2. **Accepted** — approved, in effect
3. **Superseded** — replaced by a newer ADR

Once accepted, an ADR file is **never modified** — not even to mark it
as superseded. This preserves a tamper-evident decision history.

### How supersession works

When a new decision replaces an old one:

1. The **new ADR** includes a `Supersedes` section linking to the old ADR
2. The **old ADR file** stays exactly as it was (immutable)
3. The **index below** is updated: the old ADR's status changes to
   `superseded by [new](link)`

The index is the living document; the ADR files are the immutable record.
Readers checking the index see the current state at a glance. Readers
checking a specific ADR file see the original reasoning, untouched.

## Index

| Date | ADR | Status |
|------|-----|--------|
| 2026-04-05 | [iron-gate adopter feedback — framework updates](20260405-iron-gate-adopter-feedback.md) | proposed |
| 2026-04-05 | [License change to CC BY-SA 4.0](20260405-license-cc-by-sa-4.md) | proposed |
