---
kind: decision-record
from: governor-reviewer
to: all
id: 20260827T201324Z-governor-reviewer-6gg4
timestamp: 2026-08-27T20:13:24Z
model: human-directed-gpt-5
immutable: true
references-to:
  - 20260827T195959Z-coder-tester-krhk
  - 20260827T200617Z-coder-tester-h32h
  - 20260827T192914Z-governor-reviewer-hsj8
topic: v270-rc1-necessary-scope-correction-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding approval: exact two-path v2.7.0-rc.1 correction

This record transcribes the governor-reviewer's in-session decision,
"approved as proposed", given in direct response to the exact recommendation
and decision request published in `krhk`.

## Decision

The necessary scope-estimate correction proposed in `krhk` is approved exactly
as bounded:

1. temporarily reopen only source branch `release-v2.7.0-rc.1` for one
   Maintainer push; `reconstruct/v2.7` remains immutable;
2. replay exact merged MR !1686 source commit
   `e9312bf0db97ba5d91342bf802e756ca74cfe6dc` without modification, changing
   only `agf/docs/build/hooks/check_commissioning_links.py` and
   `agf/docs/site/docs/start/commission.md`;
3. regenerate Checkpoint-2 and inventory evidence, update MR !2163, and
   immediately return the source branch to No-one push access with
   Maintainer-only merge/unprotect and force-push disabled; and
4. treat the next automatically required exact-head MR pipeline as the
   corrected acceptance run, cancelling any demonstrably redundant push
   pipeline.

This is a necessary correction under the merged scope-estimate rule, not a
waiver of the mandatory green exact-head pipeline and not an expansion of the
four-issue product scope. The existing Checkpoint-1 SHA and metadata commit
remain unchanged; the final source head/tree, patch hashes, and Section B/C
proofs must be regenerated and re-presented.

After stabilization, record the proportional learning stated in `krhk` about
including post-base consumer-check repairs in future release impact analysis.

No merge, tag, release publication, package/OCI publication, or stable v2.7.0
promotion is authorized by this correction decision. Existing Gate 2 and RC
publication gates remain in force.
