---
id: coord-scanner-resolution-fix
status: pending-gate1
topic: coord-scanner-resolution
gate1_approved_by: ""
gate1_approved_at: ""
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

Fix `agov coord status` and `agov handoff next` so explicit
`references-to:` links using bare coordination IDs resolve the referenced
handoff the same way links using `<id>.md` filenames do.

The immediate bug is in the Rust `agov` scanner path: `frontmatter_refs`
returns bare IDs unchanged, while descendant lookup keys are stored as
filenames with `.md`. The reference edge is dropped, so already-resolved
handoffs still appear pending.

## Scope

### In scope

- Normalize front matter `references-to:` values in
  `tooling/rust/crates/agov/src/main.rs` so readable IDs, ULIDs, and legacy IDs
  without `.md` are converted to `<id>.md` before descendant lookup.
- Preserve existing behavior for references that already include `.md`.
- Add or update Rust tests proving:
  - a later `decision-record` with a bare ID in `references-to:` resolves a
    handoff;
  - an existing `<id>.md` reference still resolves;
  - non-resolution `note` messages may act as transitive links but do not
    resolve by themselves.
- Scan and document the two existing `todo!()` stubs in
  `agov-coord/src/handoff_tracker.rs` and `agov-coord/src/channel_reader.rs`
  as out of scope for this fix unless the implementation must touch them.

### Out of scope

- Schema changes.
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
| `tooling/rust/crates/agov/src/main.rs` | Normalize `frontmatter_refs` outputs and add/update tests |
| `docs/coordination/<gate2-handoff>.md` | Gate 2 handoff after implementation |

## Test strategy

- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov`
- `cargo test --manifest-path tooling/rust/Cargo.toml`
- `./tooling/agov coord status --recent 10 --roles` on the repository after
  implementation should no longer list the stale handoffs that are explicitly
  referenced by `20260622T202034Z-governor-reviewer-8ma3.md`.
- `./tooling/agov plan validate docs/plans/coord-scanner-resolution-fix.md`
- `./tooling/agov coord validate docs/coordination/<handoff>.md`
- `git diff --check`

## Risks

| Risk | Mitigation |
|---|---|
| False resolution of unrelated text | Only normalize values already in structured `references-to:` front matter |
| Expanding governance semantics silently | Keep resolution-capable kinds unchanged: `ack`, `completion`, `decision-record` |
| Breaking existing `.md` references | Preserve names that already end with `.md` |
| Leaving `agov-coord` stubs confusing | Document scan results; defer stubs unless implementation requires them |
| Queue still noisy due other scanner bugs | Validate against the current cleanup decision record and report any remaining entries |

## Rollback

Revert the Rust scanner/test commit. No schema or coordination data migration is
required.
