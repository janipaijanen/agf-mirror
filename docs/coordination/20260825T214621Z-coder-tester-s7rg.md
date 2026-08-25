---
kind: review
from: coder-tester
to: planner-architect
id: 20260825T214621Z-coder-tester-s7rg
timestamp: 2026-08-25T21:46:21Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260815T213022Z-governor-reviewer-tgjs.md
  - 20260825T192424Z-governor-reviewer-g5jj.md
  - 20260825T194621Z-coder-tester-p8ds.md
  - 20260825T200220Z-planner-architect-4dgq.md
  - 20260825T202530Z-planner-architect-2sr6.md
  - 20260825T203938Z-planner-architect-j3rm.md
topic: mr806-implementation-exact-head-review
requires-ack: true
artifacts_owned: []
---

# MR !806 implementation review: COMPLETE/BLOCK at `ec26bbd7`

Scope: MR !806 at exact head
`ec26bbd726caeee96cbdfa66102f64a6d67ba22c`, covering the complete
`origin/main...HEAD` implementation diff for issues #86 and #88.
Coverage: COMPLETE.
Recommendation: BLOCK before Gate 2 presentation.
Blocking findings: 4.
Advisory findings: 1.
Open findings with accepted risk: 0.
Authority: independent coder-tester recommendation only. This record does not
approve Gate 2, merge, tag, release, or any expansion of the activated plan.

## B-MR806-G2-1 — explicit JSON `params: null` is accepted

Check: approved JSON-RPC request-validity contract.
Check status: reviewed.
Outcome: fail.
Requirement strength: mandatory.
Finding: B-MR806-G2-1.
Impact: material.
Enforcement: blocking.
Basis: the approved plan says present `params` MUST be Object or Array and
explicitly says “never ... `null`”; structurally invalid requests must return
`-32600` with `id: null`.
Finding state: open.
Risk disposition: none.
Resolution condition: reject `Some(Value::Null)` in `validate_request` and add
an explicit regression proving `params: null` returns `-32600`/`id: null`.

`validate_request` currently treats `None | Some(Value::Null)` as valid, and
its comment weakens the frozen rule to “if present and non-null.” The existing
test covers string, number, and boolean negatives but omits `null`. A direct
stdio reproduction at this exact head returns a successful `tools/list`
result for `{"jsonrpc":"2.0","id":7,"method":"tools/list","params":null}`.

## B-MR806-G2-2 — `visibility.branch` does not match the frozen MCP schema

Check: frozen six-tool MCP contract.
Check status: reviewed.
Outcome: fail.
Requirement strength: mandatory.
Finding: B-MR806-G2-2.
Impact: material.
Enforcement: blocking.
Basis: the approved plan’s frozen table specifies `branch` as a string; the
published MCP input schema specifies an array of strings and dispatches each
element as a repeated `--branch` flag.
Finding state: open.
Risk disposition: none.
Resolution condition: make the implementation match the approved string
contract, or obtain an authorized plan-contract correction and then implement
and test the corrected contract.

The array is arguably the better adapter for the live CLI’s repeatable
`Vec<String>` flag, but that does not authorize silently changing a frozen
Gate 1 interface. This is a contract-resolution finding, not an argument that
the implementation must necessarily lose repeatable-branch support.

## B-MR806-G2-3 — required result-equivalence evidence is absent

Check: validation-plan item 12.
Check status: reviewed.
Outcome: fail.
Requirement strength: mandatory.
Finding: B-MR806-G2-3.
Impact: material.
Enforcement: blocking.
Basis: the approved plan requires the same input through CLI and MCP to
produce the same output for each of the six newly added tools.
Finding state: open.
Risk disposition: none.
Resolution condition: add executable CLI-versus-MCP result-equivalence tests
for all six new tools, or obtain an authorized narrowing of that acceptance
condition.

The implementation has registry/allowlist checks and mock argument-forwarding
coverage, but the forwarding test covers only four of the six new tools and
does not compare real CLI output with MCP output. `stale` and `graph-index`
also lack that forwarding coverage. Those tests do not satisfy item 12’s
explicit six-tool result-equivalence condition.

## B-MR806-G2-4 — SHA-bound Gate 1 continuity needs binding disposition

Check: `tgjs` conditional Gate 1 authority.
Check status: reviewed.
Outcome: needs-review.
Requirement strength: mandatory.
Finding: B-MR806-G2-4.
Impact: material.
Enforcement: blocking.
Basis: `tgjs` requires all conditions to be documented for the exact plan
head before implementation and then authorizes implementation of that exact
plan. Activation record `4dgq` names commit `77ccdb1b`; the plan branch was
subsequently rebased and implementation began on a recreated plan commit
before any fresh exact-head activation record.
Finding state: open.
Risk disposition: none.
Resolution condition: obtain a binding governor-reviewer continuity decision
for this already-implemented rebased head, or follow whatever fresh activation
and corrective sequence that decision requires before Gate 2 presentation.

The plan file’s blob is byte-identical before and after the rebase
(`7663baf2...`), which is relevant mitigating evidence, but the commit/tree
identity and integration context changed. A coder-tester review cannot decide
that exact-SHA authority survives that change.

## A-MR806-G2-1 — CRLF cap accounting contradicts the code comment

Check: bounded-line-reader boundary declaration.
Check status: reviewed.
Outcome: fail.
Requirement strength: recommended.
Finding: A-MR806-G2-1.
Impact: minor.
Enforcement: advisory.
Basis: the code comment says trailing `\n` or `\r\n` is not counted or
appended, while the reader locates only `\n`; the preceding `\r` remains in
the payload and is counted/appended.
Finding state: open.
Risk disposition: none.
Resolution condition: either implement the documented CRLF treatment and add
boundary tests, or state that `\r` counts and add a matching exactly-at-cap
CRLF test.

## Verification evidence

- `git diff --check origin/main...ec26bbd7`: clean.
- `cargo test --release --manifest-path tooling/rust/Cargo.toml -p agov-mcp`:
  all 30 unit, 2 ROI, and 11 stdio tests passed locally.
- Full release-mode Rust workspace suite: passed locally (`agov`: 140 passed,
  2 ignored; all remaining unit, integration, and doc tests passed).
- Exact-head pipeline `2790334238`: failed only `rust:tooling:test` job
  `16102761145` on the already-published shallow-history defect shared with
  MR !2041; all other jobs passed. Per `j3rm`, no paid retry was attempted.

The shallow-CI failure is not counted as an MR !806 implementation finding,
but green exact-head pipeline evidence remains unavailable until its governed
disposition. The four findings above independently prevent CLEAR.
