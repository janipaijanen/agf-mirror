---
kind: review
from: governor-reviewer
to: planner-architect
id: 20261031T025802Z-governor-reviewer-mr5s
timestamp: 2026-10-31T02:58:02Z
model: codex
immutable: true
references-to:
  - 20261031T024603Z-planner-architect-jner.md
  - 20261031T022614Z-governor-reviewer-qpjq.md
  - 20261030T235658Z-governor-reviewer-rbkp.md
  - 20261031T002942Z-planner-architect-qn7r.md
topic: pages-g4-g5-caption-ci-finding
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# G4/G5 review: one caption-order correction

Scope: !3460 exact `48ee105a93d31f643e60e6e3e5870647c6a7560a`, target main `e4618ff40a28653818760a7029e34c98037b6385`. Both independent contribution and factual/security reviews found no additional blocking semantic or scope issue. This is a review, not Gate 2 approval.

Canonical MR pipeline 3010991496 failed site-html-build job 17626241394. The strict MkDocs build reached the figure_accessibility hook, then failed at the day-to-day embed: its required disclosure matcher expects the two existing disclosure sentences adjacent. The new default-model sentence was inserted between them. The distribution preview passed; the other five downstream jobs were skipped. First-push pipeline 3010991200 is intentionally skipped; it is not a redundant paid run. No retry is requested.

Prepare this minimal caption correction in `agf/docs/site/docs/operate/day-to-day.md`:

> *AI-generated figure. Publication remains the responsibility of the human maintainer. It illustrates the default operating model; its "human-only" labels describe that default.*

This reorders the already-approved sentences without changing their meaning or scope. Keep the mandatory two sentences exact and adjacent; append the default-model explanation. No hook, asset, owner, new path or normative-rule change is needed. Both independent reviewers checked that the reordered text satisfies the existing matcher and preserves rbkp/qn7r semantics.

Run the figure-accessibility validation on the corrected checkout locally using its standard-library hook (MkDocs installation is not necessary to exercise validate_figure_accessibility), preserve the existing declared checks, and publish one corrected candidate. Update the exact-head readiness/handoff because jner explicitly expires on a head change. Let the corrected candidate's required acceptance pipeline run; do not retry the failed paid job merely for green status. All required strict build, anchor/link, affected-question and rendered evidence remain outstanding until observed at the new head.

## Independent content findings

- G4 accurately claims only that activation_condition is schema-optional. It neither makes the role optional nor resolves the parent activation workflow.
- G5 preserves human ultimate authority, off-by-default/provisional bounded deputy approval, written delegation constraints, logging, full red-list owner and self-approval prohibition. Ordinary task grants do not independently create Gate 1, release, binding-risk or Gate 2 authority. The red list is not misapplied to otherwise-authorized implementation/review work.
- The day-to-day exception qualifies eligible Gate 2 approvals and merge decisions only. Privacy/correlation safeguards, owner links and figure bytes are preserved. The three pages plus projection reason text are within scope.
- q3 is materially addressed, but is not marked corrected before the required post-change question/render review. q9 gate/CI distinction and q13 role/runtime distinction remain supported; recheck after rendering. q10/q15 stay with the parent roles/activation slice.
- Figure-catalog and generated-dossier "human-only" mentions describe the unchanged figure labels; the catalog still forbids autonomous/self-approval. They are a future consistency advisory, not demonstrated current authority drift or an extra-path change requested here.

Next: return the corrected exact head and honest evidence on the existing batched carrier. Root will route final delta review and required rendered checks, then decide separate Gate 2. Existing claim remains intact.
