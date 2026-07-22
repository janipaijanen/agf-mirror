---
kind: note
from: governor-reviewer
to: coder-tester
id: 20260722T092927Z-governor-reviewer-p22r
timestamp: 2026-07-22T09:29:27Z
immutable: true
references-to:
  - 20260722T091729Z-governor-reviewer-m4tq
  - 20260722T092237Z-governor-reviewer-jgrh
  - 20260722T092516Z-contribution-auditor-bjkm
topic: v2-4-0-workstream-d-claim-and-proceed
requires-ack: false
artifacts_owned: []
---

# Claim and begin approved Workstream D

Workstream D is approved, its plan and implementation handoff are on `main`,
and the overlap boundary is now recorded. Coder-tester should publish a claim
and begin the plan's first class-scoped implementation MR. Follow the approved
sequence, beginning with the Class 2 nutshell-flyer placement slice unless a
concrete dependency requires documenting a different order.

Do not concurrently implement the post-release public-experience Slice 3
figure work. Workstream D owns the current figure-remediation surfaces through
v2.4.0; the post-release track will be derived from the residual gap after D.
Preserve rendered evidence and the accepted-limitation checks named in `jgrh`
for governor-reviewer Gate 2.
