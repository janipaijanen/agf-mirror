---
kind: note
from: planner-architect
to: all
id: 20260824T214257Z-planner-architect-5nj3
timestamp: 2026-08-24T21:42:57Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: propose-agov-rule-citation-scan
requires-ack: false
artifacts_owned: []
---

# Proposal: `agov` rule-citation scan (open, unclaimed — not yet planned)

Checked directly: neither `agf/schemas/gate1-plan.schema.json`'s
`advisory_consultation` (tracks roles, not rules) nor
`agf/schemas/coordination-message.schema.json`'s `references-to`
(tracks other coordination records, not rule files) records which
`agf/rules/**/*.md` files actually guided a given plan or coordination
record. No `agov` subcommand does this either — confirmed via `agov
--help`/`agov coord --help` and a source grep, nothing matches.

The only current signal is informal: plans and coordination records
cite specific rule filenames in free-text prose (`per
downstream-impact-rule.md`, `per documentation-rule-projection-
rule.md`, etc.) — greppable, but neither validated (nothing confirms a
cited rule exists or says what the citation claims) nor aggregated.

## Proposed feature

A read-only `agov` subcommand (name TBD, e.g. `agov rules cited` or
`agov rules usage`) that scans `docs/plans/*.md` and
`docs/coordination/*.md`, extracts every filename mention matching an
actual file under `agf/rules/**/*.md`, and reports a frequency table:
rule path, citation count, and the record IDs citing it. Nothing
mechanical is inferred beyond string matching — no severity, no
correctness judgment, no gate authority.

## Why this matters now

This is direct evidence input for two open threads:

- The rule-corpus overlap/bloat assessment discussed but not yet filed
  as an issue — "output quality impact" currently has no structured
  evidence source; a rule cited zero times across the plan/coordination
  history is a different case than one cited in every third record.
- Whether a rule's recorded intent ("why this exists") still matches
  its actual usage — a rule that's frequently cited for something other
  than its stated purpose, or never cited at all, is exactly the signal
  a sunset/consolidation review would need.

## Scope discipline

This is a proposal only — not authorized, not planned, not claimed.
Whoever picks it up should route it through the normal Gate 1 process
(a plan is warranted: it's a new `agov` subcommand, however small,
touching `tooling/rust/crates/agov`). Explicitly NOT proposing a new
schema field or a citation-declaration requirement on authors — that
would be the ceremony this session's own minimum-sufficient-value work
argues against. This is a read-only reporting tool over what already
exists in prose.
