---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261030T232641Z-governor-reviewer-p134
timestamp: 2026-10-30T23:26:41Z
model: Codex
immutable: true
references-to:
  - 20261030T202840Z-governor-reviewer-pjh4.md
  - 20261030T222355Z-governor-reviewer-r8ss.md
  - 20261030T231007Z-governor-reviewer-8p8k.md
  - 20261030T231732Z-planner-architect-hhqh.md
topic: source-plans-publication-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: publish the approved source plan and parity amendment

Under full human governor delegation pjh4, approve only !3454 exact
`794ccdc80654688361bf76be18baa5043a0e819d`, without squash, into main at
`https://gitlab.com/janipaijanen/agents-govern-framework.git`.
Required target: `33ba5655262d5fd83f19bfe460cea876da415dc5`.
Required resulting tree: `be761521dbcdcc51fbfa457d38fe4a2cd50bf675`.
Exact pipeline 3010656905 passed all mandatory jobs: 17624111610,
17624111609 and 17624111608. Publish this unchanged decision on approved
shared ref `origin/coord/source-parity-feedback` before the action.

Both independent reviewers cleared this exact head. The original source plan
and its manifest preserve the r8ss design. The parity lifecycle delta from
reviewed f4ad46e2 matches 8p8k: approval metadata, all consultations consulted,
resolved contribution/factual cells, the old whole-table checkpoint, the narrow
baseline exception and post-approval rollback. No unreviewed product scope is
carried. Claude authored and delivered this exact candidate in hhqh; author
agreement is not substituted for the independent reviews.

Diff to target adds only the two approved plans and their two projection
manifests. No adopter source, live manifest, tooling, rules, schema, public site
or release artifact changes. A stale “proposed” heading in the approved parity
plan is minor advisory cleanup: metadata and explicit lifecycle body correctly
name 8p8k. Correct it with a later necessary plan edit rather than spend a
separate acceptance run on the heading. Do not alter the approved design under
that cleanup.

Re-fetch target immediately before merge. Recheck exact SHA, passed pipeline
and job identities, resolved discussions, approvals and any new notes. Removing
the Draft prefix is authorized metadata preparation; it must not move the SHA.
Require mergeable and the bound tree, merge the exact SHA without squash, then
verify target/source parent order, tree and four-file diff. Any target advance,
source change, new material finding or uncertainty stops this action. Complete
within pjh4's expiry. Monitor ordinary main CI; if Pages unexpectedly runs,
verify its acceptance before dependent publication. Record objective completion.

This completes plan publication only. !3459 remains held until every required
job passes and the mandated independent rendered desktop/mobile table checks
are actually observed. Successful site HTML and static semantic evidence do not
replace the rendered check. Current host resource pressure is not acceptance.
No G4/G5 completion, whole-plan completion, release/version, tag or manual
site deployment is approved here.
