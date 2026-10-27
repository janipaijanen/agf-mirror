---
id: 20261027T230355Z-v212-release-freeze-corrections
status: approved
gate1_approved_by: 20261027T231633Z-governor-reviewer-ma21
gate1_approved_at: "2026-10-27T23:16:33Z"
topic: v212-release-content-freeze-corrections
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Non-author plan-stage review is CLEAR after its sole supply-chain finding
      was corrected: official anonymous asset discovery, fresh temporary
      directory, checksum and provenance verification before extraction or
      execution, bounded read-only commands, evidence redaction, and cleanup
      are now explicit. A fresh exact-head implementation review remains
      required before Gate 2.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Non-author plan-stage review is CLEAR on corrected content SHA-256
      e59024751c044208d0d97df23e533454ca29e620b33b4f41677d45caa06c00b4.
      A renewed review is CLEAR on sequencing-corrected content SHA-256
      d5d9e8511d7ec9ac5004b8dc7195a1008ff41ad8c324b8eca7912d82bd275b05,
      after CI exposed six predecessor overlaps; the blocking check now reports
      six acknowledged and zero unacknowledged. This later metadata-only head
      records those reviews without changing scope, authority, evidence
      requirements, or acceptance.
affected_paths:
  - docs/plans/20261027T230355Z-v212-release-freeze-corrections.md
  - agf/CHANGELOG.md
  - agf/docs/site/docs/adopt/mcp-server.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/rules/workflow/coordination-publication-rule.md
  - docs/reviews/documentation-estate/20261027-v212-release-content-freeze.md
  - docs/reviews/rule-projection/20261027T230355Z-v212-release-freeze-corrections.yaml
depends_on:
  - 20261026T151646Z-next-release-scope-intake
  - 20260909T203636Z-v211-release-publication
  - 20260921T040743Z-assistant-rule-drift-resolution
  - 20261021T220000Z-retire-windows-support
  - 20261023T155559Z-agov-strict-args-ref-list
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20261025T192020Z-supersedes-adopter-docs
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: close the v2.12 release-content freeze

## Intent and authority

Resolve the bounded findings from the v2.12 documentation-estate
release-content-freeze sweep before deriving the release-publication metadata
allowlist. Frozen-intake decision
`20261027T215726Z-governor-reviewer-a2nh` selected v2.12.0, retained MR !3098,
and admitted exactly two ordered documentation slices. Both slices are now
merged with exact-head Gate 2 evidence.

This plan reconciles that already-frozen content. It adds no third `IN` item,
changes no coordination behavior, and authorizes no version bump, release
branch, protected-tag mutation, tag, package, release, or deployment. A
separate release-publication child plan remains mandatory after these
corrections merge.

## Preconditions already discharged

- Tooling coexistence implementation MR !3311 merged under decision `gnqg`.
- Homepage modernization MR !3314 merged under decision `j2p6`, after
  independent web-UX and documentation-estate CLEAR reviews.
- The stranded Gate 2 record for MR !3256 was published through MR !3257.
- Bridge record `3mts`, cited by five binding governor records, was published
  through MR !3284. These two audit-chain repairs contain no product change.
- Frozen `CANDIDATE` rows remain unpromoted. MR !3066 and issues #105, #111,
  #113, #119, #128, #133, #136, #137 and #138 remain open; MR !3097 remains
  evidence only; issue #129 remains withdrawn; issue #134 remains blocked.

## Exact implementation scope

### Changelog reconciliation

Add three entries under `agf/CHANGELOG.md` `[Unreleased]`:

1. **Added:** the evidence-calibrated third-party tooling coexistence guide and
   discovery surfaces, naming the README pointer, site navigation, authority
   and data-flow checklist, and bounded Ponytail/Caveman evidence.
2. **Changed:** the homepage's four canonical routes and responsive,
   light/dark, keyboard-focus-aware presentation, including the calibrated
   coexistence route.
3. **Fixed:** v2.11 coordination feature shipment prose, after verification
   against the published v2.11 artifact.

Preserve the existing MR !3098 skill-command correction. Do not add a root
`CHANGELOG.md` content row: the root changelog delegates adopter-facing rows to
`agf/CHANGELOG.md`, and this slice adds no repository-operation change.

### Shipment-status corrections

Correct only these stale statements:

- `mcp-server.md`: publication-hold fields and tooling shipped in v2.11.0;
  compatible framework and tooling artifacts remain required before first use.
- `coordination-protocol.md`: urgency ordering and challenge-response shipped
  in v2.11.0 and are Level 5 Deployable; publication-hold support shipped in
  v2.11.0, while mixed-version readers remain unsafe.
- `coordination-publication-rule.md`: v2.11.0 was the first release carrying
  the fields; operators still verify actual artifacts and minimum supported
  versions before use.

This is a factual capability-status correction. The rule's obligations,
authority, schemas, fields, error handling, and rollout safety do not change.
Do not rewrite surrounding normative text.

### Freeze evidence and projection

Add one release-content-freeze report that records:

- the exact pre-implementation `main` base, the v2.11.0 tag object and peeled
  commit, and the reviewed product heads and merge commits for both frozen
  items; the external exact-head fact-check and Gate 2 coordination record bind
  the correction MR's final SHA, avoiding an impossible self-reference inside
  the commit being identified;
- both ordered `IN` items and retained MR !3098 changelog dispositions;
- every finding above and its final disposition;
- the two repaired coordination-chain gaps;
- the still-unpromoted `CANDIDATE` inventory; and
- the release-metadata surfaces to re-derive after this MR merges.

Add the required six-surface projection manifest. The normative owner stays the
existing publication rule; the two adopter pages receive the matching factual
status; runbook and loader changes are not applicable; validation evidence is
the released-artifact check plus exact stale-phrase assertions; distribution is
covered because all changed AGF paths ship in the framework archive/site.

## Evidence and tests

- Discover the official v2.11.0 assets through the anonymous GitLab Releases
  HTTPS API and download the Linux x86_64 archive, checksum sidecar, and
  provenance record into a fresh `mktemp -d` directory. Verify checksum
  agreement and the provenance binding to the expected tag, source commit,
  filename, and checksum before extracting or executing the binary. Then run
  only the bounded read-only commands below; record asset
  identities and results without credentials, shell tracing, local paths, or
  environment contents. Remove the temporary directory afterward. Confirm
  separately that the framework archive contains the coordination schema/rule
  surfaces used by challenge-response and publication holds. Use the existing
  `agov dist post-publication-check` path where it applies instead of inventing
  a second verifier. Run `agov --version` and `agov coord hold-status --help`,
  then use disposable fixture records with the released binary to assert urgent
  ordering through `agov handoff next`, `agov coord pending`, and
  `agov session resume`, and to assert the raising, replied, escalated, and
  expired challenge states through `agov coord pending`. File presence or help
  output alone is not behavioral evidence.
- Assert the exact stale phrases are absent and the compatible-version warning
  remains.
- `agov plan validate` and strict plans-only collision check pass.
- `agov plan projection-check` passes for the new manifest.
- MkDocs strict build and rendered link checks pass.
- `agov validate`, `agov validate adopter-docs`, changelog checks, and
  `git diff --check` pass.
- The implementation diff is confined to the six non-plan paths declared
  above; no version, Cargo, agent-definition, release-status, or CI file changes.
- Independent exact-head fact check confirms the shipment claims and changelog
  scope before Gate 2.

## Risks and controls

- **A factual correction overclaims availability.** Control: verify the
  immutable published v2.11.0 bytes, not current source or changelog alone.
- **The rule edit changes policy accidentally.** Control: exact paragraph-only
  diff, projection manifest, and contribution audit; no obligation word changes.
- **Frozen scope expands through changelog prose.** Control: exactly two new
  content rows plus one status-fix row; each maps to an already-merged frozen
  item or this factual correction.
- **Release metadata is derived too early.** Control: the child publication plan
  is drafted from the post-merge `main` base, never this plan head.

Rollback is a revert of the implementation MR before release metadata is
derived. No schema, runtime, data, tag, or published namespace changes.

## Collision disposition

The blocking plans-only check identifies six predecessor plans whose declared
paths overlap this correction: assistant-rule drift resolution, Windows
retirement, strict MCP arguments/ref-aware list, timer-reality polling,
v2.11 changelog reconciliation, and supersedes adopter documentation. Their
implementations are already merged and included in v2.11.0. They are listed in
`depends_on` so this plan explicitly sequences after and preserves their landed
results. This is not permission to overwrite an unmerged counterpart: before
implementation, fetch current refs and stop if any of those plan owners has a
new open implementation on an affected path.

## Gate request

Approve only the bounded changelog reconciliation, three shipment-status
corrections, freeze report, and projection manifest above. Gate 2 remains an
exact-head governor decision after the stated independent reviews and CI.
