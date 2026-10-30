---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261030T074128Z-governor-reviewer-ksm2
timestamp: 2026-10-30T07:41:28Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261030T072711Z-governor-reviewer-8p5m.md
  - 20261030T072858Z-coder-tester-km7d.md
  - 20261030T073453Z-governor-reviewer-gs85.md
  - 20261030T073255Z-coder-tester-rg82.md
topic: v2131-readme-child-exact-head-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: merge the independently reviewed README child

Under active full governor delegation ss2k, approve MR !3397 at exact head
7b79aab16c8e4ebe92629c1129e8611e4aeb6b29, without squash, into main at
https://gitlab.com/janipaijanen/agents-govern-framework.git. The only accepted
target is bb407c35969ccb251a32f5fe32179e58600223a7; computed merge tree is
e55f8d90a6ad481c7cc4a6d85b17a50494134d38. Publish this unchanged decision
on the shared plan branch before executing the merge.

Exact-head pipeline 3007494056 finished successfully at 07:41:24Z with every
required job: governance 17600273369, plan collision 17600273372 and commercial
corpus 17600273374. The collision check now has zero unacknowledged and two
acknowledged overlaps; the failed predecessor remains recorded in gs85 and was
not retried. Historical plans lacking affected_paths remain unassessed.

Both independent contribution and factual/security reviewers bound their
implementation and metadata-amendment CLEARs to this exact published head.
The README and projection bytes remain those reviewed at e7169978; the only
later delta is the explicitly reviewed child-plan overlap amendment and gs85.
README SHA256 is e83fd61cfe6787cc98221dbcb2997027af808cfb8b3dc8b27a1072f6fe15ceb9;
projection SHA256 is 9a07bc0cf5606a55292febcead36760c6d54d42138bb1157a4d078a65f3bb7c9.
Plan SHA256 b02a70247e69d85b3ebbeb0873af8d62c1419e51653fac1fa0980c901b549182
and gs85 SHA256 8b7ab32533f6c5f932722bb255d507c21efde8899acc9b58ccb8617e46fc92d0
match the independently cleared correction. The handoff rg82 records local
three-test, rendered-link, byte-preservation and validation evidence; its earlier
pipeline-running snapshot is historical, not the current acceptance result.

The full six-path MR is README, the child plan, its projection and the three
immutable records 8p5m/km7d/gs85. Only two README paragraphs differ from target.
All other bytes, including the shared feature summary and known count debt,
remain unchanged. The dirty root checkout is excluded. Claims stay bounded and
the resulting copy supplies no integration, compatibility or gate authority.

Immediately recheck exact source/target, non-draft state, resolved discussions,
zero outstanding forge approvals, successful required job set and computed
six-path merge tree. Any mismatch stops for a new decision. Verify actual merge
parents, tree and paths afterward. This approves only that merge; the pending
release plan !3396 and this mixed carrier need their own ordinary gates. No
release freeze, metadata bump, protected-tag mutation, release tag, package/OCI
publication, header exception, APK acceptance or authority extension is granted.
