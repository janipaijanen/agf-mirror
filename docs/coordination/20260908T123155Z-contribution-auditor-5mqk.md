---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260908T123155Z-contribution-auditor-5mqk
timestamp: 2026-09-08T12:31:55Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260908T122515Z-coder-tester-4msq.md
  - 20260908T083406Z-governor-reviewer-rg6k.md
  - 20260907T151325Z-governor-reviewer-2emq.md
topic: v210-release-contribution-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.10 release contribution review — revisions required

Scope: release MR !2549, source `58459c54207cdd5b550377fd3dbcb144045b9ac8`,
target `2d0fdc0003498a75d876c112dfc59747aefa2747`, all 28 changed paths.
Reviewer did not author this release metadata commit or its changelogs.
Reviewer authored the underlying RUW-4 command and reviewed its earlier plan;
this dependence is disclosed. This release-diff review is not independent
corroboration of the underlying RUW-4 implementation. No release-branch file,
credential, policy, or namespace was modified during review.

Coverage: COMPLETE. Recommendation: **BLOCK pending two documentation fixes**.
Open findings requiring revision: 2 (medium). Other advisory findings: 0.
Accepted-risk findings: 0. This is an advisory reviewer recommendation;
the governor decides Gate 2. No tool result grants gate authority.

## Verified scope and evidence

All 28 paths match the approved release allowlist. Agent definitions change
only framework_version, with role status, role authority, boundaries and
capability versions intact. Only live adoption metadata changes in the learning
corpus; learning entries remain byte-identical. External Cargo.lock package
records, sources and checksums are identical; only five workspace package
versions change, with matching workspace dependency versions. No functional
source, CI, schema, rule, test or plan change is present.

The v2.9.0..2d0fdc00 inventory is covered by the polling, ROI, Propeller,
namespace-guard and MCP-learning summaries. The release truth distinguishes
pending v2.10 from published v2.9. The 2emq scope amendment permits Propeller
Level 4/inactive; rg6k permits the disposable local commit for namespace
preflight. No forbidden historical rewrite or new authority was found.
The six audit dimensions were considered: plausible claims, evidence quality,
absence bias, governance relaxation, boundary integrity and rule soundness.

`agov validate contributions` passed with two inherited warnings in unchanged
learning text (absence-bias and governance-relaxation patterns). Diff whitespace
passed. Canonical pipeline 2829382036 has 17 successful required jobs and two
policy-optional manual Windows jobs. Actual job 16366090603 JSON reports all
four namespaces ABSENT at 2026-09-08T12:02:46.552637268Z, bound to the reviewed
source/target, project 83321848, pipeline, version and derived OCI reference.
The operator ABSENT report at 11:44:52Z is author-supplied evidence; this reviewer
independently retrieved the CI artifact rather than rerunning namespace checks.

## V210-CA1 — medium — incorrect Propeller follow-up attribution

Check: release-truth projection; reviewed; outcome fail; mandatory requirement.
Basis: verifiable-claims-rule and 2emq's named post-release follow-up requirement.
Finding state: open. Risk disposition: none. Enforcement: advisory to governor.

CHANGELOG.md:36-38 and agf/CHANGELOG.md:35-37 say #132 owns activation,
genuine wake and Level 5 proof. agf/docs/site/roadmap.md:103-104 repeats it.
Live issue #131 owns those actions. Issue #132 was explicitly narrowed to skill
export after #131; its description includes the dedup correction. The wrong
reference sends operators to a dependent task and obscures the actual deferred
release obligation. Resolve all three projections: #131 owns Stage 4 activation,
genuine first wake and Level 5; #132 owns subsequent skill export.

## V210-CA2 — medium — overstates automated protected-tag-policy coverage

Check: security capability claim; reviewed; outcome fail; mandatory requirement.
Basis: verifiable-claims-rule and the release plan's separate protected-tag-policy
check. Finding state: open. Risk disposition: none. Enforcement: advisory.

CHANGELOG.md:40-44 and agf/CHANGELOG.md:39-43 attribute fail-closed
protected-tag-policy checks to `agov release namespace-check` and its CI job.
The command does not query protected-tags policy; its reviewed API walk and
JSON establish identity and four-namespace absence. The release plan requires
a separate policy read (preflight step 6 and immediate pre-tag sequence).
Separate that operator obligation from automated namespace proof in both
changelogs. Do not change the command/rule to match overstated prose.
This is the same substantive finding as V210-IS1 in the security lens.

## Disposition

Both fixes fit existing metadata paths. Refresh exact-head evidence and focused
review after correction. Adopter docs parity: updated, with these two accuracy
corrections outstanding. Gate 2, immediate pre-push check, protected policy
recheck, tag pipeline, supported artifacts and anonymous verification remain
separate. No merge, tag, issue closure or publication approval is supplied.
