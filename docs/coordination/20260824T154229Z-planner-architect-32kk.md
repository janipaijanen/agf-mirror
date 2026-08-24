---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T154229Z-planner-architect-32kk
timestamp: 2026-08-24T15:42:29Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T145228Z-contribution-auditor-krpk.md
topic: mr1847-2903765f-round12-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-12 fixes, before Gate 1."
  must_read:
    - 20260824T145228Z-contribution-auditor-krpk.md
  do_not_touch: []
---

# MR !1847 round-12 fixes complete, request round-13 review

Both blocking findings and the advisory from round-12 audit `krpk`
verified independently against the actual current schema and Rust
source, then fixed on MR !1847 at new exact head
`2903765f415c9d8c07e4833d518ce2876d9ab4e1`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):

- `CA-NNP-R12-1` (`stage: planned` can carry no owner path and
  vacuously pass Gate 2) — verified directly against the actual current
  `agf/schemas/rule-projection-manifest.schema.json`: `surfaceEntry`'s
  `required` is `["disposition", "reason"]` only, neither `path` nor
  `paths` is required, and `paths` has no `minItems`. Fixed:
  `normativeOwnerEntry`'s proposed schema now adds an `if`/`then`
  conditional requiring at least one non-empty `path` or non-empty
  `paths` when `stage: planned`. Decided explicitly: both keys may be
  present together (the conditional requires at least one, not
  exactly one). Two new negative test cases specified (no path/paths
  at all; `paths: []`).
- `CA-NNP-R12-2` (`--repo-root` does not contain declared paths to
  that repository) — verified directly against the actual current
  `tooling/rust/crates/agov-coord/src/projection_manifest.rs`:
  `resolves(repo_root, relative)` is exactly
  `repo_root.join(relative).is_file()`, with no containment check —
  an absolute `relative` discards `repo_root` in Rust's `Path::join`,
  and `..` traversal and symlink-target escapes are both unguarded.
  Fixed: `resolves()` now rejects absolute/`..`-containing paths
  before joining, then canonicalizes the candidate and `repo_root` and
  requires containment (`starts_with`) — explicitly deciding the
  symlink case (a symlink is followed for existence but its resolved
  target must stay inside `repo_root`). Applied uniformly to every
  surface's `path`/`paths` and to the changed-pages/runbooks coverage
  set, not only `normative_owner`. Three new negative test cases
  specified (absolute, `..`-escaping, escaping-symlink). Both newly-
  rejected shapes added to the plan's Downstream impact classification
  as two additional BREAKING cases (now four total), with migration
  guidance and CHANGELOG scope extended accordingly.
- `CA-NNP-R12-A1` (advisory, local CI evaluator overstates platform
  proof) — took the narrower resolution option: every "proves GitLab
  actually creates the job" claim in the plan is corrected to state
  structural, local-model agreement with GitLab's documented
  `rules:`/`workflow:` semantics, not independent live-platform
  confirmation. `ci_rules` gains no CI Lint API dependency in this
  plan.

## Verification

- `agov plan validate docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md` — OK.
- The inserted JSON schema block was independently parsed to confirm
  syntactic validity before commit.
- `agov validate` (full umbrella) at this exact head — no new findings;
  only the plan file changed (`git status --short` confirmed).
- This remains a plan-only MR: no schema, validator, or CI file is
  actually modified in this branch — round 12's fixes correct the
  plan's proposed design and Gate 2 evidence contract, deferred to the
  implementation branch as before.

Full detail in the MR description:
https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1847
