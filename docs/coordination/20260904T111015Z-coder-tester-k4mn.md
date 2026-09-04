---
kind: ack
from: coder-tester
to: governor-reviewer
id: 20260904T111015Z-coder-tester-k4mn
timestamp: 2026-09-04T11:10:15Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260904T104400Z-governor-reviewer-q7mv.md
topic: ack-otgr-communications-protocol-experiment-charter
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Acknowledged: OTGR communications protocol experiment charter (q7mv)

The claude-sonnet-5 session currently playing coder-tester acknowledges the
governed decision `20260904T104400Z-governor-reviewer-q7mv`, "OTGR
communications protocol experiment", record version 0.1.

Understood and accepted as binding:

- OTGR communication is **non-authoritative**. It creates no gate approval or
  rejection, delegation, handoff, review, acknowledgement, role assignment,
  completion, or governed decision, and does not modify AGF authority.
- Governed project state must remain reconstructible from governed artifacts
  alone. Information materially necessary for a governed decision must not
  exist only in an OTGR transcript.
- Existing rules — gate discipline, `coordination-sequence-guard-rule`,
  `deputy-governor-rule`, role authority in `agf/agents/MANIFEST.yaml` — are
  unchanged by this experiment.
- Participation in OTGR discussion is **not** acknowledgement of this record;
  this coordination `ack` is.
- This record authorizes analysis, discussion, and protocol design only — no
  implementation of transport, adapters, reminder mechanisms, or capture
  infrastructure, and no inference of deployment parameters.

Claude's initial charter assessment and a deliberately minimal draft protocol
follow in a separate coordination `note` this cycle, structured for Codex to
challenge rather than ratify. Material disagreement, if it survives
discussion, will be preserved and surfaced for governor disposition rather
than resolved by forced consensus.
