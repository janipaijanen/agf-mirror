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
      `20260730T074742Z-contribution-auditor-bhhh`.
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

## Issues in scope

### #84 — `agov coord list --since` rejects the compact filename-prefix timestamp

`tooling/rust/crates/agov/src/main.rs:2495-2511`, `coord_list_cursor()`.
Currently tries `time::OffsetDateTime::parse(since, &Rfc3339)` (requires
punctuation: `2026-07-29T19:54:31Z`), then falls back to matching a full
message id or filename stem exactly. The compact form embedded as the
literal first path segment of every coordination filename
(`20260729T195431Z`, no separators) matches neither branch and is
rejected. Correction from `rk2k`: this is a readable-ID prefix format
(this project's own coordination-id convention), not a ULID — the fix
must not assume ULID parsing.

### #85 — `agov coord visibility` hard-fails on pre-existing legacy sequence collisions

`tooling/rust/crates/agov/src/main.rs:3342` (`base_sequence_collisions_visibility`,
computed once from the full base-branch history, independent of which
branch/base-branch pair is actually being compared) and `:3746` (exit-code
decision: any non-empty `sequence_collisions` is treated the same as a
genuine visibility error, regardless of relevance to the branch under
inspection). Reproduced: 15 collision groups from 330 known-legacy
`NNNNN-role.md` files in this repo's own history produce `ERR sequence
collision` (~44 lines) ahead of the one actionable `WARN` line, and the
command exits `2`. `agov session resume` handles the identical legacy
files by counting, warning once, and continuing (exit `0`) — `coord
visibility` should match that discipline for collisions that exist purely
in base history and are not caused by, or relevant to, the branch being
scanned. Correction from `rk2k`: the collision *count* is ref-dependent
(varies with which branch/base-branch pair is compared) — the acceptance
criterion is the unconditional treatment and exit semantics, not a fixed
count.

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
2. **Deterministic deduplication (B2).** A ref is eligible for
   enumeration if it is a `refs/heads/*` or `refs/remotes/<remote>/*`
   ref and is not a symbolic ref (explicitly excludes
   `refs/remotes/<remote>/HEAD`). Two eligible refs are the same branch
   if and only if they resolve to the same commit OID *and* one is the
   configured upstream of the other (matching Git's own local/remote
   tracking relationship — this project's tooling already relies on
   `git branch -vv`-style tracking metadata elsewhere, not a plain
   name-string match, since a local branch and its remote-tracking ref
   commonly share a name but a fork or renamed branch would not).
   When a local branch and its tracked remote-tracking ref are the same
   OID, report the local name (it's the one the caller can act on
   directly); when only a remote-tracking ref exists (no local branch),
   report the remote-tracking name as-is. Multiple remotes tracking the
   same OID under different names are each reported once, keyed by OID,
   not silently merged into one arbitrary name.
3. **Base-ref freshness (B1).** `--base-branch main` (the default)
   resolves to the local `main` ref today. Add resolution logic: when a
   configured upstream for the base branch exists (e.g. `main` tracks
   `origin/main`) and the upstream's OID differs from the local base
   branch's OID, prefer the upstream's OID as the effective comparison
   base and state which OID was actually used in both text and JSON
   output. This makes the documented `git fetch` then `agov coord
   visibility --base-branch main` workflow correct without requiring the
   caller to separately fast-forward local `main` — the exact failure
   mode `1dq4` named ("This checkout itself demonstrates why that
   matters: a local `main` can be far behind `origin/main` after a
   fetch"). When no upstream is configured, behavior is unchanged
   (local ref only) — this plan does not require every repository to
   have a remote.

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

## Test strategy (Gate 2 evidence)

Per `bhhh`'s minimum evidence list:

1. **Cursor equivalence**: compact timestamp, full message id, and
   RFC3339-with-punctuation for the *same* target message all return the
   identical result set.
2. **Legacy-collision tolerance**: a fixture reproducing this repo's own
   330-legacy-file/15-collision-group shape produces a summary warning and
   exit `0` when no collision involves the branch under test; a fixture
   with a collision that *does* involve the branch under test still
   produces a blocking result — both cases need their own test, not just
   the tolerant one, per `boundary-shape-testing-rule.md`'s negative-case
   discipline.
3. **Remote-only branch discovery**: a fixture with a coordination file
   only on `refs/remotes/origin/<branch>` (never checked out locally) is
   found by default `coord visibility` after a fetch, with no `--branch`
   argument.
4. **Unchanged behavior for clean local-only repositories**: a fixture
   with no legacy files and only local branches produces byte-identical
   output to current behavior — this plan must not regress the common
   case while fixing the edge cases.
5. **Deduplication (B2)**: local and remote-tracking refs at the same OID
   report once, under the local name; a remote-only ref (no local
   branch) reports under its remote-tracking name; multiple remotes at
   the same OID under different names each report once, keyed by OID;
   `refs/remotes/<remote>/HEAD` is never enumerated.
6. **Base-ref freshness (B1)**: a fixture with local `main` behind its
   configured upstream and a remote-only obligation that only resolves
   correctly against the upstream OID — the default `--base-branch main`
   workflow must find it, and output must state which OID was used. A
   second fixture with no configured upstream confirms unchanged
   local-only behavior.

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

## Rollback

Revert the governed commit/MR as a whole; no schema, data, or artifact
migration involved — this changes command-line parsing and exit-code
logic only.

## Out of scope

- #76's performance/batching work (coordinated, not merged into this
  plan, per `bhhh`).
- #86, #88 (MCP transport) and #87, #90 (assistant discovery) — separate
  plans.
- Any v2.4.6 release action.
