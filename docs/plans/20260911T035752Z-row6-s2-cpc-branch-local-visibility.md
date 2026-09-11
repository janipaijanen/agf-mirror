---
id: 20260911T035752Z-row6-s2-cpc-branch-local-visibility
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-11T17:10:30Z
topic: row6-s2-cpc-branch-local-visibility
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Per `agf/agents/MANIFEST.yaml`, foundry's infosec-sentinel is "ci
      (agov validate infosec, on commit)" — a mechanical check, not a
      separate reviewer session. Ran the actual command, precisely named
      (corrected 2026-09-11, RNP-1 round 3 — the round-1 wording said
      "ran against this plan," overstating its scope, a mistake this
      correction fixes here after the sibling S3 plan already caught the
      same error round 2): `agov validate infosec` invokes
      `ValidateCommand::Infosec` with no plan-path argument
      (`main.rs:6883-6887`), dispatching
      `agov_validators::infosec::validate_all(&agov_core::learnings_dir())`
      — it scans **learning records only**, not this plan file, its
      branch, or any API/credential security contract. Result: no
      findings in the learning corpus. This provides **no plan-security
      consultation**; it is cited only as the mechanical CI check this
      project runs on every commit. The read-only/no-write/no-credential
      reasoning for branch-local reads below is author reasoning, not a
      screening attestation, and remains an outstanding item for an
      actual infosec-sentinel or governor security consultation.
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

**Gate 1: APPROVED** at exact source head `5f80cbefa7d5a1f34c08ba1bee76768c8aadce4a`
per `20260911T171030Z-governor-reviewer-1rhh` (governor Jani: "2685 and 2586
approved gate 1", transcribed via the Codex session; recorded on
`codex/ci-shallow-history-report`, MR !2705). The disclosed plan-security
consultation limitation (see `advisory_consultation.infosec-sentinel` above)
is preserved, not converted into a technical screening attestation by this
approval. coder-tester implements; implementation Gate 2 remains separate.

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

**The on-disk filesystem is a named source too (RNP-2 round 3).** The
default (no-flag) source `handoff_next` already reads —
`read_valid_coord_messages(&dir)`, the current working tree, including
uncommitted/staged files — does not disappear when
`--include-branch-local` is added; it is still evaluated (unconditionally,
exactly as today) and its own pending set participates in dedup/conflict
on equal footing with every enumerated ref. To compare it on the same
blob-SHA basis as ref candidates, compute each on-disk file's blob hash
via `git hash-object <path>` (the same content-addressing algorithm git
uses for committed blobs — works on an uncommitted file without needing a
commit). This is the priority-class-0 source (ranks above every ref,
including local branches, in the class-priority attribution below — it is
literally what the user is looking at right now).

**Implementation precision (non-blocking, `dka4`):** hash the exact raw
bytes already parsed for schema validation — `git hash-object --no-filters
<path>` (or equivalent in-process blob hashing over the same byte
snapshot), never the filter/clean-transformed bytes git would otherwise
apply (e.g. line-ending normalization from `.gitattributes`). Using
filtered bytes could make two files this plan's own read-only validation
step treats as byte-identical hash to different blob SHAs, or vice versa,
silently breaking the dedup/conflict equality test above.

Same-content dedup reuses the **blob-SHA equality** `load_visibility_snapshot`
already computes per path (`base.get(path) == Some(blob)` at
`main.rs:4784`) — generalised here to a per-path comparison **across the
on-disk source and every enumerated ref's own inventory together**, not
`dedup_and_select_branches`'s commit/tree-OID grouping (which only catches
two refs at the *same commit*, not two refs — or the working tree and a
ref — at *different* states that happen to carry byte-identical content
at one path — exactly the "unchanged handoff blob at different commits"
case the reviewer named). After every source's independent pending set is
computed (the filesystem plus each ref): group same-path candidates by
blob SHA. All sources sharing one blob SHA for a path report as **one**
entry, attributed to the highest-priority source present in that group
(on-disk filesystem > local branch > `origin/*` > other remotes, extending
`branch_ref_class`, `main.rs:4634-4643`, with the new filesystem class 0
— for priority/attribution only, not for the dedup test itself, which is
blob-SHA equality).

**Genuine conflict** (the same nominal handoff — same path — with
*different* blob SHAs across sources, each still independently pending
per its own source) is a distinct case from same-content duplication and
MUST NOT be silently resolved by picking one: **all** distinct (source,
blob-SHA) pairs are reported as separate entries — including when the
on-disk filesystem itself is one of the conflicting sources, e.g. a
locally-edited-but-uncommitted handoff that diverges from what a branch
already committed at the same path — each labeled with its source and
exact SHA (or "working tree" for the filesystem source), with a
`CONFLICT` marker linking them. This is additive output
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
3c. **Cross-ref non-suppression (corrected 2026-09-11, RNP-2 round 3 — the
   round-2 version of this fixture used a resolving record for an
   *unrelated* handoff, which cannot exercise cross-ref pooling at all;
   the per-ref design already forbids that trivially).** A handoff at a
   given path is pending on ref A with no resolving record anywhere on
   ref A; ref B independently has a resolving record (ack/completion)
   that names **that same handoff's own ID/path** — but ref B never
   contains the pending handoff itself, only the record that would
   resolve it if pooled. Evaluated correctly (per-ref-independent), ref
   A's handoff still reports as pending — proving ref B's resolving
   record was never consulted while evaluating ref A, the actual
   suppression risk the reviewer named.
3d. **Filesystem-vs-ref dedup and conflict (RNP-2 round 3).** The on-disk
   working tree has an uncommitted handoff at a path; a local branch
   separately commits a byte-identical copy at the same path → deduped
   to one entry, attributed to the filesystem (priority class 0). A
   second case: the on-disk copy and a branch's committed copy at the
   same path have **different** bytes → both reported in the conflict
   section, the filesystem entry labeled "working tree" instead of a ref
   name/SHA.
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
6. **Unreadable ref — a new, proposed behaviour (corrected 2026-09-11,
   RNP-2 round 3; helper-name citation corrected round 4 — `gtsg`'s own
   `git_coord_files`/`load_git_messages` names had already gone stale
   again by the time this fixture was written; the causal claim was
   right, the cited function names were not).** Re-verified `coord
   visibility`'s actual current behaviour: `visibility_scan`
   (`main.rs:4419-4531`) propagates `git_candidate_inventory(&oid,
   coord_dir)?` and `load_visibility_snapshot(...)?` errors with `?`,
   aborting the **entire** scan on one bad ref — the existing behaviour
   is fail-closed-whole, the opposite of what the earlier version of
   this fixture claimed. S2 deliberately proposes
   **different, new** behaviour: a ref that fails to resolve (deleted
   mid-scan, corrupt object) is reported as a scan error for that ref
   specifically, and the scan continues for every other ref/the
   filesystem source. The result **must** carry an explicit
   `partial_coverage: true` (or equivalent) marker naming which ref
   failed whenever this happens — never an unqualified "no handoff
   found" success that looks identical to a genuinely clean, fully-
   covered scan.
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
(corrected 2026-09-11, three rounds — round 1: distribution claim fixed
inline; round 2: converted to the structured manifest; round 3: fixed a
genuine drafting bug the recheck caught — `agf/templates/AGENTS.md` was
placed under `not_applicable` even though its own stated reason said it
should be updated): normative owner not applicable; adopter explanation
not applicable; runbook not applicable; AI loaders `AGENTS.md` **and**
`agf/templates/AGENTS.md` both updated, `.claude/CLAUDE.md` not
applicable via import; validation/drift advisory only; distribution
confirmed for `agf/templates/AGENTS.md` only (root `AGENTS.md`
foundry-local, `agov` itself ships via the tooling archive, not the
`agf/` framework artifact).
`agov plan projection-check --allow-planned --base-ref origin/main` on
this manifest: OK.

## Rollback

Pure additive `agov` flag. Revert the MR → `handoff next` loses the flag;
zero behaviour change for any caller not passing it. No schema, no rule
text, no migration.

## Corrections (2026-09-11, RNP review — three rounds)

Independent review `6pq1` (round 1) found this plan's original text
inaccurate or incomplete in five ways. Recheck `gtsg` (round 2) found the
round-1 pass partial on RNP-2 and RNP-4. Recheck `20260911T130853Z-contribution-auditor-ehmq`
(round 3) BLOCKed S2 again: RNP-1 had been fixed on the sibling S3 plan
but the identical fix was never applied here; RNP-2's dedup/conflict
policy never named the on-disk filesystem as a source even though it is
`handoff_next`'s default source; fixture 6 claimed to match `coord
visibility`'s existing per-ref error behaviour, which round 3 found is
actually the opposite (current `main.rs` fails closed on the whole scan,
not per-ref); fixture 3c didn't actually exercise cross-ref suppression;
and the projection manifest had a genuine drafting bug (a path under
`not_applicable` whose own reason said it should be `updated`). Status
after all three rounds:

- **RNP-1** (unsupported consultation attestations, then: infosec wording
  overstated scope — fixed on S3 round 2 but not applied here) —
  **resolved round 3**: applied the identical S3 correction to S2's
  `advisory_consultation.infosec-sentinel`.
- **RNP-2** (branch provenance/lifecycle undefined, wrong baseline
  citation, then: wrong function citations after `main` advanced again,
  dedup claim not actually blob-SHA-based, no lifecycle/resolution
  semantics, fixture 3 didn't test what it claimed, MCP fixture had an
  escape hatch, then: on-disk filesystem never named as a dedup/conflict
  source, fixture 6's fail-partial claim was backwards relative to actual
  `main.rs` behaviour, fixture 3c didn't exercise real cross-ref
  suppression) — **resolved round 3**: added the on-disk filesystem as a
  named priority-class-0 source in dedup/conflict (fixture 3d), corrected
  fixture 6 to state plainly that per-ref fail-partial is a **new,
  proposed** behaviour (current `main.rs` fails closed on the whole scan)
  and requires an explicit `partial_coverage` marker in the result, and
  rewrote fixture 3c to use a same-ID resolving record on a different ref
  (the actual suppression risk), not an unrelated one.
- **RNP-4** (inline-only projection screen retained, stale distribution
  sentence, then: `agf/templates/AGENTS.md` placed under `not_applicable`
  with a reason that said `updated`) — **resolved round 3**: moved to
  `updated`, matching its own stated reason and the `distribution` row.
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
dedup/conflict policy including the on-disk filesystem as a named
priority-class-0 source, (e) the new per-ref fail-partial error handling
(a deliberate departure from `coord visibility`'s current fail-closed-
whole behaviour, surfaced via an explicit `partial_coverage` marker),
(f) the 11 fixtures (1, 2, 3, 3b, 3c, 3d, 4, 5, 6, 7, 8), (g) the
structured loader-pointer projection manifest. Then coder-tester
implements against current `main`; own Gate 2.
