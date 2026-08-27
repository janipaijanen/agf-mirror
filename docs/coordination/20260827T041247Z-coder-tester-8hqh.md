---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260827T041247Z-coder-tester-8hqh
timestamp: 2026-08-27T04:12:47Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260827T020147Z-coder-tester-2qnp
  - 20260824T185045Z-governor-reviewer-a86h
  - 20260824T203540Z-planner-architect-gej4
topic: issue115-projection-path-lifecycle-gate2
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 2 for MR !2125 at exact head 0b2f2f0ecc1f8486b413797e4041cf3117b54fea.
  must_read:
    - docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md
    - docs/coordination/20260827T020147Z-coder-tester-2qnp.md
  do_not_touch:
    - .claude/settings.local.json
---

# Gate 2 handoff: issue #115 projection-path lifecycle

Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Accepted risks: 0
Authority: recommendation only; governor-reviewer decides Gate 2 and merge.

## Exact candidate

- MR: !2125
- Exact head: `0b2f2f0ecc1f8486b413797e4041cf3117b54fea`
- Target: `main` at implementation base
  `a0be8ef9471b78acd399bf31666fd52d9b64f67e`
- Exact-head merge-request pipeline `2795129059`: success, 20 mandatory
  jobs successful; optional Windows preview jobs `16137112841` and
  `16137112842` remain manual.
- Key successful jobs: governance `16137112853`, Rust workspace test
  `16137112857`, clippy `16137112859`, deny `16137112855`, agov validation
  `16137112854`, and distribution validation `16137112844`.
- A duplicate same-SHA push pipeline `2795126333` was canceled after the MR
  pipeline was confirmed as the canonical strict superset. Its 15 jobs used
  102.426270 runner-seconds before cancellation; this is recorded on MR !2125.

## Implemented contract

The implementation follows the approved issue #115 plan:

- adds the `existing` / `planned` lifecycle to normative-owner manifest
  entries while requiring a non-empty path member for planned entries;
- classifies owner paths as contained-existing, contained-missing, or invalid,
  with absolute paths, traversal, repository escape, and escaping or dangling
  symlinks failing closed regardless of relaxation;
- permits `stage: planned` only for contained missing normative-owner paths
  under the explicit `--allow-planned` path, while strict single-manifest
  validation remains the Gate 2 evidence path;
- rejects stale planned markers once any declared path exists;
- adds the full-tree `projection-check-all` command, changed-manifest coverage,
  explicit repository/schema isolation, and a fail-closed local evaluator for
  the live GitLab CI rules subset;
- makes the governance validation job unconditional for merge-request, branch,
  and tag pipelines; and
- documents four breaking migration cases in `CHANGELOG.md` and mirrors the
  adopter-facing workflow rule in `.claude/rules/`.

## Verification

- `cargo test --manifest-path tooling/rust/Cargo.toml --workspace`: pass.
- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov`: pass,
  including the real-binary `projection_check_all` integration suite.
- `cargo clippy --manifest-path tooling/rust/Cargo.toml -p agov -p
  agov-coord -p agov-validators --all-targets -- -D warnings`: pass.
- `agov validate`: exit 0 with no blocking result.
- GitLab CI lint: valid.
- Schema JSON parse, Rust formatting, canonical/mirror equality, current real
  manifest corpus, and `git diff --check`: pass.
- Integration evidence covers relaxed-versus-strict behavior, stale unchanged
  manifests on owner-only diffs, positive and negative diff coverage,
  conflicting caller schema isolation, repository containment, and the current
  corpus.

## Necessary scope correction and learning

The approved estimate omitted
`tooling/rust/crates/agov-core/src/roi.rs`. Full-workspace validation exposed
the miss because its live-Clap parity/redaction audit exhaustively inventories
every command token and long flag. The correction was necessary for the new
`projection-check-all` command and `--allow-planned` flag to pass the mandatory
audit, and it was limited to those two inventory entries.

The impact-assessment learning is concrete: plans that add or rename a CLI
surface must search exhaustive cross-crate consumers—including live-Clap
parity, redaction/telemetry, MCP, help, completion, packaging, and similar
inventories—rather than estimating from parser and dispatch ownership alone.
This repeats and directly corroborates the already-published lesson behind MR
!2045; no second competing learning mechanism is proposed here.

Adopter documentation parity: complete. The canonical distributed rule,
assistant mirror, schema, command behavior, CI enforcement, and breaking
migrations are updated together.

Capability maturity: Level 4 candidate at the exact branch head. It is not
Level 5 until a supported release artifact includes the change and the release
is separately authorized and completed.

No merge, Gate 2 decision, release, tag, deployment, or paid-job retry was
performed by this handoff.
