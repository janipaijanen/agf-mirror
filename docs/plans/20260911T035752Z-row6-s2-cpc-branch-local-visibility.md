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
| `agov handoff next <role>` (`tooling/rust/crates/agov/src/main.rs`) | New optional flag `--include-branch-local`. **Without it: byte-identical to today** (only the existing `read_valid_coord_messages(&dir)` filesystem read — no git ref access added). **With it:** additionally enumerate refs via the exact same ref universe `agov coord visibility` already scans (`git_branch_refs`, `main.rs:4606-4631`: every `refs/heads/*` and `refs/remotes/<remote>/*` via one `for-each-ref`, excluding symbolic refs), and for each ref, compute its own **independent** pending-handoff set using the exact same two functions `coord visibility` already uses per ref — `git_candidate_inventory(oid, coord_dir)` (`main.rs:4718`, a path→blob-SHA map via `git ls-tree`) then `load_visibility_snapshot` (`main.rs:4759`, batch-reads only new/changed blobs via `git cat-file --batch`, validates each against the coordination-message schema) — followed by `handoff_next`'s own existing `pending_handoffs()` filter applied to that one ref's own message set. See "Lifecycle / resolution semantics" and "Dedup / conflict policy" below for exactly how per-ref results are combined into one report. |
| `AGENTS.md` "Tooling quick reference" + `agf/templates/AGENTS.md` | One-line pointer: `agov handoff next --include-branch-local` widens the search to every local and remote-tracking branch's coordination tree (same ref universe as `coord visibility`); default behaviour unchanged. |

### Lifecycle / resolution semantics (RNP-2, still open after the first correction)

The first correction fixed the citation but not the actual design
question: **within which candidate pool is "pending vs. resolved"
decided?** Two wrong answers, both named explicitly by the reviewer:

- **Wrong — merge all refs' messages into one pool, then resolve.** A
  completion/ack that exists only on ref B could suppress a handoff that
  only exists on ref A, even though ref A's own history never produced or
  saw that completion. Silent, wrong suppression.
- **Wrong — read only newly-appearing ("branch-added") handoffs per
  ref.** A handoff already resolved *on that same branch* (its own later
  commit contains the ack/completion) would be misread as still pending,
  because the resolving record itself wasn't in the "added" set being
  read.

**Decision:** resolution status is computed **once per ref, using only
that ref's own complete message set**, never a cross-ref merge. Concretely,
for each enumerated ref: run `git_candidate_inventory` + the schema-load
and validation half of `load_visibility_snapshot`'s logic to get that
ref's full parsed message list, then apply `handoff_next`'s existing
`pending_handoffs()` filter to that list alone (exactly as it is already
applied to the base tree's message list today). This is self-contained:
a ref's own ack/completion, if present at that ref's tip, correctly
excludes the handoff it resolves (no resurrection); an unrelated ref's
ack/completion is never consulted when evaluating a different ref (no
cross-ref suppression). Cross-ref combination happens **only after** each
ref's own pending set is independently final — at the output/dedup layer
below, never at the resolution layer.

### Dedup / conflict policy (RNP-2)

Same-content dedup reuses the **blob-SHA equality** `load_visibility_snapshot`
already computes per path (`base.get(path) == Some(blob)` at
`main.rs:4784`) — generalised here to a per-path comparison **across every
enumerated ref's own inventory**, not `dedup_and_select_branches`'s
commit/tree-OID grouping (which only catches two refs at the *same commit*,
not two refs at *different commits* that happen to carry byte-identical
content at one path — exactly the "unchanged handoff blob at different
commits" case the reviewer named). After every ref's independent pending
set is computed: group same-path candidates across refs by blob SHA. All
refs sharing one blob SHA for a path report as **one** entry, attributed to
the highest-priority ref present in that group (local > `origin/*` > other
remotes, reusing `branch_ref_class`, `main.rs:4634-4643`, for the priority
order only — not for the dedup test itself, which is blob-SHA equality).

**Genuine conflict** (the same nominal handoff — same path — with
*different* blob SHAs across refs, each still independently pending per
its own ref) is a distinct case from same-content duplication and MUST NOT
be silently resolved by picking one: **all** distinct (ref, blob-SHA)
pairs are reported as separate entries, each labeled with its ref and
exact SHA, with a `CONFLICT` marker linking them. This is additive output
— it does not alter `handoff_next`'s existing single-result `max_by_key`
selection for the primary "next handoff" answer (out of scope, per the
parent work order guard, to redesign that); conflict entries are a
separate, clearly-labeled diagnostic section of the report, never a
silent substitute for the selected result. No authority is inferred from
branch contents (per the parent work order and InfoSec's consultation
above) — resolving which candidate is authoritative remains a human/
governor decision, never an automatic pick.

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
3. **No duplication (same content, different commits, different refs).**
   A handoff file exists at path `docs/coordination/X.md` on both a local
   branch (committed at time T1) and a separately-diverged remote branch
   (committed at time T2, different commit SHA, same file bytes) — two
   different commit OIDs, one shared blob SHA at that path. Reported
   **once**, attributed per the class-priority rule — proving the dedup
   is blob-SHA-based, not commit-OID-based (the case the first correction
   claimed but did not actually test).
3b. **Own-branch resolution, not resurrected.** A ref's own later commit
   adds an ack/completion resolving a handoff also present earlier on
   that same ref → evaluating that ref alone (per the "Lifecycle /
   resolution semantics" decision above) correctly excludes it as
   resolved; it does not appear as pending for that ref.
3c. **Cross-ref non-suppression.** A handoff pending on ref A has no
   resolving record anywhere on ref A; a resolving record for a
   different, unrelated handoff exists on ref B → ref A's handoff still
   reports as pending (ref B's records never enter ref A's own
   resolution evaluation).
4. **Genuine conflict (same path, divergent blob SHA, both still
   independently pending on their own ref).** Two refs each declare a
   same-path handoff file with different byte content, and neither ref
   has its own resolving record → **both** are reported in the conflict
   section, each labeled with its own ref/SHA, plus the normal per-ref
   pending result is otherwise unaffected — never silently merged,
   silently dropped to one, or allowed to change the primary selected
   result.
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
8. **MCP parity.** `agov-mcp`'s `handoff_next` tool exposes the same
   `--include-branch-local` flag with the same semantics as the CLI —
   committed here as a requirement, not an option coder-tester decides at
   implementation time. No CLI/MCP divergence.

## Rule-projection screen

`docs/reviews/rule-projection/20260911T035752Z-row6-s2-cpc-branch-local-visibility.yaml`
(corrected 2026-09-11, RNP-4 — converted from inline-only prose to the
structured manifest, and its distribution claim fixed): normative owner
not applicable; adopter explanation not applicable; runbook not
applicable; AI loaders `AGENTS.md` updated, `agf/templates/AGENTS.md`
also updated (ships the same capability to adopters — not an open
question), `.claude/CLAUDE.md` not applicable via import; validation/drift
advisory only; distribution confirmed for `agf/templates/AGENTS.md` only
(root `AGENTS.md` foundry-local, `agov` itself ships via the tooling
archive, not the `agf/` framework artifact — corrected from the prior
imprecise claim).
`agov plan projection-check --allow-planned --base-ref origin/main` on
this manifest: OK.

## Rollback

Pure additive `agov` flag. Revert the MR → `handoff next` loses the flag;
zero behaviour change for any caller not passing it. No schema, no rule
text, no migration.

## Corrections (2026-09-11, RNP review — two rounds)

Independent review `6pq1` found this plan's original text inaccurate or
incomplete in five ways; a second recheck (`20260911T124355Z-contribution-auditor-gtsg`)
found the first correction pass partial on RNP-2 and RNP-4. Status after
both rounds:

- **RNP-1** (unsupported consultation attestations) — resolved round 1.
- **RNP-2** (branch provenance/lifecycle undefined, wrong baseline
  citation, then: wrong function citations after `main` advanced again,
  dedup claim not actually blob-SHA-based, no lifecycle/resolution
  semantics, fixture 3 didn't test what it claimed, MCP fixture had an
  escape hatch) — **resolved round 2**: re-verified `main.rs` a second
  time (`git_candidate_inventory`/`load_visibility_snapshot`, current as
  of `main` `5d749c51`+), added the "Lifecycle / resolution semantics"
  subsection deciding per-ref-independent resolution, corrected dedup to
  blob-SHA equality, fixed fixture 3 and added 3b/3c, and committed MCP
  parity without an escape hatch.
- **RNP-4** (inline-only projection screen retained, stale distribution
  sentence) — this finding was against S2 specifically in round 2 (S3's
  RNP-4 was already resolved round 1) — **resolved round 2**: converted
  to the structured manifest, corrected the distribution claim.
- **RNP-5** (stale "no overlap" claim) — resolved round 1, see below.
- RNP-3 applies only to the sibling S3 plan; not addressed here.

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
`coord visibility`'s existing branch-enumeration and per-ref read path,
(b) that default (no-flag) behaviour stays byte-identical, (c) the
per-ref-independent lifecycle/resolution semantics, (d) the blob-SHA
dedup/conflict policy above, (e) the 10 fixtures (1, 2, 3, 3b, 3c, 4, 5,
6, 7, 8), (f) the structured loader-pointer projection manifest. Then
coder-tester implements against current `main`; own Gate 2.
