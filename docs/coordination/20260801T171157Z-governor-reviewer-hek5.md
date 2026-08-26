---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260801T171157Z-governor-reviewer-hek5
timestamp: 2026-08-01T17:11:57Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260801T160302Z-governor-reviewer-rekr.md
  - 20260801T161522Z-planner-architect-kk3r.md
  - 20260801T161803Z-contribution-auditor-p7qe.md
topic: v2411-release-publication-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: v2.4.11 release publication

The human governor-reviewer approved Gate 1 for MR !903 at exact head
`f6743bf6e8d431a928844f659b3f9ef139295d70`.

The approved plan is `docs/plans/v2411-release-publication.md`. Its bounded
scope is the patch-level publication of the already-merged issue #97 / MR
!890 documentation corrections and MR !894 AI-assistance,
figure-accessibility, and transparency work as candidate v2.4.11.

Both non-blocking advisories from the contribution-auditor review in MR !905
are mandatory Gate 2 acceptance checks:

1. The release changelog must acknowledge that an adopter who reuses the
   supplied MkDocs configuration inherits the figure-accessibility check over
   the adopter's own content.
2. Release validation must check that `docs/academic/` or other relocated
   content references are not reintroduced by the version-surface edits.

Source branches for the plan, review trail, release implementation, and Gate 2
trail must be retained. The Four-Way Test consultation in MR !901 and the
v2.5.0 contract-core scope remain excluded.

Coder-tester is authorized to implement the approved plan on a release feature
branch and publish a Gate 2 handoff. This decision does not authorize merging
the release implementation, tagging, publishing, or closing issue #97; those
actions remain gated by exact-head Gate 2 approval and the plan's
post-publication verification sequence.
