---
id: agov-coordination-read-path-reliability
status: pending-gate1
topic: agov-coordination-read-path-reliability
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Issues #84, #85, #89 were independently reproduced and confirmed by
      contribution-auditor (`20260730T072926Z-contribution-auditor-rk2k`),
      with corrections folded into this plan (#84 is a readable-ID prefix,
      not a ULID; #85's collision count is ref-dependent, the invariant
      defect is the unconditional base-history treatment and exit
      semantics). Decomposition assignment from
      `20260730T074742Z-contribution-auditor-bhhh`. Received full CLEAR
      from `20260730T131404Z-contribution-auditor-rg8h.md`. Refreshed
      2026-08-04 against current `main` (line references, BREAKING
      impact/migration note, explicit dependency on MR !746) per governor
      scheduling decision `20260804T134723Z-governor-reviewer-q4a8.md`
      and handoff `20260804T134725Z-contribution-auditor-nej7.md`.
      Coupled review `20260804T164959Z-contribution-auditor-p438.md`
      returned BLOCK (B-V26-1, B-V26-2, B-V26-3, A-V26-1): froze the
      JSON/text output contract (`effective_base_oid`,
      `effective_base_source`, `legacy_base_collisions`, and the narrowed
      `sequence_collisions` field), added
      `docs/reference/coord-visibility.md` to affected files/acceptance
      criteria, added the mixed exit-code test case (legacy noise plus a
      genuine obligation -> exit 1, obligation rendered first), and
      refreshed ref-dependent evidence wording with both the 2026-07-30
      and 2026-08-04 measured counts. Round-4 coupled re-review
      `20260804T172356Z-contribution-auditor-61gt.md` cleared MR !746 and
      found one narrow blocker in this plan (B-V26-CONTRACT-R4-6): "Test
      strategy" items 2(a) and 4 contradicted the just-frozen output
      contract by requiring byte-identical/single-line output, when the
      contract itself adds an always-first `effective-base:` text line
      and three always-present JSON fields on every invocation, including
      the clean case. Corrected both items to assert unchanged detection
      findings/exit code plus the frozen additive fields, not byte
      identity. Requesting bounded round-5 review of this plan alone (no
      change needed to MR !746 per `61gt`).
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Parsing/exit-code/discovery fixes to read-only coordination-channel
      commands. No secret, credential, permission, or trust-boundary
      change.
---

# Plan: agov coordination read-path reliability (#84, #85, #89)

## Trigger

Filed 2026-07-30 during a Claude Code session on this repository's own
foundry work: an agent needed to find coordination records published on
an unmerged branch, and the intended tools (`agov coord list --since`,
`agov coord visibility`) both failed in ways that pushed the agent back to
raw `git log`/`git show`/`grep`. Issues #84, #85, #89 filed from that
session; independently reproduced and confirmed by contribution-auditor
(`rk2k`), which also filed #89 as an additional, independently-found
defect in the same read path.

These three issues share one property that groups them into a single
plan: they all govern whether an agent can reliably discover coordination
state, as opposed to #86/#88 (transport protocol) or #87/#90 (assistant
discoverability of the tooling itself). Per `bhhh`, coordinate with #76
(visibility performance) but keep separate acceptance criteria —
correctness/legacy-tolerance/remote-ref-discovery here, throughput there.

**Dependency on `docs/plans/issue76-coord-visibility-performance.md`
(MR !746), per governor decision
`docs/coordination/20260804T134723Z-governor-reviewer-q4a8.md`:** this
plan (`!805`) is the sole authority for default branch selection,
effective-base resolution, and remote-branch discovery semantics — `!746`
must not (and, per its own reconciled revision, does not) redefine any of
those contracts. `!746`'s batching work is what makes Slice 3's widened
default scan (local + remote-tracking branches, potentially hundreds of
refs on a branch-heavy checkout) affordable to run by default without
reintroducing the O(branches × files) subprocess-spawn cost `!746`
separately fixes. This plan's acceptance does not require `!746` to be
implemented first — Slice 3 is correct on its own, just potentially slow
on a large checkout until `!746` lands — but both should be implemented
in the same coordinated `v2.6.0` branch/MR with issue-separated commits,
per `q4a8`.

## Issues in scope

### #84 — `agov coord list --since` rejects the compact filename-prefix timestamp

`tooling/rust/crates/agov/src/main.rs:2684`, `coord_list_cursor()`
(line refreshed against current `main` at `95a8a9a6`; this plan's merge
base is 634 commits behind current `main` as of this revision — content
below re-verified directly against current source, not assumed
unchanged).
Currently tries `time::OffsetDateTime::parse(since, &Rfc3339)` (requires
punctuation: `2026-07-29T19:54:31Z`), then falls back to matching a full
message id or filename stem exactly. The compact form embedded as the
literal first path segment of every coordination filename
(`20260729T195431Z`, no separators) matches neither branch and is
rejected. Correction from `rk2k`: this is a readable-ID prefix format
(this project's own coordination-id convention), not a ULID — the fix
must not assume ULID parsing.

### #85 — `agov coord visibility` hard-fails on pre-existing legacy sequence collisions

`tooling/rust/crates/agov/src/main.rs:3722` (`base_sequence_collisions_visibility`,
computed once from the full base-branch history, independent of which
branch/base-branch pair is actually being compared) and `:3934`
(`visibility_exit_code`: any non-empty `sequence_collisions` is treated
the same as a genuine visibility error, regardless of relevance to the
branch under inspection — lines refreshed against current `main`).
Reproduced (2026-07-30, `rg8h`'s review baseline): 15 collision groups
from 330 known-legacy `NNNNN-role.md` files in this repo's own history
produced `ERR sequence collision` (~44 lines) ahead of the one actionable
`WARN` line, and the command exits `2`. Re-measured independently
(2026-08-04, `p438`'s coupled review, current `main` at `95a8a9a6`): 14
collision groups from 324 legacy files — a smaller count than the
2026-07-30 baseline because history moves and matching legacy files age
out or get validated, exactly the ref-dependency `rk2k` flagged below.
`agov session resume` handles the identical legacy files by counting,
warning once, and continuing (exit `0`) — `coord visibility` should match
that discipline for collisions that exist purely in base history and are
not caused by, or relevant to, the branch being scanned. Correction from
`rk2k`: the collision *count* is ref-dependent (varies with which
branch/base-branch pair is compared, and drifts over time as this
repository's own history grows) — the acceptance criterion is the
unconditional treatment and exit semantics, not a fixed count. Both
figures above are historical observations anchored to their own date and
ref, not claims about the count at any other time; test fixtures (see
"Test strategy" below) use a small representative fixture sized to
exercise the defect, not a reproduction of either count.

### #89 — default `agov coord visibility` only enumerates `refs/heads`, missing remote-only branches

A fresh `git fetch` populates `refs/remotes/origin/*` but does not create
local branches. Default `coord visibility` (no `--branch`) does not see
coordination published on a fetched-but-not-checked-out remote branch
unless the caller already knows to pass `--branch origin/<name>`
explicitly — which defeats the command's purpose as a *discovery* tool
for branches the caller doesn't yet know to name.

Exact-head review (`20260730T083948Z-contribution-auditor-1dq4`, findings
B1/B2) identified two gaps this plan's first draft left unaddressed, both
within #89's own accepted contract, not new scope:

- **B1 — stale base-ref resolution.** #89's contract requires the
  documented `git fetch` then `--base-branch main` workflow to compare
  against current state, not a possibly-stale local `main`. Enumerating
  remote branches is necessary but not sufficient if the *comparison
  base* itself is stale — a local `main` that hasn't been fast-forwarded
  after a fetch would make even a correctly-enumerated remote branch's
  obligations look resolved when they aren't, or vice versa.
- **B2 — remote enumeration/deduplication contract underspecified.** The
  first draft said "deduplicated against local branches tracking the
  same ref" without defining ref eligibility, symbolic-ref handling
  (`refs/remotes/<remote>/HEAD`), or how equivalence is determined when
  local and remote branch names differ.

Both are folded into Slice 3 below as explicit, testable acceptance
criteria rather than left as implementation-time judgment calls.

## Gate 1 decomposition

One plan, three independently testable acceptance criteria, because all
three touch the same command family and a shared reviewer can answer one
combined yes/no ("can an agent now reliably read coordination state
across cursor formats, legacy history, and remote-only branches?") without
needing to split by sub-issue.

### Slice 1 — compact-timestamp cursor equivalence (#84)

Accept `YYYYMMDDTHHMMSSZ` (no separators) as a third valid `--since` shape
in `coord_list_cursor()`, alongside RFC3339-with-punctuation and the full
message id/filename. Implementation approach: attempt a compact-format
parse (fixed-width, unambiguous — it is exactly the RFC3339 form with `-`
and `:` stripped) before falling through to the id/filename match, so a
successful compact parse takes the same code path as the RFC3339 branch.

### Slice 2 — legacy-collision tolerance in `coord visibility` (#85)

Change `render_visibility_text`'s (and its JSON counterpart's) exit-code
decision so that sequence collisions confined to base-branch history *and*
unrelated to the branch/base-branch pair being compared are reported as a
single summary count with a pointer to `agov coord validate` (matching
`session resume`'s existing pattern), not as a per-collision `ERR` block,
and do not by themselves force a nonzero exit. A collision that *does*
involve the branch under inspection (a real conflict a caller needs to
know about) keeps blocking behavior — this slice narrows the blast radius
of legacy noise, it does not silence genuine collisions.

### Slice 3 — remote-only branch discovery and base-ref freshness (#89)

Three sub-parts, all within #89's original contract:

1. **Branch enumeration.** Extend default `coord visibility` branch
   enumeration to include `refs/remotes/<remote>/*`, not only
   `refs/heads/*`. Explicit `--branch` selection continues to work
   unchanged; this only changes what the *default*, no-argument scan
   considers.
2. **Deterministic deduplication (B2, re-frozen per R1).** The earlier
   fix stated two rules that contradicted each other: equivalence
   required both a shared OID *and* a tracking relationship, but the
   multi-remote handling then grouped "by OID" alone — R1 correctly
   caught that these can't both be the grouping key. Single rule,
   replacing both: a ref is eligible for enumeration if it is a
   `refs/heads/*` or `refs/remotes/<remote>/*` ref and is not a symbolic
   ref (`refs/remotes/<remote>/HEAD` is never enumerated, and neither is
   any alias of the effective base ref — see item 3). All eligible refs
   are grouped by their **peeled commit OID alone** — tracking
   configuration is not part of the equivalence test, only used to break
   ties in display-name preference.

   **Display-name selection (re-frozen per R4): class priority, then a
   total order within the class.** Every eligible ref in a group falls
   into exactly one of three classes, tested in this order:

   - **Class 1 — local.** Any `refs/heads/*` ref in the group.
   - **Class 2 — origin-tracking.** Any `refs/remotes/origin/*` ref in
     the group (only reached if Class 1 is empty).
   - **Class 3 — other-remote-tracking.** Any other
     `refs/remotes/<remote>/*` ref in the group (only reached if Classes
     1 and 2 are both empty).

   The group's display name is the **lexicographically smallest
   fully-qualified ref name** (a plain string comparison of the complete
   `refs/heads/<name>` or `refs/remotes/<remote>/<name>` string, not
   just the branch's short name) among refs in the *first non-empty
   class* — this is R4's fix: the earlier "a local name" / "the origin
   name" phrasing named a class but never a member within it, so two
   local branches or two `origin/*` branches at the same OID had no
   defined winner. Lexicographic comparison on the full ref string is a
   total order (no two distinct ref strings compare equal), so this
   always selects exactly one name, including when two local aliases,
   two `origin/*` aliases, or several non-origin remotes tie within
   their class. Cardinality: **one entry per distinct OID group**,
   regardless of how many refs (local or remote, across however many
   remotes) share that OID — a group with a local branch and three
   remote-tracking refs at the same OID is one entry, not four; a group
   with only two different remotes' tracking refs at the same OID (no
   local branch) is also one entry, named by the lexicographic rule
   above.
3. **Base-ref freshness and exclusion (B1, R1).** `--base-branch main`
   (the default) resolves to the local `main` ref today. Two parts:
   - **Freshness.** When a configured upstream for the base branch
     exists (e.g. `main` tracks `origin/main`) and the upstream's OID
     differs from the local base branch's OID, prefer the upstream's
     OID as the effective comparison base and state which OID was
     actually used in both text and JSON output. This makes the
     documented `git fetch` then `agov coord visibility --base-branch
     main` workflow correct without requiring the caller to separately
     fast-forward local `main` — the exact failure mode `1dq4` named
     ("This checkout itself demonstrates why that matters: a local
     `main` can be far behind `origin/main` after a fetch"). When no
     upstream is configured, behavior is unchanged (local ref only) —
     this plan does not require every repository to have a remote.
   - **Base exclusion (R1's second finding).** Every ref that peels to
     the effective base OID — the local base branch itself and every
     remote-tracking ref at that same OID (e.g. both `main` and
     `origin/main` when they match) — is excluded from
     `scanned_branches`/candidate enumeration entirely. The base is the
     comparison point, not a candidate obligation source; a caller
     should never see "hidden obligation on `main`" when `main` is the
     branch they're already comparing against.

### Frozen output contract for Slices 2 and 3 (B-V26-1, `p438`)

`p438`'s review correctly found that "effective base appears in JSON and
text" and "base-history collisions collapse to a summary" are observable
automation contracts that cannot be left to implementation discretion
under a BREAKING plan. This subsection freezes both, grounded directly in
current `VisibilityReport`/`visibility_json`/`render_visibility_text`
(`tooling/rust/crates/agov/src/main.rs:3118`, `:3881`, `:3827`).

**New/changed `VisibilityReport` JSON fields:**

- `base_branch` (existing field, **unchanged meaning**): stays exactly
  the requested base branch name as given via `--base-branch` (default
  `"main"`) — Slice 3 does not repurpose this field to hold a resolved
  OID or ref name. This is called out explicitly because it is the field
  most likely to be misread as already covering the new behavior.
- `effective_base_oid` (**new**, string, full 40-character hex commit
  OID): the commit actually used as the comparison base after Slice 3's
  freshness rule. Always populated, including the unchanged/no-upstream
  case (it is simply the local base branch's OID then) — the field is
  never `null` or omitted, so callers never need an existence check.
- `effective_base_source` (**new**, string enum `"local"` |
  `"upstream"`): which ref supplied `effective_base_oid`. `"upstream"`
  only when a configured upstream's OID differs from the local base
  branch's OID and Slice 3's freshness rule substituted it; `"local"` in
  every other case (no upstream configured, or upstream OID matches
  local).
- `sequence_collisions` (existing field, **narrowed semantics — the one
  BREAKING change to an existing field's contents**): after this plan,
  contains **only** branch-relevant collisions — JSON
  `"type": "branch_base_seq_collision"` (Rust
  `VisibilityCollision::BranchBaseSeq`). Base-only legacy collisions
  (JSON `"type": "base_duplicate_seq"`, Rust
  `VisibilityCollision::BaseDuplicateSeq`) are removed from this array.
  Any adopter script filtering `sequence_collisions` by `"type"` to find
  `base_duplicate_seq` entries will find the array no longer contains
  them — see `legacy_base_collisions` below instead.
- `legacy_base_collisions` (**new**, object, always present — never
  `null` or omitted): `{"count": <integer>, "seqs": [<integer>, ...]}`.
  `count` is the number of distinct `seq` groups colliding purely within
  base-branch history (exactly the `VisibilityCollision::BaseDuplicateSeq`
  entries removed from `sequence_collisions` above); `seqs` lists those
  `seq` values, sorted ascending, for diagnosability without per-file
  detail. Empty case is `{"count": 0, "seqs": []}`, not an omitted field.
- `exit_code` (existing field, **decision inputs change, existing 0/1/2
  values unchanged**): `2` iff `sequence_collisions` (now branch-relevant
  only) is non-empty OR `errors` is non-empty; else `1` iff
  `unbridged_obligations` is non-empty; else `0`.
  `legacy_base_collisions.count` never drives `exit_code` by itself —
  this is the mechanism for Slice 2's "does not by themselves force a
  nonzero exit," and it composes with Slice 3 to produce the mixed case
  `p438` flagged as untested (B-V26-3, resolved in "Test strategy" item 2
  below): base-only legacy noise **plus** a genuine unbridged obligation
  still exits `1` — the legacy noise never escalates it to `2`, and never
  suppresses the `1`.

**Text rendering order (all cases, including the mixed case):**

1. Effective-base line, always first, every exit path:
   `effective-base: <oid> (<local|upstream>)`.
2. If `sequence_collisions` (branch-relevant) is non-empty: the existing
   `ERR sequence collision` block(s), unchanged per-collision detail —
   these remain genuine, individually-reported collisions.
3. If `unbridged_obligations` is non-empty: the existing
   `WARN branch-local coordination obligations not visible on base
   branch` block(s), rendered **before** the legacy-collision summary
   line (item 4) — the actionable item is never buried under legacy
   noise, resolving `p438`'s explicit ordering requirement for the mixed
   case.
4. If `legacy_base_collisions.count > 0`: a single summary line,
   `WARN <count> legacy sequence collision group(s) confined to base
   branch history; run agov coord validate for detail` — matching
   `session resume`'s existing warn-once-and-continue pattern, never a
   per-collision block.
5. If none of 2/3/4 produced output: `OK coordination visibility clean
   relative to <base_branch>` (unchanged wording), as the sole line after
   the effective-base line.

## Affected files

- `tooling/rust/crates/agov/src/main.rs` — `coord_list_cursor` (#84),
  `base_sequence_collisions_visibility`/`render_visibility_text`/
  `visibility_json` (#85), branch enumeration in the `coord visibility`
  command handler (#89).
- `tooling/rust/crates/agov/tests/coord_write.rs` — cursor-parsing tests.
- `tooling/rust/crates/agov/tests/coord_visibility.rs` — existing
  `ERR sequence collision` assertion (line 42) needs an accompanying test
  for the new tolerant behavior; do not delete the existing collision
  test without a replacement per `feature-regression-guard-rule.md` — a
  genuine, branch-relevant collision must still be caught.
- `AGENTS.md` — the "Supported commands" `coord list` description already
  documents "one line per coordination message after a given id,
  filename, or RFC3339 timestamp"; add the compact form once implemented.
- `docs/reference/coord-visibility.md` (B-V26-2, `p438`) — the current
  page directly contradicts this plan's changes in three places and was
  missing from the original affected-files list (it named only
  `AGENTS.md`, for #84): "The scanner compares local branches against the
  base branch" (line 44) must become local **and** remote-tracking
  branches, per Slice 3; the "Exit Codes" table (lines 66–72), including
  "If both obligations and collisions exist, the command exits `2`,"
  must be replaced with the frozen decision table above (branch-relevant
  collisions and scanner errors force `2`; legacy-only base-history
  collisions never do); and the page must describe the new
  `effective_base_oid`/`effective_base_source`/`legacy_base_collisions`
  fields and the fetched/effective-base freshness behavior from Slice 3.
  Checked for shipped-Pages/template parity per
  `documentation-rule-projection-rule.md`: `agf/docs/site/docs/operate/
  poll-and-coordinate.md`, `agf/docs/site/docs/adopt/tooling-and-tokens.md`,
  `agf/docs/site/docs/advanced/contracts-and-validators.md`,
  `agf/docs/site/reference/tooling-conformance.md`, and
  `agf/templates/AGENTS.md` all reference `agov coord visibility` but
  none restate the exit-code table or branch-scope claim in detail — no
  other shipped page requires a parity update for this change.

## Test strategy (Gate 2 evidence)

Per `bhhh`'s minimum evidence list:

1. **Cursor equivalence**: compact timestamp, full message id, and
   RFC3339-with-punctuation for the *same* target message all return the
   identical result set.
2. **Legacy-collision tolerance and the required exit-code matrix
   (B-V26-3, `p438`)**: a small representative fixture (not a
   330-file/15-group reproduction — see the updated "#85" evidence
   wording above) exercises all three outcomes explicitly:
   - (a) base-history-only legacy noise, no obligation ->
     `exit_code: 0`, `legacy_base_collisions.count > 0`,
     `sequence_collisions: []`, text output is the always-first
     `effective-base:` line (Frozen output contract item 1) followed by
     exactly one warning summary line for the legacy collisions (Frozen
     output contract item 4), with no `ERR` block.
   - (b) the same base-history-only legacy noise **plus** a genuine
     unbridged obligation on the branch under test -> `exit_code: 1`,
     `legacy_base_collisions.count > 0`, `sequence_collisions: []`,
     `unbridged_obligations` non-empty — text output renders the
     `WARN branch-local coordination obligations` block **before** the
     legacy-collision summary line (this is `p438`'s explicit ordering
     requirement and the exact case the prior draft's two-outcome test
     left untested). This is the live dogfooding shape: Foundry's own
     `main` has legacy base-history noise and, at any given time, may
     also have a genuine open obligation.
   - (c) a branch-relevant collision (`VisibilityCollision::BranchBaseSeq`)
     -> `exit_code: 2`, `sequence_collisions` non-empty — unchanged
     blocking behavior, still individually reported.
   Both text and JSON assertions are required for all three outcomes, not
   just exit code — a test that only checks `exit_code` would not catch a
   correct exit code paired with a wrong `legacy_base_collisions` count
   or wrong block ordering.
3. **Remote-only branch discovery**: a fixture with a coordination file
   only on `refs/remotes/origin/<branch>` (never checked out locally) is
   found by default `coord visibility` after a fetch, with no `--branch`
   argument.
4. **Unchanged detection for clean local-only repositories (corrected
   per `61gt`, finding B-V26-CONTRACT-R4-6)**: a fixture with no legacy
   files and only local branches produces the same detection findings
   and the same `exit_code` (`0`) as current behavior — **not**
   byte-identical output. Output is not byte-identical even in this
   clean case: text output gains the always-first `effective-base:` line
   (Frozen output contract item 1) and JSON output gains the three
   always-present `effective_base_oid`/`effective_base_source`/
   `legacy_base_collisions` fields (the last as `{"count": 0, "seqs":
   []}`) on every invocation, per the frozen contract above — this test
   must assert the frozen additive fields are present with their
   documented empty-case values, not assert their absence via a
   byte-identity comparison against pre-change output.
5. **Deduplication cardinality and naming (B2/R4, re-tested)**: one
   entry per OID group, tested with (a) a local branch plus one
   remote-tracking ref at the same OID — reports once, under the local
   name; (b) a remote-only ref (no local branch) — reports once, under
   its remote-tracking name; (c) two different remotes' tracking refs at
   the same OID with no local branch — reports **once** (not twice),
   under the lexicographically-smallest fully-qualified name; (d)
   `refs/remotes/<remote>/HEAD` — never enumerated; (e) **two local
   branches** at the same OID (R4) — reports once, under the
   lexicographically-smallest `refs/heads/*` name; (f) **two
   `origin/*` refs** at the same OID with no local branch (R4) — reports
   once, under the lexicographically-smallest `refs/remotes/origin/*`
   name. Cases (e) and (f) are the exact two shapes R4 found undefined
   in the prior draft.
6. **Base-ref freshness and exclusion (B1/R1)**: a fixture with local
   `main` behind its configured upstream and a remote-only obligation
   that only resolves correctly against the upstream OID — the default
   `--base-branch main` workflow must find it, `effective_base_oid` must
   equal the upstream OID and `effective_base_source` must equal
   `"upstream"`, and the text `effective-base:` line must show the same
   OID. A second fixture with no configured upstream confirms
   `effective_base_source: "local"` and unchanged local-only behavior. A
   third fixture asserts that neither the local base branch nor
   `origin/<base>` (when it matches the effective base OID) ever appears
   in `scanned_branches` or as a reported obligation candidate.
7. **Frozen JSON contract, positive and negative (B-V26-1, `p438`)**: per
   `boundary-shape-testing-rule.md`, assert the exact field shape, not
   just presence — (a) positive: `effective_base_oid` is a 40-character
   hex string, `effective_base_source` is exactly `"local"` or
   `"upstream"`, `legacy_base_collisions` is always an object with
   integer `count` and array `seqs` (never `null`/omitted, including the
   zero case), and `sequence_collisions` entries are exclusively
   `"type": "branch_base_seq_collision"` after this change; (b) negative:
   a fixture with a base-only legacy collision asserts
   `sequence_collisions` does **not** contain a `"type":
   "base_duplicate_seq"` entry (the pre-change shape) — this is the
   negative case that would silently pass if an implementation kept the
   old field contents and merely changed exit-code logic around them.
8. **Operator documentation verification (B-V26-2)**: after
   `docs/reference/coord-visibility.md` is updated, confirm no remaining
   reference to "local branches" (singular scope) or the old three-row
   exit-code table without the legacy/branch-relevant distinction — a
   grep-based check is sufficient (the page's content is not
   machine-validated), documented as a manual Gate 2 review step since
   `agov validate adopter-docs` covers presence/parity, not per-page
   semantic correctness.

All new tests are unit/integration tests in the existing
`tooling/rust/crates/agov/tests/` suite; no live GitLab API calls needed
(`coord list`/`coord visibility` operate on local Git state).

## Risks

- Loosening `coord visibility`'s exit-code strictness could mask a
  genuine collision if the "confined to base history and unrelated to the
  branch under inspection" test is implemented too permissively. Mitigated
  by the explicit negative-case test in item 2 above and by keeping the
  narrowing scoped to exactly the legacy-file shape, not collisions in
  general.
- The compact-timestamp parser must not silently accept malformed input
  (e.g. a 13-digit string) as a valid cursor. Mitigated by requiring an
  exact fixed-width match before attempting the parse.
- Preferring the upstream OID over local `main` when they diverge changes
  default output for anyone relying on today's local-only comparison.
  Mitigated by stating the effective base OID explicitly in output (both
  text and JSON) so the change is visible, not silent, and by the
  no-upstream-configured fallback preserving current behavior exactly.

## Downstream-impact classification and migration note

**BREAKING**, not ADDITIVE/PREVENTIVE, per `always/downstream-impact-rule.md`
(correcting this plan's own prior rollback text, which understated the
change — flagged by `20260804T120121Z-contribution-auditor-s7bq.md`
finding B-CR-5 against a duplicate draft of this same scope, and
confirmed applicable here by
`20260804T130826Z-contribution-auditor-3skh.md` point 3). Both Slice 2
and Slice 3 change observable command/automation exit-code contracts,
even though both changes are correctness fixes, not regressions:

- Slice 3's widened default branch enumeration can change a previously
  exit-`0` invocation (no obligations found, because the caller's remote
  branch was invisible to the old scan) to exit `1` (a real obligation is
  now correctly discovered).
- Slice 2's legacy-collision tolerance can change some previously exit-`2`
  invocations (blocked purely by legacy noise) to exit `0` or `1`.

### Three-circle impact statement

```
Impact:
  Foundry:          Yes. This project's own coordination-channel history
                     (illustrative counts under "#85" above — 15 groups/
                     330 files as of 2026-07-30, 14 groups/324 files as of
                     2026-08-04, ref-dependent and expected to drift) is
                     exactly the fixture this plan's own evidence is drawn
                     from; Foundry's own `agov coord visibility`
                     invocations change exit code today under both
                     slices.
  Direct adopters:  BREAKING. Any adopter script or CI job that branches
                     on `agov coord visibility`'s exit code (0/1/2) may
                     see a different code for the same repository state
                     after this change, for the two directions above.
  Downstream:       No schema, artifact, or file-format change. The
                     JSON/text output shape gains two always-present
                     fields (`effective_base_oid`, `effective_base_source`
                     — additive) and one always-present summary object
                     (`legacy_base_collisions` — additive), and narrows
                     one existing field's contents: `sequence_collisions`
                     no longer contains `"type": "base_duplicate_seq"`
                     entries (moved to `legacy_base_collisions`) — this
                     narrowing is the one BREAKING change to an existing
                     field, not merely additive; see "Frozen output
                     contract for Slices 2 and 3" above for the full
                     field-by-field freeze.
  Migration:        Adopter CI/scripts that treat any nonzero exit as
                     uniformly "stop" are unaffected (still nonzero when
                     a genuine obligation or collision exists). Scripts
                     that distinguish exit 1 vs 2 specifically, or that
                     relied on legacy-collision noise as a signal, should
                     re-verify their handling after upgrading.
  Evidence:         B-CR-5 (s7bq, against a duplicate draft of this same
                     scope) and 3skh point 3, both independently
                     identifying the same gap in this plan's own prior
                     rollback text.
```

## Rollback

Revert the governed commit/MR as a whole. No schema, data, or artifact
migration is involved (corrected from "no migration involved" — that
referred to file/schema migration, not the BREAKING command-contract
change documented above, which this plan now states explicitly rather
than leaving implicit).

## Out of scope

- #76's performance/batching work (coordinated, not merged into this
  plan, per `bhhh`).
- #86, #88 (MCP transport) and #87, #90 (assistant discovery) — separate
  plans.
- Any v2.4.6 release action.
