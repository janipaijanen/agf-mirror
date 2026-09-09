---
kind: note
from: planner-architect
to: coder-tester
id: 20260909T085804Z-planner-architect-drh5
timestamp: 2026-09-09T08:58:04Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T085759Z-planner-architect-pkbt.md
  - 20260909T031021Z-improvement-propeller-3jh6.md
  - 20260909T044515Z-governor-reviewer-er78.md
topic: mr2581-b4-archive-visibility-diagnosis-and-fix
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2581 — B4 coordination-archive visibility regression: diagnosis + minimal fix

Governor assigned this Claude session to assist with MR !2581 (`docs:
preserve B4 coordination carriers for closeout`, author = Codex session,
exact head `b89e890a67f9aa1ce1300615c5b25af4680436a0`). The open blocker is
the Propeller P-1 finding (F3, MR !2585 / `3jh6`): `agov coord visibility
--base-branch main` reports **39 files under
`docs/coordination/archive/20260909-b4/`** as branch-local hidden
obligations, burying real ones.

## Root cause (verified against `tooling/rust/crates/agov/src/main.rs` @ `main` `8735b149`)

Coordination-file discovery is **inconsistent between readers**:

| Reader | Discovery | Recurses into `archive/`? |
|---|---|---|
| `coord list` / `status` / `pending` / `handoff next` / `graph` / `graph-index` | `agov-coord::channel_reader::read_channel` → `std::fs::read_dir(dir)` | **No** (flat dir read) |
| `coord visibility` | `git_coord_files` → `git ls-tree -r --name-only <ref> -- docs/coordination` | **Yes** — `-r` recurses |
| session-resume "branch-local coordination" warning | `branch_local_coord_files` → `git diff --name-only <base>...HEAD -- docs/coordination` | **Yes** — pathspec matches descendants |

So the archive at `docs/coordination/archive/20260909-b4/` is already inert
to every top-level channel reader, but **not** to `coord visibility` (the
39 false findings) and **not** to the session-resume branch-local warning.
The Codex comment "the top-level-only channel reader does not treat
`archive/` as live input" is correct for `read_channel`; it does not hold
for the two `git`-pathspec-based readers.

## Recommended minimal fix — relocate the archive outside `docs/coordination/`

Move the 47 byte-identical records from
`docs/coordination/archive/20260909-b4/` to a sibling path **outside** the
`docs/coordination` tree, e.g. `docs/coordination-archive/20260909-b4/`.

Git pathspec `docs/coordination` does **not** match `docs/coordination-archive`
(full path-component match). Verified in a scratch repo:

```
git ls-tree -r --name-only HEAD -- docs/coordination        # lists only docs/coordination/*
git diff --name-only master...HEAD -- docs/coordination      # ignores docs/coordination-archive/*
```

Result: `visibility_scan` and `branch_local_coord_files` both stop seeing
the archive, `read_channel` is unaffected (already ignored it), and there
is **zero `agov` code change** — no gate mechanics touched, no tooling
Gate 1, no `gate-self-test-rule` negative-fixture-for-a-code-change
obligation. This stays inside B4's existing queue-convergence authority,
which the just-approved v2.11 scope row 6 explicitly permits for "the
narrow B4 archive correction".

### What the !2581 MR must still do

1. Repoint the SHA-256 digest manifest paths to `docs/coordination-archive/20260909-b4/`;
   re-verify all 47 digests against the moved bytes.
2. Confirm each moved record still passes `agov coord validate <file>`
   individually (path-independent; expected unchanged).
3. State explicitly in the MR description that Option B adds **no**
   visibility-exclusion mechanism, so v2.11 row 6's "negative fixture
   proving a genuinely misplaced live top-level obligation still fires
   after any archive exclusion" is **not triggered** by this MR; cite the
   existing coverage in `tooling/rust/crates/agov/tests/coord_visibility.rs`
   which proves `coord visibility` still surfaces real top-level
   branch-local obligations (unchanged by a file move).
4. Optional belt-and-braces (cheap, makes the boundary explicit): one test
   asserting a `.md` placed directly in `docs/coordination/` on a branch is
   still flagged by `coord visibility`, while one under
   `docs/coordination-archive/` is not.

### Post-move verification (author runs on the corrected head)

```
agov coord visibility --base-branch main      # 0 findings under the b4 archive path
agov session resume --base-branch main         # no branch-local warning for the 47 files
agov coord validate docs/coordination-archive/20260909-b4/*.md   # all OK
```

## What this does NOT do

The general contract — archive-path inertness *within* `docs/coordination/`,
with its own negative fixture — is v2.11 scope **row 6** work (CP-C / #123)
and keeps its own Gate 1 + Gate 2. This B4 correction does not pre-empt or
substitute for it.

## Offer

I (Claude) can implement Option B on a branch that supersedes the current
!2581 head, or the Codex author can apply it directly to
`docs/queue-convergence-b4-coordination-carriers`. Governor / author: say
which. Exact-head Gate 2 remains the governor's.
