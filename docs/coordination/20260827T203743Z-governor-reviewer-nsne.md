---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260827T203743Z-governor-reviewer-nsne
timestamp: 2026-08-27T20:37:43Z
model: human-directed-gpt-5
immutable: true
references-to:
  - 20260827T190635Z-planner-architect-qsp1
topic: release-roadmap-direction-approved-for-planning
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Roadmap direction approved; prepare governed implementation plan

The governor-reviewer's in-session response, "good approach. approved all
proposed", also accepts the roadmap direction presented from `qsp1` and the
coder-tester's proportionality refinement:

- create `docs/roadmap.md` as the single governed internal roadmap;
- retain `agf/docs/site/roadmap.md` as the adopter-facing projection and
  GitLab milestones as the mechanical projection;
- preserve the sources → radar → issue → roadmap → release distinction;
- use the proposed v2.9 tracker/release-cost, v2.10 independence/enforcement,
  v2.11 adopter-safety, and v2.12 commercial/compliance clustering as the
  planning baseline, while keeping research/radar items non-committal;
- verify stale issue claims before closure, particularly #32/#37 as shipped
  and #36 as only partial; and
- design a bounded governed maintenance path for routine roadmap adjustments
  instead of requiring a fresh full Gate 1 cycle for every non-material edit.

This is a binding direction and planning authorization, not retroactive Gate 1
approval for unplanned file, milestone, label, or issue-state mutations. The
planner-architect must prepare the proportionate Gate 1 plan, including the
maintenance-path authority boundary and projection validation, after the
v2.7.0-rc.1 release obligation is stable. Do not alter the RC branches or
release scope for this roadmap work.
