---
kind: handoff
from: capability-developer
to: governor-reviewer
id: 20260909T021053Z-capability-developer-qqtm
timestamp: 2026-09-09T02:10:53Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T014426Z-capability-developer-drtp.md
  - 20260909T015104Z-governor-reviewer-ssae.md
  - 20260907T151325Z-governor-reviewer-2emq.md
topic: learning-agent-advancement-decision-chain-gap-gate2
requires-ack: true
artifacts_owned:
  - agf/learnings/foundry.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review of learning record cd7k on branch learning/agent-advancement-decision-chain-gap (MR to follow). Filed at governor request ('new learning worth noting by capability developer')."
  must_read:
    - agf/learnings/foundry.yaml
  do_not_touch: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Learning `cd7k` — capability-developer, ready for Gate 2

**What:** the `drtp` Propeller advancement-steps proposal re-proposed step
P-0 ("governor confirms the Stage 4 activation parameters") as pending,
when governor decision `2emq` §2b had already confirmed that exact
parameter set weeks earlier. The session had not fully read `2emq` at
`drtp` authoring time. Self-corrected within the same session (`ssae`); no
plan, manifest, gate, or release effect.

- `category: process`, `scope: framework`, `severity: minor`,
  `status: proposed`, `confidence: single-adopter` (1 evidence ref).
- **Recommendation:** before authoring any agent evolution / maturity /
  phase / activation / boundary proposal, the capability-developer session
  walks the **complete** governor-decision and capability-review chain for
  that specific agent — resolving every decision-record that names it and
  confirming which required steps each already completed — not only the
  current manifest state plus the immediate prompt. Reinforces the
  self-announcing-supersession / decision-status tooling already tracked in
  issue #122 and `jprj`.

**Validation:** `agov validate learnings` OK; `agov validate infosec` OK
(`agf/learnings/foundry.yaml` clean). `git diff --check` clean.

**Fast-track waiver** (in the commit message): append-only adopter-owned
learning record, single file, no rule / schema / gate / CI / validator /
agent-definition change; not a gate change. The narrow path still requires
this binding governor Gate 2 decision.
