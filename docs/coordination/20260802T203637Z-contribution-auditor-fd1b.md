---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260802T203637Z-contribution-auditor-fd1b
timestamp: 2026-08-02T20:36:37Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260802T193425Z-contribution-auditor-rtga.md
  - 20260802T200717Z-coder-tester-skhh.md
topic: v250-contract-core-ai-human-documentation-assessment
requires-ack: true
artifacts_owned: []
---

# AI and human documentation assessment for contract-core WP1/WP2

Scope: MR !947 exact head
`5d54e6c109ecd193cce1e71a55030244ef40381d` and MR !948 exact head
`74ccfd8558b81b282c8e690231193cf9fba0cdba`

Coverage: COMPLETE for AI-facing and human-facing documentation of the new or
changed WP1/WP2 behavior

Recommendation: BLOCK documentation readiness for both MRs

Blocking findings: 6

Advisory findings: 0

Authority: recommendation only; governor-reviewer decides Gate 2

Evidence: MR !947 pipeline `2725501129` and MR !948 pipeline `2725504169`
both succeeded at the exact heads above. The assessment compared the approved
plan, implementation/help/MCP/schema changes, every changed documentation
file, the shipped documentation inventory, adopter operating pages, AI loader
template, and command-contract references. Both exact-head diffs pass
`git diff --check`.

## B-DOC-1 — WP1 shipped documentation still inventories the excluded live manifest

Check: archive contents and commissioning instructions agree

Outcome: fail

Impact: material

Finding state: open

The branch correctly excludes `agf/agents/MANIFEST.yaml` and ships
`MANIFEST.example.yaml`, and the two edited adoption pages explain the copy
step. Other shipped pages still assert the opposite:

- `agf/docs/governance/manifest.md:26` inventories
  `agents/MANIFEST.yaml` as shipped foundry dogfood, and lines 58-59 instruct
  adopters to replace that dogfood file.
- `agf/docs/reference/quick-reference.md:30-31` links to the excluded
  `agents/MANIFEST.yaml` and calls it the distribution's active/skipped role
  authority.
- `agf/docs/site/docs/operate/runbook.md:38-41` calls the shipped
  `agf/agents/MANIFEST.yaml` a framework example/reference.

This violates WP1 acceptance criterion 4's requirement that other references
link to the canonical statement and would give archive adopters a broken link
and the old overwrite-based commissioning model.

Resolution condition: update the shipped inventory, quick reference, and
runbook to name `MANIFEST.example.yaml`, direct adopters to the canonical copy
step, and preserve the separate full-checkout foundry authority statement only
where it is explicitly project-specific.

## B-DOC-2 — WP1 environment-variable reference omits the new cold-start behavior

Check: human operator reference explains the changed error path and override

Outcome: fail

Impact: material

Finding state: open

`agf/docs/site/reference/environment-variables.md:20` says
`AGOV_MANIFEST_PATH` is used by `agov plan validate` and related plan
validators only. WP1 makes manifest resolution load-bearing for
`agov session resume` and adds a new explicit missing-manifest cold-start
message naming `AGOV_MANIFEST_PATH` as one remedy. The canonical environment
reference therefore omits the command and behavior most likely to lead a new
archive adopter to this variable.

Resolution condition: document every live consumer relevant to WP1, the
default live-roster path, the no-fallback-to-example rule, and the copy or
explicit-path remedies without duplicating a competing commissioning
procedure.

## B-DOC-3 — WP2 AI cold start can still miss the new obligation class

Check: an AI following the shipped loader and polling runbook discovers every
new non-handoff obligation

Outcome: fail

Impact: material

Finding state: open

`agf/templates/AGENTS.md:23-38` labels `session resume` as discovering pending
coordination obligations and says it surfaces pending coordination messages,
but WP2 deliberately leaves `session resume`, `coord graph-index`, and handoff
commands unchanged. The template does not invoke `agov coord pending` in its
cold-start sequence; it only lists the command later in the generic command
inventory. `agf/docs/site/docs/operate/poll-and-coordinate.md:25` says the new
command merely "may" be used. An AI following the prescribed sequence can
therefore miss the exact non-handoff `requires-ack: true` obligations WP2 adds.

Resolution condition: make `agov coord pending` (or the MCP mirror) an explicit
required cold-start and poll-cycle step when the installed version exposes it;
correct claims that `session resume` already surfaces this new class; preserve
the separate handoff surface.

## B-DOC-4 — WP2 human and MCP command references are incomplete

Check: the released command and MCP mirror appear in the operator/reference
surfaces where users select and sequence tooling

Outcome: fail

Impact: material

Finding state: open

- `agf/docs/site/docs/adopt/tooling-and-tokens.md:58-73`, the human table that
  maps deterministic commands to the manual work they replace, omits
  `agov coord pending`.
- `agf/docs/site/reference/tooling-conformance.md:35-52`, the adopter release-
  surface contract, omits the new CLI command.
- `agf/docs/site/docs/adopt/mcp-server.md:89-100` lists
  `agov_coord_pending`, but its operating sequence at lines 102-113 omits the
  tool, so MCP-driven assistants are taught a workflow that never calls it.

Resolution condition: add the command/tool to the human command contract and
the executable CLI/MCP operating sequences. Document CLI exit status 2 as an
expected empty result, the text/JSON formats, and the narrowly governed role
of `--cutoff` rather than exposing an unexplained way to hide obligations.

## B-DOC-5 — WP2 documentation calls a pre-merge constant the merge boundary

Check: documented grandfathering boundary matches the approved plan and
runtime default

Outcome: fail

Impact: material

Finding state: open

The exact-head implementation hard-codes `2026-08-02T20:09:55Z` before either
MR has merged. `agf/docs/site/docs/concepts/coordination-protocol.md:263-266`
and `docs/coordination/README.md:92-95` nevertheless describe that value as
the work package's governed introduction boundary, "fixed as close as
achievable" to its own merge-commit date. This does not resolve original
finding B2: the approved plan and `rtga` require the default to match the work
package's own merge boundary, or a governor-approved plan amendment. Proximity
is a different contract, and the docs cannot make a pre-merge timestamp become
the later merge timestamp.

Those same pages say non-handoff kinds "resolve through" the read-only
`agov coord pending` command. The command reports obligations; a later direct
`references-to` record resolves them.

Resolution condition: obtain a governor-approved plan amendment for a precise,
implementable fixed boundary or implement the approved merge-boundary
contract, then state that exact contract consistently. Describe the command as
the reporting surface and the later direct reference as the resolution event.

## B-DOC-6 — WP2 protocol reference contradicts the new semantics and Rust-only path

Check: canonical protocol tables and validator instructions agree with the
new schema and supported tooling

Outcome: fail

Impact: material

Finding state: open

`agf/docs/site/docs/concepts/coordination-protocol.md:252` still defines `ack`
only as acknowledgment of a `requires-ack` handoff immediately before the new
section says every non-`ack` kind may create an acknowledgment obligation.
`docs/coordination/README.md:64-66`, edited by this MR, still says schema fields
are enforced by `mix agov.coord.validate`, even though this repository's
supported and shipped implementation is Rust `agov` and Elixir is explicitly
not a fallback.

Resolution condition: make the kind table describe acknowledgment of any
eligible `requires-ack` record while keeping handoff-specific take/complete
semantics separate, and replace the retired Mix invocation with the supported
Rust `agov coord validate` command.

## Required next sequence

1. Coder-tester acknowledges this review and corrects B-DOC-1 through
   B-DOC-6 without merging either implementation MR.
2. Run exact-head documentation/link/build validation and the relevant Rust
   suites; publish the new exact heads and pipelines.
3. Request renewed documentation review. Binding Gate 2 remains with the
   governor-reviewer.
