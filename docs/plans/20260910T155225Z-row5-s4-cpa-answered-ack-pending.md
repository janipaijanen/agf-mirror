---
id: 20260910T155225Z-row5-s4-cpa-answered-ack-pending
status: approved
gate1_approved_by: 20260910T190854Z-governor-reviewer-kr4b
gate1_approved_at: "2026-09-10T19:08:54Z"
topic: row5-s4-cpa-answered-ack-pending
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Inherited schema marker from the original Claude plan, not a new
      attestation. No exact-revision independent InfoSec clearance is
      established here. Renewed consultation is outstanding; this plan
      must not advance to Gate 1 on the strength of this marker. The
      schema cannot represent pending consultation (issue 130).
  - role: contribution-auditor
    status: consulted
    reason: >-
      Actual bounded consultation: msq8 identified material contract
      findings in the original revision. Codex now authors corrections
      under eetd/mb8k and cannot independently clear them. A fresh
      independent review of this revision remains outstanding.

---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 5 S4 — show reply evidence without changing lifecycle resolution

## Provenance and corrected baseline

Parent: docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md, S4. Codex author revision after eetd/mb8k, addressing msq8 R56-E1. This supersedes the previous plan body's inaccurate assertion that only ack resolves a handoff. The actual `handoff_tracker::is_resolved` resolves a later ack, completion OR decision-record in the transitive descendant graph. This slice preserves that baseline; its limitations are evidence for separate lifecycle work, not silently fixed here.

## Bounded change

For handoffs that the existing resolver leaves unresolved, show an optional evidence annotation `reply present — acknowledgement unconfirmed` when a strictly later review or note directly references the handoff through structured references-to and is authored by its next_agent role or governor-reviewer. A note can be a progress report, question or refusal: do not label it answered, complete, authorized, approved or probably handled. Include the matching record ID and author role so a reader can inspect it. This predicate identifies a reply candidate only; role text is not authenticated identity or gate authority.

Do not change ready/unresolved status, result membership, ordering, scheduling priority, exit codes, gate authority or automatic ack behaviour. Absent, malformed, unauthorized-role, indirect-only and prose-only references do not create the annotation. Ignore timestamp ties rather than infer later activity. Cleared items remain cleared under the current resolver, including completion and decision-record descendants. Do not duplicate or reinterpret that resolver in display code.

The handoff-next and session-resume handoff surfaces share the annotation. `coord pending` currently excludes handoffs: preserve that separation rather than adding a new handoff queue there. MCP tools must expose equivalent evidence and retain existing execution/status behaviour. Adding a machine-readable optional evidence field is preferable to requiring clients to parse the label; final field naming must be specified in implementation documentation.

## Required fixtures

1. No reply: same unresolved membership, order, status and exit code as baseline; no annotation.
2. Direct later recipient-role note and review: annotation plus exact evidence ID, unchanged unresolved status. A progress note and clarification question get only the neutral annotation, never completion/answer inference.
3. Other role, same-time record, indirect-only reference and prose-only reference: no annotation.
4. Current completion and decision-record descendant resolution remains identical. A subsequent ack clears the item exactly as before. Existing historical graph fixtures retain their results, including known weak resolution semantics which this slice does not claim to repair.
5. Mixed ready and annotated records preserve previous order and process exit code. No polling priority can be lowered based on reply metadata.
6. CLI/MCP parity for annotation evidence and baseline status; coord pending continues to exclude handoffs.

## Scope and projection

No schema/rule authority change, auto-ack, reprioritization or migration. A later proposal to change resolver semantics must separately define recipient authority, direct/transitive evidence, compatibility, and governor approval. Provide accurate operator help explaining the neutral annotation; include an adopter-facing command reference note because the behaviour is not self-explanatory from its label alone. Complete the six-surface projection screen before implementation submission, identifying the existing coordination rule as authority and the annotation as advisory evidence.

## Validation and rollback

Unit/integration fixtures above; plan schema validation; exact-head independent contribution and InfoSec review as applicable to the approved scope; required CI. Revert annotation code/help to restore prior display without changing any record or lifecycle state. This document remains pending Gate 1. The inherited consultation fields record earlier planning consultation, not a fresh clearance; renewed consultation on this revised contract is outstanding. Codex authors cannot self-clear it.

## Gate 1 decision requested after independent review

Approve neutral reply evidence for already-unresolved handoffs with unchanged membership, lifecycle, ordering and exit codes. This deliberately narrows the original proposal: no semantic "answered" claim and no lower exit weight. No implementation begins before the governor approves this corrected scope.

## Gate 1 approval recorded

Governor decision `20260910T190854Z-governor-reviewer-kr4b` approves the corrected scope following independent recheck `6r12`, with its disclosed advisory limits. Earlier pending-review language above is historical; this approval authorizes implementation, while exact-head implementation review and Gate 2 remain required. Decision published in MR !2678.

## Implementation impact assessment

This records the downstream impact of the approved scope without expanding it.

```text
Impact:
  Foundry: No lifecycle, membership, order or exit-code change; unresolved handoff displays gain neutral reply evidence.
  Direct adopters: ADDITIVE — optional reply evidence in existing handoff/session displays; no new required input.
  Downstream: Coordination record shape and agent authority unchanged; consumers may observe an optional evidence line.
  Migration: None required; omitted fields/flags preserve existing behavior.
  Evidence: Approved S4 contract and independent review S4-B1; predicate and real CLI/MCP regression fixtures.
```

Adopter docs parity: updated — coordination-protocol guidance and the applicable command reference/template projections explain the advisory behavior.
