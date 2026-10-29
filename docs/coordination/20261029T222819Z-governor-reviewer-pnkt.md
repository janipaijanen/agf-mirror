---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261029T222819Z-governor-reviewer-pnkt
timestamp: 2026-10-29T22:28:19Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T221020Z-governor-reviewer-tjhr.md
  - 20261029T222324Z-governor-reviewer-dtg2.md
  - 20261029T222445Z-planner-architect-n65t.md
  - 20261029T222324Z-contribution-auditor-s1pd.md
  - 20261029T222324Z-infosec-sentinel-nn6b.md
topic: v213-amended-publication-carriers-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: corrected v2.13 publication plan and approval trail

Under active ss2k delegation, approve exactly these two merges in order,
without squash, into main at
https://gitlab.com/janipaijanen/agents-govern-framework.git:

1. !3381 at 5c156ceda3cfe56f1834da029db8f1f0604feeb3, containing only six
   unchanged published review/approval records: deh1, mdhs, tjhr, s1pd, nn6b,
   dtg2. Canonical pipeline 3006196433 passed both required jobs.
2. !3382 at a59ca84a35225d4a84e6e8ec4e837bffbd302c8e, adding only the
   corrected approved plan and handoffs 7h3m/n65t. Canonical exact-head pipeline
   3006198681 passed all three required jobs, including plan collision.

Whole-plan and narrow Pages reviews are COMPLETE/CLEAR. Governor review confirms
the concrete version/status inventory, protected-tag and namespace controls,
source/tree identity, accepted existing Pages side effect and verified-status
separation. Independent final-head review is CLEAR at a59ca84a: body SHA-256
3727f31eae2809e90c800f7aaa1b9e65d8356204ca0f68240a67f06075717e83
matches the reviewed amended body at65a3f03a. Only approval/consultation metadata
and the faithful append-only handoff were added. dtg2/review publication preceded
approved metadata. Schema and diff checks pass; 17 overlaps are acknowledged,
zero unacknowledged, and 204 historical no-path plans remain unassessed.

The original 4rhd authorizations expired unexecuted under already-published dtg2.
Its obsolete carrier !3383 is closed at unchanged38c3dc00, unmerged, with its
shared source branch preserved. This decision grants fresh exact-head authority;
it does not treat the old head's CI or expired authority as acceptance.

Publish this unchanged decision before either merge. Freshly verify exact head,
main target, successful exact-head CI/jobs, resolved discussions and zero forge
approvals outstanding. Current main is bd341ab11e0b789ff8bebc277f33b9cbf9983336.
First compute !3381's merge tree: it may add only its six unchanged records.
Merge and verify actual parents/tree. Then recompute !3382 against that actual
new target: preserve all six records and add only the approved plan plus the two
handoffs. Verify the resulting parents/tree and three-file delta. Any unexpected
head/target/content change stops the sequence.

This record authorizes only those two merges and their verification. It grants
no version edit, policy mutation, tag, package/OCI/Release operation, deployment,
new session/timer or delegation extension. Implementation authority remains tjhr
as amended by dtg2, only after its verified-merge and green-construction-base
prerequisites. ja28 scope/expiry and ss2k expiry remain unchanged. The continuing
approval carrier receives ordinary Gate 2 here; this merge-only record becomes
exhausted only after both authorized merges and their verification are complete.
