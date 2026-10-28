---
kind: review
from: contribution-auditor
to: governor-reviewer,coder-tester
id: 20261023T193348Z-contribution-auditor-7njp
timestamp: 2026-10-23T19:33:48Z
immutable: true
references-to:
  - 20261023T155752Z-planner-architect-ehh4.md
  - 20261023T155713Z-governor-reviewer-ptds.md
topic: advisory-review-strict-mcp-args-ref-list-plan
requires-ack: false
artifacts_owned: []
---

# Advisory review: strict MCP arguments and ref-aware `coord list` plan

Requested by handoff `ehh4`. Reviewed plan `20261023T155559Z-agov-strict-args-ref-list`
at MR !3178 exact head `adda8780b159a5d882d01a66fdcb2efe60410e2d`. Performed by the AI
side of the contribution-auditor role (claude, planner-architect session), independent of the
plan's author (Codex). The trust-boundary part below is a read-through review; it is **not** the
CI `infosec-sentinel` validator, which I also ran (`agov validate infosec`: OK). Advisory
evidence only; it grants no approval and fills no advisory row by itself.

## Verdict: no blocking finding; two items for the plan to fix before Gate 1

Claims checked against `origin/main`:

- 21 tools are advertised; 4 `additionalProperties` occurrences exist in `agov-mcp/src/main.rs`
  (plan: 4 of 21). Matches.
- Empty `coord list --format json` prints the plain sentence at exit 2 today (reproduced on a
  temp directory). Matches the plan's zero-row defect.
- `hold_git.rs` already does `rev-parse`, `ls-tree`, `cat-file --batch`: batched Git reads exist to reuse.
- !3105 does touch `agov/src/main.rs`, `agov-mcp/src/main.rs` and `mcp-server.md`, as the plan says.
- The five dependency plan files exist; git 2.43 supports `--end-of-options`.
- Scope matches the governor's approved proposal (`ptds`: strict unknown keys; local read-only
  ref mode incl. MCP mapping and valid empty JSON). No scope absorption of #49, #133 or the
  dir contract; the out-of-scope list says so.

## Contribution audit

- **Finding 1 (WARN, plan gap): no classification or impact statement.** Both slices touch
  `tooling/rust/` (a `downstream-impact-rule` trigger path) and one narrows accepted MCP input,
  which is BREAKING under that rule's Parnas test (requests that "succeeded" now error). The plan
  states no mode (the MCP request boundary is a security boundary, so STRICT under
  `simplicity-rule`), no three-circle impact statement, no migration note for MCP clients that
  send stray keys, and no `Adopter docs parity` outcome. Add all four.
- **Finding 2 (WARN, scope boundary): high-risk area not declared.** "Reusing the existing
  visibility plumbing" points at `agov-coord` code (`hold_git.rs`, visibility). Coordination
  integrity mechanics and publication-hold code paths are on `high-risk-operations-rule`'s list
  (`tooling/rust/crates/agov-coord/**`). `affected_paths` names only `agov` and `agov-mcp`.
  State that `agov-coord` is called, not edited; if a helper must move or change, add that path
  and the high-risk handling before Gate 1 approves it.
- **Finding 3 (INFO):** two slices with different risk share one Gate 1. Acceptable because the
  plan makes each revertible; consider two implementation MRs.
- **Finding 4 (INFO):** `depends_on` lists `"!3105"`, an MR, not a plan id. Sequencing is
  stated in prose, so no harm, but it will not acknowledge a plan collision.
- **Finding 5 (INFO):** new ref-mode JSON is an object while non-ref JSON stays an array. State
  that MCP clients must branch on whether they passed `ref`.

## Trust-boundary review (read-through)

- **Finding 6 (WARN): leading-dash values through MCP.** The MCP layer builds a child `agov`
  argv. Passing `--ref` and the value as two arguments lets a value like `-x` be parsed as a
  flag by the CLI parser. Require the `--ref=<value>` form (or equivalent) in the MCP command
  vector and test it; the plan's own "begins with -" test should cover the MCP path, not only the CLI.
- **Finding 7 (WARN): echoed untrusted strings.** `source_ref` and the rejected-key name are
  printed in text output and diagnostics. Escape or reject control characters (ANSI sequences,
  newlines) and cap length; JSON output is safe by construction. Without this, a crafted ref or
  key can forge terminal output or log lines.
- **Finding 8 (INFO): bounds and file types.** "Bounded tree inventory" needs numeric limits
  (entry count, blob size). Read only regular blobs (modes 100644/100755), not symlinks or
  submodules, and reject `--dir` values with `..`, absolute paths, backslashes or NUL in ref mode.
- **Finding 9 (INFO): duplicate JSON keys.** A request with a repeated key is parsed by last-wins
  before the unknown-key check; decide and test that it is not an unchecked bypass for a known key.
- **Finding 10 (INFO): client-side rejection masks tailored errors.** Setting
  `additionalProperties: false` makes capable clients reject `include_forge`/`output` before
  transport, so users see a generic error instead of the tailored one. Note it in the docs.

Good: argv separation, `--end-of-options`, pinned OID, no fetch, no network, no shell, no new dependency.

Highest severity: WARN (findings 1, 2, 6, 7). None blocks; each can be resolved by editing the
plan text and test list. Author and reviewer are different assistants; the governor may pull or redo this.
