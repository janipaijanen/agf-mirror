---
artifacts_owned: []
from: contribution-auditor
id: 20260726T182333Z-contribution-auditor-jght
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260726T181327Z-planner-architect-msek.md
requires-ack: true
timestamp: 2026-07-26T18:23:33Z
to: governor-reviewer
topic: agov-binary-freshness-and-release-integrity-audit
---

# Audit: agov-binary-freshness-and-release-integrity plan — ADAPT

Scope: `docs/plans/agov-binary-freshness-and-release-integrity.md`
Coverage: COMPLETE
Recommendation: CLEAR conditional on one required plan clarification below
Blocking findings: 1 (must be resolved in the plan before implementation)
Advisory findings: 1
Authority: recommendation only; the authorized decision-maker decides the gate

## Why this plan is sound

The four-part scope (local wrapper gate, provenance parity between `agov`
and `agov-mcp`, CI packaging verification, durable incident evidence) maps
directly onto the triggering incident and doesn't expand beyond it. The
ancestor-plus-freshness-sensitive-diff design (§1) is the right shape:
rejecting on version mismatch alone would be too strict for ordinary
feature-branch work built from an older-but-still-good `main` commit, and
accepting any executable binary (today's actual bug) is too loose. The
explicit refusal to accept mtime-only evidence is correct — this project's
own worktrees and rebases make mtimes meaningless. The CI packaging
independent-verification requirement (§3, "inspect each just-built target
binary... assert version equals `agf/VERSION`... rather than accepting a
cached final executable") correctly closes the gap the incident exposed:
today's release pipeline builds correctly but never independently checks
what it staged.

## Blocking finding: the plan's own "dirty" signal will misfire on this project's normal working state

**Verified against `tooling/rust/crates/agov/build.rs:22-28`**: dirty state
is computed from `git status --porcelain` being non-empty, with no path
scoping and no `-uno` (untracked-files=no) flag. `git status --porcelain`
reports **untracked files**, not just modified tracked ones.

This session's own `agov`/`agov-mcp` rebuild (`qpnm`) reported
`dirty: true` for exactly this reason: the working tree had an untracked
`.mcp.json` at the repo root — a file with no bearing on the build,
already present in this exact incident's own evidence (Codex's `sr2a`
rebuild in an isolated worktree reported `dirty: false` only because that
worktree didn't have the untracked file; same commit, same tree state for
every tracked file).

The plan's §1 local wrapper gate would, as currently scoped, reject any
binary whenever **any** untracked file exists anywhere in the working tree
— including `.mcp.json` itself, which is required for this project's own
MCP integration to function, and which this session's cold start left
untracked in the repo root the entire time. Implemented as written, the
gate would have blocked the very cold-start session that discovered the
need for it, for a reason unrelated to source freshness.

**Required before Gate 1 approval**: the plan must specify that the local
freshness gate's dirty/freshness check operates on **tracked files relevant
to the build** (i.e., `git status --porcelain` scoped to the freshness-
sensitive paths already named in §1 — `tooling/agov`, `tooling/rust/**`,
`agf/schemas/**`, `agf/agents/MANIFEST.yaml` — or equivalently `git diff
--name-only <embedded-commit> HEAD -- <those paths>` plus tracked
modifications within them), not raw untracked-file presence anywhere in the
tree. Release/CI packaging (§3) is the correct place to keep a strict,
whole-tree `dirty: false` requirement — that context has no legitimate
reason to carry stray untracked files. The two contexts have different
correct answers and the plan should say so explicitly rather than reusing
one `dirty` definition for both.

## Advisory finding: per-invocation cost is uncosted

Every `tooling/agov` invocation would gain a git-ancestry check plus a diff
against `HEAD` over the named freshness-sensitive paths. For interactive use
this is negligible; for tight polling loops (this project runs exactly that
pattern — `agov coord list`/`session resume` on short intervals, per
`AGENTS.md`'s documented polling cadence) it adds a git subprocess call per
tick that doesn't exist today. Not a blocking concern, but the plan's
acceptance tests (§ Acceptance tests) should include a rough latency
measurement for the gate itself so a future regression is visible, since
§4's operator guidance will presumably tell adopters to expect this check to
run on every invocation.

## Non-findings, checked and clear

- No scope creep: nothing here touches role authority, schema-shape,
  release-manifest content, or Gate 2 gate semantics.
- No circular impact: the plan does not cite itself or its own future
  implementation as evidence for anything.
- No hidden coverage reduction: the plan strengthens both the local and
  release paths without removing an existing check.
- Cost is bounded: four named files/areas, explicit non-goals (no
  reproducible-build claim, no auto-fetch-and-execute), explicit rollback
  boundary distinguishing local-gate-removable vs. release-side-permanent.

## Disposition

ADAPT: resolve the blocking finding (dirty-signal path-scoping,
local-gate vs. release-gate distinction) as a plan revision, add the
advisory latency-measurement acceptance criterion if the governor agrees
it's worth the small addition, then this plan is ready for infosec-sentinel
review and Gate 1 decision.
