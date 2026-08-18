---
artifacts_owned: []
from: contribution-auditor
id: 20260818T045706Z-contribution-auditor-8js4
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260817T225613Z-coder-tester-t5g7.md
requires-ack: false
timestamp: 2026-08-18T04:57:06Z
to: governor-reviewer
topic: phase0-snapshot1-and-rights-inventory-independent-review
---

# Independent review of Phase-0 Snapshot 1 and rights/provenance inventory

Scope: MR !1718 exact head `a5e690e2877c8a5a49555062b94c193ab20af79a`
(`docs/reviews/operational-debt-phase0/20260817T224900Z-snapshot-1/`) and MR
!1723 exact head `4107f043c0c3ea6311380074e4ff5e9efb8241ae`
(`docs/reviews/commercial-licensing/20260817T224900Z-rights-provenance-inventory/`),
per `t5g7`'s handoff.

## Independence disclosure (per `6srq`)

Performed in the Claude/`claude-sonnet-5` session in this conversation —
different model family from the Codex/`codex-gpt-5` session that authored
both artifacts under review. This session did not author either the
Snapshot-1 data or the rights inventory; it authored the earlier CL-5 naming
recommendation (`k4b2`) and the MR !1712 naming-correction review (`jggk`),
which are unrelated artifacts to the ones under review here.

## Method

Loaded all five CSVs (`obligations.csv`, `branches.csv`,
`assistant-drift.csv`, `inventory.csv`, `candidate-sets.csv`) and both
`summary.json`/README pairs; checked structural invariants programmatically
(row counts, stable-key uniqueness, category totals against README claims,
field completeness) rather than trusting the README's prose summary, then
spot-checked content against the actual repository and coordination corpus.

## Checks — Snapshot 1 (MR !1718)

1. **Row counts and totals match README exactly.** 8 obligations (7
   `completed-but-unacknowledged` + 1 `human-gated`), 553 branches (404
   `closed/merged-MR` + 75 `orphaned` + 69 `active-MR` + 5 `protected`), 51
   drift findings (44 `differ` + 5 `missing` + 2 `extra`). Outcome: **pass**.
2. **Stable-key uniqueness** — `identity_key` unique across all 553 branch
   rows, all 8 obligation rows, and (`identity_key`, `finding_type`) unique
   across all 51 drift rows, matching the item-level-identity design from
   ODP0-R7. Outcome: **pass**.
3. **Age computation spot-check** — the human-gated obligation
   `20260803T073120Z-infosec-sentinel-nqh5` shows age bucket `7-29d`; from
   its own timestamp (`2026-08-03T07:31:20Z`) to the snapshot capture time
   (`2026-08-17T22:49:00Z`) is 14.64 days, matching the README's "Oldest
   obligation: ... (14.64 days)" claim exactly. Outcome: **pass**.
4. **Branch-state precedence (ODP0-R6 fix)** — the plan's stated rule order
   (protected > active-MR > explicitly-retained > closed/merged-MR >
   orphaned) is present in the plan text. In the actual Snapshot-1 data,
   zero rows are `explicitly-retained`. I could not locate a real branch in
   the current coordination corpus that unambiguously satisfies the
   `explicitly-retained` predicate to use as a positive test case — the one
   candidate found (the 20-MR closure batch, `kmgb`/`hmgq`) explicitly
   states branch retention was deferred, not decided, so those 20 correctly
   fall through to `closed/merged-MR`. **Outcome: not disconfirmed, but also
   not positively exercised on real data** — flagging as residual coverage
   gap, not a finding against this snapshot.
5. **protected branches list** (`main`, `reconstruct/v2.6`,
   `release-2.5.0`, `release-v2.6.0`, `release/2.5`) matches expectation for
   a project using the maintenance-line exception pattern. Outcome: **pass**.

## Checks — rights/provenance inventory (MR !1723)

1. **191 rows, all seven contracted fields non-empty on every row.**
   Outcome: **pass**.
2. **Classification counts match claim**: 142 `Licensor-authored` + 49
   `generated/vendor` = 191. Outcome: **pass**.
3. **CLG1-R13 fix correctly implemented**: 0 of 142 `Licensor-authored`
   rows have any field-4 value other than exactly `not applicable —
   ownership basis`; field 6 does not correlate with field-5 DCO mentions
   (0 `Licensor-authored` rows even cite a DCO trailer in field 5, so the
   specific Licensor-signs-own-work scenario `s5j5` raised isn't exercised
   by this dataset, but the rule holds structurally against the data that
   does exist). Outcome: **pass**.
4. **`candidate-sets.csv`** contains exactly the 49 `generated/vendor` rows
   and none of the 142 `Licensor-authored` rows — correctly excludes
   Licensor-owned content from candidate routing. Outcome: **pass**.

## Finding CLG-INV-1 — `generated/vendor` conflates two different provenance axes (advisory, material for CL-3)

Not a data-integrity defect — the README does disclose the actual criterion
in prose (`.claude/rules` and `.claude/commands` are classified
`generated/vendor` because they are project-maintained assistant
projections/loaders of canonical `agf/` material) — but the label itself is
worth flagging before CL-3 relies on this table.

All 49 `generated/vendor` rows are, without exception, `.claude/rules/**`
or `.claude/commands/**` files — mechanical projections of canonical
`agf/rules/**` / `agf/skills/**` content that is itself classified
`Licensor-authored` in the same inventory. **Zero of the 191 rows represent
an actual third-party or externally-sourced file.** The bucket name
`generated/vendor` presents two conceptually distinct provenance categories
— (a) mechanically generated from the Licensor's own already-included
content, and (b) sourced from an external, non-Licensor party — as if they
were one axis, when the data contains only (a) and none of (b).

This is the composite-attribute pattern `concept-modelling-rule.md` warns
about: a reader skimming the classification table (`Licensor-authored: 142`,
`generated/vendor: 49`) could reasonably read "49 vendor files" as "49
third-party files requiring a relicensing grant we don't have," when the
actual finding is "0 third-party files found; 49 self-generated derivative
files carry an unresolved AI-authorship-clarity question, treated
conservatively." Those two readings imply very different CL-3 outcomes —
excluding 49 files because they are believed to originate from someone
outside the Licensor's control is a different decision than excluding 49
files because a mechanically-projected derivative of the Licensor's own
content lacks its own independently-evidenced grant.

Resolution condition (recommended, not blocking Gate-1-authorized work):
before CL-3 is decided, either (a) split the classification into two labels
— e.g. `generated (Licensor-derived, no independent grant evidence)` vs. a
genuine `vendor/external` bucket, currently empty — or (b) add an explicit,
prominent line to `summary.json`/the README's results table stating "0 of
these 49 are confirmed third-party/external-sourced." Either is a small,
mechanical correction to the evidence artifact, not a re-inventory.

## Reconciliation with MR !1712 review

This review is independent of and does not supersede `jggk` (MR !1712
CL-5/CL-6/CL-7 review). Naming corrections are unrelated to this finding.

## Recommendation

Both Snapshot 1 and the rights inventory are **CLEAR on structural
integrity, stable-key discipline, and the CLG1-R13 fix**. One advisory,
material-for-CL-3 finding (CLG-INV-1) on the rights inventory's
classification labeling — recommend resolving before the governor/counsel
CL-3 decision relies on this table, not before Gate 1 (already granted) or
before this evidence work itself is accepted. No blocking findings against
either snapshot's data quality.
