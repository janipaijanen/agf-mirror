---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260910T041405Z-planner-architect-asjp
timestamp: 2026-09-10T04:14:05Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260910T041359Z-planner-architect-et3p.md
topic: documentation-estate-steward-directive-ack-and-plan
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Ack: documentation-estate steward directive — execution plan

Acknowledging the 2026-09-10 governor directive to (1) build the new
governed documentation-estate capability collaboratively, (2) include it as
a v2.11 IN workstream once Gate 1 approved, and (3) require the new
capability's estate-wide documentation assessment before v2.11 release
content is committed. The improved restatement is in `et3p`.

## Sequenced plan

| # | Step | Owner | Gate |
|---|---|---|---|
| S1 | Capability shape assessment — rule/checklist vs standing agent (per learning `20260910T041223Z-qvzj` and the rule-vs-agent calibration) | capability-developer (`et3p`) | returns a proposal; no MANIFEST change |
| S2 | Gate 1 plan for the capability (rule text or agent definition per S1) | planner-architect (Claude) drafts; Codex/others review independently | governor Gate 1 |
| S3 | Implementation of the capability | coder-tester per S2 decomposition | governor Gate 2 |
| S4 | v2.11 parent-scope amendment: add the capability as an IN row + add "steward estate assessment complete and its findings dispositioned" as a release-cut precondition | planner-architect | **governor Gate 1 decision** (same shape as the `#129` withdrawal / `j8bq` amendments) |
| S5 | Fold the "estate assessment before release content is frozen" precondition into the v2.11 release-publication child plan (MR !2646) | planner-architect | its own review round |
| S6 | The new capability runs its first estate-wide documentation assessment; findings feed v2.11 release content | new capability | precedes freezing the metadata allowlist + CHANGELOG scope |

## Open points for the governor

1. **S4 is a v2.11 scope expansion.** It adds an IN workstream and a new
   release-cut precondition after `sh3m`/`j8bq`. Per gate discipline it
   needs an explicit governor scope-amendment decision, not a planner
   workaround — this note requests confirmation to proceed on that basis.
2. **Timing risk.** v2.11 already has several IN workstreams without Gate 1
   plans (coordination-semantics, `#123`/CP-C, `qbka`, CP-B-2). Adding a
   new capability + its first full estate assessment as hard preconditions
   moves the release further out. Confirm that is intended, or whether the
   estate assessment should gate the CHANGELOG/metadata freeze only (a
   lighter coupling) rather than being a full IN workstream.
3. **Working title** — "documentation-estate steward" vs the assistant's
   "Product Marketing Touchpoint Keeper". Capability-developer will
   propose; governor names.

Proceeding now with S1 (routed in `et3p`) and preparing S2 in parallel.
S4's amendment waits for the governor confirmation above.
