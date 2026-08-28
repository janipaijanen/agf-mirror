---
kind: decision-record
from: governor-reviewer
to: all
id: 20260827T203738Z-governor-reviewer-2h4k
timestamp: 2026-08-27T20:37:38Z
model: human-directed-gpt-5
immutable: true
references-to:
  - 20260827T203227Z-coder-tester-5d3k
  - 20260827T203232Z-coder-tester-58mk
  - 20260827T201324Z-governor-reviewer-6gg4
topic: v270-rc1-projection-correction-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding approval: one-line RC projection declaration

This record transcribes the governor-reviewer's in-session response, "good
approach. approved all proposed", to the exact second correction request in
`5d3k` and the accompanying planner-check route in `58mk`.

The exact one-line addition of
`agf/docs/site/docs/start/commission.md` to `changed_pages_or_runbooks` in
`docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml` is
approved as the smallest truthful correction to the combined release MR.

The coder-tester is authorized to:

1. temporarily reopen only `release-v2.7.0-rc.1` for one Maintainer push;
2. add exactly that one YAML list item in a separate commit;
3. run projection-check locally against frozen target `reconstruct/v2.7`,
   regenerate final source/tree/patch evidence, push once, and immediately
   refreeze the source; and
4. use the resulting automatic exact-head MR pipeline as the corrected
   acceptance run, cancelling any demonstrably redundant push pipeline.

No new path or product scope is authorized. Inventory B/C membership and
hashes remain unchanged. No merge, tag, release/package/OCI publication, or
stable v2.7.0 promotion is authorized by this decision.

