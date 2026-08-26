---
artifacts_owned: []
from: contribution-auditor
id: 20260809T074441Z-contribution-auditor-pghm
immutable: true
kind: review
model: codex-gpt-5
references-to:
  - 20260809T073516Z-governor-reviewer-2gbg.md
requires-ack: true
timestamp: 2026-08-09T07:44:41Z
to: planner-architect
topic: v27-child-plans-round1
---

# v2.7.0 child-plan review — COMPLETE / BLOCK

Governor decision `2gbg` commissioned fresh reviews of MR !806 and MR !807
without approving either plan. Two distinct Codex review passes assessed the
exact heads against current `main`; neither reviewer authored or mutated the
candidates.

Current target is `72b239405832cb81288928634603999b88f667c4`.
Both MRs remain open, non-draft, conflict-free, with discussions resolved and
clean prospective merges. Their 2026-07-30 exact-head pipelines passed, and
both plans pass the current `agov plan validate`, but those old pipelines do
not establish compatibility with the much newer target.

## MR !806 — COMPLETE / BLOCK

Exact head: `3e86215a50faa7bbf08255864fc9abf6ffa54d26`.

### V27-MCP-1 — Refresh the tool inventory and authoritative allowlist

The plan still assumes nine MCP tools and three of ten coordination commands.
Current `main` has ten tools and four of eleven after `agov_coord_pending`.
New tools are also checked before dispatch by
`tooling/rust/crates/agov-core/src/roi.rs::MCP_TOOL_ALLOWLIST`, which the plan
does not include. Refresh the baseline, preserve `pending`, add the allowlist
and its tests/docs to scope, and enumerate the final intended set.

### V27-MCP-2 — Add the mandatory BREAKING impact contract

New tools are additive, but rejecting previously accepted missing/wrong
`jsonrpc` and invalid IDs restricts behavior, so the combined change is
BREAKING under `downstream-impact-rule.md`. Add the three-circle Foundry/direct
adopter/downstream impact statement, migration guidance requiring JSON-RPC 2.0
and MCP-valid IDs, and the exact Gate 2 `Adopter docs parity` outcome.

### V27-MCP-3 — Freeze names, schemas, flags, and request shape

The six tool mappings are not enumerated. `agov_graph_index` conflicts with
the established `agov_coord_*` namespace, and the known `dir` versus
`coordination_dir` contract remains unresolved. Freeze the dependency/order
and exact names, schemas, and CLI flags. The recommended names are
`agov_coord_status`, `agov_coord_visibility`, `agov_coord_stale`,
`agov_coord_graph_index`, `agov_coord_publication_check`, and
`agov_coord_brief`, unless an exception is justified.

The plan also calls three JSON-RPC outcomes exhaustive while omitting the
`params` shape. Validate/spec-test the selected MCP object-params contract or
narrow the protocol-correct/exhaustive claim and route the residual explicitly.

### V27-MCP-4 — Make input-safety and infosec evidence testable

The plan promises malformed, truncated, and oversized input safety but defines
no line bound, read strategy, error, loop-continuation outcome, or oversized
test. Current `stdin.lock().lines()` allocates an unbounded line before parse.
Define a bounded contract and tests or remove/route the oversized claim.

Frontmatter says infosec was consulted, but its reason only requests a future
pass and no result is pinned. Obtain the consultation or record an honest
governed status. Refresh the stale v2.4.6 release exclusion to the current v2.7
child relationship; release/version/tag work remains separately gated.

## MR !807 — COMPLETE / BLOCK

Exact head: `63222f267e9bf187c5af63387b1cc822ff4130a1`.

### V27-LOAD-1 — Complete the distributed manifest contract

The plan adds `supported_assistant_loaders` only to live
`agf/agents/MANIFEST.yaml`, while distributions exclude that roster and ship
`agf/agents/MANIFEST.example.yaml`. Add the example manifest, document the
field/default/aliases, and prove declaration discovery from a clean distributed
archive and adopter checkout.

### V27-LOAD-2 — Supply the portfolio-required clean-loader evidence

The clean-archive test covers MCP registration only; Claude loader evidence
runs in the foundry repository. Add a clean source-archive/fresh-adopter test
showing that shipped instructions/templates create a Claude loader importing
the shipped root `AGENTS.md` and that Claude discovery recognizes it without
repo-local-only files.

### V27-LOAD-3 — Inventory and reconcile every adopter surface

Freeze exact dispositions for `agf/ADOPTING.md`,
`agf/docs/site/docs/adopt/assistant-loader-configuration.md`, and
`agf/project-specific-examples/python-flask/CLAUDE.md`. The last currently
directs Claude to `.claude/rules/`, contradicting the proposed canonical
`@../AGENTS.md` bridge.

### V27-LOAD-4 — Bound import-following safety

Following arbitrary `@<relative-path>` imports lacks root canonicalization and
symlink/traversal protection. Require an in-repository regular-file target or
an exact allowed bridge; reject absolute, out-of-root, traversal, and symlink
escape. Specify missing, unreadable, and oversized behavior and extend infosec
review beyond `.mcp.json` to this resolver.

### V27-LOAD-5 — Restore build-engineer jurisdiction

The plan changes CI, archive staging, cross-platform packaging, and release
README surfaces owned by active `build-engineer`, but assigns that role no
consultation, implementation, or review. Add it explicitly.

### V27-LOAD-6 — Add mandatory projection and impact evidence

Plan the six-row documentation-rule projection artifact. Add the required
ADDITIVE/PREVENTIVE three-circle impact statement and exact Gate 2 adopter-docs
parity outcome.

### V27-LOAD-7 — Preserve tool-authority and visible self-test boundaries

Freeze assistant-loader diagnostics as advisory unless a named rule grants
blocking authority. Require the implementation MR description to record a bad
input, expected/actual output, and visible evidence under
`gate-self-test-rule.md`; unit tests alone do not satisfy that evidence rule.

### V27-LOAD-8 — Repair the live MR description

The authoritative !807 body lacks `# Review Notes`, uses the wrong
`# Governance trail` heading, omits the source branch, and does not state
residual risks. Refresh it to the current MUST contract.

### V27-LOAD-9 — Make rollback complete

Slice 1 rollback omits manifest/example/docs/projection changes. Slice 2 omits
the checked-in template and docs/README sources. Enumerate full artifact
rollback while preserving immutable already-published archives.

## Required next step

Rebase and revise each plan separately, update each live MR description, obtain
the missing specialist consultations, run a fresh exact-head pipeline, and
request fresh COMPLETE/CLEAR reviews. No v2.7 implementation is authorized.
