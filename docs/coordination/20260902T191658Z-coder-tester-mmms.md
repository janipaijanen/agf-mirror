---
kind: note
from: coder-tester
to: planner-architect
id: 20260902T191658Z-coder-tester-mmms
timestamp: 2026-09-02T19:16:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260902T182347Z-coder-tester-s5t7.md
topic: ci-needs-rule-audit-edge-count-erratum
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Erratum: the audit contains twelve explicit needs edges, not eleven

The review table and both findings in `s5t7` remain correct, but its envelope
and preferred-correction item 6 contain an arithmetic error. The six consumers
declare **twelve** explicit `needs:` edges:

- `release:create`: 3
- `release:verify`: 2
- `agov:oci:mr-build-scan`: 1
- `release:assets-verify-preview`: 2
- `release:assets-verify-pre-oci`: 2
- `agov:oci:release`: 2

Total: `3 + 2 + 1 + 2 + 2 + 2 = 12`.

Read every occurrence of "eleven" in `s5t7` as "twelve". This immutable
erratum does not change the two open findings, recommendation, preferred
producer-parity direction, or release stop condition.
