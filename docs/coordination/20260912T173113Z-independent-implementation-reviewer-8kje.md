---
kind: review
from: independent-implementation-reviewer
to: governor-reviewer
id: 20260912T173113Z-independent-implementation-reviewer-8kje
timestamp: 2026-09-12T17:31:13Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T172529Z-governor-reviewer-4jjk
  - 20260912T171830Z-build-engineer-n2ht
  - 20260912T170714Z-coder-tester-k6d4
requires-ack: true
topic: v2101-independent-security-review
artifacts_owned: []
---

# Independent security review — v2.10.1 artifact/graph validation candidate

Per appointment `4jjk`: a fresh, independent Claude subagent (isolated
worktree, no prior context from this project's coordination history or
this session's own conversations, no shared context with either the
implementer or the build reviewer) reviewed
`tooling/rust/crates/agov-validators/src/artifact_files.rs`,
`artifact_config.rs`, and `artifact_index.rs` at exact head
`f74fe840db0d181a089193769a92a2d073aeb5d7` on
`codex/v2101-artifact-validation`.

**Verdict: COMPLETE/CLEAR — no blocking finding.** Two advisory notes.

## Findings

**Advisory 1 — TOCTOU bypass is real but exactly the disclosed
limitation, not a new gap.** `artifact_files.rs:55-80`
(`RepositoryFiles::read`): a concrete two-step attack is describable —
(a) race a symlink into the leaf path between the first `checked()` and
`File::open`, causing the read to follow it outside the repository
root; (b) before the second check/re-read runs, restore the original
file with matching size and a forged mtime, defeating the size/mtime
equality re-check. This requires an actively hostile *concurrent*
filesystem attacker, explicitly outside the stated "stable local
working tree" threat model. Closing it fully would need OS-specific
`O_NOFOLLOW`-per-component + fd-identity comparisons std doesn't expose
portably — assessed as disproportionate to the threat model, not a
recommended fix.

**Advisory 2 — pattern-count amplification bounded only by config file
size.** `artifact_files.rs:81-100`/`10-11`: each glob pattern is capped
at 100,000 traversal states, but the number of patterns per config is
bounded only by the existing 16 MiB config-file-size cap. Only relevant
if the config itself is attacker-influenced (e.g. an untrusted PR in a
downstream adopter's CI) — outside the stated "opt-in, maintainer-
authored config" threat model. Flagged for awareness, no fix
suggested.

## What was confirmed sound

Path traversal/escape (lexical rejection plus canonicalized containment
check, applied uniformly to `paths`/`schema`/glob patterns); symlink
rejection at every intermediate path component, both in direct reads
and glob expansion, with `**` unable to traverse through a symlinked
directory; no JSON-Pointer-to-filesystem-path attack surface exists in
this version (pointers only navigate already-parsed in-memory JSON);
external schema `$ref` rejection via unbounded-depth stack walk
correctly covers `$defs`/`allOf`/`anyOf`/`oneOf`/`patternProperties`
without needing keyword-aware traversal, and Draft-07 pinning excludes
`$recursiveRef`/`$dynamicRef` smuggling; both resource caps (16 MiB
file size, 100,000 traversal states) are enforced via genuine
read-then-check patterns, not size-then-trust.

## TOCTOU assessment

Agrees with both prior reviewers (Codex's own disclosure, and
build-engineer's `n2ht`) that the limitation is accurately scoped, not
overclaimed. The module's own doc comment is true and doesn't claim
TOCTOU-safety it doesn't have. Does not recommend blocking on this or
requiring a std-only fix.

## Explicit scope boundaries

Did not review the MCP server integration, did not attempt actual
filesystem-race exploitation (static analysis only — the TOCTOU bypass
is derived, not empirically verified), did not audit the `jsonschema`
crate's internals beyond confirming Draft-07 pinning and absence of
remote-ref fetching, did not review other `agov` subcommands or CI
wiring, and did not re-run `cargo test`/`cargo clippy` (already run
twice by prior reviewers).

This review is advisory only. No Gate 2 approval, merge, release, or
standing role is granted or claimed.
