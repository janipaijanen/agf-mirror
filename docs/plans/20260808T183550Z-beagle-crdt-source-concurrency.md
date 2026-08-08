---
id: 20260808T183550Z-beagle-crdt-source-concurrency
status: approved
topic: beagle-crdt-source-concurrency
gate1_approved_by: "governor-reviewer direct chat approval transcribed in 20260808T183636Z-governor-reviewer-d6ps.md"
gate1_approved_at: "2026-08-08T18:36:36Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Codex independently read the supplied article, its linked Beagle and
      libdog primary repositories, Beagle's later backend status, the linked
      Chronofold paper, AGF issue #33, and the current pending concurrency
      plan. The durable assessment is
      `20260808T183635Z-contribution-auditor-gjsk.md`: add as attributed
      source/prior art, do not add a separate radar item, and sharpen issue
      #33's guarantee boundary without adopting CRDT storage or a new blocker.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Source-registry prose and a planning-only concurrency clarification.
      No credentials, learning-record content, network/runtime integration,
      authorization, schema, executable trust boundary, or blocking
      automation changes.
---

# Plan: classify Beagle/CRDT prior art and sharpen concurrency planning

## Decision and authorization

The governor approved the proposed classification directly in chat: add the
supplied article and primary supporting artifacts to `docs/sources.md`, do not
create a separate radar entry, and use the source to clarify the still-pending
issue #33 plan's distinction between mechanical convergence and governed
semantic correctness.

This plan authorizes only the bounded source-registry implementation below.
The edit to `docs/plans/coord-channel-concurrency-semantics.md` in this planning
change remains planning: it does not approve issue #33, authorize its Rust/docs
implementation, add a semantic-conflict detector, or select a release.

## Evidence classification

The article dated 2026-08-07 is a primary, author-produced design and
dogfooding report for Beagle. It is good evidence of the project's stated
architecture, trade-offs, and self-reported operation. It is not independent
evidence of correctness, scale, adoption, or effectiveness.

Registry evidence will pin:

- Beagle at `4b64468cd39de5c43c342012012f1b0f32853e0e`;
- libdog at `06a2a21de7f76046f921535f4411cbf701bfcd0d`;
- the requested article and Beagle Part III, which records the move away from
  the original RDX-backed CRDT store toward a Git-compatible backend; and
- Chronofold arXiv `2002.09511`, the linked primary research basis for the
  token-sequence CRDT discussion.

Claims about ticket count, instantaneous behavior, merge quality, or scaling
remain attributed to the author. Beagle's GitHub repository exposes no
recognized license at the pinned revision; libdog reports MIT. The source
entry must state that boundary rather than infer reuse permission.

## Implementation scope

### Source registry

Append Batch 20 to `docs/sources.md` with:

- the article classified as `technical design, tooling, source`;
- relationship `prior-art, complement`;
- action `cite, evaluate, watch`;
- the pinned Beagle/libdog revisions and linked Chronofold paper;
- an evidence boundary separating primary design/self-report from independent
  validation; and
- AGF takeaways centered on structured Markdown, derived query views, layered
  convergence, and semantic conflict rather than CRDT adoption.

### Pending issue #33 plan

Revise `docs/plans/coord-channel-concurrency-semantics.md` to:

1. distinguish file merge, ID/schema validity, causal/reference coherence,
   governed semantic consistency, and downstream program correctness;
2. state precisely which layers issue #33 covers;
3. add a documented negative scenario where two individually valid,
   distinct-file records about one topic merge and validate but imply
   incompatible governed state;
4. prevent successful Git/schema convergence from being described as semantic
   conflict resolution; and
5. defer any same-topic contradiction detector/index to a separate plan,
   triggered by an observed harmful ambiguity or a concrete adopter need.

The issue #33 plan stays `pending-gate1`; its empty approval metadata stays
empty. This approval does not extend to that plan's implementation.

## Explicit exclusions

- No `docs/radar.md` change: issue #33 already owns the mature question.
- No Git replacement, Beagle dependency, RDX/CRDT store, StrictMark adoption,
  distributed transport, or native database.
- No schema, coordination kind, Rust, CLI/MCP, CI, validator, or blocking-rule
  change in the source implementation.
- No claim that CRDT convergence guarantees syntax, governance meaning, build
  success, tests, or program correctness.
- No release/milestone assignment for issue #33.

## Validation

- `agov plan validate docs/plans/20260808T183550Z-beagle-crdt-source-concurrency.md`
- `agov coord validate` for the assessment and governor decision records
- `git diff --check`
- Manual URL and pinned-revision verification
- Manual diff check proving `docs/radar.md`, Rust, schemas, CI, rules, and
  product documentation are unchanged by the source implementation
- Manual check that issue #33 remains `pending-gate1` with no approval metadata

## Acceptance criteria

1. Batch 20 contains the article, pinned implementation sources, and
   Chronofold with clear evidence limits.
2. The registry explains that AGF already uses structured front matter over
   Markdown and Git, while Beagle is prior art rather than an adopted backend.
3. The registry preserves the article's useful warning: mechanically
   convergent merges may still be syntactically or semantically wrong.
4. No separate radar entry is added.
5. The pending issue #33 plan names all five convergence layers, scopes itself
   honestly, and includes the negative semantic-conflict scenario.
6. Issue #33 remains pending and no implementation authority is implied.
7. No executable, schema, rule-authority, version, release, or dependency
   surface changes.

## Rollback

Revert Batch 20 if factual review finds a source/provenance error. Revert the
issue #33 plan clarification independently if its framing is rejected during
advisory Gate 1 review. Neither rollback mutates historical coordination or
requires a data migration.
