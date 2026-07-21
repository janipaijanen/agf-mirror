---
id: coord-scanner-resolution-fix
status: approved
topic: coord-scanner-resolution
gate1_approved_by: "governor-reviewer merge of !338 (commit 552b52a); see decision-record 20260721T184921Z-governor-reviewer-mteh"
gate1_approved_at: "2026-07-21T17:36:08Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Local Rust coordination scanner behavior only. No credential handling,
      network access, authentication, authorization, secrets, or executable
      privilege boundary is changed.
  - role: contribution-auditor
    status: consulted
    reason: >
      Scope is constrained by governor-reviewer handoff
      20260622T202119Z-governor-reviewer-idra and coder-tester response
      20260622T200555Z-coder-tester-ttrp: explicit references resolve
      handoffs; no schema change, no reopen concept, no git-log inference, and
      no date cutoff or silent disappearance.
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: coordination scanner resolution fix

**Date:** 2026-06-23
**Role:** coder-tester
**Status:** PENDING GATE 1

---

## Intent

Finish the compatibility fix for `agov coord status`, `agov handoff next`, and
the coordination graph so exact references to historical malformed
20-character ULID-like filenames resolve the same way as sequence, valid
26-character ULID, and readable-ID references.

The original bare-ID normalization bug described by this plan was fixed in
June 2026. Current `normalize_coord_reference()` correctly handles sequence
filenames, valid 26-character ULIDs, and readable timestamp IDs. A narrower
legacy gap remains: the project briefly generated 20-character Crockford-
alphabet IDs such as `01KVK7J9N2AW8DJZQAG2`. Their files are immutable and
schema-valid historical records, but `looks_like_coord_filename()` rejects
references to them. The reference edge is discarded before descendant lookup,
so two already-resolved readable-ID handoffs still appear pending.

The defect and exact mechanism were already recorded in
`20260625T090052Z-coder-tester-s8np`. The 2026-07-21 coordination cleanup added
new exact acknowledgements and confirmed that no coordination-only workaround
can close the displayed entries while the normalizer drops those references.

## Scope

### In scope

- Extend `tooling/rust/crates/agov-coord/src/coord_validator.rs` reference-
  filename recognition with a **legacy compatibility-only** 20-character
  Crockford-alphabet form.
- Preserve all current sequence, 26-character ULID, readable-ID, `.md`-suffixed,
  bare-ID, and path-qualified reference behavior.
- Never generate a 20-character ID and never present it as a valid current
  ULID. Generation remains readable-ID by default, with valid 26-character
  ULID only on the legacy recovery option.
- Add unit and integration regression tests proving:
  - an exact historical 20-character filename reference is retained;
  - a later `ack`, `completion`, or `decision-record` resolves a handoff with
    that filename;
  - the known cleanup acknowledgements clear the two false-positive entries on
    a current repository scan;
  - current 26-character ULID, readable-ID, sequence, bare-ID, `.md`, and
    path-qualified cases remain accepted;
  - malformed non-Crockford and near-length strings remain rejected.

### Out of scope

- Schema changes.
- Accepting arbitrary 20-character strings or weakening readable/ULID ID
  generation rules.
- Renaming, editing, or replacing the historical coordination files.
- A `reopen:` field or reopen semantics.
- Git-log or merge-history inference.
- Date cutoffs or silent aging-out of old handoffs.
- Rewriting `agov-coord` stub modules.
- Changing coordination message kinds.
- Treating arbitrary `note` messages as resolution-capable.
- Broad handoff lifecycle redesign.

## Resolution semantics

For this slice, a handoff is resolved only when an explicit later coordination
descendant has a resolution-capable kind already used by the scanner:

- `ack`
- `completion`
- `decision-record`

`note` remains non-resolution-capable. It can carry a reference edge that
allows a later resolution-capable descendant to resolve the original handoff,
but the note itself does not close the handoff.

`review` is deferred. It may be useful later, but adding it now would expand
semantics beyond the minimal bug fix and deserves its own evidence.

## Affected files

| File | Change |
|---|---|
| `tooling/rust/crates/agov-coord/src/coord_validator.rs` | Recognize the historical 20-character Crockford filename form for reference normalization only; add focused unit tests |
| `tooling/rust/crates/agov/tests/` or existing coordination integration-test module | Prove legacy handoff resolution and current-format non-regression |
| `docs/coordination/<gate2-handoff>.md` | Gate 2 handoff after implementation |

## Test strategy

- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov-coord`
- `cargo test --manifest-path tooling/rust/Cargo.toml --workspace`
- `cargo clippy --manifest-path tooling/rust/Cargo.toml --workspace -- -D warnings`
- A locally built release `agov coord status --roles` on current main must no
  longer list `01KVK7J9N2AW8DJZQAG2` or `01KVK7DYM21GGQ7ZZ5RR` as pending.
- `./tooling/agov plan validate docs/plans/coord-scanner-resolution-fix.md`
- `./tooling/agov coord validate docs/coordination/<handoff>.md`
- `git diff --check`

## Risks

| Risk | Mitigation |
|---|---|
| Treating the malformed historical format as a current supported generator format | Name the predicate as legacy compatibility, document that generation is forbidden, and test generators remain unchanged |
| False resolution of unrelated text | Require exactly 20 Crockford characters before the role suffix and keep resolution kinds unchanged |
| Weakening current reference formats | Preserve and regression-test sequence, 26-character ULID, readable, bare, `.md`, and path-qualified cases |
| Hiding genuinely pending work | Only a later explicit `ack`, `completion`, or `decision-record` remains resolution-capable |
| Queue still noisy due another scanner bug | Validate against current main and report every remaining entry rather than adding an age cutoff |

## Rollback

Revert the Rust scanner/test commit. No schema or coordination data migration is
required.
