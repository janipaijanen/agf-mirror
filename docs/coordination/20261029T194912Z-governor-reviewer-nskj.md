---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261029T194912Z-governor-reviewer-nskj
timestamp: 2026-10-29T19:49:12Z
model: gpt-6-astra
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T162805Z-governor-reviewer-dn8m.md
  - 20261029T193807Z-governor-reviewer-qmba.md
topic: dist-profile-baseline-acceptance-amendment
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Accept the honest profiling baseline; defer controlled cache comparison

Under ss2k's active delegation, amend only measurement acceptance in section 4 of
`20260920T083259Z-distribution-scan-profiling-slice-b`. Accept the feature-gated
instrumentation, fixture, and repeated fresh-process fixture/real-tree baseline
with uncontrolled filesystem-cache state. The controlled cold/warm comparison was
not performed and is explicitly deferred, not deemed satisfied or relabeled.

Independent final review at implementation/report head
`11e96a285210fc23f23ff3ee35c1a1f4c37a0088` returned BLOCK solely on the unmet
original cold/warm requirement; source continuity, report fidelity, fixture and
compile-out guarantees were otherwise CLEAR. The reviewer assessed this exact
proposed resolution as acceptable if published before Gate 2. A new process,
copied checkout or container does not isolate the shared host's caches; targeted
eviction is incomplete, while global cache flushing or memory pressure would
disrupt other work. A future controlled comparison requires separately scoped
fresh disposable VM or dedicated runner/filesystem evidence.

Accept only API-call counts and measured aggregate phase durations. This evidence
supports no cache-effect, speedup, optimization, CI-critical-path or
pristine-checkout claim. Preserve all other parent and child requirements;
no algorithm, matching policy, CI wiring or dependency change is authorized.
The report retains its explicit cache qualification and initial failed audit.

The governor inspected the report and test evidence: measured source 08ae26b8
was clean, and the final commit adds only the report. Core tests pass 37 default /
38 feature; CLI 497 / 498; workspace all-features 839; manual baseline 1. Existing
OCI tests remain ignored. This is an acceptance-scope amendment, not implementation
Gate 2 approval. Re-review the final candidate against the amended requirement,
publish its handoff, and require successful exact-head MR CI before Gate 2.

The paired plan amendment and this decision may merge into main only after final
head CI succeeds, discussions are resolved and exact diff review confirms these
two files with no unrelated changes. Do not broaden scope in that publication.

Also approve Gate 2 publication of prior decision carrier !3370 at exact head
2858bbe214c4730ac0c592e5bca01115107d211b, without squash, into main. Its pipeline
3005598967 passed. Before merge, freshly verify head, successful pipeline,
resolved discussions and a merge tree adding only unchanged qmba; verify parents
and file diff afterward. Publisher plan !3369 already merged at
112f8a5c632aa91681f64fec13b5b78adf3b2100 under qmba's published conditions.

No image-risk waiver, platform-permission acceptance, release/tag publication,
deployment or extension of governor delegation is granted.
