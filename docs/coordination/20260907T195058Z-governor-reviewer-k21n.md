---
kind: decision-record
from: governor-reviewer
to: all
id: 20260907T195058Z-governor-reviewer-k21n
timestamp: 2026-09-07T19:50:58Z
model: human
immutable: true
references-to:
  - 20260907T193353Z-contribution-auditor-m7h5.md
  - 20260828T194642Z-governor-reviewer-gs18.md
  - 20260907T191930Z-governor-reviewer-qqbh.md
topic: v280-namespace-evidence-gap-gs18-superseded
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `gs18` superseded as to gap scope — issue #120 open question 3 answered

Binding in-session governor directive (2026-09-07): **"m7h5 ok, no claims
here."** Transcribed to coordination. The governor **adopts the superseding
disposition proposed in `m7h5`**.

## What is superseded

`gs18`'s "Corrective control" section is now understood to have addressed
**Layer 1 only** (the pre-tag OCI check used the wrong, non-single-sourced
name). `gs18` did not record **Layer 2**: on the standard release path, the
pre-tag namespace-absence check was not required by any rule — the v2.8.0
operator ran it as good practice, not because a governing rule mandated it.

Both layers are now accounted for:

- **Layer 1** — single-sourcing — shipped in **v2.9.0** (A2, commit
  `d1504e28`: `agov-oci-image.sh image-ref` / `verify-ref`).
- **Layer 2** — required on the standard path — delivered by the **issue
  #120 child plan** (`docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md`),
  Gate 1 approved (`qqbh`), landing in **v2.10**: a required four-namespace
  pre-tag absence obligation with a blocking pre-tag CI check, mandatory
  operator preflight + immediate pre-push rechecks, positive-access-before-
  absence semantics, and a gate-self-test negative test.

## What is NOT superseded

The v2.8.0 acceptance decision (Option A), the tag `fe96efad…`, the GitLab
Release, the generic package `67954680`, the OCI image `agov:2.8.0` @
`sha256:c81df054…`, and the classification of the v2.8.0 event specifically
as a **process-evidence gap, not an artifact defect** — all stand
unchanged. No re-tag, no re-push, no artifact replacement.

## Disposition

Issue #120 open question 3 is **answered**. Issue #120 itself closes when
the #120 child implementation merges at Gate 2. "No claims here" — this is
a governance supersession record and makes no capability or product claim
requiring separate verification.
