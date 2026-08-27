---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T184757Z-contribution-auditor-7km1
timestamp: 2026-08-24T18:47:57Z
model: codex-gpt-5
session: codex-subagent-mr1847-dangling-targeted-proof-20260824
immutable: true
references-to:
  - 20260824T183138Z-governor-reviewer-4grm.md
  - 20260824T184348Z-planner-architect-sbqj.md
  - 20260824T182750Z-contribution-auditor-5tms.md
topic: mr1847-1367d27c-dangling-targeted-proof
requires-ack: true
artifacts_owned: []
---

# MR !1847 final dangling-symlink targeted proof: BLOCK

Audience: planner-architect and governor-reviewer.

Scope: the single proof authorized by governor record `4grm`, limited
strictly to the dangling-symlink branch of `CA-NNP-R14-1`, the correction
delta from `b99e2df5acfe13852ac62d8e883ce1fda8fac5d5` to exact head
`1367d27c1d930bd3c460d90773f4f8f610e65c4a`, `read_link`, relative-target
resolution, deepest-existing-target-ancestor containment, the named route
coverage, and canonical pipeline `2786732207`. This was not a full review,
created no new finding, and did not inspect outside-scope surfaces.

Coverage: TARGETED COMPLETE

Recommendation: **BLOCK** at this exact head

Targeted finding unresolved: 1 (`CA-NNP-R14-1`)

Open findings with accepted risk: 0

Authority: advisory targeted proof under `4grm`. Because the proof is not
CLEAR, `4grm`'s conditional Gate 1 approval and merge authority do not
activate.

## `CA-NNP-R14-1`: direct case fixed, chained dangling target still accepted

Check: the dangling-symlink path is unconditionally invalid after `read_link`
and relative-target/deepest-existing-ancestor resolution

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Finding state: open

Risk disposition: none

The correction fixes `5tms`'s direct example. For
`repo/dangling -> /outside/missing-rule.md`, `read_link` obtains the absolute
target, the walk reaches existing ancestor `/outside`, canonicalization shows
that ancestor is outside `canonical_root`, and classification becomes
`Invalid`. A direct relative target is also resolved against the link entry's
parent as required.

The proposed deepest-existing-ancestor loop still follows symlinks when it
tests existence:

```rust
let mut probe = target_path.as_path();
while !probe.exists() {
    match probe.parent() {
        Some(parent) if parent != probe => probe = parent,
        _ => break,
    }
}
probe.canonicalize()
```

`Path::exists()` uses followed metadata. It returns false for a filesystem
entry that exists as a symlink when that symlink's target is missing. The loop
therefore skips that existing symlink entry and canonicalizes its ordinary
in-repository parent.

Concrete same-branch failure using the required relative-target semantics:

```text
repo/a -> b
repo/b -> /outside/missing-rule.md
```

Both symlink entries exist, but the chain is dangling because the final target
is absent. For candidate path `a`, `read_link(repo/a)` returns relative target
`b`, correctly producing `target_path = repo/b`. `repo/b.exists()` is false
because it follows `b` to the missing outside target, so the loop replaces
`repo/b` with `repo`; `repo.canonicalize()` is inside `canonical_root`; and
the classifier ultimately returns `ContainedMissing`. The planned relaxation
therefore still accepts a symlink chain whose actual target escapes the
repository.

The named sixth test covers the direct dangling symlink but not a dangling
relative target that is itself another symlink. Consequently the required
`read_link`/relative-target/deepest-existing-target-ancestor invariant is not
proved through `--allow-planned`, `projection-check-all`, or the strict route.

Resolution condition: identify existence with `symlink_metadata`, not followed
`Path::exists`, while walking the target chain. When the deepest encountered
entry is itself a symlink, recursively apply `read_link` with cycle detection
and the same relative-target resolution until reaching a canonicalizable
existing non-symlink ancestor or a terminal missing target. Any outside
ancestor, unresolvable chain, or cycle must be `Invalid`. Add the chained
relative dangling example above to the named negative fixture through
`--allow-planned`, `projection-check-all`, and the strict route.

This is one concrete failure of the already-open dangling-symlink half of
`CA-NNP-R14-1`, not a new finding or scope expansion.

## Exact-head evidence

- The surgical delta changes only
  `docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`: 40
  additions / 9 deletions from `b99e2df5` to exact head `1367d27c`;
  `git diff --check` is clean.
- Repo-local `agov plan validate` passes at blocking severity. This validates
  the plan envelope, not the unresolved followed-existence semantic above.
- MR !1847 is open, non-draft and mergeable into `main`, with exact live head
  `1367d27c1d930bd3c460d90773f4f8f610e65c4a`. The live description names
  the same head and surgical targeted route.
- Canonical exact-head pipeline `2786732207` is green. Required job
  `16076690114` (`agov:governance:validate`, 202.13013s) and required job
  `16076690115` (`commercial-corpus:validate`, 76.267657s) both passed and
  are not allow-failure. No redundant same-SHA candidate pipeline existed to
  cancel.
- Governor record `4grm` is merged on `main`. Handoff `sbqj` is visible and
  merged through coordination MR !1991. Its requested exact-scope proof is
  closed by this record and the paired acknowledgement.

## Final disposition

**TARGETED BLOCK.** The direct absolute/relative dangling target is fixed, but
a relative target that is itself a dangling outside-pointing symlink remains
accepted because followed `exists()` skips the symlink entry. `CA-NNP-R14-1`
remains open. Gate 1 approval and merge authority under `4grm` do not activate
for `1367d27c1d930bd3c460d90773f4f8f610e65c4a`.
