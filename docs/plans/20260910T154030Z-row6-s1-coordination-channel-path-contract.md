---
id: 20260910T154030Z-row6-s1-coordination-channel-path-contract
status: approved
gate1_approved_by: 20260910T190854Z-governor-reviewer-kr4b
gate1_approved_at: "2026-09-10T19:08:54Z"
topic: row6-s1-coordination-channel-path-contract
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Inherited marker from the original plan, not a fresh consultation
      attestation. Exact-revision independent review remains outstanding.
      The schema cannot express pending consultation (issue 130); do
      not use this marker as evidence that Gate 1 prerequisites are met.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Bounded review msq8 raised R56-E3 against the original plan.
      Codex now authors this correction under eetd/mb8k; that review
      does not independently clear this revision. Re-review is pending.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 6 S1 — channel membership and bounded misplaced-record discovery

## Provenance and scope

Parent: docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md, slice S1. This Codex author revision addresses msq8 R56-E3 after the Claude workload handoff. Gate 1 remains pending. It changes the reader contract and diagnostic coverage, not archive storage or record lifecycle.

The current visibility Git reader uses recursive paths while read_channel uses one directory. Fix membership consistently without claiming that excluding a file proves its obligation was resolved. No move, deletion, rename, archive migration or new archive format is authorized. Any future archive product remains a separate release proposal.

## Three separate outputs

1. **Channel records:** Files with lowercase `.md` extension directly beneath docs/coordination, with no filename-based README or non-record exclusion. Both readers use this membership rule. This says where a record belongs, not whether it is valid, live, acknowledged or approved. Existing parse/validation diagnostics remain visible.
2. **Misplaced candidate diagnostics:** inspect Git-tracked Markdown under the full docs/coordination subtree, plus direct docs siblings whose basename starts with coordination (for example docs/coordination-typo.md), and the docs/coordination-archive subtree. This finite path scope is explicit; no whole-repository or external filesystem search. A schema-valid coordination record outside the channel is an out-of-channel candidate, not automatically a live obligation. Any requires-ack/handoff fields are shown as evidence, not reinterpreted by a second resolver. A new or changed candidate must warn, including under the archive prefix. A malformed Markdown file with recognizable coordination front matter must produce an invalid-candidate diagnostic rather than silently disappear; ordinary prose is not automatically classified as coordination.
3. **Baseline archive inventory:** exact (path, Git blob ID) entries already present beneath docs/coordination-archive in the fetched target commit are summarized separately as existing out-of-channel archive records. Matching entries at the same path in a candidate ref do not become branch-local hidden obligations or new misplaced warnings. The summary must state that lifecycle/closure is not assessed. A path move, new file or changed bytes never qualifies through prefix alone. This baseline comparison is a noise-suppression boundary, not approval, closure proof or a general archiving mechanism. Existing potentially unresolved archived work remains a separate visible audit question, never reported as cleared.

Nested non-archive candidates remain misplaced even if already on target. The archive baseline exception is explicitly limited to unchanged inventory at the fetched target, not arbitrary nested paths. Without a resolvable target SHA or complete required blob reads, report incomplete coverage and do not claim zero misplaced/hidden records or apply baseline suppression.

## Implementation boundary

- Shared depth-1 membership predicate for visibility and read_channel, preserving current validation errors with no unsupported filename exclusion.
- Visibility obtains a bounded candidate path inventory from Git trees and compares exact target/ref blobs; expand the pathspec beyond docs/coordination so the sibling fixture can actually be discovered. Do not infer sibling coverage from a filtered recursive channel-only query.
- Read_channel remains the local channel reader; no remote/misplaced/archive scan is added to it. Explain that agreement covers channel membership, while visibility has additional diagnostic surfaces.
- Report ref/SHA and the scanned scopes, archive baseline count and incomplete/error states. Keep existing exit conventions; any added diagnostic exit behaviour must use the existing advisory warning convention, documented and tested, without granting new MR-blocking authority.
- Normative depth/membership contract in coordination-publication-rule.md; operator description in the coordination-protocol page and command reference; matching AGENTS.md and adopter-template pointers. Document unchanged archive inventory as unassessed, not cleared. The structured six-surface manifest is included in this Gate 1 candidate at `docs/reviews/rule-projection/20260910T154030Z-row6-s1-coordination-channel-path-contract.yaml`; its dispositions are planned obligations, not completed changes.
- CLI/MCP visibility results preserve equivalent candidate evidence and coverage; no extra database or relay authority.

## Required acceptance fixtures

1. Directly placed valid record: both readers agree on membership; normal validation and existing lifecycle behaviour are unchanged.
2. Valid handoff with requires-ack at docs/coordination/sub/record.md and docs/coordination-typo.md: each produces a misplaced-candidate diagnostic, with its exact location and acknowledgement requirement. No claim of whole-repository coverage.
3. Existing archive entry with the same target/ref path and blob: no branch-local hidden obligation or new misplaced warning, but counted in the unassessed baseline archive inventory. This replaces the original misleading requirement for an unqualified empty archive result.
4. New unresolved handoff or changed record under that archive prefix: must warn. Moving a known ID to a new path or altering bytes cannot inherit baseline exemption. A baseline archive record with requires-ack is never asserted resolved merely because it is baseline content.
5. Malformed coordination-like candidate: invalid-candidate evidence; ordinary sibling documentation does not become an obligation. Non-Markdown files are outside this scanner's declared coverage.
6. Missing base, unreadable blob, wrong repo or incomplete ref scan: explicit incomplete result, no clean-empty claim. Test both branch-added and target-present nested candidates.
7. README.md with ordinary prose is passed to normal validation and retains its invalid-record diagnostics; README.md with valid coordination front matter is included normally. Uppercase .MD and non-Markdown files retain the existing excluded membership. No silent prose/filename exemption is added to the direct channel.
8. CLI/MCP evidence parity; normal channel validation and prior lifecycle tests stay green. These fixtures fulfill the parent mandatory negative-test intent without authorizing a new blocking gate.

## Validation, rollout and rollback

Plan validation; independent contribution/InfoSec contract review; implementation unit/integration fixtures above and required exact-head CI. No mutation of existing coordination/archive records. Revert reader/help/rule changes together to roll back; no migration is required. Do not reduce required checks or reinterpret lifecycle outcomes to make the scan pass.

## Gate 1 decision after re-review

Approve the separate membership, misplaced-candidate and unassessed-baseline surfaces, the precise search scope and blob-equality exemption, the fail-explicit coverage behaviour, and projections/fixtures. This is a revised scope requiring independent consultation; Codex's prior finding is not self-clearance. Implementation waits for the governor's decision on this corrected contract.

## ssms correction round

IR-1: six-surface evidence is now part of this Gate 1 candidate, before
implementation. IR-3: removed the nonexistent filename exclusions and added
README prose/valid-record fixtures. Independent recheck remains pending.

## Gate 1 approval recorded

Governor decision `20260910T190854Z-governor-reviewer-kr4b` approves the corrected scope following independent recheck `6r12`, with its disclosed advisory limits. Earlier pending-review language above is historical; this approval authorizes implementation, while exact-head implementation review and Gate 2 remain required. Decision published in MR !2678.

## Implementation impact and migration assessment

The approved channel-membership correction changes previously recursive Git-reader behavior; this is BREAKING under downstream-impact-rule, even though the added diagnostics are advisory.

```text
Impact:
  Foundry: Yes — visibility now validates direct channel records and separates nested/sibling/archive evidence. Historical invalid records remain diagnostic and can produce advisory exit 1.
  Direct adopters: BREAKING — direct lowercase Markdown is the channel; nested records no longer enter its obligation list. Bounded candidate/archive fields and coverage are separately reported.
  Downstream: Coordination files are not mutated; output consumers must distinguish channel obligations, unassessed candidates and archive inventory. No gate authority is added.
  Migration: Read the coordination-protocol migration note; preserve existing records, route malformed or misplaced records for governed correction and update output consumers before relying on the revised scanner. No bulk archive move is authorized.
  Evidence: msq8 R56-E3 identified mismatched recursive/nonrecursive discovery and false archive closure; corrected contract approved by kr4b. Positive and negative path/blob/coverage fixtures and real CLI/MCP parity verify this bounded change.
```

Adopter docs parity: updated — protocol migration note, command scope reference and shipped template pointer explain the changed discovery contract.
