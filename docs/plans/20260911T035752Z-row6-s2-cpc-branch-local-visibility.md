---
id: 20260911T035752Z-row6-s2-cpc-branch-local-visibility
status: pending-gate1
topic: row6-s2-cpc-branch-local-visibility
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Per `agf/agents/MANIFEST.yaml`, foundry's infosec-sentinel is "ci
      (agov validate infosec, on commit)" — a mechanical check, not a
      separate reviewer session. Ran `agov validate infosec` against
      this plan: no findings. The substantive question the plan raises
      (does branch-local content reading ever write, execute, or expose
      credentials) is answered structurally below (git object reads only,
      same primitives `coord visibility` already uses) and is
      independently re-checkable by any reviewer against the live source.
      Corrected 2026-09-11 (RNP-1): the prior wording ("InfoSec confirms
      ...") implied an independent reviewer attestation this plan did not
      actually have; restated to match the mechanical-check basis this
      project's other plans use for the same role.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Corrected 2026-09-11 (RNP-1): the prior wording claimed
      contribution-auditor "confirms" scope-boundedness without citing a
      reviewer or session — an unsupported affirmative attestation.
      Independent contribution-auditor review now exists and is cited:
      `20260911T121946Z-contribution-auditor-6pq1` (Codex,
      `/root/independent_plan_review`, session distinct from this plan's
      author session `session_01L2r2kfLcdnYqGcTny6kyuH`), advisory BLOCK
      on findings RNP-1 through RNP-5. This revision resolves RNP-1,
      RNP-2, and RNP-5 for S2 (see "Corrections (2026-09-11, RNP review)"
      below); the reviewer rechecks this corrected exact head before any
      Gate 1 decision. `status: consulted` reflects that the review
      happened and is cited, not that its findings are already cleared —
      clearance is the reviewer's own recheck, not a self-assessment.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 6 S2 — CP-C: branch-local obligation visibility for `agov handoff next` (Gate 1)

## Provenance

- Parent: `docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md`
  (v2.11 IN row 6, Gate 1 `tsbh` / ratified `q862`), slice **S2**.
  Governor "Keep all in v2.11" (2026-09-10) confirms row 6 stays a hard
  cut precondition.
- `jprj` §C: a handoff on an unmerged branch is invisible to
  `agov handoff next`; only `agov coord visibility` (which already does a
  multi-branch scan) surfaces it. This session hit it repeatedly (reviews
  landing on `codex/review-*` branches).
- **Corrected baseline (2026-09-11, RNP-2):** the prior citation
  (`main.rs:4118`, "`read_channel(&dir)` on the resolved `base_branch`
  checkout") was wrong and has been re-verified against current `main`.
  `handoff_next` (`main.rs:5876`) calls `read_valid_coord_messages(&dir)`
  where `dir` is the literal `--dir` path (default `docs/coordination`)
  resolved via `repo_relative_path` — a **plain filesystem read of
  whatever is currently checked out**, including uncommitted/staged
  files. There is no git ref resolution in `handoff_next` today at all;
  "the resolved base branch" was never an accurate description of its
  behaviour. This plan's actual gap is narrower and correct in substance
  even though the citation was wrong: `handoff_next` only ever sees the
  one tree that happens to be on disk right now; it has no way to look at
  *any other ref* without this slice.

## Change

| File | Change |
|---|---|
| `agov handoff next <role>` (`tooling/rust/crates/agov/src/main.rs`) | New optional flag `--include-branch-local`. **Without it: byte-identical to today** (only the existing `read_valid_coord_messages(&dir)` filesystem read — no git ref access added). **With it:** additionally enumerate refs via the exact same ref universe `agov coord visibility` already scans (`git_branch_refs`-equivalent: every `refs/heads/*` and `refs/remotes/<remote>/*`, excluding symbolic refs — `main.rs:4604-4619`), read each ref's `docs/coordination` tree via `git_coord_files(ref_name, coord_dir)` + `git show <ref>:<path>` (no checkout, matching `coord visibility`'s own mechanism — never switches the working tree), merge the resulting candidates into the same selection logic `handoff next` already uses (no new ranking rule), and label each branch-local candidate with its source ref name **and the exact SHA it was read from**. |
| `AGENTS.md` "Tooling quick reference" + `agf/templates/AGENTS.md` | One-line pointer: `agov handoff next --include-branch-local` widens the search to every local and remote-tracking branch's coordination tree (same ref universe as `coord visibility`); default behaviour unchanged. |

### Dedup / conflict policy (RNP-2)

Reuses `dedup_and_select_branches`'s existing peeled-OID grouping and
class priority (local > `origin/*` > other remotes, `main.rs:4609-4653`):
when the *same* handoff file is byte-identical across multiple refs
(e.g. a branch that has since fast-forwarded onto the base, or two
remotes tracking the same tip), it is reported exactly once, attributed
to the highest-priority ref in its OID group. This is not new logic —
it is the same dedup `coord visibility` already performs, applied here to
avoid inventing a second, inconsistent dedup rule.

**Genuine conflict** (the same nominal handoff — same `id`/filename — with
*different* content at different OIDs across refs) is a different case
from same-content duplication and MUST NOT be silently resolved by
picking one: both candidates are reported, each labeled with its distinct
source ref and SHA, with a `CONFLICT` marker. No authority is inferred
from branch contents (per the parent work order and InfoSec's
consultation above) — resolving which candidate is authoritative remains
a human/governor decision, never an automatic pick.

## Explicitly out of scope (parent work order guard)

No change to `handoff next`'s selection or ranking algorithm. No change
to which branch is authoritative for "ready" status. No new default
behaviour — the flag is opt-in, and its absence must produce output
identical to the pre-slice binary on the same inputs (fixture 1 below).

## Fixtures (ordinary test discipline; not a `gate-self-test-rule` trigger — advisory queue display, no MR-blocking check added)

1. **Default unchanged.** Same inputs, with and without the flag absent
   in both the old and new binary → identical output (only the existing
   filesystem `--dir` read runs; no git ref access attempted).
2. **Branch-local handoff surfaced.** A `requires-ack: true` handoff
   committed only on an unmerged local branch (not present in the
   on-disk `--dir` tree) → invisible without `--include-branch-local`;
   appears, labeled with its source ref and SHA, with the flag.
3. **No duplication (same content, multiple refs).** A handoff present on
   both a local branch and its `origin/*` remote-tracking counterpart at
   the same OID is reported once, attributed per the class-priority rule
   above — not double-listed.
4. **Genuine conflict (same ID, divergent content).** Two refs each
   declare a same-named handoff file with different byte content →
   **both** are reported, each labeled with its own ref/SHA and a
   `CONFLICT` marker — never silently merged or silently dropped to one.
5. **Malformed branch-local candidate.** A schema-invalid file on a local
   branch does not crash the scan; it is reported as a parse failure
   (matching `agov coord visibility`'s existing behaviour), not silently
   dropped or silently accepted.
6. **Unreadable ref.** A ref that fails to resolve (deleted mid-scan,
   corrupt object) is reported as a scan error for that ref specifically
   — the scan continues for every other ref rather than aborting the
   whole command (matching `coord visibility`'s existing fail-partial,
   not fail-closed-whole, behaviour for per-ref errors).
7. **`--session` interaction.** `--include-branch-local` combined with
   the existing `--session` flag (row-5 S1) still annotates branch-local
   candidates with recipient-session information exactly as it does for
   base-tree candidates — no special-cased exemption.
8. **MCP parity.** `agov-mcp`'s `handoff_next` tool exposes the same flag
   with the same semantics (or the plan explicitly states it does not and
   why) — no silent CLI/MCP divergence.

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

## Corrections (2026-09-11, RNP review)

Independent review `20260911T121946Z-contribution-auditor-6pq1` found
this plan's original text inaccurate or incomplete in five ways. Status:

- **RNP-1** (unsupported consultation attestations) — **resolved**, see
  the corrected `advisory_consultation` block above.
- **RNP-2** (branch provenance/lifecycle undefined, wrong baseline
  citation) — **resolved**, see the corrected "Provenance" and "Change"
  sections and the new "Dedup / conflict policy" subsection and fixtures
  4/6/7/8 above.
- **RNP-5** (stale "no overlap" claim) — **resolved**, see below.
- RNP-3 and RNP-4 apply to the sibling S3 plan
  (`20260911T040631Z-row6-s3-issue123-affected-paths-collision-check.md`),
  not this one; not addressed here.

### RNP-5: shared-surface correction

The original commit message claimed independence from Codex's
in-progress branches. That was already stale relative to the state at
drafting time and is more stale now: row-5 S4 (!2681) and row-6 S1
(!2683/!2675) have since merged into `main`, and both touched
`tooling/rust/crates/agov/src/main.rs`, `AGENTS.md`, and
`agf/templates/AGENTS.md` — the same three files this slice's own
"Change" table edits. Logical independence (this slice adds a flag; S1/S4
added different, unrelated flags/fields) is not absence of shared files.

**Implementation-order note for coder-tester:** rebase onto current `main`
(which already contains S1/S4) before implementing, not onto this plan's
original stale base. Expect `main.rs`, `AGENTS.md`, and
`agf/templates/AGENTS.md` to already differ from this plan's drafting-time
snapshot in unrelated ways (new flags, new role-table rows); resolve by
adding this slice's specific edits on top of current content, not by
reverting or overwriting S1/S4's already-merged changes.

## Decision requested at Gate 1

Approve (a) the opt-in `--include-branch-local` flag reusing
`coord visibility`'s existing branch-enumeration and read path, (b) that
default (no-flag) behaviour stays byte-identical, (c) the dedup/conflict
policy above, (d) the 8 fixtures, (e) the loader-pointer projection. Then
coder-tester implements against current `main`; own Gate 2.
