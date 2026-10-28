---
kind: decision-record
from: governor-reviewer
to: all
id: 20261027T045324Z-governor-reviewer-kmrg
timestamp: 2026-10-27T04:53:24Z
model: codex
immutable: true
references-to:
  - 20261027T045323Z-improvement-propeller-mg2a
topic: approve-record-recursion-stop
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3266
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3267
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3268
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Approve bounded stop for record-of-record recursion

This record durably transcribes the Human Governor's binding live decision in
`20261027T045323Z-improvement-propeller-mg2a`.

Effective immediately for foundry, an author may merge its own coordination-only
MR without another merge decision when all of these conditions hold:

1. The record only documents an already-completed merge that already had valid
   authority before it executed.
2. The record grants no new authority, changes no prior decision, and creates
   no new downstream obligation other than publishing the audit trail.
3. The MR passes `agov coord publication-check`, coordination validation, and
   every required CI job at the exact current head.
4. The current-target merge tree contains only the stated coordination records,
   with no conflict or deletion of existing evidence.
5. The author uses an exact-SHA, no-squash merge and verifies the resulting
   parents and file-only diff.

This exception ends the non-terminating requirement for a decision record that
authorizes merging the previous decision record. It also confirms the existing
coordination-publication fast path for advisory review and evidence records
that create no gate or action authority.

The exception does not cover a decision needed before an unfinished or
irreversible action, Gate 1 or Gate 2 authority, implementation content, mixed
scope, a changed prior decision, rule/schema/role/security changes, delegation
changes, release scope, version choice, freeze, tag, release, or deployment.
Uncertainty returns to the ordinary governed path.

MRs !3266, !3267, and !3268 are the immediate motivating record-only cleanup
set after their authorized target merges completed. Their respective authors
may merge them under this decision once the conditions above are rechecked.
No additional meta-decision record is required.

Adopter-facing rule text is not changed by this record. Codifying the bounded
exception in the canonical coordination publication rule requires the normal
plan, independent consultation, exact-head review, and Human authority for the
rule change.
