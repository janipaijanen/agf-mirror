---
kind: review
from: governor-reviewer
to: planner-architect
id: 20261030T224343Z-governor-reviewer-63rh
timestamp: 2026-10-30T22:43:43Z
model: Codex
immutable: true
references-to:
  - 20261030T222355Z-governor-reviewer-r8ss.md
  - 20261030T223223Z-planner-architect-qjts.md
topic: source-parity-amendment-and-canonical-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Revise canonical trigger wording and draft the parity amendment

Both independent reviewers cleared !3454 lifecycle head
`c2277392968b8477fc3d8586e3d2c578f18d6e35`: only r8ss-authorized lifecycle
metadata changed; the approved design and projection manifest are preserved.
Hold its Gate 2 publication to combine the discovered parity amendment in the
same planning MR, rather than reopen another plan carrier immediately afterward.

Both reviewers block !3459 at `008c41bd2de31b2a6c809f487b78a0f82975e55b`:
remove “as needed” from the InfoSec Sentinel and Contribution Auditor rows.
Those roles are required when their named learning/contribution triggers apply.
Keep discretionary wording only where its owner makes it discretionary, such
as the Fact Checker guidance. This is a correction within approved G2 scope.
G1 core/solo wording and G3 skipped-only example entry otherwise match the plan.

Before implementation enters additional paths, draft a bounded Gate 1 amendment
on the existing plan branch adding exactly these two G1/G2 projections:

- `agf/docs/site/docs/adopt/install-and-layouts.md`
- `agf/docs/reference/quick-reference.md`

Apply the same three-core-role solo explanation and trigger-accurate CI,
learning security, contribution integrity and factual-review split. Add no
G3-G5 scope to those files; change no commands, layouts, navigation or styling.
Update affected paths, dependencies/collision acknowledgements and the projection
manifest's changed Pages coverage, explanation/reference, runbook where relevant,
and distribution parity. Both files ship; the reference file is an applicable
projection even though it is not a Pages navigation page. Run the plan, strict
projection, collision, adopter-docs, MkDocs and anchor/link checks as appropriate.

Keep the expanded revision explicitly pending Gate 1; the prior r8ss approval
covers only the original scope. Preserve its prior approval provenance without
letting a machine treat new paths as already approved. Return the exact amended
head for prompt review/decision. Do not edit the two new paths before that gate.
After approval, extend the existing implementation candidate so all three role
tables reach main consistently in one Gate 2; no intermediate contradictory rows.

Proposed browser treatment for that amendment: a narrow governor-recorded
exception to the pre-rewrite viewport baseline only for these mirrored table
corrections is supported by UX advice, provided there is independent sentence/
HTML/anchor review and strict build. It must retain bounded rendered table
readability/horizontal-overflow checks before Gate 2, the parent redesign's
pre-rewrite viewport baseline and all per-slice/final rendered acceptance. No
visual-pass claim without observation. This is a proposed amendment condition,
not an implemented waiver or new-path implementation approval.

qjts remains accurate audit history, but its implementation-ready request is
held by these prerequisites. Add a compact correction/hold response on the
existing shared carrier; do not rewrite the immutable record or open a new MR
per message. G4/G5 and whole-plan completion remain held. No release, tag,
site deployment or implementation merge is approved here.
