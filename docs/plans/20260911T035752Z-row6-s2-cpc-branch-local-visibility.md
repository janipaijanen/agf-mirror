---
id: 20260911T035752Z-row6-s2-cpc-branch-local-visibility
status: pending-gate1
topic: row6-s2-cpc-branch-local-visibility
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Adds an opt-in `--include-branch-local` flag to `agov handoff next`
      that scans other local branches' `docs/coordination` trees for
      candidate handoffs, reusing the same read path `agov coord
      visibility` already uses. InfoSec confirms this reads only the
      local checkout's already-fetched refs (no network call, no new
      credential surface), grants no authority, and cannot be used to
      inject a candidate that bypasses schema validation (parse failures
      are surfaced, not silently accepted).
  - role: contribution-auditor
    status: consulted
    reason: >-
      Bounded per the row-6 work order's explicit scope guard ("not an
      open-ended `handoff next` redesign"). Confirms this slice only
      widens the *candidate set* `handoff next` searches when the flag is
      set; it does not change selection, ranking, or default (no-flag)
      behaviour, which stays byte-identical to today.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 6 S2 — CP-C: branch-local obligation visibility for `agov handoff next` (Gate 1)

## Provenance

- Parent: `docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md`
  (v2.11 IN row 6, Gate 1 `tsbh` / ratified `q862`), slice **S2**.
  Governor "Keep all in v2.11" (2026-09-10) confirms row 6 stays a hard
  cut precondition.
- `jprj` §C: `agov handoff next <role>` reads only the checked-out base
  branch's `docs/coordination` tree (confirmed at
  `tooling/rust/crates/agov/src/main.rs:4118`,
  `read_channel(&dir)` on the resolved `base_branch` checkout) — a
  handoff on an unmerged branch is invisible to it; only
  `agov coord visibility` (which already does a multi-branch scan)
  surfaces it. This session hit it repeatedly (reviews landing on
  `codex/review-*` branches).

## Change

| File | Change |
|---|---|
| `agov handoff next <role>` (`tooling/rust/crates/agov/src/main.rs`) | New optional flag `--include-branch-local`. **Without it: byte-identical to today** (single `read_channel` call against the resolved base branch). **With it:** additionally invoke the same branch-enumeration + per-branch `docs/coordination` read that `agov coord visibility` already performs, merge the resulting candidates into the same selection logic `handoff next` already uses (no new ranking rule), and label each branch-local candidate with its source branch name in the output. |
| `AGENTS.md` "Tooling quick reference" + `agf/templates/AGENTS.md` | One-line pointer: `agov handoff next --include-branch-local` widens the search to other local branches' coordination trees; default behaviour unchanged. |

## Explicitly out of scope (parent work order guard)

No change to `handoff next`'s selection or ranking algorithm. No change
to which branch is authoritative for "ready" status. No new default
behaviour — the flag is opt-in, and its absence must produce output
identical to the pre-slice binary on the same inputs (fixture 1 below).

## Fixtures (ordinary test discipline; not a `gate-self-test-rule` trigger — advisory queue display, no MR-blocking check added)

1. **Default unchanged.** Same inputs, with and without the flag absent
   in both the old and new binary → identical output.
2. **Branch-local handoff surfaced.** A `requires-ack: true` handoff
   committed only on an unmerged local branch (not on the resolved base
   branch) → invisible without `--include-branch-local`; appears, labeled
   with its source branch, with the flag.
3. **No duplication.** A handoff present on both the base branch and a
   local branch (e.g. after a fast-forward) is not double-listed.
4. **Malformed branch-local candidate.** A schema-invalid file on a local
   branch does not crash the scan; it is reported as a parse failure
   (matching `agov coord visibility`'s existing behaviour), not silently
   dropped or silently accepted.

## Rule-projection screen

`docs/reviews/rule-projection/20260911T035752Z-row6-s2-cpc-branch-local-visibility.yaml`
— normative owner: `not applicable` (no MUST/blocking prose changes; this
is an opt-in `agov` discovery flag, same class as row-5 S1's `--session`
flag); adopter explanation: `not applicable`; runbook: `not applicable`;
AI loaders: `AGENTS.md` + `agf/templates/AGENTS.md` (`updated`, one-line
pointer), `.claude/CLAUDE.md` `not applicable` (`@../AGENTS.md` import);
validation/drift: `advisory only`; distribution: `agov` + `AGENTS.md`
templates ship in the AGF artifact.

## Rollback

Pure additive `agov` flag. Revert the MR → `handoff next` loses the flag;
zero behaviour change for any caller not passing it. No schema, no rule
text, no migration.

## Decision requested at Gate 1

Approve (a) the opt-in `--include-branch-local` flag reusing
`coord visibility`'s existing branch-enumeration and read path, (b) that
default (no-flag) behaviour stays byte-identical, (c) the 4 fixtures,
(d) the loader-pointer projection. Then coder-tester implements; own
Gate 2.
