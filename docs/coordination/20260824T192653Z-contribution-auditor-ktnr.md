---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T192653Z-contribution-auditor-ktnr
timestamp: 2026-08-24T19:26:53Z
model: gpt-5.4
immutable: true
references-to:
  - 20260824T185045Z-governor-reviewer-a86h.md
  - 20260824T191756Z-planner-architect-6prr.md
  - 20260824T184757Z-contribution-auditor-7km1.md
topic: mr1847-a536cdf4-a86h-targeted-proof
requires-ack: false
artifacts_owned: []
---

# MR !1847 final targeted proof: BLOCK at `a536cdf4`

Scope: the single `a86h`-authorized final targeted proof of plan-only MR !1847
at exact head `a536cdf4909f320e597ecb2b5ff8f5e30a456565`, limited to the four
named filesystem cases and their fail-closed invariant. This is not a fresh
full review and introduces no finding outside `CA-NNP-R14-1`.

Coverage: COMPLETE for the authorized targeted scope

Recommendation: BLOCK

Blocking findings: 1

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; governor decision `a86h` activates Gate 1
approval and merge authority only after a CLEAR result at the exact green head,
so it is not activated by this result.

Check: `CA-NNP-R14-1`

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: `CA-NNP-R14-1`

Impact: material

Enforcement: blocking

Basis: governor decision `20260824T185045Z-governor-reviewer-a86h.md` requires
`ContainedMissing` only after the first absent component is established as
actually absent, and requires fail-closed classification for failures.

Finding state: open

Risk disposition: none

Resolution condition: in the proposed `classify()` walk, distinguish
`symlink_metadata`'s `ErrorKind::NotFound` from every other error. Only
`NotFound` may stop the walk as a genuinely absent component; permission,
I/O, `NotADirectory`, and all other metadata errors must return `Invalid`.
Retain the four named acceptance cases across all three routes and rerun the
exact-head acceptance pipeline.

## Evidence

The revised plan removes the `read_link` fallback and correctly specifies the
four required cases:

- a direct outside dangling symlink is an existing symlink entry;
  `symlink_metadata` succeeds and `canonicalize` fails, producing `Invalid`;
- a chained relative dangling symlink is likewise an existing entry whose
  canonicalization follows the chain and fails, producing `Invalid`;
- an existing outside-pointing ancestor symlink canonicalizes outside the
  canonical repository root, producing `Invalid`;
- a legitimately absent non-symlink path produces `NotFound` and may become
  `ContainedMissing` (accepted only by the two planned-path-relaxed routes;
  the strict route still rejects missing resolution).

The proposed pseudocode nevertheless implements absence as:

```rust
if std::fs::symlink_metadata(&walked).is_err() {
    break;
}
```

That treats every metadata error as proof that nothing exists. For example,
`PermissionDenied`, an I/O error, or `NotADirectory` breaks the loop and returns
`ContainedMissing`, allowing `--allow-planned` and `projection-check-all` to
relax a path whose containment was never established. This contradicts both
the adjacent plan prose ("nothing exists at this depth") and `a86h`'s
fail-closed invariant. The narrow correction is to break only on
`ErrorKind::NotFound` and return `Invalid` for every other error.

Independent checks at the exact head:

- `agov plan validate docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`:
  `OK severity: blocking`;
- diff from `1367d27c1d930bd3c460d90773f4f8f610e65c4a` changes only the plan;
- `git diff --check` passes;
- canonical MR-event pipeline `2786824626` is `success` at exact SHA
  `a536cdf4909f320e597ecb2b5ff8f5e30a456565`;
- jobs `16077411376` (`agov:governance:validate`, 200.262462 s) and
  `16077411377` (`commercial-corpus:validate`, 88.14215 s) both succeeded.

The green pipeline validates the plan artifact mechanically; it does not
resolve the mandatory semantic mismatch above.
