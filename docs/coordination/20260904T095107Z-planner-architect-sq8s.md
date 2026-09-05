---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260904T095107Z-planner-architect-sq8s
timestamp: 2026-09-04T09:51:07Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260904T094956Z-contribution-auditor-6aq8.md
  - 20260903T221837Z-governor-reviewer-m3rp.md
topic: agf-development-starter-guideline-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/20260904T094827Z-agf-development-starter-guideline.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 1 for the AGF development starter guideline plan.
    - Confirm that the quick start remains a non-normative projection of AGENTS.md.
    - Confirm that implementation waits when it would delay the active v2.9.0 release lane.
  must_read:
    - docs/plans/20260904T094827Z-agf-development-starter-guideline.md
    - docs/coordination/20260904T094956Z-contribution-auditor-6aq8.md
    - AGENTS.md
  do_not_touch:
    - release branches, tags, or published releases
    - schemas, Rust code, CI, issues, or milestones
    - artifacts owned by the v2.9.0 work order
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 handoff: AGF development starter guideline

The user asked to improve the current polling/release-priority instruction and
file it as an AGF development starter guideline. The attached plan contains the
full proposed text and a bounded implementation path.

The draft converts the broad request into an executable loop: cold start,
remote-backed polling, verified release-first prioritization, evidence-bounded
agent redirection, authority stops, and a durable terminal disposition. It
also prevents the wording from becoming permission to merge, tag, publish,
expand scope, invent security severity, spam stale MRs, or interrupt required
review and incident work.

Contribution audit found one material design risk: a second normative owner
could drift from `AGENTS.md`. The plan resolves it by making the new document a
non-normative quick-start projection and keeping `AGENTS.md` authoritative.
The audit was same-session and is explicitly not presented as independent
corroboration.

Validation at handoff:

- `agov plan validate` — passed;
- `agov coord validate` on the audit — passed;
- `git diff --check` — passed.

No guideline implementation has begun. The plan is `pending-gate1`, and its
own priority section makes the active v2.9.0 release work order superior when
the two compete for the same planner capacity.

