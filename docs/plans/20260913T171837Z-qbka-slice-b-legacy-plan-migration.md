---
id: 20260913T171837Z-qbka-slice-b-legacy-plan-migration
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-13T16:53:02Z
topic: qbka-slice-b-legacy-plan-migration
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Pure frontmatter migration of historical plan files plus a new
      read-only corpus validation test. No credentials, secrets, network
      access, or write path affected.
  - role: contribution-auditor
    status: waived
    reason: >-
      Bounded, evidence-driven frontmatter correction with per-file
      historical evidence documented in this plan; a real pass is welcome
      before Gate 2 but is not required to unblock this narrow slice.
affected_paths:
  - docs/plans/agf-coord-channel-fsm.md
  - docs/plans/agf-v2-layout-s1.md
  - docs/plans/agf-deprecated-field-detection.md
  - docs/plans/elixir-migration-e3-validators.md
  - docs/plans/gate1-role-authority-tooling-retarget.md
  - docs/plans/plan-validator-advisory-role-allowlist.md
  - docs/plans/agents-govern-2.0-session-continuity-s1.md
  - docs/plans/docs-assessment-2026-06-21.md
  - docs/plans/rc1-sequence.md
  - docs/plans/release-pages-artifacts-policy.md
  - docs/plans/remove-elixir-tooling.md
  - docs/plans/validate-contributions-readme-stats.md
  - tooling/rust/crates/agov/tests/
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: qbka Slice B — legacy plan-file migration + all-corpus regression test

**Gate 1: APPROVED** — blanket pre-approval
`20260913T165302Z-governor-reviewer-8h7s` ("Jani: 'Pre-approved, implement
right away.'"), plus `20260913T171642Z-governor-reviewer-hs6j` ratifying
the two supersession dispositions this slice needs (see below).

Implements Slice B of `67kq`'s (`20260903T215909Z-coder-tester-67kq`)
recommended two-slice split, following qbka Slice A (merged, `e9f7742d`)
which froze the `superseded` provenance shape this slice reuses verbatim.

## Scope

The exact twelve files `67kq`'s census found invalid, and only those
twelve. No other plan file is touched. No plan's historical prose body is
rewritten beyond the minimum explanatory linkage `67kq` allows — only
frontmatter changes, plus one short "Correction" note per file stating
what changed and why, matching this project's own established convention
for post-hoc corrections to already-published documents.

## Per-file disposition and evidence

### Five status-spelling files (67kq's "spelling drift" group) — but corrected to their **true current status**, not merely respelled

`67kq` characterized these five as pure spelling drift (`gate1-approved`/
`gate-1-approved` -> `approved`). Investigation for this slice found that a
literal respelling to `approved` would itself misrepresent four of the
five: each describes work that has demonstrably since **shipped**, so the
historically accurate terminal status is `complete`, not `approved` --
leaving them at `approved` would be exactly the "queue truth" gap `67kq`
and the Plan Status Lifecycle documentation warn about, just in the
opposite direction (understating completed work instead of overstating
it).

| File | Old status | New status | Evidence work actually shipped |
|---|---|---|---|
| `agf-v2-layout-s1.md` | `gate1-approved` | `complete` | `agf/` is the current, live canonical framework directory (see AGENTS.md project layout). Contribution-auditor review `00224-contribution-auditor` (2026-06-13) returned `recommend-ready-for-gate2`; its `advisory_consultation[1].status: pending` is corrected to `consulted` citing that review -- it was not actually pending, the frontmatter was simply never updated after the review landed. |
| `elixir-migration-e3-validators.md` | `gate1-approved` | `complete` | Introducing commit `fac18812` ("FSM Gate 2 approved; E1-E3 planning") confirms the referenced FSM/validator work reached Gate 2 at the time. (The Elixir tree implementing it was later entirely removed -- see `remove-elixir-tooling.md` below -- but that is a separate, later event; it does not retroactively make this plan's own historical disposition anything other than complete.) |
| `gate1-role-authority-tooling-retarget.md` | `gate-1-approved` | `complete` | AGENTS.md's current "Role authority note" confirms `agf/agents/MANIFEST.yaml` (not `agents-govern/agents/MANIFEST.yaml`) is the live role authority today -- exactly the retarget this plan describes. |
| `release-pages-artifacts-policy.md` | `gate1-approved` | `complete` | Tagged releases with public Pages and release artifacts are this project's live, continuously-used release mechanism (see `workflow/release-via-merge-request-rule.md` and the GitLab Releases page named in AGENTS.md). |
| `plan-validator-advisory-role-allowlist.md` | `superseded` (valid enum, missing provenance) | `superseded` (unchanged) | See its own row below -- this one keeps its already-correct status and only gains the three required provenance fields. |

### The one already-`superseded` file: add provenance

`plan-validator-advisory-role-allowlist.md` already declares `status:
superseded` (valid) but is missing the three fields qbka Slice A requires.
Governor-ratified in `hs6j`:

```yaml
superseded_by: issue30-plan-validator-role-status-diagnostics
superseded_decision: 20260913T171642Z-governor-reviewer-hs6j.md
superseded_at: 2026-09-13T17:16:42Z
```

### One newly-superseded file (Elixir obsolescence)

`agf-coord-channel-fsm.md` proposed refactoring the Elixir
`Agov.ClaimTracker` into `Agov.ClaimFSM`. It was never approved
(`gate-1-pending`) and the entire Elixir tree was later removed
(`remove-elixir-tooling.md`, implemented), making the proposal permanently
inapplicable. Governor-ratified in `hs6j`:

```yaml
id: agf-coord-channel-fsm
status: superseded
superseded_by: remove-elixir-tooling
superseded_decision: 20260913T171642Z-governor-reviewer-hs6j.md
superseded_at: 2026-09-13T17:16:42Z
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Superseded before implementation or real consultation occurred; consultation is moot.
  - role: contribution-auditor
    status: not-applicable
    reason: Superseded before implementation or real consultation occurred; consultation is moot.
```

### Six files needing full structural reconstruction (`id`, `status`, `advisory_consultation`)

Three (`agents-govern-2.0-session-continuity-s1.md`,
`docs-assessment-2026-06-21.md`, and — for `advisory_consultation` and
`id` only, status is separately handled above — `agf-coord-channel-fsm.md`)
have partial front matter; four
(`agf-deprecated-field-detection.md`, `rc1-sequence.md`,
`remove-elixir-tooling.md`, `validate-contributions-readme-stats.md`) have
none at all. All six get `id` set to their filename stem, `status:
complete` (each verified below against the live codebase, per `67kq`'s
explicit instruction not to promote body-text labels into authority
without independent confirmation), and an `advisory_consultation` array
with both required roles marked `not-applicable`, reason: these are
historical plans that predate this project's per-plan advisory-consultation
tracking convention; no fabricated consultation record is created for a
consultation that was never actually run through today's process.
`gate1_approved_by`/`gate1_approved_at` are **not** invented with false
precision — the schema does not require them (only `id`, `status`,
`advisory_consultation` are required), and this plan does not have binding
per-file evidence precise enough to state an exact historical Gate 1
instant for pre-readable-ID-era plans; they are left absent rather than
guessed.

| File | Status evidence (live codebase check) |
|---|---|
| `agents-govern-2.0-session-continuity-s1.md` | `agov session resume` exists and is this project's documented cold-start command (`docs/reference/session-resume.md`, AGENTS.md) — the exact feature this plan proposed (`mix agov.session.resume`, later ported to Rust). |
| `docs-assessment-2026-06-21.md` | A one-time, dated (2026-06-21) four-angle documentation assessment with a completed "Full 20-item findings" section and "Slice assignments summary" — the deliverable itself is finished, not an ongoing initiative; `active` no longer describes it three months later. |
| `agf-deprecated-field-detection.md` | `agf/schemas/coordination-message.schema.json` contains `"x-deprecated": true` on the `seq` field today — the exact machine-readable deprecation surface this plan proposed. |
| `rc1-sequence.md` | **Corrected per independent review finding QBKA-B-F1** (`20260913T184748Z-coder-tester-t6hh`): the original citation of `v2.0.0` was chronologically impossible (that release predates this plan's own commit `67d01c2f`). Actual post-plan evidence: MR !153 merged at `08c9e9e7`, readable coordination IDs at `11462144`, `agov` version provenance at `27080987`, the RC readiness audit at `d80163b5`, and the `v2.2.0-rc.1`/`v2.2.0-rc.2` tags at `bd810bdf`/`cd8f10b6` — this plan's own described RC-1/RC-2 sequence, completed. `status: complete` stands on this corrected evidence. |
| `remove-elixir-tooling.md` | `tooling/elixir/` does not exist in this checkout; AGENTS.md states "No Elixir implementation is tracked." |
| `validate-contributions-readme-stats.md` | `tooling/rust/crates/agov-validators/src/readme_stats.rs` exists and its tests run in the current workspace suite (`readme_stats::tests::*`, observed passing this session). |

## Corpus regression test (the second half of `67kq`'s Slice B ask)

New test `tooling/rust/crates/agov/tests/plan_corpus_validity.rs`: iterates
every `docs/plans/*.md` file and asserts `agov plan validate` exits `0` for
all of them. This is the permanent regression guard `67kq` asked for — the
corpus census (12 invalid) is re-run as a real, CI-enforced test rather
than a one-time manual `agov plan validate` sweep, so no future plan file
can silently regress the corpus back to invalid.

## Non-goals

- No plan file's historical prose body is substantively rewritten. Each
  gets a short, dated "Correction" note (matching the convention already
  used elsewhere in this project's own rules) stating what frontmatter
  changed and why, with a pointer to this plan.
- Not touching any plan file `67kq`'s census did not flag.
- Not re-deriving or second-guessing any of the ten already-valid,
  already-`approved`/`active`/`complete` plans' existing statuses.

## Test strategy

1. Per-file: `agov plan validate docs/plans/<file>.md` exits 0 after the
   edit (12 individual checks).
2. New `plan_corpus_validity.rs` integration test: all `docs/plans/*.md`
   validate; this is the test that would have caught the original 12-file
   drift had it existed earlier.
3. Full workspace test suite, clippy, before Gate 2.

## Rollback

Frontmatter-only changes plus prose-preserving correction notes. Reverting
the branch restores the twelve files to their prior (invalid) state with
no data loss — the corpus was already invalid before this slice; reverting
is a regression, not a new risk.
