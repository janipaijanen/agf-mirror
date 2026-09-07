---
kind: review
from: infosec-sentinel
to: governor-reviewer
id: 20260907T184803Z-infosec-sentinel-qjhj
timestamp: 2026-09-07T18:48:03Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260907T173329Z-planner-architect-3g2g.md
  - 20260907T183718Z-planner-architect-pmqg.md
  - 20260907T184721Z-contribution-auditor-3dkt.md
topic: issue120-plan-infosec-screen
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# InfoSec advisory screen of !2527

Scope: all five added files at `bf62f357e78aff6e0102835f8a0b6f44c0fd89bb`,
including namespace-query, credential, disclosure, and pre-tag boundaries.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 1

Advisory findings: 1

Open findings with accepted risk: 0

Authority: advisory screening recommendation only; governor-reviewer decides
the gate. This is not the infosec validator reporting a credential block.

The direct governor assignment and fresh non-author session identity are
acknowledged in `3dkt`. This is the subsequent, separate infosec lens. No
screened artifact, credential setting, protected-tag rule, tag, registry,
package, or release object was mutated. Live read-only project evidence was
collected during the contribution audit; this screen consumes that evidence.

## RUW4-IS1 — distinguish hidden/inaccessible resources from absence

Check: section 2 authentication and HTTP-404 absence semantics

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: RUW4-IS1

Impact: material

Enforcement: blocking

Basis: the plan's fail-closed-on-any-policy/API-ambiguity contract; advisory
security judgment, not an expansion of the configured validator's authority.

Finding state: open

Risk disposition: none

The Git-tag and Release bullets unconditionally interpret HTTP 404 as absence.
GitLab documents that 404 can also mean the caller is not authorized to access
the resource. The explicit fail-closed enumeration covers 401/403 but leaves
this indistinguishable 404 case classified as success. A valid job token and
successful package enumeration do not prove repository/Release visibility:
feature access and endpoint permissions can differ. CA1's wrong project ID
illustrates why a negative resource response alone is insufficient evidence.

Resolution condition: bind the correct target identity and establish access to
each namespace under the same credential before accepting candidate absence.
Specify a supported positive-access/listing strategy, fully paginated where
appropriate, and fail closed when that access cannot be demonstrated. Include
negative tests for hidden/unauthorized 404 and wrong-project responses, plus a
positive authorized-absence case. Keep the header-only ephemeral CI credential
contract; no broader token permission is requested by this finding.

Source: [GitLab REST API troubleshooting](https://docs.gitlab.com/api/rest/troubleshooting/)
states that 404 includes inaccessible resources. The
[job-token documentation](https://docs.gitlab.com/ci/jobs/ci_job_token/)
describes endpoint-specific and triggering-user permissions. Documentation
supports the proposed credential family, but no live CI_JOB_TOKEN exercise was
performed; implementation acceptance must establish endpoint coverage.

## RUW4-IS2 — missing SPDX header in the new projection manifest

Check: changed-file SPDX screening

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: RUW4-IS2

Impact: minor

Enforcement: advisory

Basis: infosec-screen skill step 3; this is license hygiene, not a secret leak.

Finding state: open

Risk disposition: none

`docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml`
starts with descriptive comments and has no SPDX header. The other four added
Markdown files have the repository's dual-license header after front matter.

Resolution condition: add the repository-standard SPDX and copyright comments
at the beginning of the new YAML manifest and recheck the revised diff.

## Screening results

| Check | Status | Details |
|---|---|---|
| agov validate infosec | pass | Both learning files OK; 0 emitted blocks, 0 emitted warnings, 0 emitted acknowledged findings. No learning files change. |
| pre-commit hooks | skipped | No .pre-commit-config.yaml in the candidate tree. |
| SPDX headers | fail | 4 of 5 changed files pass; IS2 identifies the missing header. Scope is candidate additions, not a repository-wide license certification. |
| Project patterns / narrative disclosure | pass | Manual read of all five additions found no credential values, private cross-project linkage, or fictional-context identity leak. Public repository identifiers and variable names are functional release bindings. |
| Query/access security | fail | IS1; the explicit no-PAT, no-query-string credential, no shell tracing, and no-token-artifact controls are otherwise sound at plan level. |
| Protected tags / immutable publication | pass at plan level | Plan adds pre-tag reads and retains governor gates; no authority to mutate protected-tag configuration, publish a tag, or overwrite artifacts is granted. |

Automated infosec screening covers the configured learning corpus; it does not
prove narrative-doc safety or the correctness of an unimplemented API checker.
The design is Level 1 Planned. This screen reports no actual credential leak or
demonstrated overwrite. Owner revision and renewed exact-head consultation are
required before requesting Gate 1. The contribution findings remain separate.
