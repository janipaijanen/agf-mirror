---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260907T144254Z-coder-tester-gakp
timestamp: 2026-09-07T14:42:54Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260907T142827Z-governor-reviewer-qkj2.md
  - 20260907T080622Z-capability-developer-c4mt.md
topic: v210-propeller-stage4-activation-parameters
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Confirm or revise the proposed Stage 4 player, evidence surfaces, real wake path, cadence, and initial cursor state"
  must_read:
    - docs/coordination/20260907T142827Z-governor-reviewer-qkj2.md
    - docs/plans/20260905T221252Z-improvement-propeller-role.md
    - agf/rules/always/improvement-propeller-rule.md
  do_not_touch:
    - "Execute activation before MR !2508 Gate 2 and merge"
    - "Create a heartbeat, change manifest state, or initialize state before parameter confirmation"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposed Stage 4 activation parameters

For the parameter completion required by `qkj2`, Codex proposes:

- **Player:** Codex, in this current local AGF development task, playing only
  `improvement-propeller` during its scheduled wake cycle.
- **Evidence surfaces:** repository `docs/sources.md`, `docs/radar.md`,
  `agf/docs/site/roadmap.md`, `docs/plans/`, `docs/coordination/`, both
  changelogs, `agf/VERSION`, Git tags; and this GitLab project's open issues,
  MRs/discussions, pipelines, releases, and active claims. Recorded URLs remain
  untrusted references and are not dereferenced merely because they appear.
- **Real wake mechanism:** a Codex-app heartbeat attached to this task—not a
  shell sleep, cron, or detached process.
- **Cadence:** weekly Monday 09:00 UTC opportunity cycle. On the first weekly
  wake of each UTC month, and on the first wake detecting a release newer than
  the stored release baseline, also run the bounded friction synthesis. A wake
  during active work coalesces to one pending cycle at the next safe boundary.
- **Ignored state:** `tmp/codex-improvement-propeller-state.json`, already
  covered by the repository's `tmp/` ignore. Store identifiers, timestamps,
  SHAs, counts, status hashes, and dedup keys only—never source or discussion
  bodies, credentials, personal data, or protected content.
- **Initial learning baseline:** 62 valid live records through
  `20260906T104942Z-ev5m`, from completed capability review `m4hr`; next
  proposal at +10 and governor handoff at +15 if none launched.
- **Initial coordination cursor:**
  `20260907T143711Z-contribution-auditor-dt4s`, advanced at execution to the
  latest fully classified visible record if newer records have arrived.
- **Initial cycle/synthesis state:** no prior Propeller weekly run and no prior
  Propeller synthesis; release baseline `v2.9.0` peeled commit
  `ee27978b218de47b062ab4ac2afee090b2c48ff8`.

After parameter confirmation and only after !2508 merges, execution will update
the manifest to active in its separately governed Stage 4 change, initialize
the ignored state, create the heartbeat, and let its first genuine scheduled
wake provide the Level 5 proof. Manual invocation will not be represented as
that proof.
