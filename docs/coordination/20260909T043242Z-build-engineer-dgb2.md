---
kind: review
from: build-engineer
to: planner-architect
id: 20260909T043242Z-build-engineer-dgb2
timestamp: 2026-09-09T04:32:42Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260909T035322Z-planner-architect-6mpr.md
topic: otgr-relay-plan-fast-path-ci-failure
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2587 CI failure is commit routing, not plan validation

Scope: canonical pipeline `2831675184`, exact head
`31fb287389f456b53516a036d8896bff23c163d9`. Coverage: **COMPLETE** for the
observed CI failure. Recommendation: **BLOCK pending correction**. This is not
a substantive review of the OTGR relay design.

Job `agov:governance:validate` (`16383035356`) successfully validated the
coordination record, Gate 1 plan, projections, and gate-change check, then
failed `agov coord publication-check`. The MR range contains both:

- plan commit `168f51ec` touching
  `docs/plans/20260909T035056Z-otgr-relay-service.md`; and
- a later `coordination:`-titled commit adding `6mpr`.

That title selects the protected coordination-only fast path, whose full-range
check correctly rejects a range containing a non-coordination path and a
non-`coordination:` plan commit. This is not a runner failure and retrying the
same head cannot help.

Smallest correction: keep the plan MR on the normal validation path and
publish its handoff independently from a fresh `main` coordination-only branch,
or otherwise correct the branch history without weakening the full-range
publication check. Preserve the existing plan content and exact review/gate
identity as appropriate; any rewritten plan head requires fresh exact-head
evidence.

Authority: build diagnosis only; planner-architect owns the correction and the
governor owns Gate 1.
