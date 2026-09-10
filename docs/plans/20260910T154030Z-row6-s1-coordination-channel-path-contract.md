---
id: 20260910T154030Z-row6-s1-coordination-channel-path-contract
status: pending-gate1
topic: row6-s1-coordination-channel-path-contract
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This slice changes `agov coord visibility`'s path scan and states the
      coordination-channel path contract normatively. InfoSec must confirm
      at Gate 1 that narrowing the scan to depth-1 `docs/coordination/*.md`
      cannot hide a live obligation an attacker or a careless author places
      just outside that set — the mandatory gate-self-test (a valid live
      record at a nested path still flagged as misplaced) is the control.
      No credential or protected-content surface.
  - role: contribution-auditor
    status: consulted
    reason: >-
      The two channel readers (`agov coord visibility` via `git ls-tree -r`
      and `agov-coord::channel_reader::read_channel` via `std::fs::read_dir`)
      currently disagree on what "the channel" is. This slice makes them
      agree and writes the contract down. Contribution-auditor confirms the
      contract is "directly under `docs/coordination/`", not "anything not
      in the archive", and that `read_channel`'s existing behaviour is the
      reference (no behaviour change there), and that the gate-self-test
      proves the narrowing does not swallow a genuinely mislocated record.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 6 S1 — coordination-channel path contract (Gate 1)

## Provenance

- Parent: `docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md`
  (v2.11 IN row 6, Gate 1 approved `tsbh` / ratified `q862`), slice **S1**.
- Evidence: `agov coord visibility` scans **recursively**
  (`tooling/rust/crates/agov/src/main.rs:4650`:
  `git ls-tree -r --name-only <ref> -- docs/coordination`, plus the
  range-diff scan), while `agov-coord::channel_reader::read_channel`
  (`channel_reader.rs:13`: `std::fs::read_dir`) scans **one directory,
  non-recursively**. The B4 archive at `docs/coordination-archive/` was
  the visible instance (39 false hidden-obligation results, `3jh6`); the
  divergence is general and still latent for any nested path.

## The contract (normative, this slice writes it down)

**"The coordination channel" is exactly the set of `*.md` files directly
under `docs/coordination/` — non-recursive.** Nested subdirectories
(`docs/coordination/<anything>/…`) and sibling paths
(`docs/coordination-archive/…`, etc.) are **not** the channel. Both
`agov coord visibility` and `agov-coord::channel_reader::read_channel`
apply this definition identically.

## Change

| File | Change |
|---|---|
| `tooling/rust/crates/agov/src/main.rs` (`visibility_scan` and its helpers, ~L4640-4700) | Filter the `ls-tree -r --name-only` output and the range-diff path list to entries whose path is `docs/coordination/<name>.md` with **no further `/`** after the coord-dir prefix. Non-`.md` and nested entries are dropped from the channel set — but a nested entry that parses as a **valid live coordination record** is retained on a separate "misplaced" list and surfaced as a warning (see gate-self-test). |
| `agov-coord::channel_reader::read_channel` | **No change** — it is already the reference behaviour (depth-1, `.md` only). Add a doc-comment pointing at the contract. |
| `agf/rules/workflow/coordination-publication-rule.md` | Add one sentence stating the path contract explicitly (it already says coordination records are append-only `docs/coordination/*.md`; make the *depth* explicit and name it the shared contract both readers honour). |
| `AGENTS.md` "Coordination channel" section + `agf/templates/AGENTS.md` | One-line pointer: "the channel is `*.md` directly under `docs/coordination/`; nested paths and `docs/coordination-archive/` are not the channel." |

## Gate-self-test (mandatory — parent work order, `gate-self-test-rule`)

The implementing MR includes and shows the output of:

1. **Misplaced live record still flagged.** A schema-valid, live
   coordination record (a `handoff` with `requires-ack: true`) committed
   at `docs/coordination/sub/20260101T000000Z-planner-architect-test.md`
   (nested) and at `docs/coordination-typo.md` (sibling). `agov coord
   visibility` must **still report each as a misplaced live obligation**
   (a distinct warning line + non-zero contribution to the exit code),
   not silently drop it. Proves the narrowing is "depth-1 is the
   channel", not "ignore anything not top-level".
2. **Correctly-placed record: both readers agree.** A record at
   `docs/coordination/<ts>-<role>-<sfx>.md` is seen identically by
   `agov coord visibility` and `read_channel` (same id, same
   live/superseded classification).
3. **Archive excluded, no false positive.** With the B4 archive present
   at `docs/coordination-archive/20260909-b4/`, `agov coord visibility`
   reports **zero** hidden obligations from it (the `3jh6` regression
   shape), and the misplaced-record list is empty for that path.

## Not a gate

`agov coord visibility` is advisory discovery output, not an MR-blocking
check — but it feeds the poll contract, and S3's `collision-check` is a
sibling. The gate-self-test above is required because the parent work
order names it, not because `visibility` blocks a merge.

## Rule-projection screen

`docs/reviews/rule-projection/20260910T154030Z-row6-s1-coordination-channel-path-contract.yaml`
— six rows: **normative owner** `coordination-publication-rule.md`
(`updated`); **adopter explanation** the coordination-protocol Pages page
(`existing pointer sufficient` — it already describes the channel;
one-line depth note if the reviewer judges it needed); **runbook** `not
applicable` (no procedure changes); **AI loaders** `AGENTS.md` +
`agf/templates/AGENTS.md` (`updated`, the one-line pointer), `.claude/CLAUDE.md`
`not applicable` (`@../AGENTS.md` import); **validation/drift** `advisory
only` — the gate-self-test is the enforcement, no standing mechanical
check; **distribution** `coordination-publication-rule.md` and
`agf/templates/AGENTS.md` ship in the AGF artifact; `AGENTS.md` is
foundry-local.

## Test and acceptance

- Rust unit tests for the depth-1 path filter (top-level `.md` kept;
  nested `.md` moved to misplaced list; sibling path excluded;
  non-`.md` ignored).
- `agov coord visibility` integration test reproducing gate-self-tests 1-3.
- `read_channel` tests unchanged and still green (no behaviour change).
- Exact-head contribution + InfoSec review; binding Gate 2.

## Rollback

Revert the MR: `visibility_scan` returns to the over-broad recursive
scan. `read_channel` unaffected. The rule sentence and loader pointers
revert with it. No migration.

## Decision requested at Gate 1

Approve (a) the path contract as stated ("depth-1 `*.md` under
`docs/coordination/`, both readers"), (b) the misplaced-live-record
warning behaviour (narrowing, not blanket exclusion), (c) the three
mandatory gate-self-tests, and (d) the rule + loader projection. Then
this slice goes to coder-tester for implementation and its own Gate 2.
