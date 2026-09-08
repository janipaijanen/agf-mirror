---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260908T055221Z-coder-tester-bbsh
timestamp: 2026-09-08T05:52:21Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260907T200210Z-coder-tester-8j2h.md
  - 20260907T191930Z-governor-reviewer-qqbh.md
  - 20260908T043532Z-contribution-auditor-hqp4.md
  - 20260908T054148Z-infosec-sentinel-k5bs.md
  - 20260908T054209Z-build-engineer-r75b.md
topic: ruw4-exact-head-gate2
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 2 for MR !2539 at b4199c26abfd43090c44dc3c795331f4584de734, weighing the disclosed reviewer provenance and inherited advisory.
  must_read:
    - docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md
    - docs/coordination/20260908T043532Z-contribution-auditor-hqp4.md
    - docs/coordination/20260908T054148Z-infosec-sentinel-k5bs.md
    - docs/coordination/20260908T054209Z-build-engineer-r75b.md
  do_not_touch:
    - Release tags, version metadata, packages, OCI publication, and deployment without separate governor authority.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# RUW-4 exact-head Gate 2 handoff

MR !2539 is ready for the governor's Gate 2 decision at
`b4199c26abfd43090c44dc3c795331f4584de734`.
Canonical MR pipeline **2828211506 succeeded**, with all **22 mandatory
jobs successful**. The two Windows jobs are optional/manual by existing
policy. Gate 1 remains qqbh. This handoff grants no Gate 2 or release authority.

The implementation adds a read-only, fail-closed four-namespace absence
command, release-metadata MR job consuming the existing preview artifact,
mandatory operator preflight/pre-push checkpoints, and matching governance,
operator documentation, and negative tests. No version or changelog changed.
The b4199c26 correction supplies the full BREAKING Impact statement and
explains local operator environment setup; hqp4 resolves both hg52 findings.

## Published exact-head review evidence

- Contribution **CLEAR**, hqp4, merged in !2542. Reviewer authored the approved
  plan but not the implementation; the governor weighs this provenance.
- InfoSec **CLEAR**, k5bs, merged in !2543. Zero blocking findings; inherited
  non-blocking advisory **RUW4-IS-A1** remains open: the modified legacy
  operator page lacks an SPDX header at both base and reviewed head.
- Build **CLEAR**, r75b, merged in !2543. Zero blocking or advisory findings.

The governor assigned the InfoSec/build reviewer those roles. That session
coauthored the retained authentication correction and tests. These are complete
role-lens reviews, **not independent corroboration of that contribution**.
The primary author does not supply an independent review of this implementation.
The governor must weigh these limits when deciding Gate 2.

## Validation and evidence limits

Local workspace suite: 506 passed, 0 failed, 3 intentionally ignored; dedicated
CI jobs cover the ignored OCI/registry paths. Focused evidence: 8 namespace unit,
6 CLI/adapter, 12 CI-needs, and 15 gate-change tests passed. Exact-head CI,
Clippy, GitLab lint, strict projection and gate-change checks passed.
Negative cases stop every occupied namespace before publication and cover
identity, hidden access, pagination, malformed responses, credential separation,
and dirty/unmerged operator checkout failures.

**Adopter docs parity: updated** — owner release rule, release skill and
operator page describe the new mandatory checks and migration inputs.

Capability is **Level 4 Validated**. **Level 5 Deployable** requires the
supported release path/artifact and a release-metadata MR executing the real
namespace job successfully, with matching documentation. The implementation MR
does not change agf/VERSION, so that job correctly did not run here. Deterministic
selection/artifact/negative tests do not constitute live job-token acceptance.
The k5bs/r75b maturity statements explicitly correct hqp4's Level 1 sentence;
its contribution CLEAR result is unchanged.

No implementation correction remains requested. Next action is the binding
human Gate 2 decision at this exact SHA; any subsequent code change needs
fresh evidence. Merging this coordination record does not merge !2539.
