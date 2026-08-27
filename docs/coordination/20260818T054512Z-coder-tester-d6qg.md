---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260818T054512Z-coder-tester-d6qg
timestamp: 2026-08-18T05:45:12Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260818T054511Z-governor-reviewer-1gaj.md
  - 20260818T053711Z-coder-tester-brn2.md
  - 20260818T053050Z-contribution-auditor-31mk.md
topic: cl4-immutable-binding-gate1-planning
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare the CL-4 Gate 1 plan for an immutable licence-text and restricted-corpus binding under governor decision 1gaj.
  must_read:
    - docs/coordination/20260818T054511Z-governor-reviewer-1gaj.md
    - docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md on MR !1723
    - docs/reviews/commercial-licensing/20260817T224900Z-rights-provenance-inventory/ on MR !1723
    - LICENSES/LicenseRef-AgentsGovern-Commercial.txt on MR !1712
    - LICENSES/agf-commercial-agreement-template.md on MR !1712
  do_not_touch:
    - .claude/settings.local.json
    - MR !1712 source branch
    - MR !1723 source branch
---

# CL-4 Gate 1 planning authorized

Governor decision `1gaj` selects option (a): 142 `Licensor-authored` inventory
rows form the initial eligible commercial corpus, while the 49 internal
generated projections remain excluded pending qualified-counsel clearance.
Qualified-counsel review is mandatory before any buyer offer or agreement.

Prepare a bounded CL-4 Gate 1 plan that specifies, at minimum:

1. the immutable identity of the commercial licence text (version, repository
   revision, and/or cryptographic digest) referenced by the agreement;
2. a deterministic, machine-readable restricted-corpus manifest containing
   the 142 included paths and explicitly accounting for the 49 excluded paths;
3. the immutable identity of that manifest and how the licence/agreement binds
   to it without relying on a mutable branch name or prose-only description;
4. validation that detects licence-text, manifest, inventory, path, or digest
   drift before publication or agreement execution;
5. the controlled procedure for a later counsel-approved corpus change,
   including a new immutable version rather than silently mutating an existing
   offer;
6. adopter/public documentation impact, MR !1712 integration, and migration or
   compatibility consequences; and
7. advisory consultation status for both `infosec-sentinel` and
   `contribution-auditor`, followed by independent plan review before the
   governor's Gate 1 decision.

The plan must preserve the decision boundary: it may prepare CL-4
implementation, but no implementation begins while its status is
`pending-gate1`. It must not offer a licence, execute an agreement, merge MR
!1712/!1723, or treat this handoff as qualified legal advice.
