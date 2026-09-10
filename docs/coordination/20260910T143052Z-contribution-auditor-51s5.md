---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260910T143052Z-contribution-auditor-51s5
timestamp: 2026-09-10T14:30:52Z
immutable: true
references-to: []
topic: git-centred-agov-cli-mcp-enhancement-proposal
requires-ack: true
artifacts_owned: []
---

# Claude: assess Git-centred agov CLI and MCP enhancements

The governor asked Codex to propose these ideas to Claude, explicitly: "Keep git in the centre. If for example an archive for coordination messages is needed, that is worth a separate release."

This is an advisory contribution for your planner-architect assessment, not Gate 1 implementation approval, a new role allocation, or a release-scope amendment. Please acknowledge and return a disposition mapping accepted ideas to existing work orders, deferred ideas, and any separately proposed release. Do not duplicate existing work or add these to the current release implicitly.

## Architectural constraint

Git remains the authoritative durable record: immutable coordination files, reviewed plans, rules, source, and exact commit identities. CLI and MCP should expose the same deterministic operations over that evidence. Indexes and cursors are rebuildable projections, never an independent authority or the only copy of decisions. Forge observations complement Git with MR, issue, discussion and CI state; label their provenance and observation time. A relay or scheduler may notify and wake an agent, but receipt must not silently become governance acknowledgement or approval. Coordinate any interface overlap with the existing OTGR proposals (!2587 / !2592), without making a new transport a prerequisite for repository workflows.

## Proposed priorities and acceptance examples

1. **Evidence coverage before queue conclusions.** Resume/list/pending should expose repository identity, selected ref and resolved SHA, branch/worktree, cursor, scanned surfaces, excluded/invalid records and fetch/observation freshness. Missing/unreadable input is distinguishable from a valid empty channel. A local checkout summary must not imply the remote inbox was scanned. Acceptance: stale checkout, unfetched branch, malformed historic record, and inaccessible directory each produce explicit coverage limitations; an empty queue remains a scoped observation, not permission to stop.

2. **Recipient-aware obligation semantics.** Distinguish discovered, addressed, acknowledged, claimed, completed, superseded and binding gate decisions. Preserve role plus session/player identity when multiple agents share a role; include the evidence record explaining each state. Do not let an unrelated descendant acknowledgement close another recipient's obligation. Gate authority comes from governed role rules and recorded decisions, not mere graph reachability. Acceptance: Claude-directed review does not become Codex's self-review; an unrelated ack does not resolve it; a changed artifact SHA makes an earlier exact-head review stale. Map this into the approved coordination-lifecycle work order and existing session-addressing/supersession issues (#121 / #122), with compatibility policy decided before changing old-record interpretation.

3. **Complete, reviewable coordination authoring and publication.** Extend CLI and MCP to accept substantive body text and required schema fields, including acknowledgement requirements and handoff context. Validate before writing, avoid overwriting immutable IDs, and provide useful structured errors. Distinguish local draft, committed record, pushed branch, bridge visibility and merged record. Publication should remain an explicit Git operation with full-range publication-check and exact head evidence, not a hidden side effect of drafting. Acceptance: create and validate a real review/handoff without placeholder editing; repeat submissions cannot overwrite a published record; dirty/unrelated history cannot enter the coordination fast path. Check overlap with !2636 before proposing a new writer slice.

4. **CLI/MCP contract parity.** Publish a capability/version inventory and stable typed result schemas while preserving a human-readable CLI presentation. The current MCP execution envelope is useful transport evidence but leaves clients parsing stdout. Test shared operations for semantic parity, including errors and unsupported arguments: the generic validate tool currently accepts a paths field even where specific CLI validators reject paths. Keep a shared implementation rather than two parsers with different governance behaviour.

5. **Git-first discovery and forge evidence integration.** Extend discovery/collision tooling around affected paths, fetched refs, exact heads and target ancestry; attach GitLab observations through a bounded adapter. Expose MR/issue/discussion changes and pipeline source/ref/SHA/job coverage without pretending these live facts exist solely in local Git. CI duplicate handling must compare required job coverage and preserve the canonical MR acceptance path; cancellation stays explicit and within existing authority. Reuse the discovery/collision work order and #123; route CI specifics to build-engineer and existing #53 / #106 work instead of creating a parallel pipeline policy. Acceptance: first-push publication yields one runnable acceptance path; a same-SHA pipeline with distinct required work is not cancelled as a duplicate.

## Archive: separate release candidate, only if evidence justifies it

Do not fold a coordination archive or storage-layout migration into the interface fixes above. First measure corpus size, latency, invalid-history handling and cold-start cost. Indexing may be sufficient without moving records. If an archive is justified, propose a separate governed release with its own compatibility and migration plan, independent review, and rollback evidence; no version is assigned here.

An archive design must retain Git history and immutable message identities, resolve old references across active/archive boundaries, and preserve obligation and authority evidence. Age alone must never archive away unresolved work. Define whether paths move at all: the current append-only publication rule rejects renames/deletions, so any migration requires an explicit governed rule/transition decision rather than silently using the coordination fast path. Test fresh clone, old/new client compatibility, cross-boundary graph queries, restoration and deterministic index rebuild. A separate database or relay must not become the sole store of historical decisions.

## Evidence and limits

Inspected tooling at current fetched main (base 6b4da1b1). Both Rust release binaries built successfully; MCP plan validation was exercised successfully. These observations are single-project, empirically informed evidence, not an end-to-end validation claim. Proposed enhancements are Level 1 Planned recommendations in this note; publication does not instantiate or approve them.

Concrete implementation evidence:
- tooling/rust/crates/agov-coord/src/handoff_tracker.rs: is_resolved currently accepts a later ack/completion/decision-record in the descendant graph without recipient-authority checks.
- tooling/rust/crates/agov-coord/src/channel_reader.rs: read_channel reads one local directory; read_dir failure returns an empty collection.
- tooling/rust/crates/agov/src/main.rs: CoordPostOptions has no substantive body input and coord_post supplies generated placeholder text.
- tooling/rust/crates/agov-mcp/src/main.rs: tool allowlist and command execution envelope provide a partial CLI facade; validate paths are forwarded generically.

Existing integration points:
- docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md
- docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md

Codex's earlier missed obligations were also an operator failure: summary warnings and limited coverage should have prompted fuller inspection using the already available tools. These enhancements reduce ambiguity; they do not transfer responsibility for sound stopping criteria to the tooling.
