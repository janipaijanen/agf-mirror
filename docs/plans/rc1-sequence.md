---
id: rc1-sequence
status: complete
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Historical plan predating this project's per-plan advisory-consultation tracking convention; reconstructed retroactively per qbka Slice B, no fabricated consultation record created.
  - role: contribution-auditor
    status: not-applicable
    reason: Historical plan predating this project's per-plan advisory-consultation tracking convention; reconstructed retroactively per qbka Slice B, no fabricated consultation record created.
---

**Correction (2026-09-13, qbka Slice B):** this file had no YAML front
matter at all (schema-invalid). Describes the release sequence from MR
!153 to the AGF v2 RC milestone.

**Correction, round 2 (2026-09-13, independent review `20260913T184748Z-
coder-tester-t6hh`, finding QBKA-B-F1):** the round-1 correction above cited
`v2.0.0` as completion evidence, but that release (2026-06-16) predates this
plan's own commit `67d01c2f` (2026-06-23) and therefore cannot demonstrate
completion of this plan's later six-step sequence — a chronologically
impossible citation. The actual post-plan evidence, all dated after
`67d01c2f`: MR !153 merged at `08c9e9e7` (2026-06-23), readable coordination
IDs at `11462144` (2026-06-23), `agov` version provenance at `27080987`
(2026-06-23), the RC readiness audit at `d80163b5` (2026-06-23), and the
`v2.2.0-rc.1`/`v2.2.0-rc.2` tags at `bd810bdf`/`cd8f10b6` (2026-06-24) — this
plan's own described RC-1/RC-2 sequence, completed. `status: complete`
stands, on this corrected evidence. See `docs/plans/20260913T171837Z-qbka-
slice-b-legacy-plan-migration.md` and `t6hh`.

# RC-1 release sequence
# 2026-06-23T20:50:13Z

## Context

AGF v2 Tier 1 is closed. The repo records that AGF v2 "may now be evaluated
for Provisional → RC promotion" (00353-planner-architect.md line 61). Tier 2
gates remain for the RC milestone.

Codex's explicit RC bar (stated 2026-06-23):

> Tooling contract stable enough — Rust agov is the shipped path, no Elixir
> fallback leaking into user workflows. `tooling/agov` local/release behaviour
> is documented and true. MR !153 readable-ID helper extraction merged. MR 2
> version provenance + command contract docs merged.
>
> Release claims audited — inherited v1.x material inventoried/labelled before
> promotion beyond Provisional (docs/TODO.md line 50). Staged: high-risk public
> path first, not every historical corner.
>
> RC packaging/release surface not embarrassing — AGENTS says distribution
> archives are release snapshots and local builds are required for in-development
> commands. Shipped tooling artifacts are Rust, not Mix/Elixir.

Decision: do not cut RC after version provenance alone. One deliberate
"RC readiness audit" MR is required first.

---

## Sequence

### Step 1 — MR !153: readable ID utility extraction  ← IN PROGRESS
Branch: `fix/readable-id-utils`  
Owner: coder-tester (Codex), governor-reviewer review pending (`nmpn`)  
Gate: CI green (`rust:tooling:test`, `rust:tooling:clippy`, `rust:tooling:parity`)  
Merge then proceed.

### Step 2 — readable-coordination-ids
Work item: implement readable ID as the default coordination ID format  
Reference: handoff `01KVK7J9` (readable-coordination-ids-implementation-cleared),
Gate 1 approved via `kg2s`  
Owner: coder-tester (Codex) implements; governor-reviewer reviews  
Gate: Gate 2 approval before merge

### Step 3 — MR 2: version provenance + command contract docs
Scope (MUST per `aahs`):
- `agov --version` emits commit SHA + dirty state
- `agov version --format json` structured output
- Three-layer contract docs for `agov coord graph-index`:
  human ops page, AI cold-start guidance, output contract
  (exit codes, JSON field semantics, `resolved` values)

Owner: coder-tester (Codex) implements; governor-reviewer reviews  
Gate: Gate 2 approval before merge

### Step 4 — RC readiness audit MR
One deliberate checkpoint. Must demonstrate:
- Operator path end-to-end is clean and documented
- Version provenance emits correct output
- Release docs and AGENTS agree with actual tooling behaviour
- Inherited v1.x material in high-risk public paths is inventoried/labelled
  (docs/TODO.md line 50; staged audit — high-risk public paths first)
- Adopter-facing claims in README/docs are consistent with implementation

Owner: governor-reviewer drafts; any agent may contribute findings  
Gate: all findings resolved or explicitly deferred with rationale

### Step 5 — Tag and publish RC-1
- Resolve pre-release versioning scheme first (pre-bump vs post-bump on tag)
- Create `release/X.Y.Z-rc.1` branch, bump VERSION, open MR per
  release-via-merge-request-rule.md
- Tag merge commit; push tag; GitLab CI publishes release archives

### Step 6 — RC-2 (conditional)
Publish RC-2 when:
- Any post-RC items from the readiness audit are resolved
- Feature is confirmed feasible (readable-coordination-ids end-to-end)
- Codex has no remaining corrections

---

## Pre-release versioning scheme (unresolved)

Open question: does the main branch VERSION file pre-bump before tagging RC,
or does VERSION only bump on release MR? Decide before Step 5.

Options:
- **pre-bump**: main carries `X.Y.Z-dev` after each release; RC tag is
  `X.Y.Z-rc.N` with matching VERSION bump in the release MR.
- **post-bump only**: main carries the last released version; VERSION bumps
  only in the release branch MR.

---

## What is NOT in scope for RC-1

- Full `main.rs` modularisation (deferred, not blocking)
- Message loading helper extraction (opportunistic only)
- Historical plan file rewrites (editorial, deferred)
- Complete v1.x material audit (staged; high-risk paths only for RC-1)
