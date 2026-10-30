---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261030T073453Z-governor-reviewer-gs85
timestamp: 2026-10-30T07:34:53Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261030T072711Z-governor-reviewer-8p5m.md
  - 20261030T072858Z-coder-tester-km7d.md
topic: readme-child-collision-evidence-correction
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correct collision evidence and reapprove the unchanged README scope

Under current delegation ss2k, acknowledge the legitimate blocking failure in
MR !3397 pipeline 3007476950, job 17600154237. At tracked head e7169978, the
changed-plan check finds two unacknowledged README overlaps. The earlier local
scan inspected the plan corpus but did not include the untracked child in its
changed-only pairing. Therefore 8p5m's zero-assessed-overlap statement was
insufficient evidence. Preserve that immutable record; this is its append-only
correction, not a green retry or waiver of the collision gate.

Reapprove the exact two-paragraph scope of child plan
20261030T072047Z-v2131-readme-journey-followup with these explicit dependencies:

- 20260921T040743Z-assistant-rule-drift-resolution: preserve its loader, rule,
  count and history work. The child changes only its named setup/coexistence
  paragraphs, with no declaration that the older plan is complete.
- 20261027T052500Z-tooling-coexistence-discoverability: the older approved plan
  supplied the README paragraph and then-permitted “probably compatible” copy.
  Explicitly authorize replacing only that README projection with this child's
  independently reviewed proposed/Level 2/Theoretical wording, matching the
  already-approved canonical Pages correction. This is a narrow current
  documentation decision, not a claim of compatibility or a general override
  of the prior plan. No unrelated old scope is reopened, superseded or closed.

The README implementation at e71699781785b1f059402ff63e8bc1dabc57b58b and
projection remain byte-identical through this correction. Both independent
reviewers already cleared those exact bytes. This correction changes only the
child plan's dependency/evidence/approval metadata and this decision; existing
claim km7d remains the bounded allocation. The full amended candidate requires
fresh independent metadata review and a successful exact-head pipeline before
ordinary Gate 2. Publish this corrected plan and record before any further
product mutation or Gate 2 action. Do not retry the failed job: the corrected commit supplies the
next required acceptance run. No merge or release authority is granted here.

Operational learning: a changed-only check over an untracked new plan cannot
establish its overlaps. Stage or commit the plan and verify that it participates
in changed-plan pairing before publishing future Gate 1 evidence. No tooling
change is authorized by this lesson; the 204 historical no-affected_paths plans
remain unassessed. The dirty root checkout remains untouched, all other README
bytes remain preserved, and delegation still expires 18:00Z today.
