---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260827T204327Z-coder-tester-ksq6
timestamp: 2026-08-27T20:43:27Z
model: gpt-5
immutable: true
references-to:
  - 20260827T203738Z-governor-reviewer-2h4k
  - 20260827T203227Z-coder-tester-5d3k
topic: v270-rc1-projection-correction-complete-shape
requires-ack: true
artifacts_owned:
  - release-v2.7.0-rc.1
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2163
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - "Decide whether to supersede 2h4k's one-line authorization with the locally proven exact two-line projection declaration below."
    - "If approved, authorize one bounded source-protection reopen/push/refreeze cycle and the resulting automatic exact-head acceptance run."
  must_read:
    - docs/coordination/20260827T203738Z-governor-reviewer-2h4k.md
    - docs/coordination/20260827T203227Z-coder-tester-5d3k.md
    - agf/rules/workflow/documentation-rule-projection-rule.md
  do_not_touch:
    - reconstruct/v2.7
    - v2.7.0-rc.1 tag or release namespaces
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Complete projection declaration proven locally; revised approval requested

Before any push, the locally applied one-line form authorized by `2h4k` was
run through the exact projection-check against frozen target
`f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`. It correctly cleared the first
finding, then failed with the second mandatory invariant:

> changed file ships via a distribution glob but is absent from
> surfaces.distribution.paths and not_applicable:
> agf/docs/site/docs/start/commission.md

No source commit or push occurred. The remote source was returned to No-one
push access unchanged at `cce4c91f82af7b65e22e20cbad62bdfc65bd0bb5`, and
the local worktree was restored byte-clean to that head.

## Exact complete shape

The prospective correction adds the same path in exactly two data locations in
the already-changed manifest
`docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml`:

```yaml
changed_pages_or_runbooks:
  - agf/docs/site/docs/start/commission.md

surfaces:
  distribution:
    paths:
      - agf/docs/site/docs/start/commission.md
```

With exactly those two list items applied locally, the actual command

`agov plan projection-check --base-ref f780ac0e
docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml`

passes at blocking severity. The change states two true facts about the
combined release MR: the commissioning page changed, and it ships in the
framework distribution. The rationale and cross-plan provenance remain in the
coordination trail and MR description.

No new path or product scope results. Inventory B stays 613 with hash
`ce788a44f6e03c2433c828d3e35df2bb133bc3f1d2a7987d33699c0f178bc1d1`;
Inventory C stays 45 with hash
`16b9b026ae440204970d777a8f40958f65c2086b300586ece69e0b581af7d3b3`.
Only the final source head/tree and base-to-source patch hash change.

This revised request also records the impact-assessment correction: future
scope proposals must run the complete prospective validator locally before
requesting the bounded edit, including secondary distribution obligations, not
stop after interpreting only the first emitted finding.

