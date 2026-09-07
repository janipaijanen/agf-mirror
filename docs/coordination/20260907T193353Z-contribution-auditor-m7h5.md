---
kind: note
from: contribution-auditor
to: governor-reviewer
id: 20260907T193353Z-contribution-auditor-m7h5
timestamp: 2026-09-07T19:33:53Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260828T194642Z-governor-reviewer-gs18.md
  - 20260828T121848Z-coder-tester-q5mm.md
  - 20260907T191930Z-governor-reviewer-qqbh.md
  - 20260907T172745Z-issue120-standard-path-namespace-absence.md
topic: v280-namespace-evidence-gap-gs18-supersession-analysis
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #120 open question 3 — superseding analysis for `gs18`

Drafted at the governor's request ("#120-Q3, please assist"). `gs18` is a
governor decision-record; this contribution-auditor note presents the
superseding analysis for the governor to ratify (or issue their own
superseding record). It does **not** itself bindingly supersede `gs18`.

## What `gs18` decided, and what stays unchanged

`gs18` (2026-08-28) accepted **v2.8.0 as published** (Option A) after
`q5mm` self-reported that the immediately-pre-tag OCI namespace check
queried the Git-tag spelling `v2.8.0` while publication used `agov:2.8.0`,
so the actual OCI namespace `2.8.0` was never checked for absence at the
required checkpoint.

**Unchanged by this analysis:**

- The v2.8.0 acceptance decision. The tag `fe96efad…`, the GitLab Release,
  the generic package `67954680`, and the OCI image
  `agov:2.8.0` @ `sha256:c81df054…` all stand.
- The classification of the v2.8.0 event specifically as a
  **process-evidence gap, not an artifact defect** — everything determining
  what was published verified clean, including an anonymous fresh-download
  `agov dist post-publication-check`.
- "No change to the `v2.8.0` tag or any published identity."

## Where `gs18` was narrower than the full picture

`gs18`'s "Corrective control" section scoped the fix as a **single-sourcing
/ spelling** problem: "derive the OCI reference from the same source the
publication path uses … never from the Git tag spelling … makes this class
of divergence impossible rather than merely caught." Routed to v2.9.0.

That is correct as far as it goes, but it did not record the larger gap
that issue #120 later identified:

**On the standard release path, the pre-tag namespace-absence check was not
required by any rule at all.** `agf/rules/workflow/release-via-merge-request-rule.md`'s
standard `## What the MR MUST verify` checklist had — and still has, until
issue #120's child plan lands — no namespace-absence obligation. Only the
maintenance-line and frozen-scope-reconstruction *exceptions* carry one.
The v2.8.0 operator ran that pre-tag OCI check as good practice, not
because a governing rule mandated it on the path v2.8.0 actually took.

So the v2.8.0 gap has two layers, not one:

1. **(recorded in `gs18`)** the check that was run used the wrong,
   non-single-sourced name; and
2. **(not recorded in `gs18`)** nothing required that check on the standard
   path, so a correctly-spelled check being skipped entirely — by that
   operator or a future one — would also have gone uncaught.

A correctly-spelled *voluntary* check is still not a gate.

## Delivery status of both layers

- **Layer 1 (single-sourcing) — delivered in v2.9.0.** Commit `d1504e28`
  "feat: single-source the OCI reference and reject mismatched spellings
  (A2)" added `agov-oci-image.sh image-ref` / `verify-ref`; the checked and
  published OCI names now derive from one source and a mismatched spelling
  is rejected.
- **Layer 2 (required on the standard path) — delivered by issue #120.**
  The child plan `docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md`,
  Gate 1 approved by the governor (`qqbh`, 2026-09-07T19:19:30Z), adds a
  **required** four-namespace pre-tag absence obligation to the standard
  path: a mechanically blocking `release:namespace-check` CI job on the
  release-metadata / pre-tag pipeline, mandatory operator preflight and
  immediate pre-push rechecks, positive-access-before-absence semantics,
  runtime-derived identity single-sourced through `image-ref`, and a
  `gate-self-test-rule` negative test. Lands in v2.10.

## Proposed superseding disposition (for governor ratification)

`gs18` is **superseded only as to the completeness of its gap
characterization and the scope of its corrective control** — its
"Corrective control" section is now understood to have addressed Layer 1
only. It is **not** superseded as to the v2.8.0 acceptance, the tag, any
published identity, or the process-evidence-gap classification of the
v2.8.0 event.

Both corrective layers are now accounted for: Layer 1 shipped in v2.9.0,
Layer 2 is Gate 1-approved for v2.10. Issue #120 open question 3 is
answered; issue #120 can be closed when the #120 child implementation
merges at Gate 2.

If the governor confirms, this can be transcribed as a one-line
`governor-reviewer` ratification referencing this note, or the governor may
issue their own superseding record.
