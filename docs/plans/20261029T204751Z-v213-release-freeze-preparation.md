---
id: 20261029T204751Z-v213-release-freeze-preparation
status: approved
gate1_approved_by: 20261029T205351Z-governor-reviewer-mbsk
gate1_approved_at: "2026-10-29T20:53:51Z"
topic: v213-release-freeze-preparation
affected_paths:
  - agf/docs/site/docs/operate/poll-and-coordinate.md
  - docs/reviews/rule-projection/20261029T204751Z-v213-release-freeze-preparation.yaml
  - CHANGELOG.md
  - agf/CHANGELOG.md
  - docs/reviews/documentation-estate/20261029-v213-release-content-freeze.md
  - docs/reviews/next-release-freeze-evidence/20261029-v213-apk-residual-risk.md
  - docs/plans/20261029T204751Z-v213-release-freeze-preparation.md
depends_on:
  - 20261029T163752Z-v213-release-scope
  - 20261027T045944Z-record-only-publication-termination
  - 20260920T083259Z-distribution-scan-profiling-slice-b
  - 20261029T192050Z-dist-profile-roi-amendment
  - 20261028T194451Z-ci-reproducibility-pins
  - 20261029T192922Z-release-publisher-api-amendment
  - 20261027T230355Z-v212-release-freeze-corrections
  - 20261027T235807Z-v212-release-publication
  - 20261021T220000Z-retire-windows-support
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20261025T192020Z-supersedes-adopter-docs
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent Codex contribution audit CLEAR at feb398a05594fd4d6eb33e46fb213e457b25458c;
      all freeze prerequisites, role boundaries and five detected overlaps covered;
      no blocking or advisory findings. Historical unassessed plans remain explicit.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Independent Codex InfoSec advisory CLEAR at feb398a05594fd4d6eb33e46fb213e457b25458c;
      bounded risk evidence and explicit governor disposition required, no production-write
      authority; Rust infosec validator and manual plan screening passed.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Prepare the v2.13 content freeze without publishing a release

## Purpose and entry conditions

The approved v2.13 scope requires reconciled changelogs, a complete documentation
estate sweep, combined validation and a durable residual-risk record before its
separate release-publication plan. Existing implementation plans do not own those
edits. This child owns exactly the seven paths above; it adds no release feature.

Planning and advisory review may happen while publisher !3358 CI runs.
Implementation waits for published Gate 1 approval, binding publisher Gate 2,
and the publisher's verified merge. Start from a clean isolated branch at the
then-current main, identify the exact combined implementation SHA, and reobserve
all three IN slices and their prior authority. Root's dirty checkout is excluded.
Record-only permission !3354 and profiling !3372 are already merged; publisher
fb338ba0 is a reviewed candidate, not yet a merged fact at plan drafting time.

No version bump, release-provenance or docs-manifest/status change, agent metadata
change, tag/protection/package/OCI/Release operation, deployment, new feature or
new timer/session is authorized. The release-publication child is not drafted
until the approved scope's freeze prerequisites actually hold.

## Players and ownership

Request a governor assignment for this bounded freeze cycle: an existing Codex
reviewer acts as the active documentation-estate-steward while Claude is unavailable;
Codex release owner/coder-tester authors changelogs and the residual-risk report.
The steward assesses currency/consistency and writes only its assessment report;
it does not author changelogs, freeze their scope or change normative content.
The governor retains gate and finding-disposition authority. Name the actual
players in the approval/handoff, preserving separate author and steward work.
This is a temporary player allocation, not a role activation/status change.

## Changelog reconciliation

The three older changelog owners named in `depends_on` acknowledge overlapping
paths, not a transfer of their claims or an assertion that their plans are closed.
Preserve the landed Windows retirement and coordination supersession semantics,
and all v2.11 reconciliation entries within historical sections. This child edits
only the current Unreleased bodies; it does not reopen those implementations.


Edit only the existing Unreleased bodies in both changelogs. Preserve historical
entries and their headings byte-for-byte; leave version/date release headings for
the later publication plan. Inventory every first-parent merge after v2.12.0's
peeled commit e045d20b073ce31341cd0d460469fac60cdbab5a through the chosen baseline.
The estate report crosswalks every merge to a release entry, administrative
release-truth upkeep, or no-shipped-byte disposition. Do not advertise plans and
coordination carriers as product features.

- Adopter changelog: bounded record-only publication termination and the factual
  urgency-status correction below only. Explain
  its conjunctive limits, prior published/exhausted authority, unchanged record
  and exact-head checks; ordinary Human review remains valid. Call it additive
  but authority-sensitive, with no schema/tooling/CI behavior change. Cite the
  actual adopter paths: rules/workflow/coordination-publication-rule.md,
  docs/concepts/coordination-protocol.md and templates/AGENTS.md. Evidence status
  remains Empirically informed; do not claim broad adopter validation.
- Root changelog: opt-in distribution profiling and its qualified baseline;
  diagnostic absent from default release binaries; no cache-effect or speedup
  claim. Then describe foundry-only CI work: Rust 1.97.1, checksummed rustup-init,
  verified digest-pinned Alpine and create-only publisher, advisory drift check.
  Do not imply full-job reproducibility or adopter CI changes. Include v2.12
  post-publication status synchronization as administrative upkeep.

## Full documentation-estate sweep

Apply all six dimensions and boundaries in
agf/agents/documentation-estate-steward.yaml. Declare the full touchpoint set,
exclusions with reasons, baseline SHA and completion criterion before assessment.
Include root adopter/loader surfaces, agf templates, site and non-site docs,
normative/customize rule surfaces, learning current-version cells, manifests and
roadmaps; reconcile docs/reviews/rule-projection manifests. Embed the complete
classified touchpoint inventory in the named report, without editing manifests.

A changed-page check alone is insufficient. Prior full-estate findings may be
reused only with explicit baseline/byte-identity evidence and a fresh complete
surface inventory and whole-tree validation; label reused evidence honestly.
Rate each dimension and disposition every finding. Unknown, unresolved or referred
findings do not clear the release freeze. Anything requiring source edits beyond
the seven-path scope, structural/normative changes or other-role judgment is
escalated; amend Gate 1 before editing extra paths. Preliminary intake found the
record-only owner, site explanation, template and projection manifest coherent;
that is not a substitute for this complete sweep.

## Refreshed APK residual-risk evidence

After publisher merge, write the named report and link the historical durable
note 20261028T210122Z-planner-architect-2k4p. Preserve that immutable note; update
its release-cli-era assumptions in this new report. Bind the actual merge SHA,
official Alpine tag and linux/amd64 digest, installed curl/jq/unzip then tar/gzip
path, versions, CA inventory, scanner/database timestamps and report hashes.
Reference the verified zero-HIGH/CRITICAL base/runtime results and independent
review, with their date/coverage limits; do not claim vulnerability freedom or
that the obsolete release-cli findings were waived/remediated in place.

Explain mutable APK repositories and consequences: tool/CA/version drift, registry
availability and lack of bit-for-bit full-job reproducibility. Record the actual
controls, stop conditions and reviewed rollback boundary. APK package pinning is
explicitly deferred by the release scope. Request an explicit governor disposition
for this bounded residual during Gate 2, without adding a builder-image project,
suppressing a scanner finding or silently extending existing risk acceptance.
Any new vulnerability finding still requires remediation or a separate explicit
disposition before the freeze can clear. Report read-only platform-probe evidence
separately from production-write verification; no throwaway Release/tag tests.

## Validation, evidence and completion

After all three implementations are merged, use the local release build and
capture full Rust workspace and shipped distribution validation on the combined
candidate. Name exact commands, SHA, counts, ignores and environment; preserve
failures and diagnose them without unchanged paid retries. Run all applicable
adopter-doc, projection (strict), site and governance checks using actual supported
commands. Record exact commands rather than inventing a convenience command.
If a required tool is missing, report the gap rather than falling back to Elixir.

This change adds only docs, so do not invent implementation tests. Preserve the
version, provenance, docs-manifest/status, tag and artifact state. Verify the
entire diff and all historical changelog sections; validate plans/coordination and
changed-plan overlaps. Obtain independent contribution/security review of the
final reports/changelogs and successful exact-head CI before binding Gate 2.

The assessment distinguishes the combined implementation baseline from the final
docs candidate SHA. Record any intervening main changes and revalidate affected
claims; do not freeze a stale head. Once this bounded change is merged and all
prerequisites hold, the release owner may propose a separate publication plan
naming that exact freeze SHA. This plan grants no release-publication authority.

Rollback removes only this child's documentation delta, leaving implemented
features intact and release content unfrozen until corrected evidence is approved.


## Pending bounded amendment: stale urgency release status

The full-estate sweep found that `docs/operate/poll-and-coordinate.md` still
calls urgency ordering Level 4 and pending release, including a development-CLI
heading. Canonical protocol text and previously verified immutable v2.11 release
artifacts establish shipment at Level 5. This page was byte-identical across
v2.12; a fresh full inventory exposed a prior sweep omission.

The proposed amendment adds exactly the polling page and this plan's projection
manifest to affected_paths. Change only the heading to "Reading urgency in v2.11
and later" and replace the stale maturity/shipment sentence with "This behavior
shipped in v2.11.0 and is Level 5 Deployable when using compatible v2.11.0-or-later
artifacts." Preserve the urgency tiers, advisory/no-permission boundary, command
and challenge guidance. Add an Unreleased Fixed entry in the already-owned adopter
changelog; every historical changelog section remains byte-identical.

The new six-row projection manifest records this release-fact correction without
changing a rule, schema or operating step. Existing projection manifests and
release-status/governance manifests remain unchanged. Validate it strictly after
the page edit and run the whole-inventory projection check, strict site build,
shipped docs tests and independent fidelity review. The estate report must retain
the finding and its actual disposition rather than relabel the prior sweep complete.

The original five-path approval mbsk remains valid. The two additional paths and
this correction are pending a new governor amendment decision; do not implement
them from this draft. No other expansion, version/status edit, release/tag authority
or vulnerability disposition is requested. Any additional out-of-scope finding
still requires its own reviewed amendment.
