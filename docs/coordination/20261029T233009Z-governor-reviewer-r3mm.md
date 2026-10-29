---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261029T233009Z-governor-reviewer-r3mm
timestamp: 2026-10-29T23:30:09Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T232240Z-governor-reviewer-dta3.md
topic: v213-metadata-decision-carrier-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: preserve the v2.13 metadata decision on main

Under active ss2k authority, approve only MR !3387 at
755063ef3b5473f5b5f9b3bf739d492c7ff31165, without squash, into main at
https://gitlab.com/janipaijanen/agents-govern-framework.git. Current target is
e37f3f605496af5f570b85bb3d74866a98ff0fe8. The sole added file is unchanged
20261029T232240Z-governor-reviewer-dta3.md. Canonical pipeline 3006334262
passed both mandatory jobs: governance 17593052923 and commercial corpus
17593052924. Fresh MR state is mergeable, discussions resolved, zero outstanding
forge approvals. Full-range publication validation previously passed against
05638619; repeat it against the actual current target before merge and verify
the computed merge adds only that unchanged record.

The carried dta3 was shared at this same commit on
coordination/v213-metadata-gate2 before its authorized metadata merge !3385.
That merge is a46c2e1c8754103fce45b6bdabe023c3ee79b2b6, with verified
parents 05638619/cb714986 and tested tree f03ea9899. The standard release
sequence then passed the fresh canonical four-namespace ABSENT check at
2026-10-29T23:26:30.035715781Z and pushed annotated tag object
a75803f2f791779fb1ec82aad92eb150cdfab56b, peeled to that actual main merge.
The independently authorized handoff !3386 then merged as current target
e37f3f60; its actual parents/tree show only unchanged qe18 added. These facts
preserve chronology; they are not retroactive authorization.

The tag pipeline 3006335980 is running with all 23 expected mandatory jobs.
No release-complete/public-verification/status-sync claim is made here. dta3
contains a continuing post-publication header-repair proposal obligation and
therefore requires ordinary Gate 2, supplied by this record. Its exception is
not enlarged or renewed.

Publish this unchanged record on a shared branch before the single authorized
merge. Recheck exact source, target, full diff, CI, discussions and approvals;
merge without squash and verify actual first parent e37f3f60, second parent
755063ef and computed tree. Any change stops for reassessment. This record
only authorizes that merge and its verification. It grants no release action,
new obligation, timer/session, rule exception or authority extension. Its own
merge-only authority is exhausted once that merge and verification finish;
its carrier may then use bounded record-only termination only if every
condition in the publication rule is independently evidenced.
