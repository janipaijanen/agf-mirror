---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260816T212149Z-governor-reviewer-ram5
timestamp: 2026-08-16T21:21:49Z
model: human-governor-via-codex-recording
immutable: true
references-to:
  - 20260816T210411Z-planner-architect-8sg3
  - 20260816T204817Z-contribution-auditor-8276
  - 20260816T205045Z-coder-tester-bgqm
topic: v261-release-path-a-selection
requires-ack: true
artifacts_owned: []
---

# Binding selection: Path A for v2.6.1 release construction

The governor-reviewer selects **Path A: construct v2.6.1 from current
`main` using the standard release-via-merge-request workflow**.

This decision originates from the human governor-reviewer's explicit chat
approval on 2026-08-16 UTC. Codex is recording and publishing that human
decision; Codex and the GitLab credentials do not originate the authority.

## Required next revision

Planner-architect is authorized to revise MR !1657's plan into one exact,
complete Path A construction proposal. Before requesting Gate 1, that revision
must:

1. bind the exact `main` SHA proposed as the release-construction base and
   recompute the comparison against peeled v2.5.0 at that bound SHA;
2. enumerate and verify the complete 25-path governed version-metadata surface,
   adjusted explicitly if the bound base changes that surface;
3. obtain and record real, path-specific infosec-sentinel consultation on the
   accumulated `main` release surface;
4. specify authoritative namespace-absence checks for the `v2.6.1` Git tag,
   GitLab Release, generic package, and OCI tag;
5. require the pre-OCI asset build and verification gate for the exact v2.6.1
   mandatory-target set before any real tag pipeline; and
6. preserve the complete standard release-via-merge-request validation,
   Gate 2, immutable-tag, and post-publication verification requirements.

Current evidence at decision time is `origin/main`
`c39997b342dcc0fbd25e9f48e05a54c470aeaec4`, whose push pipeline
`2764286116` passed. This is evidence for the selected direction, not a
permanent binding of the implementation base; the next plan revision must bind
the exact then-proposed SHA and revalidate it.

## Explicit authority boundary

Path A is selected because the required functional and corrective commits are
already on `main` and the standard workflow is proportionate. The selection
does **not** assume accumulated `main` content is clean merely because it was
merged previously. Cleanliness and release suitability remain evidence claims
to be established by the exact revision, consultation, and validation above.

This record does not itself approve MR !1657 at Gate 1, create or authorize a
release branch, waive consultation, approve Gate 2 or a merge, create a tag or
namespace object, or authorize publication. Those decisions remain at their
governed gates.
