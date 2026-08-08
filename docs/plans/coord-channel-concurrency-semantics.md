---
id: coord-channel-concurrency-semantics
status: pending-gate1
topic: coord-channel-concurrency-semantics
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: spec + validator addition; no auth, secrets, or tenant-isolation surface
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor available to consult. Flag for Gate 2 review — this plan formalises coordination-channel semantics, which touches governance structure per concept-modelling-rule (ordering guarantee is intrinsic to the channel design, not derived from any one writer).
---

# Plan: Coordination Channel Concurrency Semantics (GitLab #33)

**Date:** 2026-07-20T07:47:31Z
**Status:** PENDING GATE 1 APPROVAL
**Sequencing:** First in the coord-cli-ci-distribution cluster. `coord-cli-write-and-watch.md`
(GitLab #32, #36) depends on the ordering/collision answer this plan gives.

**Planning clarification (2026-08-08):** Beagle/CRDT prior-art review
(`20260808T183635Z-contribution-auditor-gjsk.md`) added the layered-convergence
boundary and negative semantic-conflict scenario below. This does not approve
this plan or expand it into CRDT storage or semantic-conflict enforcement.

---

## Intent

Formalise, in writing, what concurrency guarantees the coordination channel
already provides by construction (append-only files, one file per message,
readable-ID naming) and what it does not — then close the one concrete gap
found while grounding this plan in the current validator: **duplicate-ID
detection today only compares files passed on a single CLI invocation, not
the new files against the rest of the channel.**

This is a documentation-first plan. The behaviour it formalises already
holds in practice (git mechanics + the existing readable-ID scheme); the
validator gap is the only functional change.

## Problem (grounded in the actual codebase)

`tooling/rust/crates/agov-coord/src/readable_id.rs` already generates IDs of
the form `YYYYMMDDThhmmssZ-<role-slug>-<4-6 char suffix>`, where the suffix
is 4 characters drawn from a 32-symbol Crockford-style alphabet (see
`readable_suffix()`). `tooling/rust/crates/agov/src/main.rs::duplicate_id_errors`
already flags duplicate `id:` values — but only among the files given to a
single `agov coord validate <files...>` invocation.

`.gitlab-ci.yml`'s `agov:governance:validate` job invokes it like this:

```bash
git diff --name-only -z "$base"...HEAD -- 'docs/coordination/*.md' \
  | xargs -0r "$agov_bin" coord validate
```

This validates only the files changed in the current branch/MR against each
other. It does **not** check the new files against IDs already committed to
`main`. Two agents working in separate worktrees, each adding one new
coordination message, will each pass CI individually. If (astronomically
unlikely but not impossible — see probability note below) their generated
suffixes collide on the same role and the same wall-clock second, or if a
human hand-writes a manual ID that happens to match an existing one, the
collision is invisible until someone runs `agov coord validate` (or
`coord status`) over the whole directory after both branches have merged.

This is exactly the failure mode issue #33 asks the spec to name explicitly
and the validator to catch.

## What the channel already guarantees (formalise, do not change)

1. **Append-only by convention, conflict-free by construction for the
   common case.** Every message is a new file. Two agents in two worktrees
   adding two different files never produce a git text-merge conflict —
   there is no shared line to conflict over. This holds regardless of
   commit order or rebase order.

2. **Per-writer ordering only.** A single writer's own messages are
   naturally ordered by wall-clock timestamp (and, transitively, by
   `references-to` chains it builds). **There is no global ordering
   guarantee across concurrent writers.** Two agents in separate worktrees
   who each write a message "at the same time" have no defined relative
   order until their branches are merged, and even after merging, filename
   or timestamp sort does **not** establish causality — only explicit
   `references-to` edges do. This matches what
   `agf/docs/site/docs/concepts/coordination-protocol.md` already says
   informally ("The references make the work thread navigable; they are
   not a magic conflict resolver") — this plan makes the ordering claim
   explicit and testable rather than implicit.

3. **Collision probability is small but not zero, and must be stated
   honestly (per verifiable-claims-rule), not hand-waved.** The 4-character
   suffix draws from a 32-symbol alphabet: 32⁴ = 1,048,576 possible
   suffixes per (role, second) pair. Two concurrent writers acting under
   the *same role slug* in the *same wall-clock second* collide with
   probability approximated by the birthday bound: for `n` writers in one
   role-second bucket, collision probability ≈ `n(n-1) / (2 × 1,048,576)`.
   For the realistic case of 2 simultaneous writers of the same role, that
   is ≈ 0.00005% per second-bucket — small, but the channel has run for
   months and role concurrency (e.g. two `coder-tester` sessions) is a
   real and growing pattern as issue #36/#37 make automated polling more
   common. The spec must document this as "collisions are rare, not
   impossible" rather than claim they cannot happen.

4. **Merge semantics.** Two branches that each add a distinct
   readable-ID-named file merge without manual conflict resolution —
   this is a mechanical consequence of (1) and is the acceptance-criteria
   test scenario below. If two branches independently produce the *same*
   filename (a real collision, or two humans hand-picking the same manual
   ID), git **will** flag a real merge conflict on that file — this is the
   correct, visible failure mode, not a bug to work around.

5. **Rebase semantics.** Rebasing one worktree's coordination-touching
   branch onto a base that has since gained new coordination files is
   equivalent to the merge case per file: no conflict unless a filename
   collision exists. `agov coord validate` run post-rebase (or in CI on
   the rebased branch) is the safety net for the collision case, once the
   validator gap above is closed.

## Layered convergence and the semantic boundary

“Conflict-free” is not one guarantee. For this channel it separates into five
layers:

1. **File merge:** distinct file additions merge without a Git text conflict.
2. **Record validity:** filenames/IDs are unique and each record satisfies its
   schema and named invariants.
3. **Causal coherence:** `references-to` edges and explicit supersession make
   the relevant sequence reconstructible; timestamp order alone is not
   causality.
4. **Governed semantic consistency:** two valid records do not imply
   incompatible obligations, claims, or binding decisions about one topic.
5. **Downstream correctness:** code, documentation, tests, and human decisions
   remain correct after the concurrent work is combined.

This plan covers layers 1 and 2 and documents the layer-3 ordering boundary. It
does **not** claim to solve layers 4 or 5. A clean Git merge plus a green schema
validation therefore means mechanically convergent records, not necessarily a
coherent or correct governed outcome.

The implementation documentation must include this negative scenario: two
worktrees independently add different, schema-valid filenames with the same
`topic`; one record says an obligation is cleared and the other says it remains
blocked, with neither referencing the other. Git merges both and the current
schema validates both. That outcome demonstrates the boundary; it must not be
presented as successful semantic conflict resolution.

Any future advisory same-topic contradiction/index check requires a separate
plan after an observed harmful ambiguity or concrete adopter need. It must
account for legitimate dissent, later correction, role authority, explicit
supersession, and false positives before receiving even advisory status.

## Scope

**In scope:**
- Rewrite the "Parallel work and conflicts" section of
  `agf/docs/site/docs/concepts/coordination-protocol.md` to state the
  guarantees above explicitly (ordering: per-writer only; collision
  probability: stated, not hand-waved; merge/rebase: conflict-free by
  construction, real collision surfaces as a real git conflict).
- Extend `agf/rules/workflow/coordination-id-rule.md` with a short
  "Concurrency" section cross-referencing the protocol doc, so the rule
  (which already governs ID generation and filename conventions) and the
  concept doc do not drift (message-integrity per marketing-tone-rule's
  sibling discipline, applied here to internal governance docs).
- Extend `agov coord validate` so duplicate-ID checking considers the
  **existing directory contents**, not only the files passed on the
  command line. Concretely: when validating files that live inside a
  coordination directory, also scan sibling `.md` files in that directory
  for `id:` collisions, not just the files given as arguments.
- Add a naming-collision test scenario (two worktrees, each with one new
  distinct-ID file) that merges without manual conflict resolution, as a
  documented and reproducible test — this is the acceptance criterion's
  required "documented test scenario."

**Out of scope (explicitly deferred, per the issue text):**
- Locking or claims TTL redesign (`claim`/`expires_on` semantics are
  unchanged).
- Distributed, non-git transports.
- Native CRDT/RDX storage, Beagle or StrictMark adoption, Git replacement, or
  automatic semantic merging.
- Same-topic contradiction detection or any validator claim that mechanically
  convergent records are semantically consistent. The negative scenario above
  documents this limit; it does not add a blocker.
- Promoting the existing advisory naming-drift warnings
  (`add_readable_drift_findings` in `coord_validator.rs`) to blocking.
  `agf/docs/site/docs/concepts/coordination-protocol.md` and
  `coordination-id-rule.md` already state these are advisory
  ("Validators SHOULD warn, not block"); tool-authority-boundary-rule
  requires a named rule to grant blocking authority before any validator
  output can halt work, and no such rule change is proposed here. Only
  **ID collisions** (already blocking today, within a single invocation)
  gain wider scope; naming drift stays advisory.

## Affected files

| File | Change |
|---|---|
| `agf/docs/site/docs/concepts/coordination-protocol.md` | Rewrite "Parallel work and conflicts" section: explicit ordering guarantee, collision probability, merge/rebase semantics, layered-convergence boundary, and links to both documented scenarios |
| `agf/rules/workflow/coordination-id-rule.md` | Add short "Concurrency" section, cross-reference the protocol doc (no duplication — link, per marketing-tone-rule message-integrity discipline applied internally) |
| `tooling/rust/crates/agov-coord/src/coord_validator.rs` | Add a directory-aware duplicate-ID check (new function, e.g. `duplicate_id_errors_against_dir`) usable independently of the CLI-argument-only check already in `main.rs::duplicate_id_errors` |
| `tooling/rust/crates/agov/src/main.rs` | Wire the new directory-aware check into `validate_coord_files` when the files being validated live under a coordination directory; keep existing behaviour for files outside one |
| `tooling/rust/crates/agov-coord/tests/` (or `agov` crate tests) | New test: fixture directory with an existing "committed" message + a new file whose `id:` collides — assert `coord validate` on the new file alone (not just alongside the old one) reports the collision |
| `.gitlab-ci.yml` | No change required — `agov:governance:validate` already passes changed files; the fix lives inside `coord validate`'s own directory-scan behaviour so no CI wiring changes |

## Migration needed

No schema change. No behavioural change to already-passing validations —
this closes a detection gap (previously-undetected collisions now detected)
and adds documentation. Existing coordination files are unaffected.

## Implementation notes

- The directory-aware check should resolve "the coordination directory" from
  the parent directory of the file(s) being validated (matching how
  `agov coord status --dir` etc. already take a `--dir` default of
  `docs/coordination`), not require a new flag on `coord validate` — keep
  the existing `agov coord validate <files...>` signature unchanged so CI's
  `xargs` invocation keeps working unmodified.
- Read sibling files with the *same* validator path used elsewhere
  (`agov_coord::coord_validator::validate_file`) so a malformed sibling file
  does not itself crash the scan — skip files that fail to parse (they are
  already reported by their own CI run when they were introduced) and only
  compare `id:` values among files that parse successfully.
- Keep the check additive: a file is reported as colliding only if its `id:`
  matches a sibling file **it did not itself introduce in this invocation**
  (i.e. still report intra-invocation duplicates exactly as today via the
  existing `duplicate_id_errors`, and add the new cross-file-on-disk check
  as a second pass).

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | Directory-aware duplicate check finds a collision between a new file and a pre-existing sibling on disk | `tooling/rust/crates/agov-coord/src/coord_validator.rs` (`#[cfg(test)]`) |
| Unit | Directory-aware check does not false-positive on two files with distinct IDs | same |
| Integration (documented, manual) | Two git worktrees, each branched from the same base, each adds one new readable-ID-named coordination file; merge both branches into a throwaway integration branch; assert `git merge` produces zero conflicts and `agov coord validate` on the full directory reports 0 duplicate-ID errors | Documented as a reproducible shell transcript in the protocol doc per this issue's acceptance criteria |
| Boundary (documented, negative) | Two worktrees add distinct, schema-valid records with the same topic and incompatible governed meaning; demonstrate that Git/schema convergence succeeds while semantic consistency remains unresolved | Documented beside the integration scenario; no automated detector or blocking result in this plan |

TDD order: write the directory-collision unit test first (RED against
current `coord_validator.rs`, which has no such function), implement the
new check (GREEN), then wire into `main.rs`, then write docs.

## Risks

| Risk | Mitigation |
|---|---|
| Directory-aware scan is slow on very large channels | Coordination channels are bounded by project lifetime, not user data volume; `docs/coordination/` is already scanned in full by `coord status`, `coord stale`, `coord graph` with no reported performance issue. Not a concern at current or foreseeable scale. |
| Scope creep into promoting naming-drift warnings to blocking | Explicitly out of scope (see above); requires a separate rule change first per tool-authority-boundary-rule |
| Documentation drift between `coordination-id-rule.md` and the protocol concept doc | Cross-reference, not duplicate, per this plan's own Scope section |
| Mechanical convergence is overstated as semantic correctness | Document the five layers and the negative same-topic scenario; reserve any semantic detector for a separate evidenced plan |

## Rollback

Revert is trivial: the new directory-aware check is additive (a new
function plus one call site); reverting removes only the wider collision
detection, not any previously-passing validation. Doc changes revert
independently with no code dependency.

## Assumption flagged for the identity/accountability cluster

This plan does not change what identity a coordination message's `from:`
field carries or how it is established — it only concerns **filename and
`id:` collision detection**, which is orthogonal to sender identity. If the
identity/accountability cluster (issues #35/#45/#34/#47) introduces new
sender-identity or provenance fields, those fields do not change this
plan's collision-detection logic (which keys only on the `id:` field).
