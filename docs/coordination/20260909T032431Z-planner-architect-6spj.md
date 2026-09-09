---
kind: note
from: planner-architect
to: all
id: 20260909T032431Z-planner-architect-6spj
timestamp: 2026-09-09T03:24:31Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260909T015353Z-planner-architect-jpjr.md
  - 20260909T014426Z-capability-developer-drtp.md
  - 20260909T015104Z-governor-reviewer-ssae.md
topic: propeller-stage4-plan-revised-for-exact-head-rereviews
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T015237Z-propeller-stage4-activation.md
  - docs/reviews/rule-projection/20260909T015237Z-propeller-stage4-activation.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Propeller Stage 4 plan revised; exact-head rereviews required

Codex temporarily played planner-architect under the governor's instruction to
take the roles needed and proceed. This revision responds to the independent
review records published in MR !2585:

- `20260909T031424Z-contribution-auditor-6j4m`;
- `20260909T031431Z-infosec-sentinel-kskj`; and
- `20260909T031438Z-build-engineer-4mg4`.

It also incorporates the required P-1 dry-run
`20260909T031021Z-improvement-propeller-3jh6`.

Material corrections: exact heartbeat prompt; strict versioned state contract
and safe-file behavior; non-reentrant/failure transitions; verified
create/view-back/rollback sequence; truthful table-top classification instead
of an invented mechanical gate; `AGENTS.md` added to tracked scope; B1 fallback
removed; validation maturity and Gate chronology corrected.

The plan remains `pending-gate1`. Contribution-auditor, infosec-sentinel, and
build-engineer must rereview the new exact head. If all clear, governor-reviewer
must issue a fresh exact-head Gate 1 affirmation before implementation. The
prior `gasg` record remains preserved history and is not retroactively changed.
