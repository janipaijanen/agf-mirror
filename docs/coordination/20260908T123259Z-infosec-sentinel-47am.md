---
kind: review
from: infosec-sentinel
to: governor-reviewer
id: 20260908T123259Z-infosec-sentinel-47am
timestamp: 2026-09-08T12:32:59Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260908T122525Z-coder-tester-khdg.md
  - 20260908T123155Z-contribution-auditor-5mqk.md
  - 20260908T054148Z-infosec-sentinel-k5bs.md
topic: v210-release-infosec-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.10 release InfoSec review — security claim correction required

Scope: release MR !2549, source `58459c54207cdd5b550377fd3dbcb144045b9ac8`,
target `2d0fdc0003498a75d876c112dfc59747aefa2747`, all 28 changed paths.
Reviewer did not author this release metadata commit or its changelogs.
Reviewer authored the underlying RUW-4 command and reviewed its earlier plan;
this dependence is disclosed. This release-diff review is not independent
corroboration of the underlying RUW-4 implementation. No release-branch file,
credential, policy, or namespace was modified during review.

Coverage: COMPLETE. Recommendation: **BLOCK pending V210-IS1 prose correction**.
Open findings requiring revision: 1 (medium). Advisory groups: 1 (inherited SPDX).
Accepted-risk findings: 0. Advisory recommendation only; governor decides Gate 2.

## Screening

| Check | Result | Evidence |
|---|---|---|
| agov validate infosec | pass | two learning files/blocks; zero warnings or acknowledged findings |
| pre-commit | skipped | no .pre-commit-config.yaml at reviewed head |
| SPDX | inherited advisory | five changed Markdown/YAML files have no file-local header at either base or head; listed below |
| project patterns | pass | no credential value/private key, fictional-context identity, foreign codename or cross-record exposure introduced |

All 28 metadata paths were read. Learning entries are byte-identical; no agent
boundary, role status, dependency source, external lock record, checksum, CI
or executable implementation changes. The release JSON contains identifiers
and namespace status, no credential. Review performed no credential use or
protected-tag-policy/namespace mutation.

Canonical pipeline 2829382036 passed all 17 required jobs; two Windows jobs
are optional/manual. cargo-deny job 16366090609 reports advisories, bans,
licenses and sources OK. CVE scan 16366090599 reports zero vulnerabilities
for Cargo.lock. OCI build/scan 16366090602 reports zero vulnerabilities for
the built Alpine 3.22.5 image. No failed required job was hidden by manual status.

The real release:namespace-check job 16366090603 passed. Independently retrieved
artifact dist/release-namespace-absence.json binds four ABSENT outcomes to
source 58459c54, target 2d0fdc00, project 83321848, pipeline 2829382036,
version 2.10.0 and registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.10.0
at 2026-09-08T12:02:46.552637268Z. The author also reports operator ABSENT at
11:44:52Z; the reviewer did not independently rerun that operator checkpoint.
These timestamps do not substitute for the mandatory immediate pre-push recheck.

## V210-IS1 — medium — automated policy-check capability overclaim

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Basis: verifiable-claims-rule; approved release plan keeps protected-tag policy
as a separate operator check. Finding state: open. Risk disposition: none.
Enforcement: advisory reviewer recommendation, not a mechanical gate decision.

Both changelogs (CHANGELOG.md:40-44; agf/CHANGELOG.md:39-43) include
protected-tag policy in the checks ascribed to the command/CI job. Neither
its API calls nor the success artifact verifies that policy. An operator could
mistake green namespace proof for policy verification and omit the separate
mandatory read. Resolution: clearly distinguish automated namespace/identity
checks from the unchanged separate operator protected-tag-policy check.
This duplicates V210-CA2 across lenses, not a second independent defect.

## V210-IS-A1 — inherited SPDX hygiene (non-blocking)

Check status: reviewed. Outcome: needs-review. Requirement strength: recommended
for this metadata-only MR because all omissions predate it. Impact: minor.
Enforcement: advisory. Basis: infosec-screen skill SPDX step.
Finding state: open. Risk disposition: none.

Missing file-local headers at both base and head: AGENTS.md, CHANGELOG.md,
agf/CHANGELOG.md, agf/docs/site/roadmap.md, agf/templates/learning-record.yaml.
No existing license header is removed or altered. Address through separately
scoped repository policy/header work; do not silently broaden this release diff.

## Boundary

No new secret, dependency or execution defect found in the release metadata.
Revise the false security-capability claim before Gate 2 and refresh exact-head
review evidence. Actual CI execution now exists, but supported release artifact,
immediate pre-push check, protected-policy read, tag pipeline and anonymous
artifact verification are still future evidence. No release authority supplied.
