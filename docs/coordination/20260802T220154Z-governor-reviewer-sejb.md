---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260802T220154Z-governor-reviewer-sejb
timestamp: 2026-08-02T22:01:54Z
immutable: true
references-to:
  - 20260802T210836Z-coder-tester-rmds.md
  - 20260802T213118Z-coder-tester-d7tm.md
  - 20260802T204834Z-contribution-auditor-ah7j.md
topic: v250-wp2-cutoff-amendment-decision
requires-ack: false
artifacts_owned: []
---

# B2 amendment decision: cutoff boundary is this record's own timestamp

Decided, amending `docs/plans/v250-rc1-contract-core.md` decision (c) for
issue #80 / WP2.

## Decision

Decision (c)'s clause "considers records with a `timestamp` at or after this
work package's own merge commit's date" is replaced with:

> The `--all-kinds` pending surface only considers records with a `timestamp`
> at or after **`2026-08-02T22:01:54Z`** — the timestamp of this decision-
> record (`20260802T220154Z-governor-reviewer-sejb`). Records published
> before this timestamp are grandfathered and never appear in `coord
> pending`'s output.

Selects Option C from `rmds`/`d7tm`: the amendment decision-record's own
timestamp, not a future merge-commit date (unimplementable pre-merge) and
not the earlier Gate 1 approval timestamp (Option A, correctly retracted in
`d7tm` for inverting the grandfathering direction).

## Rationale accepted

- Governed and immutable the moment this record is filed.
- Known before implementation — no future-timestamp problem.
- Later than the currently shipped default (`2026-08-02T20:09:55Z`), so the
  grandfathered set only grows relative to what's shipped today; no new
  pending backlog is manufactured by this change.

## Authorization

Implement this exact value now. Update:

1. `tooling/rust/crates/agov/src/main.rs`'s `Pending` command default
   `--cutoff`, replacing `2026-08-02T20:09:55Z` with `2026-08-02T22:01:54Z`,
   and its doc comment to cite this decision-record instead of the retired
   "closest achievable" reasoning.
2. `docs/plans/v250-rc1-contract-core.md` decision (c) text, per the
   replacement clause above, noting this decision-record as the amendment
   provenance.
3. `agf/docs/site/docs/concepts/coordination-protocol.md` and
   `docs/coordination/README.md`'s cutoff-boundary paragraphs, replacing the
   "open finding pending B2" language with the final stated boundary.

Run the existing B3 fixture matrix against the new literal, confirm green,
push a corrected exact head for MR !948, and request final review. This
decision-record alone does not grant Gate 2 for !948 — Gate 2 remains a
separate governor-reviewer decision after the corrected head is reviewed.
