---
kind: handoff
from: capability-developer
to: planner-architect
id: 20260909T040330Z-capability-developer-5jde
timestamp: 2026-09-09T04:03:30Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260909T013812Z-capability-developer-jprj.md
  - 20260909T014426Z-capability-developer-drtp.md
topic: coordination-protocol-propeller-assessment
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Independently challenge this Propeller-derived coordination-protocol assessment and its evidence strength.
    - Deduplicate and sequence the proposed adjustments against issues #121, #122, #123, MR !2027, and the v2.11 coordination-semantics workstream.
    - Return either a bounded Gate 1 work-order proposal or a material challenge-response explaining what should be narrowed, deferred, or rejected.
  must_read:
    - docs/coordination/20260909T013812Z-capability-developer-jprj.md
    - MR !2585 record 20260909T031021Z-improvement-propeller-3jh6.md
    - GitLab issues #121, #122, and #123
    - GitLab MR !2027
    - agf/docs/site/docs/concepts/coordination-protocol.md
  do_not_touch:
    - coordination schema, rules, tooling, or agent definitions before approved Gate 1
    - issue or roadmap priority without governor disposition
    - binding governor decisions
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Assess and turn the Propeller coordination findings into a governed work order

## Assessment to challenge

The file-based, append-only coordination protocol is **Level 5 Deployable for
its stated core envelope**: one repository, asynchronous work, durable audit
history, modest concurrency, and one accountable human governor. The evidence
for broader applicability is only **Empirically informed**, not universal. The
P-1 attended run inspected 82 new coordination records, 78 open MRs, and 64
issues; it validates the method and reproduces defects in this foundry project,
but it does not establish feasibility in every project or topology.

The synthesized diagnosis is: **too much transport ceremony and too little
machine-readable lifecycle semantics**. Preserve append-only history,
exact-head reviews, human gate authority, conservative obligation visibility,
and mechanically verified publication. Reduce repetitive branch/MR/ack work
by making lifecycle state explicit and batching only where integrity remains
provable.

## Existing work to retain and deduplicate

1. **#121 — session-level addressing.** Concurrent sessions sharing one role
   cannot tell which session owns a handoff. Consider optional session identity,
   active-role registration, and claimed-item visibility without making model
   provenance an authority key.
2. **#122 — self-announcing supersession.** Append-only corrections are sound,
   but an earlier record looks live unless the reader finds its successor.
   Surface `supersedes` relationships in `list`, `pending`, and `status`; require
   a durable coordination record or bridge for binding off-channel directions.
3. **#123 — plan/workstream collision detection.** Normalize affected paths
   and provide advisory overlap/sequencing evidence before Gate 1; do not let a
   tool assign authority or resolve collisions.
4. **MR !2027 — priority and material challenge-response.** Add bounded
   priority semantics with tightly controlled `immediate` authority, and reuse
   existing message kinds for contested findings rather than inventing a new
   adjudicator or message vocabulary.

## Adjustments not yet cleanly owned

### CP-A — substantive response versus acknowledgement state

An authorized `review`, `completion`, or `note` may substantively answer a
`requires-ack` handoff while `agov handoff next` still presents the original as
ready. This creates duplicate work or ack-only coordination MRs. Assess either:

- an `answered — acknowledgement pending` state; or
- narrowly allowing an authorized, directly referencing response to satisfy
  acknowledgement while preserving the distinct take/complete lifecycle.

Prefer inclusion in the existing coordination-semantics work order if its
schema and tooling footprint stays bounded.

### CP-B — per-record publication and release-base churn

The strongest observed ceremony cost is one branch, MR, pipeline, merge, and
protected-branch movement per append-only record. During v2.10, coordination
merges repeatedly invalidated the frozen release base. Assess a separate small
Gate 1 plan for:

- multiple independently valid append-only records in one coordination MR;
- a release-lane coordination hold that batches **non-urgent** records until
  the target hold clears; and
- an urgent bypass preserving independent publication when another actor must
  see a live obligation immediately.

Any batching mechanism must retain full-range publication checking, individual
record validation, causal references, immutable authorship, and no hidden
implementation commits. It must not allow release work to suppress urgent
security or gate evidence.

### CP-C — branch-local and archive visibility

`agov handoff next` reads the base branch while `coord visibility` separately
finds branch-local obligations. B4 also demonstrated that an inert archive
under `docs/coordination/` can produce 39 false hidden-obligation findings.
Fold branch-aware collision/obligation surfacing into #123 if practical. Define
archive paths as inert only with a negative fixture proving a real live
top-level obligation still fires.

## Explicit operating-envelope boundaries

The durable repository channel should not claim to be the transport for every
case:

- real-time incident response may use a faster operational channel first, with
  a sanitized durable bridge once safe;
- security, privacy, legal, labor, and protected discussion content stays in
  its appropriate restricted process, with only the minimum safe decision or
  obligation recorded here;
- raw chat/discussion retrieval belongs to OTGR or another evidence transport,
  not this coordination schema;
- multi-repository or multi-governor federation remains unvalidated; and
- disposable, exploratory, or human-only projects may rationally reject the
  ceremony entirely.

## Requested disposition

Return a planner assessment that explicitly answers:

1. whether the existing #121/#122/#123/!2027 cluster plus CP-A is one coherent
   Gate 1 work order or should be split;
2. whether CP-B deserves its own plan and which invariants make batching safe;
3. whether CP-C belongs in #123, B4 corrective work, or a separate issue;
4. which claims above are validated, merely informed, or still theoretical;
5. the smallest release sequence that improves the protocol without adding
   another layer of ceremony.

This is an advisory capability-developer handoff. It creates no implementation,
priority, role, gate, release, or schema authority.
