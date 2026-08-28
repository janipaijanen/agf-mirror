---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T194642Z-governor-reviewer-gs18
timestamp: 2026-08-28T19:46:42Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260828T121848Z-coder-tester-q5mm.md
  - 20260828T110944Z-governor-reviewer-prn4.md
topic: v280-accepted-with-recorded-namespace-evidence-gap
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.8.0 accepted as published, with the namespace evidence gap recorded

This record transcribes the governor-reviewer's decision to proceed to
v2.9.0, which presupposes accepting v2.8.0 as published. It answers the
blocked disposition request `q5mm`. **Option A is selected.**

If the governor intended option B (classify the attempt failed and direct
a v2.8.1 cycle), a superseding record corrects this; nothing here is
irreversible, and the tag is untouched either way.

## Decision: v2.8.0 stands as published

The defect `q5mm` self-reported is real and correctly escalated: the
mandatory immediately-pre-tag namespace check queried the OCI tag as
`v2.8.0`, while the publication script's `image_ref()` derives from
`AGOV_VERSION` and publishes `agov:2.8.0`. The actual namespace was
therefore never checked for absence at the required checkpoint, and
because OCI tags are mutable, a successful push is not retrospective
proof of prior absence. That reasoning is accepted in full.

It is accepted as a **process-evidence gap, not an artifact defect**.
Everything determining what was published verifies clean and
independently:

- MR !2181 merged as `a33ff510223580d05873617afddd097a55c1009d`;
- annotated tag `fe96efad946d50ac5cbb4bf30febfd784ece27c9` peels to that
  merge commit;
- tag pipeline `2799546936` passed every job, with Trivy 0.66 reporting
  zero vulnerabilities;
- GitLab Release and generic package `67954680` bind to that pipeline;
- OCI tag `agov:2.8.0` published at 2026-08-28T11:56:04.625Z, digest
  `sha256:c81df054006f448fbb5f48ff573f54bc7b923e92eb21632a69455f40a381139e`;
- an independent **anonymous fresh-download**
  `agov dist post-publication-check` passed, including published OCI
  digest evidence and packaged-binary self-report at `a33ff510`.

Supporting, and explicitly **evidence rather than proof**: the registry
tag listing read earlier the same day ran `2.3.0` through `2.6.1` with no
`2.7.x` or `2.8.x` entry. That makes prior occupancy of `agov:2.8.0`
implausible; it does not reconstruct the point-in-time check, and is not
recorded as if it did.

Re-cutting a version to re-prove content already verified by anonymous
download would cost a version number and produce no new assurance about
the artifact, while leaving the spelling defect that caused the gap
unfixed in either case.

## Corrective control — required, routed to v2.9.0

The pre-tag namespace check must derive the OCI reference from the same
source the publication path uses — `image_ref()` / `AGOV_VERSION` — never
from the Git tag spelling. Deriving the checked name and the published
name from one source makes this class of divergence impossible rather
than merely caught.

This is a v2.9.0 work item, not a v2.8.0 remediation. `v2.8.0` and every
published identity remain untouched.

## Coder-tester is unblocked

Proceed with the remaining closure steps `q5mm` stopped before: the
fresh-clone checks at the exact `v2.8.0` tag, including the new Claude
Code session `/context` check that closes `7bkm`, then file the
completion record. Record the accepted gap in that completion so the
release's evidence trail states it plainly rather than reading as if the
check had passed.

Also record the reconstruction learning `hgte` proposed, now that the
release obligation is stable.

## Not authorized

No change to the `v2.8.0` tag or any published identity. No re-push, no
re-tag, no artifact replacement. A future defect in v2.8.0 is corrected
by v2.8.1 under the ordinary workflow.
