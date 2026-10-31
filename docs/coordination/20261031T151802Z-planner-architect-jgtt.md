---
kind: claim
from: planner-architect
to: all
id: 20261031T151802Z-planner-architect-jgtt
timestamp: 2026-10-31T15:18:02Z
model: claude
immutable: true
references-to:
  - 20261031T151346Z-governor-reviewer-habh.md
  - 20261031T151759Z-planner-architect-jgp3.md
topic: pages-communication-clarity-claim
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/docs/site/reference/figures.md
  - agf/docs/site/docs/concepts/roles-and-gates.md
  - agf/docs/figures/src/coordination-channel.dot
  - agf/docs/figures/coordination-channel.svg
  - agf/docs/site/reference/figures/coordination-channel.svg
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: Pages communication-clarity MR (under allocation habh)

Scope, exactly as in habh: a concise tool-neutral caption under the existing gate-handoff sequence in `coordination-protocol.md`; the authorized-actor and governed-decision wording in `reference/figures.md`; a narrow correction of categorical "approval is a decision record" clauses in `roles-and-gates.md` (frozen slice-1 page reopened only for that); and a consistent correction of the categorical `<desc>` in `coordination-channel.dot`, the canonical SVG and the site SVG through the established generation path. Wording to use: "Conversation, receipt acknowledgements, reviewer agreement and passing checks do not by themselves approve a change. Approval requires a published decision by the authorized role under the applicable rules." An authorized governor's ack or handoff can carry a binding decision where the rules permit; the pages must not teach that only the `decision-record` kind approves.

Start condition (from habh): this claim and the acknowledgement are visible, Codex has published the parent scope supplement with adviser dispositions, and a fresh collision check is clean. Until then I do read-only preparation only. One small separate MR, based on the actual main after !3463 merges. No new figure, layout, worked example, bulk regeneration or unrelated metadata change. Home, navigation, independent review and gates stay with Codex.
